use chia_protocol::{Bytes32, SpendBundle};
use chia_puzzles::offer::SettlementPaymentsSolution;
use chia_sdk_driver::{CatLayer, Layer, NftInfo, Puzzle};
use chia_traits::Streamable;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::ToTreeHash;
use clvmr::{Allocator, NodePtr};
use indexmap::IndexMap;

use crate::{
    compress_offer_bytes, decode_offer_data, decompress_offer_bytes, encode_offer_data, Make,
    OfferBuilder, OfferError, ParsedOffer, Take,
};

#[derive(Debug, Clone)]
pub struct Offer {
    spend_bundle: SpendBundle,
}

impl Offer {
    pub fn new(spend_bundle: SpendBundle) -> Self {
        Self { spend_bundle }
    }

    pub fn build(coin_ids: Vec<Bytes32>) -> OfferBuilder<Make> {
        Self::build_with_nonce(Self::nonce(coin_ids))
    }

    pub fn build_with_nonce(nonce: Bytes32) -> OfferBuilder<Make> {
        OfferBuilder::new(nonce)
    }

    pub fn nonce(mut coin_ids: Vec<Bytes32>) -> Bytes32 {
        coin_ids.sort();
        coin_ids.tree_hash().into()
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, OfferError> {
        Ok(self.spend_bundle.to_bytes()?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, OfferError> {
        Ok(SpendBundle::from_bytes(bytes)?.into())
    }

    pub fn compress(&self) -> Result<Vec<u8>, OfferError> {
        compress_offer_bytes(&self.to_bytes()?)
    }

    pub fn decompress(bytes: &[u8]) -> Result<Self, OfferError> {
        Self::from_bytes(&decompress_offer_bytes(bytes)?)
    }

    pub fn encode(&self) -> Result<String, OfferError> {
        encode_offer_data(&self.compress()?)
    }

    pub fn decode(text: &str) -> Result<Self, OfferError> {
        Self::decompress(&decode_offer_data(text)?)
    }

    pub fn take(self, allocator: &mut Allocator) -> Result<OfferBuilder<Take>, OfferError> {
        Ok(self.parse(allocator)?.take())
    }

    pub fn parse(self, allocator: &mut Allocator) -> Result<ParsedOffer, OfferError> {
        let mut parsed = ParsedOffer {
            aggregated_signature: self.spend_bundle.aggregated_signature,
            coin_spends: Vec::new(),
            requested_payments: IndexMap::new(),
        };

        for coin_spend in self.spend_bundle.coin_spends {
            if coin_spend.coin.parent_coin_info != Bytes32::default() {
                parsed.coin_spends.push(coin_spend);
                continue;
            }

            if coin_spend.coin.amount != 0 {
                parsed.coin_spends.push(coin_spend);
                continue;
            }

            let solution = coin_spend.solution.to_clvm(allocator)?;
            let settlement_solution = SettlementPaymentsSolution::from_clvm(allocator, solution)?;

            let puzzle = coin_spend.puzzle_reveal.to_clvm(allocator)?;

            let puzzle = Puzzle::parse(allocator, puzzle);

            let mut asset_id = Bytes32::default();

            if let Ok(Some(cat_layer)) = CatLayer::<NodePtr>::parse_puzzle(allocator, puzzle) {
                asset_id = cat_layer.asset_id;
            } else if let Ok(Some(nft)) = NftInfo::<NodePtr>::parse(allocator, puzzle) {
                asset_id = nft.0.launcher_id;
            }

            parsed
                .requested_payments
                .entry(asset_id)
                .or_insert_with(|| (puzzle, Vec::new()))
                .1
                .extend(settlement_solution.notarized_payments);
        }

        Ok(parsed)
    }
}

impl From<SpendBundle> for Offer {
    fn from(spend_bundle: SpendBundle) -> Self {
        Self::new(spend_bundle)
    }
}

impl From<Offer> for SpendBundle {
    fn from(offer: Offer) -> Self {
        offer.spend_bundle
    }
}

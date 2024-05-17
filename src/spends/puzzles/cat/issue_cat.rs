use chia_bls::PublicKey;
use chia_protocol::{Bytes32, Coin, CoinSpend};
use chia_puzzles::{
    cat::{CatArgs, CatSolution, CoinProof, EverythingWithSignatureTailArgs, CAT_PUZZLE_HASH},
    LineageProof,
};
use clvm_traits::clvm_quote;
use clvm_utils::CurriedProgram;
use clvmr::NodePtr;

use crate::{ChainedSpend, CreateCoinWithMemos, RunTail, SpendContext, SpendError};

pub struct IssueCat {
    parent_coin_id: Bytes32,
    conditions: Vec<NodePtr>,
}

pub struct CatIssuanceInfo {
    pub asset_id: Bytes32,
    pub lineage_proof: LineageProof,
    pub eve_coin: Coin,
}

impl IssueCat {
    pub fn new(parent_coin_id: Bytes32) -> Self {
        Self {
            parent_coin_id,
            conditions: Vec::new(),
        }
    }

    pub fn condition(mut self, condition: NodePtr) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn conditions(mut self, conditions: impl IntoIterator<Item = NodePtr>) -> Self {
        self.conditions.extend(conditions);
        self
    }

    pub fn multi_issuance(
        self,
        ctx: &mut SpendContext,
        public_key: PublicKey,
        amount: u64,
    ) -> Result<(ChainedSpend, CatIssuanceInfo), SpendError> {
        let tail_puzzle_ptr = ctx.everything_with_signature_tail_puzzle()?;

        let tail = ctx.alloc(CurriedProgram {
            program: tail_puzzle_ptr,
            args: EverythingWithSignatureTailArgs { public_key },
        })?;
        let asset_id = ctx.tree_hash(tail).into();

        self.condition(ctx.alloc(RunTail {
            program: tail,
            solution: NodePtr::NIL,
        })?)
        .finish_raw(ctx, asset_id, amount)
    }

    pub fn finish_raw(
        self,
        ctx: &mut SpendContext,
        asset_id: Bytes32,
        amount: u64,
    ) -> Result<(ChainedSpend, CatIssuanceInfo), SpendError> {
        let cat_puzzle_ptr = ctx.cat_puzzle()?;

        let inner_puzzle = ctx.alloc(clvm_quote!(self.conditions))?;
        let inner_puzzle_hash = ctx.tree_hash(inner_puzzle).into();

        let puzzle = ctx.alloc(CurriedProgram {
            program: cat_puzzle_ptr,
            args: CatArgs {
                mod_hash: CAT_PUZZLE_HASH.into(),
                tail_program_hash: asset_id,
                inner_puzzle,
            },
        })?;

        let puzzle_hash = ctx.tree_hash(puzzle).into();
        let eve_coin = Coin::new(self.parent_coin_id, puzzle_hash, amount);

        let solution = ctx.serialize(CatSolution {
            inner_puzzle_solution: (),
            lineage_proof: None,
            prev_coin_id: eve_coin.coin_id(),
            this_coin_info: eve_coin,
            next_coin_proof: CoinProof {
                parent_coin_info: self.parent_coin_id,
                inner_puzzle_hash,
                amount,
            },
            prev_subtotal: 0,
            extra_delta: 0,
        })?;

        let puzzle_reveal = ctx.serialize(puzzle)?;
        ctx.spend(CoinSpend::new(eve_coin, puzzle_reveal, solution));

        let chained_spend = ChainedSpend {
            parent_conditions: vec![ctx.alloc(CreateCoinWithMemos {
                puzzle_hash,
                amount,
                memos: vec![puzzle_hash.to_vec().into()],
            })?],
        };

        let issuance_info = CatIssuanceInfo {
            asset_id,
            lineage_proof: LineageProof {
                parent_parent_coin_id: eve_coin.parent_coin_info,
                parent_inner_puzzle_hash: inner_puzzle_hash,
                parent_amount: eve_coin.amount,
            },
            eve_coin,
        };

        Ok((chained_spend, issuance_info))
    }
}

#[cfg(test)]
mod tests {
    use clvmr::Allocator;

    use crate::{test::TestWallet, Chainable, CreateCoinWithMemos, StandardSpend};

    use super::*;

    #[tokio::test]
    async fn test_cat_issuance() -> anyhow::Result<()> {
        let mut allocator = Allocator::new();
        let mut wallet = TestWallet::new(&mut allocator, 1).await;
        let ctx = &mut wallet.ctx;

        let (issue_cat, _cat_info) = IssueCat::new(wallet.coin.coin_id())
            .condition(ctx.alloc(CreateCoinWithMemos {
                puzzle_hash: wallet.puzzle_hash,
                amount: 1,
                memos: vec![wallet.puzzle_hash.to_vec().into()],
            })?)
            .multi_issuance(ctx, wallet.pk, 1)?;

        StandardSpend::new()
            .chain(issue_cat)
            .finish(ctx, wallet.coin, wallet.pk)?;

        wallet.submit().await?;

        Ok(())
    }
}
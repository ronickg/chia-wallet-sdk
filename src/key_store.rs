use chia_bls::{PublicKey, Signature};
use chia_protocol::CoinSpend;
use clvm_traits::Result;
use clvmr::Allocator;

mod secret_key_store;

pub use secret_key_store::*;

pub trait KeyStore: Send + Sync {
    fn next_derivation_index(&self) -> u32;
    fn derive_keys(&mut self, count: u32);
    fn public_key(&self, index: u32) -> PublicKey;

    fn derive_keys_until(&mut self, index: u32) {
        if index < self.next_derivation_index() {
            return;
        }
        self.derive_keys(index - self.next_derivation_index() + 1);
    }
}

pub trait Signer {
    fn sign_message(&self, index: u32, message: &[u8]) -> Signature;

    fn partial_sign_coin_spends(
        &self,
        allocator: &mut Allocator,
        coin_spends: &[CoinSpend],
        agg_sig_me_extra_data: [u8; 32],
    ) -> Result<Signature>;
}

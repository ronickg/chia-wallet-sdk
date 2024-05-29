use chia_bls::PublicKey;
use chia_protocol::Bytes32;
use chia_puzzles::{
    did::{DidArgs, DID_INNER_PUZZLE_HASH},
    singleton::SingletonStruct,
    standard::{StandardArgs, STANDARD_PUZZLE_HASH},
    EveProof, Proof,
};
use chia_sdk_types::puzzles::DidInfo;
use clvm_traits::ToClvm;
use clvm_utils::{tree_hash_atom, CurriedProgram, ToTreeHash, TreeHash};
use clvmr::NodePtr;

use crate::{puzzles::SpendableLauncher, spend_builder::SpendConditions, SpendContext, SpendError};

use super::StandardDidSpend;

pub trait CreateDid {
    fn create_eve_did<M>(
        self,
        ctx: &mut SpendContext<'_>,
        inner_puzzle_hash: Bytes32,
        recovery_did_list_hash: Bytes32,
        num_verifications_required: u64,
        metadata: M,
    ) -> Result<(SpendConditions, DidInfo<M>), SpendError>
    where
        M: ToClvm<NodePtr>;

    fn create_custom_standard_did<M>(
        self,
        ctx: &mut SpendContext<'_>,
        recovery_did_list_hash: Bytes32,
        num_verifications_required: u64,
        metadata: M,
        synthetic_key: PublicKey,
    ) -> Result<(SpendConditions, DidInfo<M>), SpendError>
    where
        M: ToClvm<NodePtr>,
        Self: Sized,
    {
        let inner_puzzle_hash = CurriedProgram {
            program: STANDARD_PUZZLE_HASH,
            args: StandardArgs { synthetic_key },
        }
        .tree_hash()
        .into();

        let (create_did, did_info) = self.create_eve_did(
            ctx,
            inner_puzzle_hash,
            recovery_did_list_hash,
            num_verifications_required,
            metadata,
        )?;

        let did_info = StandardDidSpend::new()
            .recreate()
            .finish(ctx, synthetic_key, did_info)?;

        Ok((create_did, did_info))
    }

    fn create_standard_did(
        self,
        ctx: &mut SpendContext<'_>,
        synthetic_key: PublicKey,
    ) -> Result<(SpendConditions, DidInfo<()>), SpendError>
    where
        Self: Sized,
    {
        self.create_custom_standard_did(ctx, tree_hash_atom(&[]).into(), 1, (), synthetic_key)
    }
}

impl CreateDid for SpendableLauncher {
    fn create_eve_did<M>(
        self,
        ctx: &mut SpendContext<'_>,
        p2_puzzle_hash: Bytes32,
        recovery_did_list_hash: Bytes32,
        num_verifications_required: u64,
        metadata: M,
    ) -> Result<(SpendConditions, DidInfo<M>), SpendError>
    where
        M: ToClvm<NodePtr>,
    {
        let metadata_ptr = ctx.alloc(&metadata)?;
        let metadata_hash = ctx.tree_hash(metadata_ptr);

        let did_inner_puzzle_hash = CurriedProgram {
            program: DID_INNER_PUZZLE_HASH,
            args: DidArgs {
                inner_puzzle: TreeHash::from(p2_puzzle_hash),
                recovery_did_list_hash,
                num_verifications_required,
                metadata: metadata_hash,
                singleton_struct: SingletonStruct::new(self.coin().coin_id()),
            },
        }
        .tree_hash()
        .into();

        let launcher_coin = self.coin();
        let (chained_spend, eve_coin) = self.spend(ctx, did_inner_puzzle_hash, ())?;

        let proof = Proof::Eve(EveProof {
            parent_coin_info: launcher_coin.parent_coin_info,
            amount: launcher_coin.amount,
        });

        let did_info = DidInfo {
            launcher_id: launcher_coin.coin_id(),
            coin: eve_coin,
            did_inner_puzzle_hash,
            p2_puzzle_hash,
            proof,
            recovery_did_list_hash,
            num_verifications_required,
            metadata,
        };

        Ok((chained_spend, did_info))
    }
}
use crate::{ffi::Output, Allocate, ClvmValue, Program};
use chia::{
    bls,
    clvm_traits::{clvm_quote, ClvmEncoder, FromClvm, ToClvm},
    clvm_utils::{self, CurriedProgram, TreeHash},
    protocol::{self, Bytes32},
    puzzles::nft::{self, NFT_METADATA_UPDATER_PUZZLE_HASH},
};
use chia_wallet_sdk::{self as sdk, HashedPtr, SpendContext};
use clvmr::{
    run_program,
    serde::{node_from_bytes, node_from_bytes_backrefs},
    ChiaDialect, NodePtr, MEMPOOL_MODE,
};

pub struct ClvmAllocator(pub(crate) SpendContext);

pub fn new_clvm() -> Box<ClvmAllocator> {
    Box::new(ClvmAllocator(SpendContext::new()))
}

// pub struct Output {
//     pub cost: u64,
//     pub value: Box<Program>,
// }

impl ClvmAllocator {
    pub fn new() -> Box<Self> {
        Box::new(Self(SpendContext::new()))
    }

    pub fn nil(&mut self) -> Box<Program> {
        Box::new(Program::new(NodePtr::NIL))
    }

    pub fn deserialize(&mut self, value: Vec<u8>) -> Result<Box<Program>, String> {
        let ptr = node_from_bytes(&mut self.0.allocator, &value).map_err(|e| e.to_string())?;
        Ok(Box::new(Program::new(ptr)))
    }

    pub fn deserialize_with_backrefs(
        &mut self,
        value: Vec<u8>, // Accepts a byte slice
    ) -> Result<Box<Program>, String> {
        let ptr =
            node_from_bytes_backrefs(&mut self.0.allocator, &value).map_err(|e| e.to_string())?;
        Ok(Box::new(Program::new(ptr)))
    }

    pub fn tree_hash(&self, program: &Program) -> Result<[u8; 32], String> {
        let hash = self.0.tree_hash(program.ptr);
        Ok(hash.to_bytes())
    }

    pub fn run(
        &mut self,
        puzzle: &Program,
        solution: &Program,
        max_cost: u64,
        mempool_mode: bool,
    ) -> Result<Output, String> {
        let mut flags = 0;

        if mempool_mode {
            flags |= MEMPOOL_MODE;
        }

        let result = run_program(
            &mut self.0.allocator,
            &ChiaDialect::new(flags),
            puzzle.ptr,
            solution.ptr,
            max_cost,
        )
        .map_err(|error| error.to_string())?;

        Ok(Output {
            value: Box::new(Program::new(result.1)),
            cost: result.0,
        })
    }

    pub fn curry(&mut self, program: &Program, args: Vec<Program>) -> Result<Box<Program>, String> {
        let mut args_ptr = self.0.allocator.one();

        for arg in args.into_iter().rev() {
            args_ptr = self
                .0
                .allocator
                .encode_curried_arg(arg.ptr, args_ptr)
                .map_err(|error| error.to_string())?;
        }

        let ptr = self
            .0
            .alloc(&CurriedProgram {
                program: program.ptr,
                args: args_ptr,
            })
            .map_err(|error| error.to_string())?;

        Ok(Box::new(Program::new(ptr)))
    }

    pub fn pair(&mut self, first: &ClvmValue, rest: &ClvmValue) -> Result<Box<Program>, String> {
        let first_ptr = first
            .allocate(&mut self.0.allocator)
            .map_err(|e| e.to_string())?;
        let rest_ptr = rest
            .allocate(&mut self.0.allocator)
            .map_err(|e| e.to_string())?;

        let pair_ptr = self
            .0
            .allocator
            .new_pair(first_ptr, rest_ptr)
            .map_err(|e| e.to_string())?;

        Ok(Box::new(Program::new(pair_ptr)))
    }

    pub fn alloc(&mut self, value: &ClvmValue) -> Result<Box<Program>, String> {
        let ptr = value
            .allocate(&mut self.0.allocator)
            .map_err(|e| e.to_string())?;
        Ok(Box::new(Program::new(ptr)))
    }
}

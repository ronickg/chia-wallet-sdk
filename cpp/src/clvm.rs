use chia::{
    clvm_traits::{ClvmDecoder, FromClvm},
    clvm_utils::{tree_hash, CurriedProgram},
};
use chia_wallet_sdk::SpendContext;
use clvmr::{
    serde::{node_from_bytes, node_from_bytes_backrefs, node_to_bytes, node_to_bytes_backrefs},
    Allocator, NodePtr,
};

use crate::{NodePtrWrapper, Program};

pub struct ClvmAllocator(pub(crate) SpendContext);

impl ClvmAllocator {
    pub fn new() -> Result<Self, String> {
        Ok(Self(SpendContext::new()))
    }

    pub fn get_allocator(&mut self) -> &mut Allocator {
        &mut self.0.allocator
    }
}

// Implementation of the extern functions for cxx
pub fn deserialize(allocator: &mut ClvmAllocator, bytes: &[u8]) -> Result<Box<Program>, String> {
    match node_from_bytes(&mut allocator.0.allocator, bytes) {
        Ok(ptr) => Ok(Box::new(Program {
            ctx: Box::new(ClvmAllocator(SpendContext::new())),
            ptr: NodePtrWrapper::from(ptr),
        })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn nil(allocator: &mut ClvmAllocator) -> Result<Box<Program>, String> {
    Ok(Box::new(Program {
        ctx: Box::new(ClvmAllocator(SpendContext::new())),
        ptr: NodePtrWrapper::from(NodePtr::NIL),
    }))
}

pub fn deserialize_with_backrefs(
    allocator: &mut ClvmAllocator,
    bytes: &[u8],
) -> Result<Box<Program>, String> {
    match node_from_bytes_backrefs(&mut allocator.0.allocator, bytes) {
        Ok(ptr) => Ok(Box::new(Program {
            ctx: Box::new(ClvmAllocator(SpendContext::new())),
            ptr: NodePtrWrapper::from(ptr),
        })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn create_allocator() -> Box<ClvmAllocator> {
    Box::new(ClvmAllocator::new().unwrap())
}

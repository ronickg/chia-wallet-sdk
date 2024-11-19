use std::{num::TryFromIntError, string::FromUtf8Error};

use chia::{
    clvm_traits::{ClvmDecoder, FromClvm},
    clvm_utils::{tree_hash, CurriedProgram},
};
use clvmr::{
    serde::{node_to_bytes, node_to_bytes_backrefs},
    Allocator, NodePtr, SExp,
};

use crate::types::OptionalVec;
use crate::ClvmAllocator;

// Wrapper type for NodePtr
#[derive(Clone, Copy)]
pub struct NodePtrWrapper(NodePtr);

impl From<NodePtr> for NodePtrWrapper {
    fn from(ptr: NodePtr) -> Self {
        NodePtrWrapper(ptr)
    }
}

impl From<NodePtrWrapper> for NodePtr {
    fn from(wrapper: NodePtrWrapper) -> Self {
        wrapper.0
    }
}

pub struct Program {
    pub(crate) ctx: Box<ClvmAllocator>, // Rust-managed allocator
    pub(crate) ptr: NodePtrWrapper,
}

// Implement the functions at module level to match the FFI declarations
pub fn new_program(allocator: Box<ClvmAllocator>, ptr: &NodePtrWrapper) -> Box<Program> {
    Box::new(Program {
        ctx: allocator,
        ptr: *ptr,
    })
}

impl Program {
    pub fn is_atom(&self) -> bool {
        NodePtr::from(self.ptr).is_atom()
    }

    pub fn is_pair(&self) -> bool {
        NodePtr::from(self.ptr).is_pair()
    }

    pub fn tree_hash(&self) -> Result<Vec<u8>, String> {
        Ok(tree_hash(&self.ctx.0.allocator, self.ptr.into())
            .to_bytes()
            .to_vec())
    }

    pub fn serialize(&self) -> Result<Vec<u8>, String> {
        node_to_bytes(&self.ctx.0.allocator, self.ptr.into()).map_err(|e| e.to_string())
    }

    pub fn to_atom(&self) -> Result<OptionalVec, String> {
        match self.ctx.0.allocator.sexp(NodePtr::from(self.ptr)) {
            SExp::Atom => Ok(OptionalVec {
                has_value: true,
                data: self.ctx.0.allocator.atom(self.ptr.into()).as_ref().to_vec(),
            }),
            _ => Ok(OptionalVec {
                has_value: false,
                data: Vec::new(),
            }),
        }
    }
}

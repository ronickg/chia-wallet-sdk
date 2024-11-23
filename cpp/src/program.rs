use std::{num::TryFromIntError, string::FromUtf8Error};

use chia::{
    clvm_traits::{ClvmDecoder, FromClvm},
    clvm_utils::{tree_hash, CurriedProgram},
};
use clvmr::{
    serde::{node_to_bytes, node_to_bytes_backrefs},
    Allocator, NodePtr, SExp,
};

// use crate::ffi::OptionalVec; // Add this line
use crate::ClvmAllocator;

pub struct Program {
    pub(crate) ptr: NodePtr,
}

impl Program {
    pub fn new(ptr: NodePtr) -> Self {
        Self { ptr }
    }

    pub fn is_atom(&self) -> bool {
        self.ptr.is_atom()
    }

    pub fn is_pair(&self) -> bool {
        self.ptr.is_pair()
    }

    pub fn to_string(&self, allocator: &ClvmAllocator) -> Result<String, String> {
        match allocator.0.allocator.sexp(self.ptr) {
            SExp::Atom => {
                let bytes = allocator.0.allocator.atom(self.ptr).as_ref().to_vec();
                String::from_utf8(bytes).map_err(|e| e.to_string())
            }
            SExp::Pair(..) => Err("Cannot convert pair to string".to_string()),
        }
    }

    // pub fn to_bytes(&self, allocator: &ClvmAllocator) -> Result<Vec<u8>, String> {
    //     node_to_bytes(allocator.0.allocator, self.ptr).map_err(|e| e.to_string())
    // }

    // #[napi]
    // pub fn to_atom(&self) -> Result<Option<Uint8Array>> {
    //     match self.alloc().sexp(self.ptr) {
    //         SExp::Atom => Ok(Some(
    //             self.alloc().atom(self.ptr).as_ref().to_vec().into_js()?,
    //         )),
    //         SExp::Pair(..) => Ok(None),
    //     }
    // }
    pub fn to_atom(&self, allocator: &ClvmAllocator) -> Result<Vec<u8>, String> {
        match allocator.0.allocator.sexp(self.ptr) {
            SExp::Atom => {
                let bytes = allocator.0.allocator.atom(self.ptr).as_ref().to_vec();
                Ok(bytes)
            }
            SExp::Pair(..) => Err("Cannot convert pair to atom".to_string()),
        }
    }
}

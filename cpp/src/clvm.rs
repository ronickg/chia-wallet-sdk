use crate::{Allocate, ClvmValue, Program};
use chia::{
    clvm_traits::{ClvmDecoder, FromClvm},
    clvm_utils::{tree_hash, CurriedProgram},
};
use chia_wallet_sdk::SpendContext;
use clvmr::{
    serde::{node_from_bytes, node_from_bytes_backrefs, node_to_bytes, node_to_bytes_backrefs},
    Allocator, NodePtr,
};
pub struct ClvmAllocator(pub(crate) SpendContext);

pub fn clvm_new_allocator() -> Box<ClvmAllocator> {
    Box::new(ClvmAllocator(SpendContext::new()))
}

impl ClvmAllocator {
    pub fn new_allocator() -> Box<Self> {
        Box::new(Self(SpendContext::new()))
    }

    pub fn nil(&mut self) -> Box<Program> {
        Box::new(Program::new(NodePtr::NIL))
    }

    pub fn deserialize(&mut self, value: &[u8]) -> Result<Box<Program>, String> {
        let ptr = node_from_bytes(&mut self.0.allocator, value).map_err(|e| e.to_string())?;
        Ok(Box::new(Program::new(ptr)))
    }

    pub fn alloc(&mut self, value: &ClvmValue) -> Result<Box<Program>, String> {
        let ptr = value
            .allocate(&mut self.0.allocator)
            .map_err(|e| e.to_string())?;
        Ok(Box::new(Program::new(ptr)))
    }
}

// use chia::{
//     clvm_traits::{ClvmDecoder, FromClvm},
//     clvm_utils::{tree_hash, CurriedProgram},
// };
// use chia_wallet_sdk::SpendContext;
// use clvmr::{
//     serde::{node_from_bytes, node_from_bytes_backrefs, node_to_bytes, node_to_bytes_backrefs},
//     Allocator, NodePtr,
// };

// use crate::{clvm_value::Allocate, ffi::ClvmResult, ClvmValue, NodePtrWrapper, Program};

// pub struct ClvmAllocator(pub(crate) SpendContext);

// impl ClvmAllocator {
//     pub fn new() -> Result<Self, String> {
//         Ok(Self(SpendContext::new()))
//     }

//     pub fn get_allocator(&mut self) -> &mut Allocator {
//         &mut self.0.allocator
//     }
// }

// // Implementation of the extern functions for cxx
// pub fn deserialize(allocator: &mut ClvmAllocator, bytes: &[u8]) -> Result<Box<Program>, String> {
//     match node_from_bytes(&mut allocator.0.allocator, bytes) {
//         Ok(ptr) => Ok(Box::new(Program {
//             ctx: Box::new(ClvmAllocator(SpendContext::new())),
//             ptr: NodePtrWrapper::from(ptr),
//         })),
//         Err(e) => Err(e.to_string()),
//     }
// }

// pub fn nil(allocator: &mut ClvmAllocator) -> Result<Box<Program>, String> {
//     Ok(Box::new(Program {
//         ctx: Box::new(ClvmAllocator(SpendContext::new())),
//         ptr: NodePtrWrapper::from(NodePtr::NIL),
//     }))
// }

// pub fn alloc(allocator: &mut ClvmAllocator, value: &ClvmValue) -> Result<Box<Program>, String> {
//     match value.allocate(allocator) {
//         Ok(ptr) => Ok(Box::new(Program {
//             ctx: allocator, // Use the passed allocator directly
//             ptr,
//         })),
//         Err(e) => Err(e.to_string()),
//     }
// }
// // pub fn alloc(allocator: &mut ClvmAllocator, value: &ClvmValue) -> ClvmResult {
// //     match value.allocate(allocator) {
// //         Ok(ptr) => ClvmResult {
// //             success: true,
// //             error: String::new(),
// //             node_ptr_value: Box::new(ptr),
// //         },
// //         Err(e) => ClvmResult {
// //             success: false,
// //             error: e.to_string(),
// //             node_ptr_value: Box::new(NodePtrWrapper::from(NodePtr::NIL)),
// //         },
// //     }
// // }

// pub fn deserialize_with_backrefs(
//     allocator: &mut ClvmAllocator,
//     bytes: &[u8],
// ) -> Result<Box<Program>, String> {
//     match node_from_bytes_backrefs(&mut allocator.0.allocator, bytes) {
//         Ok(ptr) => Ok(Box::new(Program {
//             ctx: Box::new(ClvmAllocator(SpendContext::new())),
//             ptr: NodePtrWrapper::from(ptr),
//         })),
//         Err(e) => Err(e.to_string()),
//     }
// }

// pub fn create_allocator() -> Box<ClvmAllocator> {
//     Box::new(ClvmAllocator::new().unwrap())
// }

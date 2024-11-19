use chia::clvm_traits::ToClvm;
use chia_wallet_sdk::SpendContext;
use clvmr::{
    serde::{node_from_bytes, node_to_bytes, node_to_bytes_backrefs},
    Allocator, NodePtr, SExp,
};

pub mod clvm;
pub mod clvm_value;
// pub mod coin;
pub mod program;

pub use clvm::*;
pub use clvm_value::*;
// pub use coin::*;
pub use program::*;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type ClvmAllocator;
        type Program;
        type ClvmValue;

        fn clvm_new_allocator() -> Box<ClvmAllocator>;
        fn nil(self: &mut ClvmAllocator) -> Box<Program>;
        fn deserialize(self: &mut ClvmAllocator, value: &[u8]) -> Result<Box<Program>>;
        fn alloc(self: &mut ClvmAllocator, value: &ClvmValue) -> Result<Box<Program>>;

        fn new_string_value(value: String) -> Box<ClvmValue>;

        fn is_atom(self: &Program) -> bool;
        fn is_pair(self: &Program) -> bool;
        fn to_string(self: &Program, allocator: &ClvmAllocator) -> Result<String>;
    }
}

// pub struct ClvmAllocator(pub(crate) SpendContext);

// pub fn clvm_new_allocator() -> Box<ClvmAllocator> {
//     Box::new(ClvmAllocator(SpendContext::new()))
// }

// impl ClvmAllocator {
//     pub fn new_allocator() -> Box<Self> {
//         Box::new(Self(SpendContext::new()))
//     }

//     fn nil(&mut self) -> Box<Program> {
//         Box::new(Program::new(NodePtr::NIL))
//     }

//     fn deserialize(&mut self, value: &[u8]) -> Result<Box<Program>, String> {
//         let ptr = node_from_bytes(&mut self.0.allocator, value).map_err(|e| e.to_string())?;
//         Ok(Box::new(Program::new(ptr)))
//     }

//     fn alloc(&mut self, value: &ClvmValue) -> Result<Box<Program>, String> {
//         let ptr = value
//             .allocate(&mut self.0.allocator)
//             .map_err(|e| e.to_string())?;
//         Ok(Box::new(Program::new(ptr)))
//     }
// }

// pub struct Program {
//     pub(crate) ptr: NodePtr,
// }

// impl Program {
//     pub fn new(ptr: NodePtr) -> Self {
//         Self { ptr }
//     }

//     fn is_atom(&self) -> bool {
//         self.ptr.is_atom()
//     }

//     fn is_pair(&self) -> bool {
//         self.ptr.is_pair()
//     }

//     fn to_string(&self, allocator: &ClvmAllocator) -> Result<String, String> {
//         match allocator.0.allocator.sexp(self.ptr) {
//             SExp::Atom => {
//                 let bytes = allocator.0.allocator.atom(self.ptr).as_ref().to_vec();
//                 String::from_utf8(bytes).map_err(|e| e.to_string())
//             }
//             SExp::Pair(..) => Err("Cannot convert pair to string".to_string()),
//         }
//     }
// }

// pub enum ClvmValue {
//     Float(f64),
//     Integer(u64),
//     String(String),
//     Bool(bool),
//     Program(Program),
//     Bytes(Vec<u8>),
//     Array(Vec<ClvmValue>),
// }

// pub(crate) trait Allocate {
//     fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String>;
// }

// impl Allocate for ClvmValue {
//     fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
//         match self {
//             ClvmValue::Float(f) => f.allocate(allocator),
//             ClvmValue::Integer(i) => i.allocate(allocator),
//             ClvmValue::String(s) => s.allocate(allocator),
//             ClvmValue::Bool(b) => b.allocate(allocator),
//             ClvmValue::Bytes(b) => b.allocate(allocator),
//             ClvmValue::Array(arr) => arr.allocate(allocator),
//             ClvmValue::Program(prog) => prog.allocate(allocator),
//         }
//     }
// }

// fn new_string_value(value: String) -> Box<ClvmValue> {
//     Box::new(ClvmValue::String(value))
// }

// impl Allocate for f64 {
//     fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
//         if self.is_infinite() {
//             return Err("Value is infinite".to_string());
//         }

//         if self.is_nan() {
//             return Err("Value is NaN".to_string());
//         }

//         if self.fract() != 0.0 {
//             return Err("Value has a fractional part".to_string());
//         }

//         if *self > 9_007_199_254_740_991.0 {
//             return Err("Value is larger than MAX_SAFE_INTEGER".to_string());
//         }

//         if *self < -9_007_199_254_740_991.0 {
//             return Err("Value is smaller than MIN_SAFE_INTEGER".to_string());
//         }

//         let value = *self as i64;

//         if (0..=67_108_863).contains(&value) {
//             allocator
//                 .new_small_number(value as u32)
//                 .map_err(|e| e.to_string())
//         } else {
//             allocator
//                 .new_number(value.into())
//                 .map_err(|e| e.to_string())
//         }
//     }
// }

// impl Allocate for u64 {
//     fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
//         allocator
//             .new_number((*self).into())
//             .map_err(|e| e.to_string())
//     }
// }

// impl Allocate for String {
//     fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
//         allocator
//             .new_atom(self.as_bytes())
//             .map_err(|e| e.to_string())
//     }
// }

// impl Allocate for bool {
//     fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
//         allocator
//             .new_small_number(u32::from(*self))
//             .map_err(|e| e.to_string())
//     }
// }

// impl Allocate for Vec<u8> {
//     fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
//         allocator.new_atom(self).map_err(|e| e.to_string())
//     }
// }

// impl Allocate for Vec<ClvmValue> {
//     fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
//         let mut items: Vec<NodePtr> = Vec::with_capacity(self.len());

//         for item in self {
//             let node_ptr = item.allocate(allocator)?;
//             items.push(node_ptr.into());
//         }

//         items.to_clvm(allocator).map_err(|e| e.to_string())
//     }
// }

// impl Allocate for Program {
//     fn allocate(&self, _allocator: &mut Allocator) -> Result<NodePtr, String> {
//         Ok(self.ptr)
//     }
// }

// pub fn allocate_value(value: &ClvmValue, allocator: &mut Allocator) -> Result<NodePtr, String> {
//     value.allocate(allocator)
// }

// // Implementation of the C++ interface
// impl ClvmValue {
//     pub fn from_bytes(bytes: &[u8]) -> Result<Box<ClvmValue>, String> {
//         Ok(Box::new(ClvmValue::Bytes(bytes.to_vec())))
//     }

//     pub fn from_string(s: &str) -> Result<Box<ClvmValue>, String> {
//         Ok(Box::new(ClvmValue::String(s.to_string())))
//     }

//     pub fn from_bool(b: bool) -> Result<Box<ClvmValue>, String> {
//         Ok(Box::new(ClvmValue::Bool(b)))
//     }

//     pub fn from_float(f: f64) -> Result<Box<ClvmValue>, String> {
//         Ok(Box::new(ClvmValue::Float(f)))
//     }

//     pub fn from_int(i: u64) -> Result<Box<ClvmValue>, String> {
//         Ok(Box::new(ClvmValue::Integer(i)))
//     }
// }

// // Module-level functions for FFI
// pub fn from_bytes(bytes: &[u8]) -> Result<Box<ClvmValue>, String> {
//     ClvmValue::from_bytes(bytes)
// }

// pub fn from_string(s: &str) -> Result<Box<ClvmValue>, String> {
//     ClvmValue::from_string(s)
// }

// pub fn from_bool(b: bool) -> Result<Box<ClvmValue>, String> {
//     ClvmValue::from_bool(b)
// }

// pub fn from_float(f: f64) -> Result<Box<ClvmValue>, String> {
//     ClvmValue::from_float(f)
// }

// pub fn from_int(i: i64) -> Result<Box<ClvmValue>, String> {
//     ClvmValue::from_int(i as u64)
// }

// use chia::protocol::{Bytes32, Coin as ProtocolCoin};
// // use cxx;
// use crate::NodePtrWrapper;
// use clvmr::{Allocator, NodePtr};

// pub mod clvm;
// pub mod clvm_value;
// pub mod coin;
// pub mod program;

// pub use clvm::*;
// pub use clvm_value::*;
// pub use coin::*;
// pub use program::*;

// // Single CXX bridge for the entire crate
// #[cxx::bridge]
// pub mod ffi {

//     pub struct OptionalVec {
//         pub has_value: bool,
//         pub data: Vec<u8>,
//     }

//     #[derive(Debug, Clone)]
//     struct CBytes32 {
//         bytes: [u8; 32],
//     }

//     #[derive(Debug, Clone)]
//     struct Coin {
//         parent_coin_info: CBytes32,
//         puzzle_hash: CBytes32,
//         amount: u64,
//     }

//     struct ClvmResult {
//         success: bool,
//         error: String,
//         node_ptr_value: Box<NodePtrWrapper>,
//     }

//     extern "Rust" {
//         type Program;
//         type ClvmAllocator;
//         type NodePtrWrapper;
//         type ClvmValue;

//         //Utility functions
//         fn from_hex(hex_str: &str) -> Vec<u8>;
//         fn to_hex(bytes: &[u8]) -> String;

//         // Program functions
//         fn new_program(allocator: Box<ClvmAllocator>, ptr: &NodePtrWrapper) -> Box<Program>;
//         fn is_atom(self: &Program) -> bool;
//         fn is_pair(self: &Program) -> bool;
//         fn serialize(self: &Program) -> Result<Vec<u8>>;
//         fn to_atom(self: &Program) -> Result<OptionalVec>;

//         fn create_allocator() -> Box<ClvmAllocator>;
//         fn deserialize(allocator: &mut ClvmAllocator, bytes: &[u8]) -> Result<Box<Program>>;
//         fn deserialize_with_backrefs(
//             allocator: &mut ClvmAllocator,
//             bytes: &[u8],
//         ) -> Result<Box<Program>>;
//         fn nil(allocator: &mut ClvmAllocator) -> Result<Box<Program>>;

//         //Coin
//         fn new_coin(parent_coin_info: CBytes32, puzzle_hash: CBytes32, amount: u64) -> Coin;
//         fn get_coin_id(coin: &Coin) -> CBytes32;

//         fn from_bytes(bytes: &[u8]) -> Result<Box<ClvmValue>>;
//         fn from_string(s: &str) -> Result<Box<ClvmValue>>;
//         fn from_bool(b: bool) -> Result<Box<ClvmValue>>;
//         fn from_float(f: f64) -> Result<Box<ClvmValue>>;
//         fn from_int(i: i64) -> Result<Box<ClvmValue>>;
//         fn allocate_value(value: &ClvmValue, allocator: &mut ClvmAllocator) -> ClvmResult;

//     }
// }
// // use chia::protocol::{Bytes32, Coin as ProtocolCoin};

// // mod program;
// // pub use program::*;
// // mod clvm;
// // pub use clvm::*;

// // #[cxx::bridge]
// // mod ffi {
// //     #[derive(Debug, Clone)]
// //     struct CBytes32 {
// //         bytes: [u8; 32],
// //     }

// // #[derive(Debug, Clone)]
// // struct Coin {
// //     parent_coin_info: CBytes32,
// //     puzzle_hash: CBytes32,
// //     amount: u64,
// // }

// //     extern "Rust" {
// //         fn from_hex(hex_str: &str) -> Vec<u8>;
// //         fn to_hex(bytes: &[u8]) -> String;
// //         fn new_coin(parent_coin_info: CBytes32, puzzle_hash: CBytes32, amount: u64) -> Coin;
// //         fn get_coin_id(coin: &Coin) -> CBytes32;
// //     }
// // }

// // use ffi::*;

// // fn from_hex(hex_str: &str) -> Vec<u8> {
// //     hex::decode(hex_str).unwrap_or_default()
// // }

// // fn to_hex(bytes: &[u8]) -> String {
// //     hex::encode(bytes)
// // }

// // fn to_raw_bytes32(b: &Bytes32) -> CBytes32 {
// //     CBytes32 {
// //         bytes: b.as_ref().try_into().unwrap(),
// //     }
// // }

// // fn from_raw_bytes32(raw: &CBytes32) -> Bytes32 {
// //     Bytes32::new(raw.bytes)
// // }

// // fn new_coin(parent_coin_info: CBytes32, puzzle_hash: CBytes32, amount: u64) -> Coin {
// //     Coin {
// //         parent_coin_info,
// //         puzzle_hash,
// //         amount,
// //     }
// // }

// // fn get_coin_id(coin: &Coin) -> CBytes32 {
// //     let protocol_coin = ProtocolCoin {
// //         parent_coin_info: from_raw_bytes32(&coin.parent_coin_info),
// //         puzzle_hash: from_raw_bytes32(&coin.puzzle_hash),
// //         amount: coin.amount,
// //     };

// //     to_raw_bytes32(&protocol_coin.coin_id())
// // }

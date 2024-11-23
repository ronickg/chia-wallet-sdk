#[no_mangle]
pub extern "C" fn return_one() -> i32 {
    1
}

// use chia::clvm_traits::ToClvm;
// use chia_wallet_sdk::SpendContext;
// use clvmr::{
//     serde::{node_from_bytes, node_to_bytes, node_to_bytes_backrefs},
//     Allocator, NodePtr, SExp,
// };

// use crate::ClvmValueArray;

// pub mod clvm;
// pub mod clvm_value;
// // pub mod coin;
// pub mod program;

// pub use clvm::*;
// pub use clvm_value::*;
// // pub use coin::*;
// pub use program::*;

// // impl ClvmValueArray {
// //     pub fn new(values: Vec<ClvmValue>) -> Self {
// //         ClvmValueArray { values }
// //     }
// // }
// #[cxx::bridge]
// mod ffi {
//     pub struct Output {
//         pub cost: u64,
//         pub value: Box<Program>,
//     }

//     extern "Rust" {
//         type ClvmAllocator;
//         type Program;
//         type ClvmValue;
//         type ClvmValueArray;
//         type ClvmArrayBuilder;

//         //clvm
//         fn new_clvm() -> Box<ClvmAllocator>;
//         fn nil(self: &mut ClvmAllocator) -> Box<Program>;
//         fn deserialize(self: &mut ClvmAllocator, value: Vec<u8>) -> Result<Box<Program>>;
//         fn deserialize_with_backrefs(
//             self: &mut ClvmAllocator,
//             value: Vec<u8>,
//         ) -> Result<Box<Program>>;
//         fn alloc(self: &mut ClvmAllocator, value: &ClvmValue) -> Result<Box<Program>>;
//         fn tree_hash(self: &ClvmAllocator, program: &Program) -> Result<[u8; 32]>;
//         fn run(
//             self: &mut ClvmAllocator,
//             puzzle: &Program,
//             solution: &Program,
//             max_cost: u64,
//             mempool_mode: bool,
//         ) -> Result<Output>;
//         fn curry(
//             self: &mut ClvmAllocator,
//             program: &Program,
//             args: Vec<Program>,
//         ) -> Result<Box<Program>>;

//         fn new_string_value(value: String) -> Box<ClvmValue>;
//         fn new_float_value(value: f64) -> Box<ClvmValue>;
//         fn new_int_value(value: u64) -> Box<ClvmValue>;
//         fn new_bool_value(value: bool) -> Box<ClvmValue>;
//         fn new_bytes_value(value: Vec<u8>) -> Box<ClvmValue>; // Changed from &[u8] to Vec<u8>
//         fn new_program_value(program: Box<Program>) -> Box<ClvmValue>;
//         fn new_array_value(value: Box<ClvmValueArray>) -> Box<ClvmValue>;
//         //clvm_value
//         // Builder methods
//         fn array_builder() -> Box<ClvmArrayBuilder>;
//         fn add_value(self: &mut ClvmArrayBuilder, value: Box<ClvmValue>) -> &mut ClvmArrayBuilder;
//         fn build_from_array(array: Box<ClvmArrayBuilder>) -> Box<ClvmValue>;

//         fn is_atom(self: &Program) -> bool;
//         fn is_pair(self: &Program) -> bool;
//         fn to_string(self: &Program, allocator: &ClvmAllocator) -> Result<String>;
//         fn to_atom(self: &Program, allocator: &ClvmAllocator) -> Result<Vec<u8>>;

//         //utils

//         fn from_hex(hex_str: &str) -> Vec<u8>;
//         fn to_hex(bytes: Vec<u8>) -> String;
//     }
// }

// fn from_hex(hex_str: &str) -> Vec<u8> {
//     hex::decode(hex_str).unwrap_or_default()
// }

// fn to_hex(bytes: Vec<u8>) -> String {
//     hex::encode(bytes)
// }

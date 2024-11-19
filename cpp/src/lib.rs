use chia::protocol::{Bytes32, Coin as ProtocolCoin};
// use cxx;
use crate::NodePtrWrapper;
use clvmr::{Allocator, NodePtr};

pub mod clvm;
pub mod clvm_value;
pub mod coin;
pub mod program;

pub use clvm::*;
pub use clvm_value::*;
pub use coin::*;
pub use program::*;

// Single CXX bridge for the entire crate
#[cxx::bridge]
pub mod ffi {

    pub struct OptionalVec {
        pub has_value: bool,
        pub data: Vec<u8>,
    }

    #[derive(Debug, Clone)]
    struct CBytes32 {
        bytes: [u8; 32],
    }

    #[derive(Debug, Clone)]
    struct Coin {
        parent_coin_info: CBytes32,
        puzzle_hash: CBytes32,
        amount: u64,
    }

    struct ClvmResult {
        success: bool,
        error: String,
        node_ptr_value: Box<NodePtrWrapper>,
    }

    extern "Rust" {
        type Program;
        type ClvmAllocator;
        type NodePtrWrapper;
        type ClvmValue;

        //Utility functions
        fn from_hex(hex_str: &str) -> Vec<u8>;
        fn to_hex(bytes: &[u8]) -> String;

        // Program functions
        fn new_program(allocator: Box<ClvmAllocator>, ptr: &NodePtrWrapper) -> Box<Program>;
        fn is_atom(self: &Program) -> bool;
        fn is_pair(self: &Program) -> bool;
        fn serialize(self: &Program) -> Result<Vec<u8>>;
        fn to_atom(self: &Program) -> Result<OptionalVec>;

        fn create_allocator() -> Box<ClvmAllocator>;
        fn deserialize(allocator: &mut ClvmAllocator, bytes: &[u8]) -> Result<Box<Program>>;
        fn deserialize_with_backrefs(
            allocator: &mut ClvmAllocator,
            bytes: &[u8],
        ) -> Result<Box<Program>>;
        fn nil(allocator: &mut ClvmAllocator) -> Result<Box<Program>>;

        //Coin
        fn new_coin(parent_coin_info: CBytes32, puzzle_hash: CBytes32, amount: u64) -> Coin;
        fn get_coin_id(coin: &Coin) -> CBytes32;

        fn from_bytes(bytes: &[u8]) -> Result<Box<ClvmValue>>;
        fn from_string(s: &str) -> Result<Box<ClvmValue>>;
        fn from_bool(b: bool) -> Result<Box<ClvmValue>>;
        fn from_float(f: f64) -> Result<Box<ClvmValue>>;
        fn from_int(i: i64) -> Result<Box<ClvmValue>>;
        fn allocate_value(value: &ClvmValue, allocator: &mut ClvmAllocator) -> ClvmResult;

    }
}
// use chia::protocol::{Bytes32, Coin as ProtocolCoin};

// mod program;
// pub use program::*;
// mod clvm;
// pub use clvm::*;

// #[cxx::bridge]
// mod ffi {
//     #[derive(Debug, Clone)]
//     struct CBytes32 {
//         bytes: [u8; 32],
//     }

// #[derive(Debug, Clone)]
// struct Coin {
//     parent_coin_info: CBytes32,
//     puzzle_hash: CBytes32,
//     amount: u64,
// }

//     extern "Rust" {
//         fn from_hex(hex_str: &str) -> Vec<u8>;
//         fn to_hex(bytes: &[u8]) -> String;
//         fn new_coin(parent_coin_info: CBytes32, puzzle_hash: CBytes32, amount: u64) -> Coin;
//         fn get_coin_id(coin: &Coin) -> CBytes32;
//     }
// }

// use ffi::*;

// fn from_hex(hex_str: &str) -> Vec<u8> {
//     hex::decode(hex_str).unwrap_or_default()
// }

// fn to_hex(bytes: &[u8]) -> String {
//     hex::encode(bytes)
// }

// fn to_raw_bytes32(b: &Bytes32) -> CBytes32 {
//     CBytes32 {
//         bytes: b.as_ref().try_into().unwrap(),
//     }
// }

// fn from_raw_bytes32(raw: &CBytes32) -> Bytes32 {
//     Bytes32::new(raw.bytes)
// }

// fn new_coin(parent_coin_info: CBytes32, puzzle_hash: CBytes32, amount: u64) -> Coin {
//     Coin {
//         parent_coin_info,
//         puzzle_hash,
//         amount,
//     }
// }

// fn get_coin_id(coin: &Coin) -> CBytes32 {
//     let protocol_coin = ProtocolCoin {
//         parent_coin_info: from_raw_bytes32(&coin.parent_coin_info),
//         puzzle_hash: from_raw_bytes32(&coin.puzzle_hash),
//         amount: coin.amount,
//     };

//     to_raw_bytes32(&protocol_coin.coin_id())
// }

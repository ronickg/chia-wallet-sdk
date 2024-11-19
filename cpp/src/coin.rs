use chia::protocol::{Bytes32, Coin as ProtocolCoin};
use crate::ffi::{CBytes32, Coin};  // Add this line

pub fn from_hex(hex_str: &str) -> Vec<u8> {
    hex::decode(hex_str).unwrap_or_default()
}

pub fn to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn to_raw_bytes32(b: &Bytes32) -> CBytes32 {
    CBytes32 {
        bytes: b.as_ref().try_into().unwrap(),
    }
}

pub fn from_raw_bytes32(raw: &CBytes32) -> Bytes32 {
    Bytes32::new(raw.bytes)
}

pub fn new_coin(parent_coin_info: CBytes32, puzzle_hash: CBytes32, amount: u64) -> Coin {
    Coin {
        parent_coin_info,
        puzzle_hash,
        amount,
    }
}

pub fn get_coin_id(coin: &Coin) -> CBytes32 {
    let protocol_coin = ProtocolCoin {
        parent_coin_info: from_raw_bytes32(&coin.parent_coin_info),
        puzzle_hash: from_raw_bytes32(&coin.puzzle_hash),
        amount: coin.amount,
    };

    to_raw_bytes32(&protocol_coin.coin_id())
}

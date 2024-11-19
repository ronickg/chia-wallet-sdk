use crate::Program;
use chia::clvm_traits::ToClvm;
use clvmr::Allocator;
use clvmr::NodePtr;
pub enum ClvmValue {
    Float(f64),
    Integer(u64),
    String(String),
    Bool(bool),
    Program(Program),
    Bytes(Vec<u8>),
    Array(Vec<ClvmValue>),
}

pub(crate) trait Allocate {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String>;
}

impl Allocate for ClvmValue {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        match self {
            ClvmValue::Float(f) => f.allocate(allocator),
            ClvmValue::Integer(i) => i.allocate(allocator),
            ClvmValue::String(s) => s.allocate(allocator),
            ClvmValue::Bool(b) => b.allocate(allocator),
            ClvmValue::Bytes(b) => b.allocate(allocator),
            ClvmValue::Array(arr) => arr.allocate(allocator),
            ClvmValue::Program(prog) => prog.allocate(allocator),
        }
    }
}

pub fn new_string_value(value: String) -> Box<ClvmValue> {
    Box::new(ClvmValue::String(value))
}

impl Allocate for f64 {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        if self.is_infinite() {
            return Err("Value is infinite".to_string());
        }

        if self.is_nan() {
            return Err("Value is NaN".to_string());
        }

        if self.fract() != 0.0 {
            return Err("Value has a fractional part".to_string());
        }

        if *self > 9_007_199_254_740_991.0 {
            return Err("Value is larger than MAX_SAFE_INTEGER".to_string());
        }

        if *self < -9_007_199_254_740_991.0 {
            return Err("Value is smaller than MIN_SAFE_INTEGER".to_string());
        }

        let value = *self as i64;

        if (0..=67_108_863).contains(&value) {
            allocator
                .new_small_number(value as u32)
                .map_err(|e| e.to_string())
        } else {
            allocator
                .new_number(value.into())
                .map_err(|e| e.to_string())
        }
    }
}

impl Allocate for u64 {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        allocator
            .new_number((*self).into())
            .map_err(|e| e.to_string())
    }
}

impl Allocate for String {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        allocator
            .new_atom(self.as_bytes())
            .map_err(|e| e.to_string())
    }
}

impl Allocate for bool {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        allocator
            .new_small_number(u32::from(*self))
            .map_err(|e| e.to_string())
    }
}

impl Allocate for Vec<u8> {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        allocator.new_atom(self).map_err(|e| e.to_string())
    }
}

impl Allocate for Vec<ClvmValue> {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        let mut items: Vec<NodePtr> = Vec::with_capacity(self.len());

        for item in self {
            let node_ptr = item.allocate(allocator)?;
            items.push(node_ptr.into());
        }

        items.to_clvm(allocator).map_err(|e| e.to_string())
    }
}

impl Allocate for Program {
    fn allocate(&self, _allocator: &mut Allocator) -> Result<NodePtr, String> {
        Ok(self.ptr)
    }
}

pub fn allocate_value(value: &ClvmValue, allocator: &mut Allocator) -> Result<NodePtr, String> {
    value.allocate(allocator)
}

// Define our variant types using an enum
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
//     fn allocate(&self, allocator: &mut ClvmAllocator) -> Result<NodePtrWrapper, String>;
// }

// impl Allocate for ClvmValue {
//     fn allocate(&self, allocator: &mut ClvmAllocator) -> Result<NodePtrWrapper, String> {
//         match self {
//             ClvmValue::Float(value) => value.allocate(allocator),
//             ClvmValue::Integer(value) => value.allocate(allocator),
//             ClvmValue::String(value) => value.allocate(allocator),
//             ClvmValue::Bool(value) => value.allocate(allocator),
//             ClvmValue::Program(value) => value.allocate(allocator),
//             ClvmValue::Bytes(value) => value.allocate(allocator),
//             ClvmValue::Array(value) => value.allocate(allocator),
//         }
//     }
// }

// impl Allocate for f64 {
//     fn allocate(&self, allocator: &mut ClvmAllocator) -> Result<NodePtrWrapper, String> {
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
//                 .get_allocator()
//                 .new_small_number(value as u32)
//                 .map(NodePtrWrapper::from)
//                 .map_err(|e| e.to_string())
//         } else {
//             allocator
//                 .get_allocator()
//                 .new_number(value.into())
//                 .map(NodePtrWrapper::from)
//                 .map_err(|e| e.to_string())
//         }
//     }
// }

// impl Allocate for u64 {
//     fn allocate(&self, allocator: &mut ClvmAllocator) -> Result<NodePtrWrapper, String> {
//         allocator
//             .get_allocator()
//             .new_number((*self).into())
//             .map(NodePtrWrapper::from)
//             .map_err(|e| e.to_string())
//     }
// }

// impl Allocate for String {
//     fn allocate(&self, allocator: &mut ClvmAllocator) -> Result<NodePtrWrapper, String> {
//         allocator
//             .get_allocator()
//             .new_atom(self.as_bytes())
//             .map(NodePtrWrapper::from)
//             .map_err(|e| e.to_string())
//     }
// }

// impl Allocate for bool {
//     fn allocate(&self, allocator: &mut ClvmAllocator) -> Result<NodePtrWrapper, String> {
//         allocator
//             .get_allocator()
//             .new_small_number(u32::from(*self))
//             .map(NodePtrWrapper::from)
//             .map_err(|e| e.to_string())
//     }
// }

// impl Allocate for Vec<u8> {
//     fn allocate(&self, allocator: &mut ClvmAllocator) -> Result<NodePtrWrapper, String> {
//         allocator
//             .get_allocator()
//             .new_atom(self)
//             .map(NodePtrWrapper::from)
//             .map_err(|e| e.to_string())
//     }
// }

// impl Allocate for Vec<ClvmValue> {
//     fn allocate(&self, allocator: &mut ClvmAllocator) -> Result<NodePtrWrapper, String> {
//         let mut items: Vec<NodePtr> = Vec::with_capacity(self.len());

//         for item in self {
//             let node_ptr = item.allocate(allocator)?;
//             items.push(node_ptr.into());
//         }

//         items
//             .to_clvm(allocator.get_allocator())
//             .map(NodePtrWrapper::from)
//             .map_err(|e| e.to_string())
//     }
// }

// impl Allocate for Program {
//     fn allocate(&self, _allocator: &mut ClvmAllocator) -> Result<NodePtrWrapper, String> {
//         Ok(self.ptr)
//     }
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

// pub fn allocate_value(value: &ClvmValue, allocator: &mut ClvmAllocator) -> ClvmResult {
//     match value.allocate(allocator) {
//         Ok(node_ptr) => ClvmResult {
//             success: true,
//             error: String::new(),
//             node_ptr_value: Box::new(node_ptr),
//         },
//         Err(err_str) => ClvmResult {
//             success: false,
//             error: err_str,
//             node_ptr_value: Box::new(NodePtrWrapper::from(NodePtr::NIL)),
//         },
//     }
// }

// use crate::Program;
// use chia::clvm_traits::ToClvm;
// use clvmr::{Allocator, NodePtr};
// use std::convert::TryFrom;

// use crate::ffi::ClvmResult;
// // Define our variant types using an enum instead of Either9
// pub enum ClvmValue {
//     Float(f64),
//     Integer(u64), // Using i64 instead of BigInt for C++ compatibility
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
//             ClvmValue::Float(value) => value.allocate(allocator),
//             ClvmValue::Integer(value) => value.allocate(allocator),
//             ClvmValue::String(value) => value.allocate(allocator),
//             ClvmValue::Bool(value) => value.allocate(allocator),
//             ClvmValue::Program(value) => value.allocate(allocator),
//             ClvmValue::Bytes(value) => value.allocate(allocator),
//             ClvmValue::Array(value) => value.allocate(allocator),
//         }
//     }
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
//         let mut items = Vec::with_capacity(self.len());

//         for item in self {
//             items.push(item.allocate(allocator)?);
//         }

//         items.to_clvm(allocator).map_err(|e| e.to_string())
//     }
// }

// impl Allocate for Program {
//     fn allocate(&self, _allocator: &mut Allocator) -> Result<NodePtr, String> {
//         Ok(self.ptr.into())
//     }
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
//     // Note: You might need to adjust this since your ClvmValue::from_int takes u64
//     ClvmValue::from_int(i as u64)
// }

// pub fn allocate_value(value: &ClvmValue, allocator: &mut Allocator) -> ClvmResult {
//     match value.allocate(allocator) {
//         Ok(node_ptr) => ClvmResult {
//             success: true,
//             error: String::new(),     // Empty string for no error
//             node_ptr_value: node_ptr, // You'll need to implement a way to convert NodePtr to u32
//         },
//         Err(err_str) => ClvmResult {
//             success: false,
//             error: err_str,
//             node_ptr_value: 0, // Zero for invalid/null pointer
//         },
//     }
// }

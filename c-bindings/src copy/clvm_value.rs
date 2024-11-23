use std::ffi::CStr;

use chia::clvm_traits::ToClvm;
use clvmr::{Allocator, NodePtr};
use libc::{c_char, c_double, c_void, size_t}; // Explicit C types

use crate::{BytesHandle, ProgramHandle};

#[repr(C)]
pub enum ClvmValueType {
    Number,
    // BigInt,
    // Boolean,
    String,
    // Program,
    // Bytes,
    // Array,
}

#[repr(C)]
pub union ClvmValueData {
    number: c_double,
    bigint: u64,
    boolean: bool,
    string: *mut c_char,
    program: *mut ProgramHandle,
    bytes: *mut BytesHandle,
    array: *mut ClvmValueArray,
}

#[repr(C)]
pub struct ClvmValue {
    value_type: ClvmValueType,
    data: ClvmValueData,
}

#[repr(C)]
pub struct ClvmValueArray {
    items: *mut ClvmValue,
    len: size_t,
}

impl ClvmValue {
    pub unsafe fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        match self.value_type {
            ClvmValueType::Number => {
                let num = self.data.number;
                if !num.is_finite() {
                    return Err("Invalid number value".to_string());
                }
                if num.fract() != 0.0 {
                    return Err("Fractional values not supported".to_string());
                }
                if num > 9_007_199_254_740_991.0 || num < -9_007_199_254_740_991.0 {
                    return Err("Number out of range".to_string());
                }

                let value = num as i64;
                if (0..=67_108_863).contains(&value) {
                    allocator.new_small_number(value as u32)
                } else {
                    allocator.new_number(value.into())
                }
                .map_err(|e| e.to_string())
            }

            ClvmValueType::String => {
                let c_str = CStr::from_ptr(self.data.string);
                let bytes = c_str.to_bytes();
                allocator.new_atom(bytes).map_err(|e| e.to_string())
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn clvm_value_free(value: *mut ClvmValue) {
    if !value.is_null() {
        unsafe {
            let value = Box::from_raw(value);
            match value.value_type {
                ClvmValueType::String => {
                    let _ = Box::from_raw(value.data.string as *mut c_char);
                }
                _ => {}
            }
        }
    }
}

// fn allocate_bigint(allocator: &mut Allocator, value: u64) -> Result<NodePtr, &'static str> {
//     allocator
//         .new_number(value.into())
//         .map_err(|_| "Failed to allocate bigint")
// }

// fn allocate_string(allocator: &mut Allocator, value: *mut c_char) -> Result<NodePtr, &'static str> {
// let c_str = unsafe { std::ffi::CStr::from_ptr(value) };
// let bytes = c_str.to_bytes();
// allocator
//     .new_atom(bytes)
//     .map_err(|_| "Failed to allocate string")
// }

// fn allocate_bool(allocator: &mut Allocator, value: bool) -> Result<NodePtr, &'static str> {
//     allocator
//         .new_small_number(u32::from(value))
//         .map_err(|_| "Failed to allocate bool")
// }

// fn allocate_program(
//     allocator: &mut Allocator,
//     value: *mut Program,
// ) -> Result<NodePtr, &'static str> {
//     let program = unsafe { &*value };
//     Ok(program.ptr)
// }

// fn allocate_bytes(allocator: &mut Allocator, bytes: Bytes) -> Result<NodePtr, &'static str> {
//     let slice = unsafe { std::slice::from_raw_parts(bytes.data, bytes.len) };
//     allocator
//         .new_atom(slice)
//         .map_err(|_| "Failed to allocate bytes")
// }

// fn allocate_array(
//     allocator: &mut Allocator,
//     value: *mut ClvmValueArray,
// ) -> Result<NodePtr, &'static str> {
//     let array = unsafe { &*value };
//     let slice = unsafe { std::slice::from_raw_parts(array.items, array.len) };

//     let mut items = Vec::with_capacity(array.len);
//     for item in slice {
//         let node_ptr =
//             clvm_value_allocate(allocator as *mut _ as *mut c_void, item as *const ClvmValue);
//         if node_ptr.is_null() {
//             return Err("Failed to allocate array item");
//         }
//         items.push(unsafe { *node_ptr });
//     }

//     items
//         .to_clvm(allocator)
//         .map_err(|_| "Failed to allocate array")
// }

// #[no_mangle]
// pub extern "C" fn clvm_value_free(value: *mut ClvmValue) {
//     if !value.is_null() {
//         unsafe {
//             let value = Box::from_raw(value);
//             match value.value_type {
//                 ClvmValueType::String => {
//                     let _ = Box::from_raw(value.data.string as *mut c_char);
//                 }
//                 ClvmValueType::Program => {
//                     let _ = Box::from_raw(value.data.program);
//                 }
//                 ClvmValueType::Bytes => {
//                     let bytes = value.data.bytes;
//                     let _ = Box::from_raw(bytes.data);
//                 }
//                 ClvmValueType::Array => {
//                     let array = Box::from_raw(value.data.array);
//                     let slice = std::slice::from_raw_parts_mut(array.items, array.len);
//                     for item in slice {
//                         clvm_value_free(item);
//                     }
//                     let _ = Box::from_raw(array.items);
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

// // Constructor functions
// #[no_mangle]
// pub extern "C" fn create_clvm_number(value: c_double) -> *mut ClvmValue {
//     let clvm_value = ClvmValue {
//         value_type: ClvmValueType::Number,
//         data: ClvmValueData { number: value },
//     };
//     Box::into_raw(Box::new(clvm_value))
// }

// #[no_mangle]
// pub extern "C" fn create_clvm_bigint(value: u64) -> *mut ClvmValue {
//     let clvm_value = ClvmValue {
//         value_type: ClvmValueType::BigInt,
//         data: ClvmValueData { bigint: value },
//     };
//     Box::into_raw(Box::new(clvm_value))
// }

// #[no_mangle]
// pub extern "C" fn create_clvm_string(value: *mut c_char) -> *mut ClvmValue {
//     let clvm_value = ClvmValue {
//         value_type: ClvmValueType::String,
//         data: ClvmValueData { string: value },
//     };
//     Box::into_raw(Box::new(clvm_value))
// }

// #[no_mangle]
// pub extern "C" fn create_clvm_bool(value: bool) -> *mut ClvmValue {
//     let clvm_value = ClvmValue {
//         value_type: ClvmValueType::Boolean,
//         data: ClvmValueData { boolean: value },
//     };
//     Box::into_raw(Box::new(clvm_value))
// }

// #[no_mangle]
// pub extern "C" fn create_clvm_program(value: *mut Program) -> *mut ClvmValue {
//     let clvm_value = ClvmValue {
//         value_type: ClvmValueType::Program,
//         data: ClvmValueData { program: value },
//     };
//     Box::into_raw(Box::new(clvm_value))
// }

// #[no_mangle]
// pub extern "C" fn create_clvm_bytes(data: *mut u8, len: size_t) -> *mut ClvmValue {
//     let bytes = Bytes { data, len };
//     let clvm_value = ClvmValue {
//         value_type: ClvmValueType::Bytes,
//         data: ClvmValueData { bytes },
//     };
//     Box::into_raw(Box::new(clvm_value))
// }

// #[no_mangle]
// pub extern "C" fn create_clvm_array(items: *mut ClvmValue, len: size_t) -> *mut ClvmValue {
//     let array = Box::new(ClvmValueArray { items, len });
//     let clvm_value = ClvmValue {
//         value_type: ClvmValueType::Array,
//         data: ClvmValueData {
//             array: Box::into_raw(array),
//         },
//     };
//     Box::into_raw(Box::new(clvm_value))
// }

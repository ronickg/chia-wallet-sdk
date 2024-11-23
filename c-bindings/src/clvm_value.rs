use std::ffi::CStr;

use chia::clvm_traits::ToClvm;
use clvmr::{number::number_from_u8, Allocator, NodePtr};
use libc::{c_char, c_double, c_void, size_t}; // Explicit C types

use crate::{bytes_destroy, BytesHandle, BytesWrapper, ProgramHandle};

#[repr(C)]
pub enum ClvmValueType {
    Number,
    BigInt,
    // Boolean,
    String,
    // Program,
    // Bytes,
    Array,
}

#[repr(C)]
pub union ClvmValueData {
    number: c_double,
    bigint: *mut BytesHandle,
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

impl ClvmValueArray {
    pub fn new(values: Vec<ClvmValue>) -> Self {
        let len = values.len();
        let mut boxed_values = values.into_boxed_slice();
        let items = boxed_values.as_mut_ptr();
        std::mem::forget(boxed_values);

        Self { items, len }
    }

    pub unsafe fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        let items = std::slice::from_raw_parts(self.items, self.len);
        let mut nodes = Vec::with_capacity(self.len);

        for item in items {
            nodes.push(item.allocate(allocator)?);
        }

        nodes.to_clvm(allocator).map_err(|e| e.to_string())
    }
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
            ClvmValueType::BigInt => {
                let bytes_handle = &*(self.data.bigint);
                let wrapper = BytesWrapper::from_handle(bytes_handle);
                let num = number_from_u8(wrapper.as_slice());
                allocator.new_number(num).map_err(|e| e.to_string())
            }

            ClvmValueType::String => {
                let c_str = CStr::from_ptr(self.data.string);
                let bytes = c_str.to_bytes();
                allocator.new_atom(bytes).map_err(|e| e.to_string())
            }

            ClvmValueType::Array => {
                if self.data.array.is_null() {
                    return Err("Null array pointer".to_string());
                }
                let array = &*self.data.array;
                array.allocate(allocator)
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn clvm_array_destroy(array: *mut ClvmValueArray) {
    if !array.is_null() {
        let array = Box::from_raw(array);
        if !array.items.is_null() {
            // Reconstruct Vec to properly deallocate
            let items = Vec::from_raw_parts(array.items, array.len, array.len);
            // items will be dropped here, which will drop each ClvmValue
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn clvm_array_from_values(
    values: *const ClvmValue,
    len: size_t,
) -> *mut ClvmValueArray {
    if values.is_null() && len > 0 {
        return std::ptr::null_mut();
    }

    // Allocate new memory for items
    let items = if len > 0 {
        let layout = std::alloc::Layout::array::<ClvmValue>(len).unwrap();
        let ptr = std::alloc::alloc(layout) as *mut ClvmValue;

        // Copy values
        std::ptr::copy_nonoverlapping(values, ptr, len);
        ptr
    } else {
        std::ptr::null_mut()
    };

    Box::into_raw(Box::new(ClvmValueArray { items, len }))
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
                ClvmValueType::BigInt => {
                    bytes_destroy(value.data.bigint); // Need to free the bytes handle
                }
                ClvmValueType::Array => {
                    if !value.data.array.is_null() {
                        clvm_array_destroy(value.data.array);
                    }
                }
                _ => {}
            }
        }
    }
}

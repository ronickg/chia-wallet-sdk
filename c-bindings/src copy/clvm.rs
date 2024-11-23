use std::sync::Arc;

use chia::{
    bls,
    clvm_traits::{ClvmDecoder, FromClvm},
    clvm_utils::{tree_hash, CurriedProgram},
};
use chia_wallet_sdk::SpendContext;
use clvmr::NodePtr;

use crate::{program_new, ClvmValue, ProgramHandle};

pub struct ClvmAllocator {
    pub(crate) context: SpendContext,
}

pub struct ClvmHandle {
    pub(crate) inner: Arc<ClvmAllocator>,
}

// FFI-safe functions
#[no_mangle]
pub extern "C" fn clvm_create() -> *mut ClvmHandle {
    let allocator = ClvmAllocator {
        context: SpendContext::new(),
    };

    // Wrap in Arc and convert to raw pointer
    let handle = Box::new(ClvmHandle {
        inner: Arc::new(allocator),
    });
    Box::into_raw(handle)
}

#[no_mangle]
pub unsafe extern "C" fn clvm_clone(ptr: *const ClvmHandle) -> *mut ClvmHandle {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        // Clone the Arc, increasing reference count
        let handle = Box::new(ClvmHandle {
            inner: (*ptr).inner.clone(),
        });
        Box::into_raw(handle)
    }
}

// Create NIL program
#[no_mangle]
pub extern "C" fn clvm_nil_program(handle: *const ClvmHandle) -> *mut ProgramHandle {
    if handle.is_null() {
        return std::ptr::null_mut();
    }

    program_new(NodePtr::NIL)
}

// Allocate
#[no_mangle]
pub unsafe extern "C" fn clvm_alloc(
    ptr: *mut ClvmHandle,
    value: *const ClvmValue,
) -> *mut ProgramHandle {
    if ptr.is_null() || value.is_null() {
        return std::ptr::null_mut();
    }

    // Safely dereference the input pointers.
    let handle = unsafe { &mut *ptr };
    let value = unsafe { &*value };

    // Attempt to allocate the value.
    let ptr = match value.allocate(&mut handle.inner.context.allocator) {
        Ok(p) => p,
        Err(_) => return std::ptr::null_mut(),
    };

    // Wrap and return the allocated program.
    program_new(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn clvm_destroy(ptr: *mut ClvmHandle) {
    if !ptr.is_null() {
        unsafe {
            // Drop the Box and Arc
            drop(Box::from_raw(ptr));
            // Arc will clean up ClvmAllocator when last reference is dropped
        }
    }
}

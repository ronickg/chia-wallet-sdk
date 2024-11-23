use chia::protocol::Bytes32;
use std::{cell::RefCell, sync::Arc};

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
    pub(crate) inner: RefCell<ClvmAllocator>,
}

// FFI-safe functions
#[no_mangle]
pub extern "C" fn clvm_create() -> *mut ClvmHandle {
    let allocator = ClvmAllocator {
        context: SpendContext::new(),
    };

    let handle = Box::new(ClvmHandle {
        inner: RefCell::new(allocator),
    });
    Box::into_raw(handle)
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
pub unsafe extern "C" fn clvm_allocate(
    handle: *const ClvmHandle,
    value: *const ClvmValue,
) -> *mut ProgramHandle {
    if handle.is_null() || value.is_null() {
        return std::ptr::null_mut();
    }

    let handle_ref = &*handle;
    let value_ref = &*value;

    // Borrow mutably using RefCell
    if let Ok(mut allocator) = handle_ref.inner.try_borrow_mut() {
        match value_ref.allocate(&mut allocator.context.allocator) {
            Ok(node_ptr) => program_new(node_ptr),
            Err(_) => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
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

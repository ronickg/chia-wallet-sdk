use chia::protocol::Bytes32;
use libc::{c_char, c_double, c_void, size_t};
use std::{ffi::CString, sync::Arc}; // Explicit C types

use chia::{
    bls,
    clvm_traits::{ClvmDecoder, FromClvm},
    clvm_utils::{tree_hash, CurriedProgram},
};
use chia_wallet_sdk::SpendContext;
use clvmr::{
    serde::{node_to_bytes, node_to_bytes_backrefs},
    Allocator, NodePtr, SExp,
};

use crate::{BytesHandle, BytesWrapper, ClvmHandle};

//ProgramHandle
pub struct ProgramHandle {
    pub(crate) ptr: NodePtr,
}

#[no_mangle]
pub extern "C" fn program_new(ptr: NodePtr) -> *mut ProgramHandle {
    let program = Box::new(ProgramHandle { ptr });
    Box::into_raw(program)
}

#[no_mangle]
pub unsafe extern "C" fn program_destroy(ptr: *mut ProgramHandle) {
    if !ptr.is_null() {
        unsafe {
            drop(Box::from_raw(ptr));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn program_is_atom(program: *const ProgramHandle) -> bool {
    if program.is_null() {
        return false;
    }
    unsafe { (*program).ptr.is_atom() }
}

#[no_mangle]
pub unsafe extern "C" fn program_is_pair(program: *const ProgramHandle) -> bool {
    if program.is_null() {
        return false;
    }
    unsafe { (*program).ptr.is_pair() }
}

#[no_mangle]
pub unsafe extern "C" fn program_tree_hash(
    handle: *const ClvmHandle,
    program: *const ProgramHandle,
) -> Bytes32 {
    if handle.is_null() || program.is_null() {
        return Bytes32::default();
    }

    let handle_ref = &*handle;
    let program_ref = &*program;

    // Borrow context to get allocator reference
    if let Ok(allocator) = handle_ref.inner.try_borrow() {
        let hash = tree_hash(&allocator.context.allocator, program_ref.ptr);
        hash.into()
    } else {
        Bytes32::default()
    }
}

#[no_mangle]
pub unsafe extern "C" fn program_to_string(
    handle: *const ClvmHandle,
    program: *const ProgramHandle,
) -> *mut c_char {
    if handle.is_null() || program.is_null() {
        return std::ptr::null_mut();
    }

    let handle_ref = &*handle;
    let program_ref = &*program;

    if let Ok(allocator) = handle_ref.inner.try_borrow() {
        match allocator.context.allocator.sexp(program_ref.ptr) {
            SExp::Atom => {
                let bytes = allocator
                    .context
                    .allocator
                    .atom(program_ref.ptr)
                    .as_ref()
                    .to_vec();
                match String::from_utf8(bytes) {
                    Ok(s) => match CString::new(s) {
                        Ok(c_str) => c_str.into_raw(),
                        Err(_) => std::ptr::null_mut(),
                    },
                    Err(_) => std::ptr::null_mut(),
                }
            }
            SExp::Pair(..) => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn program_to_number(
    handle: *const ClvmHandle,
    program: *const ProgramHandle,
) -> c_double {
    if handle.is_null() || program.is_null() {
        return f64::NAN;
    }

    let handle_ref = &*handle;
    let program_ref = &*program;

    if let Ok(allocator) = handle_ref.inner.try_borrow() {
        match allocator.context.allocator.sexp(program_ref.ptr) {
            SExp::Atom => match allocator.context.allocator.small_number(program_ref.ptr) {
                Some(n) => n as f64,
                None => f64::NAN,
            },
            SExp::Pair(..) => f64::NAN,
        }
    } else {
        f64::NAN
    }
}

#[no_mangle]
pub unsafe extern "C" fn program_to_bigint_bytes(
    handle: *const ClvmHandle,
    program: *const ProgramHandle,
) -> *mut BytesHandle {
    if handle.is_null() || program.is_null() {
        return std::ptr::null_mut();
    }

    let handle_ref = &*handle;
    let program_ref = &*program;

    if let Ok(allocator) = handle_ref.inner.try_borrow() {
        match allocator.context.allocator.sexp(program_ref.ptr) {
            SExp::Atom => {
                let num = allocator.context.allocator.number(program_ref.ptr);
                let bytes = num.to_signed_bytes_be();
                BytesWrapper::from_slice(&bytes).into_handle()
            }
            SExp::Pair(..) => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn program_first(
    handle: *const ClvmHandle,
    program: *const ProgramHandle,
) -> *mut ProgramHandle {
    if handle.is_null() || program.is_null() {
        return std::ptr::null_mut();
    }

    let handle_ref = &*handle;
    let program_ref = &*program;

    if let Ok(allocator) = handle_ref.inner.try_borrow() {
        match allocator.context.allocator.sexp(program_ref.ptr) {
            SExp::Pair(f, _) => program_new(f),
            _ => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn program_rest(
    handle: *const ClvmHandle,
    program: *const ProgramHandle,
) -> *mut ProgramHandle {
    if handle.is_null() || program.is_null() {
        return std::ptr::null_mut();
    }

    let handle_ref = &*handle;
    let program_ref = &*program;

    if let Ok(allocator) = handle_ref.inner.try_borrow() {
        match allocator.context.allocator.sexp(program_ref.ptr) {
            SExp::Pair(_, r) => program_new(r),
            _ => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn string_destroy(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

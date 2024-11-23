use chia::protocol::Bytes32;
use std::sync::Arc;

use chia::{
    bls,
    clvm_traits::{ClvmDecoder, FromClvm},
    clvm_utils::{tree_hash, CurriedProgram},
};
use chia_wallet_sdk::SpendContext;
use clvmr::NodePtr;

use crate::ClvmHandle;

//ProgramHandle
pub struct ProgramHandle {
    ptr: NodePtr,
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
        return Bytes32::default(); // Return zero-filled array
    }

    let handle_ref = &*handle;
    let program_ref = &*program;

    // Calculate tree hash and convert directly to Bytes32
    let hash = tree_hash(&handle_ref.inner.context.allocator, program_ref.ptr);
    hash.into() // Converts TreeHash into Bytes32 using the From implementation
}

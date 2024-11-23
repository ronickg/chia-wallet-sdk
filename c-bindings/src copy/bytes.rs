use chia::protocol::Bytes;

#[repr(C)]
pub struct BytesHandle {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

pub struct BytesWrapper {
    inner: Vec<u8>,
}

impl From<BytesWrapper> for Bytes {
    fn from(wrapper: BytesWrapper) -> Self {
        Bytes::new(wrapper.inner)
    }
}

impl From<Bytes> for BytesWrapper {
    fn from(bytes: Bytes) -> Self {
        Self {
            inner: bytes.into_inner(),
        }
    }
}

impl BytesWrapper {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        Self {
            inner: slice.to_vec(),
        }
    }

    pub fn as_handle(&mut self) -> BytesHandle {
        BytesHandle {
            ptr: self.inner.as_mut_ptr(),
            len: self.inner.len(),
            cap: self.inner.capacity(),
        }
    }

    pub fn into_chia_bytes(self) -> Bytes {
        self.into()
    }

    // Create from Chia Bytes
    pub fn from_chia_bytes(bytes: Bytes) -> Self {
        bytes.into()
    }
}

#[no_mangle]
pub extern "C" fn bytes_create() -> *mut BytesHandle {
    let mut bytes = BytesWrapper::new();
    let handle = bytes.as_handle();
    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub unsafe extern "C" fn bytes_destroy(handle: *mut BytesHandle) {
    if !handle.is_null() {
        let bytes = Box::from_raw(handle);
        if !bytes.ptr.is_null() {
            let _ = Vec::from_raw_parts(bytes.ptr, bytes.len, bytes.cap);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn bytes_from_slice(data: *const u8, len: usize) -> *mut BytesHandle {
    if data.is_null() && len > 0 {
        return std::ptr::null_mut();
    }

    let slice = if len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(data, len)
    };

    let mut bytes = BytesWrapper::from_slice(slice);
    let handle = bytes.as_handle();
    std::mem::forget(bytes);

    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub unsafe extern "C" fn bytes_copy_to(
    handle: *const BytesHandle,
    out: *mut u8,
    out_len: usize,
) -> bool {
    if handle.is_null() {
        return false;
    }

    let bytes = &*handle;
    if bytes.len > out_len || bytes.ptr.is_null() {
        return false;
    }

    if bytes.len > 0 {
        std::ptr::copy_nonoverlapping(bytes.ptr, out, bytes.len);
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn bytes_len(handle: *const BytesHandle) -> usize {
    if handle.is_null() {
        return 0;
    }
    (*handle).len
}

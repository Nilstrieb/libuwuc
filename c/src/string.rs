#[no_mangle]
pub(crate) unsafe extern "C" fn memset(ptr: *mut u8, constant: u8, len: usize) {
    for i in 0..len {
        unsafe {
            *ptr.add(i) = constant;
        }
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn strlen(s: *const u8) -> usize {
    libuwuc::mem::strlen(s)
}
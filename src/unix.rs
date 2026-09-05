use core::ffi::{CStr, c_char, c_int, c_void};

pub(crate) fn dlerror_string_lossy() -> String {
    let e = unsafe { dlerror() };
    if e.is_null() { String::new() } else { unsafe { CStr::from_ptr(e) }.to_string_lossy().into() }
}

pub(crate) const RTLD_LAZY : c_int = 1;
extern "C" {
    pub(crate) fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    pub(crate) fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    pub(crate) fn dlerror() -> *const c_char;
    pub(crate) fn dlclose(handle: *mut c_void) -> c_int;
}

use crate::Library;

use core::ffi::{c_char, c_int, c_void};
use core::num::NonZero;



pub(crate) const RTLD_LAZY : c_int = 1;
extern "C" {
    pub(crate) fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    pub(crate) fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}



/// \[[man.archlinux.org](https://man.archlinux.org/man/dlclose.3.en)\]
/// dlclose
///
/// # Safety
/// ❌ This is a **fundamentally unsound** operation that **may do nothing** and **invalidates everything** ❌
///
/// See [`Library::close_unsafe_unsound_possible_noop_do_not_use_in_production`] for the full rant.
///
pub(crate) unsafe fn dlclose(handle: Library) -> Result<(), NonZero<c_int>> {
    extern "C" { fn dlclose(handle: Library) -> Option<NonZero<c_int>>; }
    match unsafe { dlclose(handle) } {
        None        => Ok(()),
        Some(err)   => Err(err),
    }
}

/// \[[man.archlinux.org](https://man.archlinux.org/man/dlerror.3.en)\]
/// dlerror
///
pub(crate) mod dlerror {
    use super::*;

    extern "C" { fn dlerror() -> *const c_char; }

    /// \[[man.archlinux.org](https://man.archlinux.org/man/dlerror.3.en)\]
    /// drop(dlerror())
    ///
    pub(crate) fn clear() {
        let _ = unsafe { dlerror() };
    }

    /// \[[man.archlinux.org](https://man.archlinux.org/man/dlerror.3.en)\]
    /// dlerror() -> [Option]&lt;impl Deref&lt;Target = [CStr]&gt;&gt;
    ///
    #[cfg(not(feature = "alloc"))] pub(crate) fn to_cstring() -> Option<&'static core::ffi::CStr> {
        None
    }

    /// \[[man.archlinux.org](https://man.archlinux.org/man/dlerror.3.en)\]
    /// dlerror() -> [Option]&lt;impl Deref&lt;Target = [CStr]&gt; + \'static&gt;
    ///
    #[cfg(feature = "alloc")] pub(crate) fn to_cstring() -> Option<alloc::ffi::CString> {
        let ptr = unsafe { dlerror() };
        if ptr.is_null() { return None }
        Some(unsafe { core::ffi::CStr::from_ptr(ptr) }.into())
    }
}

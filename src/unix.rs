use crate::Library;

use core::ffi::{CStr, c_char, c_int, c_void};
use core::num::NonZero;
use core::ptr::NonNull;

#[cfg(not(feature = "alloc"))] type CString = &'static core::ffi::CStr;
#[cfg(    feature = "alloc" )] type CString = alloc::ffi::CString;



pub(crate) const RTLD_LAZY : c_int = 1;



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
    /// dlerror() -> [Option]&lt;impl Deref&lt;Target = [CStr]&gt; <span style="opacity: 25%">+ \'static?</span>&gt;
    ///
    pub(crate) fn to_cstring() -> Option<CString> {
        let ptr = unsafe { dlerror() };
        if ptr.is_null() { return None }
        #[cfg(    feature = "alloc" )] return Some(unsafe { core::ffi::CStr::from_ptr(ptr) }.into());
        #[cfg(not(feature = "alloc"))] return None; // XXX
    }
}

/// \[[man.archlinux.org](https://man.archlinux.org/man/dlopen.3.en)\]
/// dlopen
///
/// # Safety
/// -   Flags such as `RTLD_NOLOAD` may allow [`Library`] to potentially dangle, which is unsound.
/// -   ???
///
pub(crate) unsafe fn dlopen(filename: &CStr, flags: c_int) -> Result<Library, Option<CString>> {
    extern "C" { fn dlopen(filename: *const c_char, flags: c_int) -> Option<Library>; }
    unsafe { dlopen(filename.as_ptr(), flags) }.ok_or_else(dlerror::to_cstring)
}

/// \[[man.archlinux.org](https://man.archlinux.org/man/dlsym.3.en)\]
/// dlsym
///
pub(crate) fn dlsym(handle: Library, symbol: &CStr) -> Result<NonNull<c_void>, Option<CString>> {
    extern "C" { fn dlsym(handle: Library, symbol: *const c_char) -> Option<NonNull<c_void>>; }
    unsafe { dlsym(handle, symbol.as_ptr()) }.ok_or_else(dlerror::to_cstring)
}

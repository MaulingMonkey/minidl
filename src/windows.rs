use crate::Library;

use core::ffi::{CStr, c_void, c_char};
use core::fmt::{self, Debug, Formatter};
use core::ptr::NonNull;

pub(crate) const ERROR_BAD_EXE_FORMAT       : Error = Error::from_u32(0x00C1);
pub(crate) const ERROR_INVALID_PARAMETER    : Error = Error::from_u32(87);
pub(crate) const ERROR_MOD_NOT_FOUND        : Error = Error::from_u32(0x007E);



#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub(crate) struct Error(
    u32,

    // I'd like to use winresult::ErrorHResultOrCode for automatic natvis.
    // However, it has no const fn for construction from u32 at this time.
    //#[cfg(not(feature = "winresult"))] u32,
    //#[cfg(    feature = "winresult" )] winresult::ErrorHResultOrCode,
);

impl Error {
    pub(crate) const fn from_u32(code: u32) -> Self { Self(code) }
    pub(crate) const fn to_u32(self) -> u32 { self.0 }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)\]
    /// GetLastError
    ///
    pub(crate) fn get_last() -> Self {
        extern "system" { fn GetLastError() -> Error; }
        unsafe { GetLastError() }
    }

    const fn as_str(self) -> Option<&'static str> {
        Some(match self {
            ERROR_BAD_EXE_FORMAT    => "ERROR_BAD_EXE_FORMAT",
            ERROR_INVALID_PARAMETER => "ERROR_INVALID_PARAMETER",
            ERROR_MOD_NOT_FOUND     => "ERROR_MOD_NOT_FOUND",
            _other                  => return None,
        })
    }
}

impl Debug for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let code = self.to_u32();
        if let Some(err) = self.as_str() {
            fmt.write_str(err)
        } else if code < 0x10000 {
            write!(fmt, "{code}")
        } else {
            write!(fmt, "0x{code:08X}")
        }
    }
}

#[cfg(feature = "winresult")] impl From<Error> for winresult::ErrorHResultOrCode { fn from(error: Error) -> Self { error.to_u32().into() } }
#[cfg(feature = "winresult")] impl From<winresult::ErrorHResultOrCode> for Error { fn from(error: winresult::ErrorHResultOrCode ) -> Error { Error::from_u32(error.to_u32()) } }
#[cfg(feature = "winresult")] impl From<winresult::ErrorCode         > for Error { fn from(error: winresult::ErrorCode          ) -> Error { Error::from_u32(error.to_u32()) } }
#[cfg(feature = "winresult")] impl From<winresult::HResultError      > for Error { fn from(error: winresult::HResultError       ) -> Error { Error::from_u32(error.to_u32()) } }

#[cfg(feature = "winresult")] #[test] fn test_error_codes() {
    use winresult::ERROR;
    assert_eq!(ERROR_BAD_EXE_FORMAT     .to_u32(), ERROR::BAD_EXE_FORMAT    .to_u32());
    assert_eq!(ERROR_INVALID_PARAMETER  .to_u32(), ERROR::INVALID_PARAMETER .to_u32());
    assert_eq!(ERROR_MOD_NOT_FOUND      .to_u32(), ERROR::MOD_NOT_FOUND     .to_u32());
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)\]
/// GetProcAddress
///
pub(crate) mod get_proc_address {
    use super::*;

    extern "system" { fn GetProcAddress(hModule: Library, lpProcName: *const c_char) -> Option<NonNull<c_void>>; }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)\]
    /// GetProcAddress
    ///
    pub(crate) fn by_name(module: Library, proc_name: &CStr) -> Result<NonNull<c_void>, Error> {
        // SAFETY: ✔️
        //  - `hModule`     ✔️ is a valid, non-dangling, loaded hmodule, as implied by `Library`'s existence.
        //  - `lpProcName`  ✔️ is a valid, non-dangling, `\0`-terminated string containing no interior `\0`s, as implied by `CStr`'s existence.
        unsafe { GetProcAddress(module, proc_name.as_ptr()) }.ok_or_else(Error::get_last)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)\]
    /// GetProcAddress
    ///
    pub(crate) fn by_ordinal(module: Library, ordinal: u16) -> Result<NonNull<c_void>, Error> {
        // SAFETY: ✔️
        //  - `hModule`     ✔️ is a valid, non-dangling, loaded hmodule, as implied by `Library`'s existence.
        //  - `lpProcName`  ✔️ is a WORD/u16, meeting GetProcAddress's documented requirement:
        //                  "If this parameter is an ordinal value, it must be in the low-order word; the high-order word must be zero."
        unsafe { GetProcAddress(module, ordinal as usize as *mut _) }.ok_or_else(Error::get_last)
    }
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary)\]
/// FreeLibrary
///
/// Attempt to unload the library.
///
/// # Safety
/// ❌ This is a **fundamentally unsound** operation that **may do nothing** and **invalidates everything** ❌
///
/// See [`Library::close_unsafe_unsound_possible_noop_do_not_use_in_production`] for the full rant.
///
pub(crate) unsafe fn free_library(module: Library) -> Result<(), Error> {
    extern "system" { fn FreeLibrary(hModule: Library) -> u32; }
    match unsafe { FreeLibrary(module) } {
        0   => Err(Error::get_last()),
        1.. => Ok(()),
    }
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)\]
/// LoadLibraryW
pub(crate) fn load_library_w(file_name: &[u16]) -> Result<Library, Error> {
    extern "system" { fn LoadLibraryW(lpFileName: *const u16) -> Option<Library>; }
    unsafe { LoadLibraryW(wcstr0(file_name)?) }.ok_or_else(Error::get_last)
}



/// Validate a slice can be treated as a wide equivalent of [`CStr`] (no interior `\0`s, terminal `\0`.)
/// Returns [Err]\([ERROR_INVALID_PARAMETER]\) if it can't.
fn wcstr0(s0: &[u16]) -> Result<*const u16, Error> {
    match s0 {
        [ s @ .., 0 ] if !s.contains(&0)    => Ok(s0.as_ptr()),
        _                                 => Err(ERROR_INVALID_PARAMETER),
    }
}

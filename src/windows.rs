//! Microsoft Windows specific methods and types
#![allow(dead_code)] // I have not yet committed to exposing `*_w` as accepting `&[u16]`, so those are all pub(crate) instead of pub for now.



use crate::{Library, Never};

use core::ffi::{CStr, c_void, c_char};
use core::fmt::{self, Debug, Formatter};
use core::ptr::null_mut;


/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary)\]
/// FreeLibrary
///
/// # Safety
///
/// ❌ This is a **fundamentally unsound** operation that **may do nothing** and **invalidates everything** ❌
///
/// See [`Library::close_unsafe_unsound_possible_noop_do_not_use_in_production`] for the full rant.
///
pub(crate) unsafe fn free_library(module: Library) -> Result<(), Error> {
    extern "system" { fn FreeLibrary(hModule: Library) -> u32; }
    match FreeLibrary(module) {
        0 => Err(Error::get_last()),
        _ => Ok(()),
    }
}

#[allow(non_camel_case_types)] pub(crate) type TP_CALLBACK_INSTANCE = *mut c_void;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/threadpoolapiset/nf-threadpoolapiset-freelibrarywhencallbackreturns)\]
/// FreeLibraryWhenCallbackReturns
///
/// # Safety
///
/// ❌ This is a **fundamentally unsound** operation that **may do nothing** and **invalidates everything** ❌
///
/// See [`Library::close_unsafe_unsound_possible_noop_do_not_use_in_production`] for the full rant.
///
pub(crate) unsafe fn free_library_when_callback_returns(pci: &mut TP_CALLBACK_INSTANCE, r#mod: Library) {
    extern  "system" { fn FreeLibraryWhenCallbackReturns(pci: &mut TP_CALLBACK_INSTANCE, r#mod: Library); }
    FreeLibraryWhenCallbackReturns(pci, r#mod)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibraryandexitthread)\]
/// FreeLibraryAndExitThread
///
/// # Safety
///
/// ❌ This is a **fundamentally unsound** operation that **may do nothing** and **invalidates everything** ❌
///
/// See [`Library::close_unsafe_unsound_possible_noop_do_not_use_in_production`] for the full rant.
///
pub(crate) unsafe fn free_library_and_exit_thread(module: Library, exit_code: u32) -> ! {
    extern "system" { fn FreeLibraryAndExitThread(hModule: Library, exit_code: u32) -> !; }
    FreeLibraryAndExitThread(module, exit_code)
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlea)\]
/// GetModuleHandleA
///
pub fn get_module_handle_a(lib_file_name: &CStr) -> Result<Library, Error> {
    extern "system" { fn GetModuleHandleA(lpModuleName: *const c_char) -> Option<Library>; }
    //valid_wcstr0_or(lib_file_name, ERROR_INVALID_PARAMETER)?; // unnecessary: already enforced by CStr
    unsafe { GetModuleHandleA(lib_file_name.as_ptr()) }.ok_or_else(Error::get_last)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)\]
/// GetModuleHandleW
///
/// **Note:** `lib_file_name` should contain a terminal `\0`, but no interior `\0`s.
/// `minidl` will return `ERROR_INVALID_PARAMETER` (87) if this precondition is not met.
///
pub(crate) fn get_module_handle_w(lib_file_name: &[u16]) -> Result<Library, Error> {
    extern "system" { fn GetModuleHandleW(lpModuleName: *const u16   ) -> Option<Library>; }
    valid_wcstr0_or(lib_file_name, ERROR_INVALID_PARAMETER)?;
    unsafe { GetModuleHandleW(lib_file_name.as_ptr()) }.ok_or_else(Error::get_last)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandleexa)\]
/// GetModuleHandleExA
///
/// ### Safety
/// These values for `flags` may be unsound:
/// -   `GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS` &mdash; will misinterpret `lib_file_name`
/// -   `GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT` &mdash; could result in [`Library`] dangling
///
/// Additionally, new versions of windows could introduce new flags with new unsound meanings to `flags`.
///
pub unsafe fn get_module_handle_ex_a(flags: u32, lib_file_name: &CStr) -> Result<Library, Error> {
    extern "system" { fn GetModuleHandleExA(dwFlags: u32, lpModuleName: *const c_char, phModule: &mut Option<Library>) -> BOOL; }
    let mut library = None;
    //valid_wcstr0_or(lib_file_name, ERROR_INVALID_PARAMETER)?; // unnecessary: already enforced by CStr
    match GetModuleHandleExA(flags, lib_file_name.as_ptr(), &mut library) {
        0 => Err(Error::get_last()),
        _ => library.ok_or(ERROR_UNKNOWN),
    }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandleexw)\]
/// GetModuleHandleExW
///
/// **Note:** `lib_file_name` should contain a terminal `\0`, but no interior `\0`s.
/// `minidl` will return `ERROR_INVALID_PARAMETER` (87) if this precondition is not met.
///
/// ### Safety
/// These values for `flags` may be unsound:
/// -   `GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS` &mdash; will misinterpret `lib_file_name`
/// -   `GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT` &mdash; could result in [`Library`] dangling
///
/// Additionally, new versions of windows could introduce new flags with new unsound meanings to `flags`.
///
pub(crate) unsafe fn get_module_handle_ex_w(flags: u32, lib_file_name: &[u16]) -> Result<Library, Error> {
    extern "system" { fn GetModuleHandleExW(dwFlags: u32, lpModuleName: *const u16,    phModule: &mut Option<Library>) -> BOOL; }
    let mut library = None;
    valid_wcstr0_or(lib_file_name, ERROR_INVALID_PARAMETER)?;
    match GetModuleHandleExW(flags, lib_file_name.as_ptr(), &mut library) {
        0 => Err(Error::get_last()),
        _ => library.ok_or(ERROR_UNKNOWN),
    }
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibrarya)\]
/// LoadLibraryA
///
pub fn load_library_a(lib_file_name: &CStr) -> Result<Library, Error> {
    extern "system" { fn LoadLibraryA(lpFileName: *const c_char) -> Option<Library>; }
    //valid_wcstr0_or(lib_file_name, ERROR_INVALID_PARAMETER)?; // unnecessary: already enforced by CStr
    unsafe { LoadLibraryA(lib_file_name.as_ptr()) }.ok_or_else(Error::get_last)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)\]
/// LoadLibraryW
///
/// **Note:** `lib_file_name` should contain a terminal `\0`, but no interior `\0`s.
/// `minidl` will return `ERROR_INVALID_PARAMETER` (87) if this precondition is not met.
///
pub(crate) fn load_library_w(lib_file_name: &[u16]) -> Result<Library, Error> {
    extern "system" { fn LoadLibraryW(lpFileName: *const u16) -> Option<Library>; }
    valid_wcstr0_or(lib_file_name, ERROR_INVALID_PARAMETER)?;
    unsafe { LoadLibraryW(lib_file_name.as_ptr()) }.ok_or_else(Error::get_last)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexa)\]
/// LoadLibraryExA
///
/// ### Safety
/// These values for `flags` may be unsound:
/// -   `DONT_RESOLVE_DLL_REFERENCES` &mdash; "Do not use this value; [...]", likely to leave things in a partially initialized state.
/// -   `LOAD_LIBRARY_AS_DATAFILE[_EXCLUSIVE]` &mdash; ???
/// -   `LOAD_LIBRARY_AS_IMAGE_RESOURCE` &mdash; ???
///
/// Additionally, new versions of windows could introduce new flags with new unsound meanings to `flags`.
///
pub unsafe fn load_library_ex_a(lib_file_name: &CStr, file: Option<Never>, flags: u32) -> Result<Library, Error> {
    extern "system" { fn LoadLibraryExA(lpFileName: *const c_char, hFile: *mut c_void, dwFlags: u32) -> Option<Library>; }
    //valid_wcstr0_or(lib_file_name, ERROR_INVALID_PARAMETER)?; // unnecessary: already enforced by CStr
    unsafe { LoadLibraryExA(lib_file_name.as_ptr(), none2null(file), flags) }.ok_or_else(Error::get_last)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw)\]
/// LoadLibraryExW
///
/// **Note:** `lib_file_name` should contain a terminal `\0`, but no interior `\0`s.
/// `minidl` will return `ERROR_INVALID_PARAMETER` (87) if this precondition is not met.
///
/// ### Safety
/// These values for `flags` may be unsound:
/// -   `DONT_RESOLVE_DLL_REFERENCES` &mdash; "Do not use this value; [...]", likely to leave things in a partially initialized state.
/// -   `LOAD_LIBRARY_AS_DATAFILE[_EXCLUSIVE]` &mdash; ???
/// -   `LOAD_LIBRARY_AS_IMAGE_RESOURCE` &mdash; ???
///
/// Additionally, new versions of windows could introduce new flags with new unsound meanings to `flags`.
///
pub(crate) unsafe fn load_library_ex_w(lib_file_name: &[u16], file: Option<Never>, flags: u32) -> Result<Library, Error> {
    extern "system" { fn LoadLibraryExW(lpFileName: *const u16,    hFile: *mut c_void, dwFlags: u32) -> Option<Library>; }
    valid_wcstr0_or(lib_file_name, ERROR_INVALID_PARAMETER)?;
    unsafe { LoadLibraryExW(lib_file_name.as_ptr(), none2null(file), flags) }.ok_or_else(Error::get_last)
}



fn valid_wcstr0_or(string: &[u16], error: Error) -> Result<(), Error> {
    match string {
        [base @ .., nul] if !base.contains(&0) && *nul == 0 => Ok(()),
        _ => Err(error)
    }
}

fn none2null<T>(_none: Option<Never>) -> *mut T { null_mut() }



pub(crate) const ERROR_BAD_EXE_FORMAT           : Error = Error(0x00C1);
pub(crate) const ERROR_BUFFER_OVERFLOW          : Error = Error(0x006F);
pub(crate) const ERROR_INVALID_ORDINAL          : Error = Error(182);
pub(crate) const ERROR_INVALID_PARAMETER        : Error = Error(87);
pub(crate) const ERROR_MOD_NOT_FOUND            : Error = Error(0x007E);
pub(crate) const ERROR_UNKNOWN                  : Error = Error(0x027B);

type BOOL = u32;

extern "system" { pub(crate) fn GetProcAddress(hModule: *mut c_void, lpProcName: *const c_char) -> *mut c_void; }



#[cfg(feature = "winresult")]
pub use winresult::ErrorHResultOrCode as Error;

#[cfg(not(feature = "winresult"))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Error(pub(crate) u32);

#[cfg(not(feature = "winresult"))]
impl Error {
    pub(crate) fn get_last() -> Self {
        extern "system" { fn GetLastError() -> Error; }
        unsafe { GetLastError() }
    }

    pub(crate) fn as_str(&self) -> Option<&'static str> {
        Some(match *self {
            ERROR_BAD_EXE_FORMAT            => "ERROR_BAD_EXE_FORMAT",
            ERROR_BUFFER_OVERFLOW           => "ERROR_BUFFER_OVERFLOW",
            ERROR_INVALID_ORDINAL           => "ERROR_INVALID_ORDINAL",
            ERROR_INVALID_PARAMETER         => "ERROR_INVALID_PARAMETER",
            ERROR_MOD_NOT_FOUND             => "ERROR_MOD_NOT_FOUND",
            ERROR_UNKNOWN                   => "ERROR_UNKNOWN",
            _other                          => return None
        })
    }
}

#[cfg(not(feature = "winresult"))]
impl Debug for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if let Some(str) = self.as_str() {
            fmt.write_str(str)
        } else if self.0 < 0xFFFF {
            write!(fmt, "{}", self.0)
        } else {
            write!(fmt, "0x{:08X}", self.0)
        }
    }
}

#[cfg(not(feature = "winresult"))]
impl From<Error> for u32 {
    fn from(error: Error) -> Self { error.0 }
}

#[cfg(not(feature = "winresult"))]
#[test] fn error_vs_winresult() {
    use winresult::ERROR;
    assert_eq!(ERROR_BAD_EXE_FORMAT     .0, ERROR::BAD_EXE_FORMAT       .to_u32());
    assert_eq!(ERROR_BUFFER_OVERFLOW    .0, ERROR::BUFFER_OVERFLOW      .to_u32());
    assert_eq!(ERROR_INVALID_PARAMETER  .0, ERROR::INVALID_PARAMETER    .to_u32());
    assert_eq!(ERROR_MOD_NOT_FOUND      .0, ERROR::MOD_NOT_FOUND        .to_u32());
    assert_eq!(ERROR_UNKNOWN            .0, ERROR::UNKNOWN              .to_u32());
}

use core::ffi::{c_void, c_char};

pub(crate) const ERROR_BAD_EXE_FORMAT : i32 = 0x00C1;
pub(crate) const ERROR_MOD_NOT_FOUND  : i32 = 0x007E;
extern "system" {
    pub(crate) fn GetProcAddress(hModule: *mut c_void, lpProcName: *const c_char) -> *mut c_void;
    pub(crate) fn LoadLibraryW(lpFileName: *const u16) -> *mut c_void;
    pub(crate) fn FreeLibrary(hModule: *mut c_void) -> u32;
}

use core::ffi::{c_void, c_char};
use core::fmt::{self, Debug, Formatter};

pub(crate) const ERROR_BAD_EXE_FORMAT : Error = Error(0x00C1);
pub(crate) const ERROR_MOD_NOT_FOUND  : Error = Error(0x007E);
extern "system" {
    pub(crate) fn GetLastError() -> Error;
    pub(crate) fn GetProcAddress(hModule: *mut c_void, lpProcName: *const c_char) -> *mut c_void;
    pub(crate) fn LoadLibraryW(lpFileName: *const u16) -> *mut c_void;
    pub(crate) fn FreeLibrary(hModule: *mut c_void) -> u32;
}



#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub(crate) struct Error(pub u32);

impl Error {
    pub(crate) fn get_last() -> Self { unsafe { GetLastError() } }
}

impl Debug for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            ERROR_BAD_EXE_FORMAT    => fmt.write_str("ERROR_BAD_EXE_FORMAT"),
            ERROR_MOD_NOT_FOUND     => fmt.write_str("ERROR_MOD_NOT_FOUND"),
            Self(code)              => write!(fmt, "0x{code:08X}"),
        }
    }
}

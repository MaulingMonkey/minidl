use core::ffi::{c_void, c_char};
use core::fmt::{self, Debug, Formatter};

pub(crate) const ERROR_BAD_EXE_FORMAT : Error = Error::from_u32(0x00C1);
pub(crate) const ERROR_MOD_NOT_FOUND  : Error = Error::from_u32(0x007E);
extern "system" {
    pub(crate) fn GetProcAddress(hModule: *mut c_void, lpProcName: *const c_char) -> *mut c_void;
    pub(crate) fn LoadLibraryW(lpFileName: *const u16) -> *mut c_void;
    pub(crate) fn FreeLibrary(hModule: *mut c_void) -> u32;
}



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
    assert_eq!(ERROR_MOD_NOT_FOUND      .to_u32(), ERROR::MOD_NOT_FOUND     .to_u32());
}

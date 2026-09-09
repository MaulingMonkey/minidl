#![cfg(windows)]
#![allow(non_snake_case)] // Win32 fn names

use minidl::*;

use core::ffi::c_char;
use core::fmt::{self, Debug, Formatter};

#[cfg(    feature = "std" )] use std::io::Result;
#[cfg(not(feature = "std"))] type Result<T> = core::result::Result<T, IgnoreError>;

#[allow(dead_code)] // Imported methods not actually used
struct Example {
    OutputDebugStringA: unsafe extern "system" fn (_: *const c_char),
    Invalid_Optional:   Option<unsafe extern "system" fn (_: *const c_char)>,
    Invalid_Required:   unsafe extern "system" fn (_: *const c_char),
}

impl Example {
    pub fn new() -> Result<Self> {
        Self::from(Library::load("kernel32.dll")?)
    }

    pub fn from(lib: Library) -> Result<Self> {
        unsafe{Ok(Self{
            OutputDebugStringA: lib.sym(c"OutputDebugStringA")?,
            Invalid_Optional:   lib.sym_opt(c"Invalid_Optional"),
            Invalid_Required:   lib.sym(c"Invalid_Required")?,
        })}
    }
}

impl Debug for Example {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Example {{ ... }}")
    }
}

#[test] fn bad_load() {
    let e = Library::load("does_not_exist_invalid.dll").expect_err("Invalid DLL should've failed to load");
    let e = format!("{}", e);
    assert!(!cfg!(feature = "alloc") || e.contains("does_not_exist_invalid"), "{}", e);
}

#[test] fn bad_sym() {
    let _e = Example::new().expect_err("Example should've failed to load Invalid_Required");
    #[cfg(feature = "std")] {
        let e = format!("{_e}");
        assert!(!e.contains("Invalid_Optional"), "{}", e);
        assert!( e.contains("Invalid_Required"), "{}", e);
    }
}

#[test] fn ok_sym() {
    unsafe {
        let OutputDebugStringA : unsafe extern "system" fn (_: *const c_char)
            = Library::load("kernel32.dll").unwrap()
            .sym(c"OutputDebugStringA").unwrap();

        OutputDebugStringA(c"Hello, world!".as_ptr() as _);
    }
}

#[allow(dead_code)] #[derive(Debug)] struct IgnoreError(());
impl From<minidl::LoadLibraryError      > for IgnoreError { fn from(_: minidl::LoadLibraryError     ) -> Self { Self(()) } }
impl From<minidl::UnloadLibraryError    > for IgnoreError { fn from(_: minidl::UnloadLibraryError   ) -> Self { Self(()) } }
impl From<minidl::MissingSymbolError<'_>> for IgnoreError { fn from(_: minidl::MissingSymbolError   ) -> Self { Self(()) } }

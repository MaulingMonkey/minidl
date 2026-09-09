#![cfg(unix)]

use minidl::*;

use core::ffi::{c_char, c_int};
use core::fmt::{self, Debug, Formatter};

#[cfg(    feature = "std" )] use std::io::Result;
#[cfg(not(feature = "std"))] type Result<T> = core::result::Result<T, IgnoreError>;

#[allow(dead_code)] // Imported methods not actually used
struct Example {
    puts:               unsafe extern "C" fn (_: *const c_char) -> c_int,           // ✔️ correct signature
    invalid_optional:   Option<unsafe extern "C" fn (_: *const c_char) -> c_int>,   // ⚠️ should never exist, and never used, so any signature "should" be OK
    invalid_required:   unsafe extern "C" fn (_: *const c_char) -> c_int,           // ⚠️ should never exist, and never used, so any signature "should" be OK
}

impl Debug for Example {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Example {{ ... }}")
    }
}

impl Example {
    pub fn new() -> Result<Self> {
        Self::from(Library::load("/lib/x86_64-linux-gnu/libc.so.6")?)
    }

    pub fn from(lib: Library) -> Result<Self> {
        // SAFETY: ⚠️ see per-member notes in example struct
        unsafe{Ok(Self{
            puts:               lib.sym(c"puts")?,
            invalid_optional:   lib.sym_opt(c"invalid_optional"),
            invalid_required:   lib.sym(c"invalid_required")?,
        })}
    }
}

#[test] fn bad_load() {
    let e = Library::load("libdoes_not_exist_invalid.so").expect_err("Invalid SO should've failed to load");
    let e = format!("{}", e);
    assert!(!cfg!(feature = "alloc") || e.contains("does_not_exist_invalid"), "error does not contain library name: {e}");
}

#[test] fn load_unload() {
    if !std::env::var_os("CI").is_some() {
        let lib = Library::load("/lib/x86_64-linux-gnu/libc.so.6").expect("loading libc.so.6");
        // SAFETY: ✔️ not used in production, doesn't seem to crash from unloading libc.so.6 in testing on WSL either.
        unsafe { lib.close_unsafe_unsound_possible_noop_do_not_use_in_production() }.expect("unloading libc.so.6");
    }
}

#[test] fn bad_sym() {
    let _e = Example::new().expect_err("Example should've failed to load invalid_required");
    #[cfg(feature = "std")] {
        let e = format!("{_e}");
        assert!(!e.contains("invalid_optional"), "{e}");
        assert!( e.contains("invalid_required"), "{e}");
    }
}

#[test] fn ok_sym() {
    // SAFETY: ✔️ see bellow
    unsafe {
        // ✔️ correct signature
        let puts : unsafe extern "C" fn (_: *const c_char) -> c_int
            = Library::load("/lib/x86_64-linux-gnu/libc.so.6").unwrap()
            .sym(c"puts").unwrap();

        // ✔️ valid string, no other preconditions
        puts(c"Hello, world!".as_ptr().cast());
    }
}

#[allow(dead_code)] #[derive(Debug)] struct IgnoreError(());
impl From<minidl::LoadLibraryError      > for IgnoreError { fn from(_: minidl::LoadLibraryError     ) -> Self { Self(()) } }
impl From<minidl::UnloadLibraryError    > for IgnoreError { fn from(_: minidl::UnloadLibraryError   ) -> Self { Self(()) } }
impl From<minidl::MissingSymbolError<'_>> for IgnoreError { fn from(_: minidl::MissingSymbolError   ) -> Self { Self(()) } }

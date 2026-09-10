#![cfg(unix)]

use minidl::*;

use core::ffi::{c_char, c_int};
use core::fmt::{self, Debug, Formatter};

#[cfg(    feature = "std" )] use std::io::Result;
#[cfg(not(feature = "std"))] type Result<T> = core::result::Result<T, dev::StringError>;

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
    let e = Example::new().expect_err("Example should've failed to load invalid_required");
    let e = format!("{e}");
    assert!(!e.contains("invalid_optional"), "{e}");
    assert!( e.contains("invalid_required"), "{e}");
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

/// Nothing in dlerror()'s documentation implies the error messages are stable or unlocalized.
///
/// Run tests like this with e.g.:
/// ```sh
/// cargo test --no-default-features --features ""          --  --include-ignored
/// cargo test --no-default-features --features "alloc"     --  --include-ignored
/// cargo test --no-default-features --features "std"       --  --include-ignored
/// ```
///
#[ignore = "exact error messages returned by dlerror() are subject to change, this is only provided for manually testing to bring error messages into alignment"]
#[test] fn exact_dlerror_messages() {
    // LoadLibraryError
    let load_err = Library::load("libdoes_not_exist_invalid.so").expect_err("invalid library should've failed to load");
    assert_eq!(format!("{load_err}"), match cfg!(feature = "alloc") {
        true    => "could not load library: libdoes_not_exist_invalid.so: cannot open shared object file: No such file or directory",
        false   => "could not load library",
    });

    #[cfg(any(/* false */))] { // MissingSymbolError currently has no dlerror() payload, so don't bother inspecting it
        let libc = Library::load("/lib/x86_64-linux-gnu/libc.so.6").expect("unable to load libc");
        let sym_err = unsafe { libc.sym::<*mut core::ffi::c_void>(c"invalid_required") }.expect_err("libc unexpectedly contained invalid_required");
        assert_eq!(format!("{sym_err:?}"), "...");
    }

    #[cfg(any(/* false */))] { // UnloadLibraryError - yes, this is *incredibly* unsound.  In fact it just crashes with SIGSEGV: invalid memory reference.
        let unload_err = unsafe { minidl::Library::from_ptr(0x12345678_usize as *mut _).unwrap().close_unsafe_unsound_possible_noop_do_not_use_in_production().expect_err("invalid library should've failed to unload") };
        assert_eq!(format!("{unload_err:?}"), "...");
    }
}

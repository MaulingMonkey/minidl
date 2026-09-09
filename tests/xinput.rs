#![cfg(windows)]
#![allow(non_snake_case)] // XInput fn names

use minidl::*;

use core::ffi::c_void;

#[cfg(    feature = "std" )] use std::io::Result;
#[cfg(not(feature = "std"))] type Result<T> = core::result::Result<T, IgnoreError>;

#[allow(dead_code)] // Imported methods not actually used
struct XInput {
    XInputGetState:     unsafe extern "system" fn (dwUserIndex: u32, pState: *mut c_void) -> u32, // ⚠️ `pState` should perhaps be more specifically NonNull<XINPUT_STATE>, but meh.
    XInputGetStateEx:   unsafe extern "system" fn (dwUserIndex: u32, pState: *mut c_void) -> u32, // ⚠️ `pState` should perhaps be more specifically NonNull<XINPUT_STATE>, but meh.
}

impl XInput {
    pub fn new() -> Result<Self> { Self::from(Library::load("xinput1_3.dll")?) }
    pub fn from(lib: Library) -> Result<Self> {
        // SAFETY: ✔️ see per-member notes in example struct
        unsafe{Ok(Self{
            XInputGetState:     lib.sym(c"XInputGetState")?,
            XInputGetStateEx:   lib.sym_by_ordinal(100)?,
        })}
    }
}

#[test] fn load_unload() {
    if !std::env::var_os("CI").is_some() {
        let lib = Library::load("xinput1_3.dll").expect("loading xinput1_3.dll");
        // SAFETY: ✔️ not used in production, doesn't seem to crash from unloading XInput 1.3 in testing either.
        unsafe { lib.close_unsafe_unsound_possible_noop_do_not_use_in_production() }.expect("unloading xinput1_3.dll");
    }
}

#[test] fn ok_sym() {
    let xinput = XInput::new();
    if !std::env::var_os("CI").is_some() {
        xinput.expect("XInput");
    }
}

#[allow(dead_code)] #[derive(Debug)] struct IgnoreError(());
impl From<minidl::LoadLibraryError      > for IgnoreError { fn from(_: minidl::LoadLibraryError     ) -> Self { Self(()) } }
impl From<minidl::UnloadLibraryError    > for IgnoreError { fn from(_: minidl::UnloadLibraryError   ) -> Self { Self(()) } }
impl From<minidl::MissingSymbolError<'_>> for IgnoreError { fn from(_: minidl::MissingSymbolError   ) -> Self { Self(()) } }

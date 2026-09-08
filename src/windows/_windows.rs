use crate::Library;

use core::ffi::{CStr, c_void, c_char};
use core::fmt::{self, Debug, Formatter};
use core::ptr::NonNull;

include!("Error.rs");
include!("free_library.rs");
include!("get_proc_address.rs");
include!("load_library.rs");

/// Validate a slice can be treated as a wide equivalent of [`CStr`] (no interior `\0`s, terminal `\0`.)
/// Returns [Err]\([ERROR_INVALID_PARAMETER]\) if it can't.
///
fn wcstr0(s0: &[u16]) -> Result<*const u16, Error> {
    match s0 {
        [ s @ .., 0 ] if !s.contains(&0)    => Ok(s0.as_ptr()),
        _                                 => Err(ERROR_INVALID_PARAMETER),
    }
}

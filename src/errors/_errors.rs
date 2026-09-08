//! [`minidl`](crate) errors.  Note these are re-exported by the crate root.

use crate::*;

use core::fmt::{self, Debug, Display, Formatter};

// support
include!("ErrorPath.rs");
include!("Symbol.rs");

// actual errors
include!("LoadLibraryError.rs");
include!("MissingSymbolError.rs");
include!("UnloadLibraryError.rs");

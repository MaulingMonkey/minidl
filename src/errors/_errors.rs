//! [`minidl`](crate) errors.  Note these are re-exported by the crate root.

use crate::*;

use core::fmt::{self, Debug, Display, Formatter};

include!("ErrorPath.rs");

include!("LoadLibraryError.rs");
include!("MissingSymbolError.rs");
include!("UnloadLibraryError.rs");

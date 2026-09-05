//! [`minidl`](crate) errors.  Note these are re-exported by the crate root.

use crate::*;

use core::fmt::{self, Debug, Display, Formatter};



/// A requested symbol did not exist inside a given [`Library`].
///
/// ## Returned by
/// -   [`Library::sym`]
/// -   [`Library::sym_by_ordinal`]
///
#[derive(Clone, Debug)] pub struct MissingSymbolError<'a> {
    pub(crate) symbol: Symbol<'a>,
}

impl Display for MissingSymbolError<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.symbol {
            Symbol::Name(name)          => write!(fmt, "Symbol {name:?} missing from library"),
            Symbol::Ordinal(ordinal)    => write!(fmt, "Symbol @{ordinal} missing from library"),
        }
    }
}

impl std::error::Error for MissingSymbolError<'_> {
    fn description(&self) -> &str { "symbol missing from library" }
}

impl<'a> From<MissingSymbolError<'a>> for std::io::Error {
    fn from(value: MissingSymbolError<'a>) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("{value}"))
    }
}




/// A [`Library`] failed to unload.
///
/// ## Returned by
/// -   [`Library::close_unsafe_unsound_possible_noop_do_not_use_in_production`]
///
#[derive(Clone, Debug)] pub struct UnloadLibraryError {
    #[cfg(unix)]    pub(crate) dlerror: std::sync::Arc<str>,
    #[cfg(windows)] pub(crate) error: windows::Error,

    #[cfg(not(any(unix, windows)))] pub(crate) _non_exhaustive: (),
}

impl Display for UnloadLibraryError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        #[cfg(unix      )] return write!(fmt, "could not unload library: {}", self.dlerror);
        #[cfg(windows   )] return write!(fmt, "could not unload library (error code: {:?})", self.error);
        // other platforms: NYI
    }
}

impl std::error::Error for UnloadLibraryError {
    fn description(&self) -> &str { "could not unload library" }
}

impl From<UnloadLibraryError> for std::io::Error {
    fn from(error: UnloadLibraryError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error)
    }
}

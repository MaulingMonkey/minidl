//! [`minidl`](crate) errors.  Note these are re-exported by the crate root.

use crate::*;

use core::fmt::{self, Debug, Display, Formatter};



/// A [`Library`] failed to load.
///
/// ## Returned by
/// -   [`Library::load`]
///
#[derive(Clone, Debug)] pub struct LoadLibraryError {
    #[cfg(unix)]    pub(crate) dlerror: std::sync::Arc<str>,
    #[cfg(windows)] pub(crate) error:   windows::Error,
    #[cfg(windows)] pub(crate) path:    std::path::PathBuf,

    #[cfg(not(any(unix, windows)))] pub(crate) _not_supported: (),
}

impl Display for LoadLibraryError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        #[cfg(unix)] return write!(fmt, "could not load library: {}", self.dlerror);
        #[cfg(windows)] {
            let path = self.path.display();
            return match self.error {
                ERROR_BAD_EXE_FORMAT    => write!(fmt, "could not load library: ERROR_BAD_EXE_FORMAT returned loading {path} (wrong architecture? x86 on x86-64 or vicea versa?)"),
                error                   => write!(fmt, "could not load library: {error:?} returned loading {path}"),
            };
        }
    }
}

impl std::error::Error for LoadLibraryError {
    fn description(&self) -> &str {
        #[cfg(unix)] return &*self.dlerror;
        #[cfg(windows)] return match self.error {
            ERROR_BAD_EXE_FORMAT    => "could not load library: ERROR_BAD_EXE_FORMAT (typically the DLL architecture doesn't match the process architecture)",
            ERROR_MOD_NOT_FOUND     => "could not load library: ERROR_MOD_NOT_FOUND (wrong path or no such library)",
            _other                  => "could not load library (unknown error)",
        };
        #[allow(unreachable_code)] "could not load library: minidl not implemented for this platform"
    }
}

impl From<LoadLibraryError> for std::io::Error {
    fn from(error: LoadLibraryError) -> Self {
        use std::io::{Error, ErrorKind};
        #[cfg(unix)] return Error::new(ErrorKind::Other, error); // TODO: consider parsing `error.dlerror` for keywords to set ErrorKind? ...no, that's probably a bad idea
        #[cfg(windows)] return Error::new(match error.error {
            ERROR_BAD_EXE_FORMAT    => ErrorKind::InvalidData,
            ERROR_MOD_NOT_FOUND     => ErrorKind::NotFound,
            _other                  => ErrorKind::Other,
        }, error);
        #[allow(unreachable_code)] Error::new(ErrorKind::Unsupported, "minidl doesn't implement loading libraries on this platform")
    }
}



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

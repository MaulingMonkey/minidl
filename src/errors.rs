//! [`minidl`](crate) errors.  Note these are re-exported by the crate root.

use crate::*;

use core::fmt::{self, Debug, Display, Formatter};



/// A [`Library`] failed to load.
///
/// ## Returned by
/// -   [`Library::load`]
///
#[derive(Debug)] #[non_exhaustive] pub struct LoadLibraryError {
    #[cfg(all(unix, feature = "alloc"   ))] pub(crate) dlerror: alloc::sync::Arc<str>,
    #[cfg(all(windows                   ))] pub(crate) error:   windows::Error,
    #[cfg(all(windows, feature = "std"  ))] pub(crate) path:    std::path::PathBuf,
}

impl Display for LoadLibraryError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        #[cfg(unix)] {
            #[cfg(    feature = "alloc" )] return write!(fmt, "could not load library: {}", self.dlerror);
            #[cfg(not(feature = "alloc"))] return write!(fmt, "could not load library");
        }

        #[cfg(windows)] {
            #[cfg(feature = "std")] let path = self.path.display();

            #[cfg(feature = "std")] return match self.error {
                ERROR_BAD_EXE_FORMAT    => write!(fmt, "could not load library: ERROR_BAD_EXE_FORMAT loading {path} (wrong architecture? x86 on x86-64 or vicea versa?)"),
                error                   => write!(fmt, "could not load library: {error:?} loading {path}"),
            };

            #[cfg(not(feature = "std"))] return match self.error {
                ERROR_BAD_EXE_FORMAT    => write!(fmt, "could not load library: ERROR_BAD_EXE_FORMAT (wrong architecture? x86 on x86-64 or vicea versa?)"),
                error                   => write!(fmt, "could not load library: {error:?}"),
            };
        }
    }
}

#[cfg(feature = "std")] impl std::error::Error for LoadLibraryError {
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

#[cfg(feature = "std")] impl From<LoadLibraryError> for std::io::Error {
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
#[derive(Debug)] pub struct MissingSymbolError<'symbol> {
    pub(crate) symbol: Symbol<'symbol>,
}

impl Display for MissingSymbolError<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.symbol {
            Symbol::Name(name)          => write!(fmt, "Symbol {name:?} missing from library"),
            Symbol::Ordinal(ordinal)    => write!(fmt, "Symbol @{ordinal} missing from library"),
        }
    }
}

#[cfg(feature = "std")] impl std::error::Error for MissingSymbolError<'_> {
    fn description(&self) -> &str { "symbol missing from library" }
}

#[cfg(feature = "std")] impl<'symbol> From<MissingSymbolError<'symbol>> for std::io::Error {
    fn from(value: MissingSymbolError<'symbol>) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, std::format!("{value}"))
    }
}




/// A [`Library`] failed to unload.
///
/// ## Returned by
/// -   [`Library::close_unsafe_unsound_possible_noop_do_not_use_in_production`]
///
#[derive(Debug)] #[non_exhaustive] pub struct UnloadLibraryError {
    #[cfg(all(unix, feature = "alloc"   ))] pub(crate) dlerror: alloc::sync::Arc<str>,
    #[cfg(all(windows                   ))] pub(crate) error: windows::Error,
}

impl Display for UnloadLibraryError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        #[cfg(unix)] {
            #[cfg(    feature = "alloc" )] return write!(fmt, "could not unload library: {}", self.dlerror);
            #[cfg(not(feature = "alloc"))] return write!(fmt, "could not unload library");
        }
        #[cfg(windows)] return write!(fmt, "could not unload library (error code: {:?})", self.error);
        // other platforms: NYI
    }
}

#[cfg(feature = "std")] impl std::error::Error for UnloadLibraryError {
    fn description(&self) -> &str { "could not unload library" }
}

#[cfg(feature = "std")] impl From<UnloadLibraryError> for std::io::Error {
    fn from(error: UnloadLibraryError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error)
    }
}

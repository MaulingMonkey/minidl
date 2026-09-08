//! [`minidl`](crate) errors.  Note these are re-exported by the crate root.

use crate::*;

use core::fmt::{self, Debug, Display, Formatter};



/// A [`Library`] failed to load.
///
/// ## Returned by
/// -   [`Library::load`]
///
#[derive(Debug)] #[non_exhaustive] pub struct LoadLibraryError {
    #[cfg(all(unix,     feature = "alloc"   ))] pub(crate) dlerror: Option<alloc::ffi::CString>,
    #[cfg(all(unix, not(feature = "alloc")  ))] pub(crate) dlerror: Option<&'static core::ffi::CStr>,
    #[cfg(all(windows                       ))] pub(crate) error:   windows::Error,
    #[cfg(windows)] #[allow(dead_code)]         pub(crate) path:    ErrorPath,
}

impl Display for LoadLibraryError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        #[cfg(unix)] return match self.dlerror.as_ref() {
            Some(err)   => write!(fmt, "could not load library: {}", CStrDisplay(err)),
            None        => write!(fmt, "could not load library"),
        };

        #[cfg(windows)] {
            #[cfg(feature = "alloc")] let path = &self.path;

            #[cfg(feature = "alloc")] return match self.error {
                ERROR_BAD_EXE_FORMAT    => write!(fmt, "could not load library: ERROR_BAD_EXE_FORMAT loading {path} (wrong architecture? x86 on x86-64 or vicea versa?)"),
                error                   => write!(fmt, "could not load library: {error:?} loading {path}"),
            };

            #[cfg(not(feature = "alloc"))] return match self.error {
                ERROR_BAD_EXE_FORMAT    => write!(fmt, "could not load library: ERROR_BAD_EXE_FORMAT (wrong architecture? x86 on x86-64 or vicea versa?)"),
                error                   => write!(fmt, "could not load library: {error:?}"),
            };
        }
    }
}

#[cfg(feature = "std")] impl std::error::Error for LoadLibraryError {
    fn description(&self) -> &str {
        #[cfg(unix)] return match self.dlerror.as_ref().map(|cs| cs.to_str()) {
            Some(Ok(utf8))  => utf8,
            Some(Err(_))    => "could not load library (and dlerror() returned non-utf8 error message)",
            None            => "could not load library (unknown error)",
        };
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
    #[cfg(all(unix,     feature = "alloc"   ))] pub(crate) dlerror: Option<alloc::ffi::CString>,
    #[cfg(all(unix, not(feature = "alloc")  ))] pub(crate) dlerror: Option<&'static core::ffi::CStr>,
    #[cfg(all(unix                          ))] pub(crate) ret:     core::num::NonZero<core::ffi::c_int>,
    #[cfg(all(windows                       ))] pub(crate) error:   windows::Error,
}

impl Display for UnloadLibraryError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        #[cfg(unix)] return {
            write!(fmt, "could not unload library: dlclose(...) returned {}", self.ret)?;
            if let Some(dlerror) = self.dlerror.as_ref() {
                write!(fmt, " ({})", CStrDisplay(dlerror))?;
            }
            Ok(())
        };
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



pub(crate) enum ErrorPath {
    Unknown, // path not available (no `feature = "alloc"`?)
    #[cfg(feature = "alloc" )] C(alloc::ffi::CString),
    #[cfg(feature = "alloc" )] Utf8(alloc::string::String),
    #[cfg(feature = "std"   )] OS(std::path::PathBuf), // MSRV doesn't implement OsStr::display, only Path::display, so we choose the later
}

impl From<& str> for ErrorPath { fn from(_path: & str) -> Self { #[cfg(feature = "alloc")] return ErrorPath::Utf8(_path.into()); #[allow(unreachable_code)] ErrorPath::Unknown } }
impl From<&CStr> for ErrorPath { fn from(_path: &CStr) -> Self { #[cfg(feature = "alloc")] return ErrorPath::C(   _path.into()); #[allow(unreachable_code)] ErrorPath::Unknown } }
#[cfg(feature = "alloc" )] impl From<alloc::string::String  > for ErrorPath { fn from(path: alloc::string::String   ) -> Self { Self::Utf8(path.into()) } }
#[cfg(feature = "alloc" )] impl From<alloc::ffi::CString    > for ErrorPath { fn from(path: alloc::ffi::CString     ) -> Self { Self::C(path.into()) } }
#[cfg(feature = "std"   )] impl From<&std::path::Path       > for ErrorPath { fn from(path: &std::path::Path        ) -> Self { Self::OS(path.into()) } }
#[cfg(feature = "std"   )] impl From< std::path::PathBuf    > for ErrorPath { fn from(path:  std::path::PathBuf     ) -> Self { Self::OS(path.into()) } }
#[cfg(feature = "std"   )] impl From<&std::ffi::OsStr       > for ErrorPath { fn from(path: &std::ffi::OsStr        ) -> Self { Self::OS(std::path::Path::new(path).into()) } }
#[cfg(feature = "std"   )] impl From< std::ffi::OsString    > for ErrorPath { fn from(path:  std::ffi::OsString     ) -> Self { Self::OS(std::path::PathBuf::from(path).into()) } }

impl Debug for ErrorPath {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::Unknown                               => fmt.write_str("???"),
            #[cfg(feature = "alloc" )] Self::C(str)     => Debug::fmt(str, fmt),
            #[cfg(feature = "alloc" )] Self::Utf8(str)  => Debug::fmt(str, fmt),
            #[cfg(feature = "std"   )] Self::OS(str)    => Debug::fmt(str, fmt),
        }
    }
}

impl Display for ErrorPath {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::Unknown                               => fmt.write_str("???"),
            #[cfg(feature = "alloc" )] Self::C(str)     => match str.to_str() {
                // TODO: replace all this with CStr::display if it stabilizes
                Ok(str) => Display::fmt(str, fmt),
                Err(_) => {
                    use core::fmt::Write;
                    for b in str.to_bytes().iter().copied() {
                        fmt.write_char(match b {
                            0    ..= 0x7F   => char::from(b),
                            0x80 ..         => '?',
                        })?;
                    }
                    Ok(())
                }
            },
            #[cfg(feature = "alloc" )] Self::Utf8(str)  => Display::fmt(str, fmt),
            #[cfg(feature = "std"   )] Self::OS(str)    => Display::fmt(&str.display(), fmt),
        }
    }
}

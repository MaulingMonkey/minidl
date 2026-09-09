/// A [`Library`] failed to load.
///
/// ## Returned by
/// -   [`Library::load`]
///
#[derive(Debug)] #[non_exhaustive] pub struct LoadLibraryError {
    #[cfg(all(unix,     feature = "alloc"   ))] pub(crate) dlerror: Option<alloc::ffi::CString>,
    #[cfg(all(unix, not(feature = "alloc")  ))] pub(crate) dlerror: Option<&'static core::ffi::CStr>,
    #[cfg(all(windows                       ))] pub(crate) error:   windows::Error,
    #[cfg(all(windows                       ))] pub(crate) path:    ErrorPath,
}

impl Display for LoadLibraryError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        #[cfg(unix)] return match self.dlerror.as_ref() {
            Some(err)   => write!(fmt, "could not load library: {}", CStrDisplay(err)),
            None        => write!(fmt, "could not load library"),
        };

        #[cfg(windows)] return {
            use windows::*;

            let path = &self.path;
            if core::matches!(path, ErrorPath::Unknown) {
                match self.error {
                    ERROR_BAD_EXE_FORMAT    => write!(fmt, "could not load library: ERROR_BAD_EXE_FORMAT (wrong architecture? x86 on x86-64 or vicea versa?)"),
                    error                   => write!(fmt, "could not load library: {error:?}"),
                }
            } else {
                match self.error {
                    ERROR_BAD_EXE_FORMAT    => write!(fmt, "could not load library: ERROR_BAD_EXE_FORMAT loading {path} (wrong architecture? x86 on x86-64 or vicea versa?)"),
                    error                   => write!(fmt, "could not load library: {error:?} loading {path}"),
                }
            }
        };
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
            windows::ERROR_BAD_EXE_FORMAT   => "could not load library: ERROR_BAD_EXE_FORMAT (typically the DLL architecture doesn't match the process architecture)",
            windows::ERROR_MOD_NOT_FOUND    => "could not load library: ERROR_MOD_NOT_FOUND (wrong path or no such library)",
            _other                          => "could not load library (unknown error)",
        };
        #[allow(unreachable_code)] "could not load library: minidl not implemented for this platform"
    }
}

#[cfg(feature = "std")] impl From<LoadLibraryError> for std::io::Error {
    fn from(error: LoadLibraryError) -> Self {
        use std::io::{Error, ErrorKind};
        #[cfg(unix)] return Error::new(ErrorKind::Other, error); // TODO: consider parsing `error.dlerror` for keywords to set ErrorKind? ...no, that's probably a bad idea
        #[cfg(windows)] return Error::new(match error.error {
            windows::ERROR_BAD_EXE_FORMAT   => ErrorKind::InvalidData,
            windows::ERROR_MOD_NOT_FOUND    => ErrorKind::NotFound,
            _other                          => ErrorKind::Other,
        }, error);
        #[allow(unreachable_code)] Error::new(ErrorKind::Unsupported, "minidl doesn't implement loading libraries on this platform")
    }
}

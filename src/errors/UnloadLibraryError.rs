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

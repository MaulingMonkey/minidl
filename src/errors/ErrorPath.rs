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

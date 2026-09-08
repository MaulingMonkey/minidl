/// A [Library] name (e.g. `"kernel32.dll"`) or path (e.g. `"path/to/my/lib.so"`.)
///
/// Can be passed to <code>[Library]::[load](Library::load)</code>.  Must be a <code>&amp;[str]</code> unless built with:
/// -   `feature = "alloc"` (allows <code>\[&amp;\][alloc::string::String]</code>)
/// -   `feature = "std"` (allows <code>\[&amp;\][std::path::Path]\[[Buf](std::path::PathBuf)\]</code> and <code>\[&amp;\][std::ffi::OsStr]\[[ing](std::ffi::OsString)\]</code>)
///
#[allow(private_bounds)]
pub trait NameOrPath : sealed::NameOrPath {}

// Explicit enumeration for better generation of documentation (a wildcard impl would have an opaque "Implementors" section):
#[cfg(all()             )] impl NameOrPath for &               str      {}
#[cfg(feature = "alloc" )] impl NameOrPath for &alloc::string::String   {}
#[cfg(feature = "alloc" )] impl NameOrPath for  alloc::string::String   {}
// XXX: we could *maybe* support CStr[ing] on unix?  However:
//  - CStr encoding is ambiguous on Windows (utf8? windows-1252? cp437?)
//  - Lack of AsRef<NStr> would require special casing.
#[cfg(feature = "std"   )] impl NameOrPath for &std::path::Path         {}
#[cfg(feature = "std"   )] impl NameOrPath for &std::path::PathBuf      {}
#[cfg(feature = "std"   )] impl NameOrPath for  std::path::PathBuf      {}
#[cfg(feature = "std"   )] impl NameOrPath for &std::ffi::OsStr         {}
#[cfg(feature = "std"   )] impl NameOrPath for &std::ffi::OsString      {}
#[cfg(feature = "std"   )] impl NameOrPath for  std::ffi::OsString      {}

mod sealed {
    #![allow(private_bounds)]
    pub(crate) trait NameOrPath : Into<crate::errors::ErrorPath> + AsRef<crate::NStr> {}
    impl<T: Into<crate::errors::ErrorPath> + AsRef<crate::NStr>> NameOrPath for T {}
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)\]
/// LoadLibraryW
///
pub(crate) fn load_library_w(file_name: &[u16]) -> Result<Library, Error> {
    extern "system" { fn LoadLibraryW(lpFileName: *const u16) -> Option<Library>; }
    unsafe { LoadLibraryW(wcstr0(file_name)?) }.ok_or_else(Error::get_last)
}

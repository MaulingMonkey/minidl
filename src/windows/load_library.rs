/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)\]
/// LoadLibraryW
///
pub(crate) fn load_library_w(file_name: &crate::WCStr) -> Result<Library, Error> {
    extern "system" { fn LoadLibraryW(lpFileName: *const u16) -> Option<Library>; }
    // SAFETY: ✔️ `file_name` is a valid, non-dangling, `\0`-terminated string containing no interior `\0`s, as implied by `WCStr`'s existence.
    unsafe { LoadLibraryW(file_name.as_ptr()) }.ok_or_else(Error::get_last)
}

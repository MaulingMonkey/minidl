/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)\]
/// GetProcAddress
///
pub(crate) mod get_proc_address {
    use super::*;

    extern "system" { fn GetProcAddress(hModule: Library, lpProcName: *const c_char) -> Option<NonNull<c_void>>; }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)\]
    /// GetProcAddress
    ///
    pub(crate) fn by_name(module: Library, proc_name: &CStr) -> Result<NonNull<c_void>, Error> {
        // SAFETY: ✔️
        //  - `hModule`     ✔️ is a valid, non-dangling, loaded hmodule, as implied by `Library`'s existence.
        //  - `lpProcName`  ✔️ is a valid, non-dangling, `\0`-terminated string containing no interior `\0`s, as implied by `CStr`'s existence.
        unsafe { GetProcAddress(module, proc_name.as_ptr()) }.ok_or_else(Error::get_last)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)\]
    /// GetProcAddress
    ///
    pub(crate) fn by_ordinal(module: Library, ordinal: u16) -> Result<NonNull<c_void>, Error> {
        // SAFETY: ✔️
        //  - `hModule`     ✔️ is a valid, non-dangling, loaded hmodule, as implied by `Library`'s existence.
        //  - `lpProcName`  ✔️ is a WORD/u16, meeting GetProcAddress's documented requirement:
        //                  "If this parameter is an ordinal value, it must be in the low-order word; the high-order word must be zero."
        unsafe { GetProcAddress(module, ordinal as usize as *mut _) }.ok_or_else(Error::get_last)
    }
}

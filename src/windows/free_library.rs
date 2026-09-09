/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary)\]
/// FreeLibrary
///
/// Attempt to unload the library.
///
/// # Safety
/// ❌ This is a **fundamentally unsound** operation that **may do nothing** and **invalidates everything** ❌
///
/// See [`Library::close_unsafe_unsound_possible_noop_do_not_use_in_production`] for the full rant.
///
pub(crate) unsafe fn free_library(module: Library) -> Result<(), Error> {
    extern "system" { fn FreeLibrary(hModule: Library) -> u32; }
    // SAFETY: ❌ this is incredibly unsound (see fn docs)
    match unsafe { FreeLibrary(module) } {
        0   => Err(Error::get_last()),
        1.. => Ok(()),
    }
}

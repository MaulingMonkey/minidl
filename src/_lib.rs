#![doc = include_str!("../Readme.md")]
#![no_std]
#![deny(non_snake_case, unreachable_patterns)] // catch typoed match ... { ERROR_... => ... }

#[cfg(feature = "alloc")]   extern crate alloc;
#[cfg(feature = "std")]     extern crate std;

#[path = "errors/_errors.rs"] pub mod errors; #[doc(hidden)] pub use errors::*;
#[path = "util/_util.rs"] mod util; #[allow(unused_imports)] pub(crate) use util::*;

#[cfg(unix   )] mod unix   ; #[cfg(unix   )] use unix::*;
#[cfg(windows)] #[path = "windows/_windows.rs"] mod windows; #[cfg(windows)] use windows::*;

use core::ffi::{CStr, c_void};
use core::mem::{align_of, size_of, transmute_copy};
use core::ptr::NonNull;

/// A loaded library handle.
///
/// # Constructors
/// -   [`Library::load`]               &mdash; Load a library, forever, or return <code>[Err]\([std::io::Error])</code>.
///
/// # Symbols (most of these functions implicitly transmute! Use extreme caution.)
/// -   [`Library::has_sym`]            &mdash; Check if a symbol, `c"name"`, exists in the library.
/// -   [`Library::sym`]                &mdash; Load a symbol from the library by `c"name"`, or return <code>[Err]\([MissingSymbolError])</code>.
/// -   [`Library::sym_opt`]            &mdash; Load a symbol from the library by `c"name"`, or return [`None`].
/// -   [`Library::sym_by_ordinal`]     &mdash; Load a symbol from the library by ordinal, or return <code>[Err]\([MissingSymbolError])</code>.
/// -   [`Library::sym_opt_by_ordinal`] &mdash; Load a symbol from the library by ordinal, or return [`None`].
///
/// # Interop
/// -   [`Library::from_ptr`]           &mdash; Wrap a forever-loaded library in [`Library`] for interop purpouses.
/// -   [`Library::from_non_null`]      &mdash; Wrap a forever-loaded library in [`Library`] for interop purpouses.
/// -   [`Library::as_ptr`]             &mdash; Return a raw handle pointer for interop purpouses.
/// -   [`Library::as_non_null`]        &mdash; Return a raw handle pointer for interop purpouses.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Library(NonNull<c_void>);
unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl Library {
    /// Load a library, forever.
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `LoadLibraryW(path)`
    /// | Unix      | `dlopen(path, ...)`
    #[cfg(feature = "std")] // required for Path[Buf]
    pub fn load(path: impl AsRef<std::path::Path> + Into<std::path::PathBuf>) -> core::result::Result<Self, LoadLibraryError> {
        #[cfg(windows)] return {
            use std::os::windows::ffi::OsStrExt;
            let filename = path.as_ref().as_os_str().encode_wide().chain([0].iter().copied()).collect::<alloc::vec::Vec<u16>>();
            windows::load_library_w(&filename).map_err(|error| LoadLibraryError { error, path: path.into().into() })
        };

        #[cfg(unix)] return {
            use std::os::unix::ffi::OsStrExt;
            let filename = path.as_ref().as_os_str().as_bytes().iter().copied().chain([0].iter().copied()).collect::<alloc::vec::Vec<u8>>();
            unix::dlerror::clear();
            unsafe { unix::dlopen(CStr::from_bytes_with_nul(&filename).map_err(|_| LoadLibraryError { dlerror: unix::dlerror::to_cstring() })?, RTLD_LAZY) }
                .map_err(|dlerror| LoadLibraryError { dlerror })
        };

        #[cfg(not(any(unix, windows)))] LoadLibraryError { _not_supported: () }
    }

    /// Wrap a forever-loaded library in [`Library`] for interop purpouses.
    ///
    /// Wrap a [`winapi::shared::minwindef::HMODULE`](https://docs.rs/winapi/0.3/winapi/shared/minwindef/type.HMODULE.html) with `Library::from_ptr(handle.cast())`.<br>
    /// Wrap a [`windows::Win32::Foundation::HMODULE`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HMODULE.html) with `Library::from_ptr(handle.0 as _)`.
    ///
    /// # Safety
    ///
    /// If `handle` is not null, it is expected to be a valid library handle for the duration of the program.
    ///
    /// # Platform
    ///
    /// | OS        | Expects   |
    /// | --------- | --------- |
    /// | Windows   | [`libloaderapi.h`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/)-compatible `HMODULE`
    /// | Unix      | [`dlfcn.h`](https://pubs.opengroup.org/onlinepubs/7908799/xsh/dlfcn.h.html)-compatible handle
    pub unsafe fn from_ptr(handle: *mut c_void) -> Option<Self> { Some(Self::from_non_null(NonNull::new(handle)?)) }

    /// Wrap a forever-loaded library in [`Library`] for interop purpouses.
    ///
    /// # Safety
    ///
    /// `handle` is expected to be a valid library handle for the duration of the program.
    ///
    /// # Platform
    ///
    /// | OS        | Expects   |
    /// | --------- | --------- |
    /// | Windows   | [`libloaderapi.h`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/)-compatible `HMODULE`
    /// | Unix      | [`dlfcn.h`](https://pubs.opengroup.org/onlinepubs/7908799/xsh/dlfcn.h.html)-compatible handle
    pub unsafe fn from_non_null(handle: NonNull<c_void>) -> Self { Self(handle) }

    /// Return a raw handle pointer for interop purpouses.
    ///
    /// Acquire a [`winapi::shared::minwindef::HMODULE`](https://docs.rs/winapi/0.3/winapi/shared/minwindef/type.HMODULE.html) with `handle.as_ptr() as HMODULE`.<br>
    /// Acquire a [`windows::Win32::Foundation::HMODULE`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HMODULE.html) with `HMODULE(handle.as_ptr() as _)`.
    ///
    /// # Safety
    ///
    /// Don't use this pointer to unload the library.
    pub fn as_ptr(&self) -> *mut c_void { self.0.as_ptr() }

    /// Return a raw handle pointer for interop purpouses.
    ///
    /// # Safety
    ///
    /// Don't use this pointer to unload the library.
    pub fn as_non_null(&self) -> NonNull<c_void> { self.0 }

    /// Load a symbol from the library.
    /// Limiting symbol names to basic ASCII is likely wise.
    ///
    /// # Safety
    ///
    /// This function implicitly transmutes!  Use extreme caution.
    ///
    /// # Platform
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `GetProcAddress(..., name)`
    /// | Unix      | `dlsym(..., name)`
    pub unsafe fn sym<'symbol, T>(self, name: &'symbol CStr) -> core::result::Result<T, MissingSymbolError<'symbol>> {
        self.sym_opt(name).ok_or_else(|| MissingSymbolError { symbol: Symbol::Name(name) })
    }

    /// Load a symbol from the library.
    /// Limiting symbol names to basic ASCII is likely wise.
    ///
    /// # Safety
    ///
    /// This function implicitly transmutes!  Use extreme caution.
    ///
    /// # Platform
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `GetProcAddress(..., name)`
    /// | Unix      | `dlsym(..., name)`
    pub unsafe fn sym_opt<T>(self, name: &CStr) -> Option<T> {
        let _ = FnPtrChecks::<T>::ASSERT;

        #[cfg(windows)] let func = windows::get_proc_address::by_name(self, name).ok()?;
        #[cfg(unix   )] let func = unix::dlsym(self, name).ok()?;

        // SAFETY: ✔️
        //  * `T`   ✔️ is asserted to be the same alignment and size as `*mut c_void` via assert at start of function
        //  * `T`   ✔️ is assumed compatible with `*mut c_void` per the documented safety contract of this unsafe function
        Some(transmute_copy::<NonNull<c_void>, T>(&func))
    }

    /// Load a symbol from the library by ordinal.
    ///
    /// # Safety
    ///
    /// This function implicitly transmutes!  Use extreme caution.
    /// Additionally, DLL ordinals are typically unstable and might change between minor versions of the same DLL, breaking your imports in nastily subtle ways.
    /// If a function name is available, use it instead!
    ///
    /// # Platform
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `GetProcAddress(..., MAKEINTRESOURCE(ordinal))`
    /// | <strike>Unix</strike> | `Err(...)`
    pub unsafe fn sym_by_ordinal<T>(self, ordinal: u16) -> core::result::Result<T, MissingSymbolError<'static>> {
        self.sym_opt_by_ordinal(ordinal).ok_or_else(||MissingSymbolError { symbol: Symbol::Ordinal(ordinal) })
    }

    /// Load a symbol from the library by ordinal.
    ///
    /// # Safety
    ///
    /// This function implicitly transmutes!  Use extreme caution.
    /// Additionally, DLL ordinals are typically unstable and might change between minor versions of the same DLL, breaking your imports in nastily subtle ways.
    /// If a function name is available, use it instead!
    ///
    /// # Platform
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `GetProcAddress(..., MAKEINTRESOURCE(ordinal))`
    /// | <strike>Unix</strike> | `None`
    pub unsafe fn sym_opt_by_ordinal<T>(self, _ordinal: u16) -> Option<T> {
        let _ = FnPtrChecks::<T>::ASSERT;

        #[cfg(unix   )] return None;
        #[cfg(windows)] return {
            let func = windows::get_proc_address::by_ordinal(self, _ordinal).ok()?;

            // SAFETY: ✔️
            //  * `T`   ✔️ is asserted to be the same alignment and size as `*mut c_void` via assert at start of function
            //  * `T`   ✔️ is assumed compatible with `*mut c_void` per the documented safety contract of this unsafe function
            Some(transmute_copy::<NonNull<c_void>, T>(&func))
        }
    }

    /// Check if a symbol existing in the library.
    /// Limiting symbol names to basic ASCII is likely wise.
    ///
    /// # Platform
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `!!GetProcAddress(..., name)`
    /// | Unix      | `!!dlsym(..., name)`
    pub fn has_sym(self, name: &CStr) -> bool {
        // SAFETY: ✔️ cast to `*mut c_void` should always be safe.
        let s : Option<*mut c_void> = unsafe { self.sym_opt(name) };
        s.is_some()
    }

    /// Attempt to unload the library.
    ///
    /// # Safety
    /// ❌ This is a **fundamentally unsound** operation that **may do nothing** and **invalidates everything** ❌
    ///
    /// You are practically guaranteed undefined behavior from some kind of dangling pointer or reference.
    /// Several platforms don't even bother implementing unloading.
    /// Those that do implement unloading often have ways of opting out of actually unloading.
    /// I'd argue the only valid use case for this fn is unit testing the unloading of your DLLs, when *someone else* was crazy enough to unload libraries in production.
    ///
    /// Based on that reasoning, this crate has been designed around the assumption that you will never do this.
    /// It lacks lifetimes and implements <code>[Library] : [Copy]</code>, which makes use-after-free bugs easy.
    /// Be especially careful - all of the following are invalidated when this function is called:
    /// *   All fns/pointers previously returned by `sym*` for this library, including through copies of `self`.
    /// *   The [`Library`] itself, and any [`Copy`] thereof.  This means even `self.has_sym(...)` is no longer sound to call, despite the fn being safe, even though it will compile without error (as self is `Copy`)!
    ///
    /// ## Safe Alternatives
    /// *   Restart the entire process (fine for production since process shutdown is actually tested)
    /// *   Use a sub-process and restart that (will also make your code more stable if a hot-reloading "plugin" crashes)
    /// *   Simply leak the library (fine for dev builds)
    ///     *   Export a function to free memory, join threads, close file handles, etc. if you want to reduce memory use / file locks
    ///     *   Load a temporary copy of the library instead of the original if you hate having a file lock on the original library
    ///
    /// ## Unsafe Alternatives
    /// A wrapper or crate with "better" support for this fundamentally flawed operation might:
    /// *   Limit support to plugin-shaped dynamic libraries that opt-in to claiming they're safe to unload (export a special fn/symbol?)
    /// *   Actively test unloading to catch the bugs in those libraries
    /// *   Introduce lifetimes (e.g. [`libloading::Symbol`](https://docs.rs/libloading/0.8.1/libloading/struct.Symbol.html)), or make fn pointers private, to help combat fn pointer invalidation bugs
    /// *   Not implement [`Copy`] for [`Library`] (or equivalent)
    /// *   Not default-implement [`Clone`] for [`Library`] (or equivalent - properly implemented refcounting might be OK)
    /// *   Implement [`Drop`] for [`Library`] (or equivalent) if you're arrogant enough to claim your unloading code is actually safe/sound.
    ///
    /// ## This might do nothing useful whatsoever
    /// *   ["musl’s dynamic loader loads libraries permanently for the lifetime of the process, until it exits or calls exec.<br>[...] only the musl behavior can satisfy the robustness conditions musl aims to provide"](https://wiki.musl-libc.org/functional-differences-from-glibc.html#Unloading-libraries)
    /// *   ["Prior to Mac OS X 10.5, only bundles could be unloaded.  Starting in Mac OS X 10.5, dynamic libraries may also be unloaded."](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/dlclose.3.html)
    /// *   [`RTLD_NODELETE`](https://linux.die.net/man/3/dlopen) makes `dlclose` a noop
    /// *   [`GET_MODULE_HANDLE_EX_FLAG_PIN`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandleexa#parameters) makes `FreeLibrary*` a noop
    ///
    /// ## Threads are a problem
    /// Any thread (including those started automatically by the library itself on load) containing any library code anywhere in it's callstack will exhibit undefined behavior:
    /// *   Unwinding through library code via panic/exceptions/SEH will presumably use dangling unwinding information pointers
    /// *   Callstack snapshots (for allocation tracking, sentry.io event reporting, etc.) will contain dangling symbol pointers etc.
    /// *   Dangling instruction pointers (through execution or returning to library code) will, presumably:
    ///     *   Crash if unmapped, or mapped without execution permissions
    ///     *   Execute random, possibly "unreachable" code - or data misinterpreted as code - if mapped with execution permissions
    ///
    /// These are among the many reasons windows offers such a wide variety of unloading functions (lacking in POSIX systems):
    /// *   [`FreeLibrary`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary)
    /// *   [`FreeLibraryAndExitThread`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibraryandexitthread)
    /// *   [`FreeLibraryWhenCallbackReturns`](https://learn.microsoft.com/en-us/windows/win32/api/threadpoolapiset/nf-threadpoolapiset-freelibrarywhencallbackreturns)
    ///
    /// Some related reading:
    /// *   [What is the point of FreeLibraryAndExitThread?](https://devblogs.microsoft.com/oldnewthing/20131105-00/?p=2733) (The Old New Thing)
    /// *   [When is the correct time to call FreeLibraryWhenCallbackReturns?](https://devblogs.microsoft.com/oldnewthing/20151225-00/?p=92711) (The Old New Thing)
    ///
    /// Additionally, there are serious limitations on what `DllMain` / destructors can do without deadlocking or worse, limiting the DLL's ability to fix any of this:
    /// *   [Dynamic-Link Library Best Practices](https://learn.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-best-practices) (learn.microsoft.com)
    /// *   <https://github.com/mity/old-new-win32api#dllmain> (links to The Old New Thing)
    /// *   [Why are DLLs unloaded in the "wrong" order?](https://devblogs.microsoft.com/oldnewthing/20050523-05/?p=35573) (The Old New Thing)
    ///
    /// ## Callbacks are a problem
    /// The library has many ways of creating pointers into itself which will dangle if the library is unloaded:
    /// *   Error handling
    ///     *   [`std::panic::set_hook`](https://doc.rust-lang.org/std/panic/fn.set_hook.html) (100% safe - which other code can then [`take_hook`](https://doc.rust-lang.org/std/panic/fn.take_hook.html), preventing you from unregistering it!)
    ///     *   [Signal handlers](https://en.cppreference.com/w/c/program/signal)
    ///     *   [Unhandled exception handlers](https://learn.microsoft.com/en-us/windows/win32/debug/using-a-vectored-exception-handler)
    /// *   Windows callbacks
    ///     *   [Window Procedures](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-procedures)
    ///     *   [COM Classes](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coregisterclassobject)
    ///     *   ...
    /// *   `Box<dyn Anything>`
    /// *   ...
    ///
    /// ## String literals and other `'static` references are a problem
    /// Allocator debugging code often likes to register references or pointers to `__FILE__` (common in C++ macros) or [`core::file!()`](https://doc.rust-lang.org/core/macro.file.html) (Rust).
    /// These are generally treated - by 100% safe code - as having `'static` lifetimes, and not deep copied.
    /// While this is sound within the context of a single module, those references and pointers will dangle - with all the undefined behavior that comes with that - if those references and pointers ever end up in another module that outlasts the library.
    ///
    /// This is only the most ubiquitous example of the larger problem:  Every reference or pointer to `const`, `static`, or literal variables of the library becomes a ticking timebomb and hazard in 100% "safe" code.
    ///
    /// ## Testing is a problem
    /// Nobody tests unloading dynamic libraries.  Nobody.
    /// I can't even unload debug CRTs without triggering assertions.
    /// Calling this function will simply invoke broken untested code.
    ///
    /// # Platform
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `FreeLibrary(...)`
    /// | Unix      | `dlclose(...)`
    pub unsafe fn close_unsafe_unsound_possible_noop_do_not_use_in_production(self) -> core::result::Result<(), UnloadLibraryError> {
        #[cfg(windows)] return windows::free_library(self).map_err(|error| UnloadLibraryError { error });
        #[cfg(unix)] return unix::dlclose(self).map_err(|ret| UnloadLibraryError { dlerror: unix::dlerror::to_cstring(), ret });
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] enum Symbol<'symbol> {
    Name(&'symbol CStr),
    Ordinal(u16), // windows only
}



struct FnPtrChecks<F>(F);
impl<F> FnPtrChecks<F> {
    pub const ASSERT : () = const {
        assert!(align_of::<F>() == align_of::<*mut c_void>(), "symbol result has wrong alignment");
        assert!(size_of ::<F>() == size_of ::<*mut c_void>(), "symbol result has wrong size");
    };
}

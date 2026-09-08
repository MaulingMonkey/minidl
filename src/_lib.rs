#![doc = include_str!("../Readme.md")]
#![no_std]
#![deny(non_snake_case, unreachable_patterns)] // catch typoed match ... { ERROR_... => ... }
#![cfg_attr(not(all(feature = "std", feature = "winresult")), allow(dead_code, unused_imports))] // ignore many dead_code false positives without --all-features

#[cfg(feature = "alloc")]   extern crate alloc;
#[cfg(feature = "std")]     extern crate std;

include!("core/_core.rs");
#[path = "errors/_errors.rs"] pub mod errors; #[doc(hidden)] pub use errors::*;
include!("util/_util.rs");

#[cfg(unix   )] #[path = "unix/_unix.rs"        ] mod unix;
#[cfg(windows)] #[path = "windows/_windows.rs"  ] mod windows;

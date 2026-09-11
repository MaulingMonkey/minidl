# 0.2.0 (unreleased)

### Breaking Changes (0.1.6 → 0.2.0)
-   MSRV bumped from Rust 1.54 to 1.79
-   Symbol names are now expected to be [`CStr`](https://doc.rust-lang.org/core/ffi/struct.CStr.html)s (use the new `c"literal c string"` syntax as of rustc 1.77 and `edition = "2021"`!)
-   Functions now return `minidl` specific errors (although they're still convertable to `std::io::Error` assuming `feature = "std"`)
-   Some small fn signature changes (`&self` → `self`)

### New Features (0.2.0)
-   `#![no_std]` is now supported (see new crate features: `"alloc"`, `"std"`)
-   [`winresult`](https://docs.rs/winresult) interop (see new crate feature: `"winresult"`)
-   Mild future proofing (reduced wildcard imports)



# 0.1.x

### 0.1.6
-   Introduced <code>[Library](https://docs.rs/minidl/0.1.6/minidl/struct.Library.html)::[close_unsafe_unsound_possible_noop_do_not_use_in_production](https://docs.rs/minidl/0.1.6/minidl/struct.Library.html#method.close_unsafe_unsound_possible_noop_do_not_use_in_production)</code>, mostly to document why you shouldn't.
-   More documentation
-   Fix for Rust 1.54

### 0.1.5
-   Upstreamed <code>[Library](https://docs.rs/minidl/0.1.5/minidl/struct.Library.html)::{[sym](https://docs.rs/minidl/0.1.5/minidl/struct.Library.html#method.sym_by_ordinal)\[[_opt](https://docs.rs/minidl/0.1.5/minidl/struct.Library.html#method.sym_opt_by_ordinal)\][_by_ordinal](https://docs.rs/minidl/0.1.5/minidl/struct.Library.html#method.sym_opt_by_ordinal), [has_sym](https://docs.rs/minidl/0.1.5/minidl/struct.Library.html#method.has_sym)}</code>

### 0.1.4
-   Cargo.toml: link repository, readme

### 0.1.3
-   Made [`Library`](https://docs.rs/minidl/0.1.3/minidl/struct.Library.html) `#[repr(transparent)]`
-   Drop "MSRV" messaging

### 0.1.2
-   Nightly → 1.54 MSRV

### 0.1.1
-   Documentation fixes

### 0.1.0
-   The initial version!

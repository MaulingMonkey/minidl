#[cfg(    feature = "std" )] type NStr = std::ffi::OsStr;
#[cfg(not(feature = "std"))] type NStr = str;

#[cfg(unix      )] type NCStr = core::ffi::CStr;
#[cfg(windows   )] type NCStr = crate::WCStr;

#[cfg(unix      )] type NChar = u8;
#[cfg(windows   )] type NChar = u16;

pub(crate) fn with_ncstr0<R>(
    s:          impl AsRef<NStr>,
    _on_buf:    impl FnOnce(&NCStr) -> R,
    _on_nul:    impl FnOnce() -> R,
    _on_oom:    impl FnOnce() -> R,
) -> R {
    let s = s.as_ref();

    #[cfg(all(    feature = "std",  unix    ))] let src = std::os::unix::ffi::OsStrExt::as_bytes(s).iter().copied();
    #[cfg(all(    feature = "std",  windows ))] let src = std::os::windows::ffi::OsStrExt::encode_wide(s);
    #[cfg(all(not(feature = "std"), unix    ))] let src = s.bytes();
    #[cfg(all(not(feature = "std"), windows ))] let src = s.encode_utf16();
    let src = src.chain([0]);
    let n = src.clone().count();

    // this fn isn't necessairly specific to paths, but paths are what I'm using it for currently, so paths are what I'm tuning it for.
    macro_rules! for_path_length { ( $n:expr ) => { if n <= {$n} { return with_buf::<R, {$n}>(src.clone(), _on_buf, _on_nul, _on_oom); } } }
    //r_path_length!(  255); // _POSIX_PATH_MAX
    //r_path_length!(  260); // MAX_PATH (windows, 256 + 4 for extension)
    for_path_length!( 1024); // MAXPATHLEN (OS X)
    //r_path_length!(32768); // NTFS limit (windows)
    for_path_length!(65536); // arbitrary

    #[cfg(not(feature = "alloc"))] return _on_oom();
    #[cfg(    feature = "alloc" )] return {
        // TODO: consider length limiting even when alloc is enabled?
        let buf = src.collect::<alloc::vec::Vec<_>>();
        match NCStr::from_bytes_with_nul(&buf) {
            Ok(cstr)    => _on_buf(cstr),
            Err(_)      => _on_nul(),
        }
    };

    #[inline(never)] // avoid inlining variable length stack allocations
    fn with_buf<R, const N: usize>(
        mut src0:   impl Iterator<Item = NChar>,
        on_buf:     impl FnOnce(&NCStr) -> R,
        on_nul:     impl FnOnce() -> R,
        on_oom:     impl FnOnce() -> R,
    ) -> R {
        // TODO: replace logic with https://doc.rust-lang.org/std/primitive.slice.html#method.write_iter if it stabilizes
        let mut buffer : [NChar; N] = [0; N];
        let mut dst = buffer.iter_mut();
        while let Some(value) = src0.next() {
            match dst.next() {
                Some(dst)   => *dst = value,
                None        => return on_oom(),
            }
        }
        let n = N - dst.len();
        match NCStr::from_bytes_with_nul(&buffer[..n]) {
            Ok(cstr)    => on_buf(cstr),
            Err(_)      => on_nul(),
        }
    }
}

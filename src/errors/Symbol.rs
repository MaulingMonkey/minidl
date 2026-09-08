#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub(crate) enum Symbol<'symbol> {
    Name(&'symbol core::ffi::CStr),
    Ordinal(u16), // windows only
}

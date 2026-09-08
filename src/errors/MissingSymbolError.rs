/// A requested symbol did not exist inside a given [`Library`].
///
/// ## Returned by
/// -   [`Library::sym`]
/// -   [`Library::sym_by_ordinal`]
///
#[derive(Debug)] pub struct MissingSymbolError<'symbol> {
    pub(crate) symbol: Symbol<'symbol>,
}

impl Display for MissingSymbolError<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.symbol {
            Symbol::Name(name)          => write!(fmt, "Symbol {name:?} missing from library"),
            Symbol::Ordinal(ordinal)    => write!(fmt, "Symbol @{ordinal} missing from library"),
        }
    }
}

#[cfg(feature = "std")] impl std::error::Error for MissingSymbolError<'_> {
    fn description(&self) -> &str { "symbol missing from library" }
}

#[cfg(feature = "std")] impl<'symbol> From<MissingSymbolError<'symbol>> for std::io::Error {
    fn from(value: MissingSymbolError<'symbol>) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, std::format!("{value}"))
    }
}

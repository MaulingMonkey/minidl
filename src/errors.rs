//! [`minidl`](crate) errors.  Note these are re-exported by the crate root.

use crate::*;

use core::fmt::{self, Debug, Display, Formatter};



/// A requested symbol did not exist inside a given [`Library`].
///
/// ## Returned by
/// -   [`Library::sym`]
/// -   [`Library::sym_by_ordinal`]
///
#[derive(Clone, Debug)] pub struct MissingSymbolError<'a> {
    pub(crate) symbol: Symbol<'a>,
}

impl<'a> Display for MissingSymbolError<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.symbol {
            Symbol::Name(name)          => write!(fmt, "Symbol {name:?} missing from library"),
            Symbol::Ordinal(ordinal)    => write!(fmt, "Symbol @{ordinal} missing from library"),
        }
    }
}

impl<'a> From<MissingSymbolError<'a>> for std::io::Error {
    fn from(value: MissingSymbolError<'a>) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("{value}"))
    }
}

// TODO: replace with CStr::display once stabilized

#[cfg_attr(not(unix), allow(dead_code))]
pub(crate) struct CStrDisplay<'s>(pub &'s core::ffi::CStr);

impl core::fmt::Display for CStrDisplay<'_> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0.to_str() {
            Ok(str) => fmt.write_str(str),
            Err(_) => {
                use core::fmt::Write;
                for b in self.0.to_bytes().iter().copied() {
                    fmt.write_char(match b {
                        0 ..= 0x7F  => char::from(b),
                        0x80 ..     => '?',
                    })?;
                }
                Ok(())
            },
        }
    }
}

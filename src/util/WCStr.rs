#[repr(transparent)] pub(crate) struct WCStr([u16]);

impl WCStr {
    /// XXX: easier to write generic code over NCStr with this.
    /// I'd extend CStr with from_units_with_nul instead, but that requires extension traits and I'm lazy.
    /// I admit my code crimes.
    #[doc(hidden)] pub fn from_bytes_with_nul(units: &[u16]) -> Result<&WCStr, FromUnitsWithNulError> {
        Self::from_units_with_nul(units)
    }

    pub fn from_units_with_nul(units: &[u16]) -> Result<&WCStr, FromUnitsWithNulError> {
        match units {
            [body@.., 0] if !body.contains(&0)  => Ok(unsafe { core::mem::transmute(units) }),
            _other                              => Err(FromUnitsWithNulError(())),
        }
    }

    pub fn as_ptr(&self) -> *const u16 { self.0.as_ptr() }
}



pub(crate) struct FromUnitsWithNulError(());

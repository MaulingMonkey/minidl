/// If built without `feature =-"std"`, [`minidl`] errors cannot be implicitly converted to [`std::io::Error`].
/// However, we *can* convert said errors to strings through [`core::fmt::Formatter`], which is useful for inspecting the error messages.
/// This type simply allows implicit conversion through `minidl_result?` expressions.
///
#[allow(dead_code)] #[derive(Debug)] pub struct StringError(String);

impl From<minidl::LoadLibraryError      > for StringError { fn from(err: minidl::LoadLibraryError   ) -> Self { Self(format!("{err}")) } }
impl From<minidl::UnloadLibraryError    > for StringError { fn from(err: minidl::UnloadLibraryError ) -> Self { Self(format!("{err}")) } }
impl From<minidl::MissingSymbolError<'_>> for StringError { fn from(err: minidl::MissingSymbolError ) -> Self { Self(format!("{err}")) } }

impl core::fmt::Display for StringError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.write_str(&self.0)
    }
}

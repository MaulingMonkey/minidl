pub(crate) struct FnPtrChecks<F>(F);

impl<F> FnPtrChecks<F> {
    pub const ASSERT : () = const {
        use core::ffi::c_void;
        use core::mem::{align_of, size_of};

        assert!(align_of::<F>() == align_of::<*mut c_void>(), "symbol result has wrong alignment");
        assert!(size_of ::<F>() == size_of ::<*mut c_void>(), "symbol result has wrong size");
    };
}

use super::*;

// BSize
binding! {
    /// A two-dimensional size.
    /// - [`Size`] represents a C++ `BSize` class instance which your code has ownership, [`SizeFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BSize` class's documentation](https://www.haiku-os.org/docs/api/classBSize.html) for more details.
    #[doc(alias = "BSize")]
    #[doc(alias = "Size")]
    class Size
        = SizeFromCpp<false>(BSize) impl
        SizeMethods
}
impl<const FROM_CPP: bool> SizeFromCpp<FROM_CPP> {
    /// Initializes a BSize object with both dimensions set to B_SIZE_UNSET.
    ///
    /// See [C++ `BSize::BSize()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#a0b5ebd8b8b5f4c0b5c761a4a373b0190).
    pub fn new() -> SizeFromCpp<FROM_CPP> {
        unsafe { SizeFromCpp(ffi::BSize_new()) }
    }
    /// Initializes a BSize object from another BSize.
    ///
    /// See [C++ `BSize::BSize()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#aebd79552dc1dfc26306d1672c21c98fc).
    pub fn new_with_size<S: SizeMethods>(other: &S) -> SizeFromCpp<FROM_CPP> {
        unsafe {
            let other = other.as_ptr();
            SizeFromCpp(ffi::BSize_new1(other))
        }
    }
    /// Initializes a BSize object with the specified width and height values.
    ///
    /// See [C++ `BSize::BSize()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#a8508a9b19f7ed6e7c7885d635a9002e1).
    pub fn new_with_float(width: c_float, height: c_float) -> SizeFromCpp<FROM_CPP> {
        unsafe { SizeFromCpp(ffi::BSize_new2(width, height)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SizeFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

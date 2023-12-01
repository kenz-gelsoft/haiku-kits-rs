use super::*;


// BSize
    /// This trait represents [C++ `BSize` class](https://www.haiku-os.org/docs/api/classBSize.html)'s methods and inheritance.
    ///
    /// See [`SizeFromCpp`] documentation for the class usage.
pub trait SizeMethods: RustBindingMethods {
    /// The vertical dimension.
    ///
    /// See [C++ `BSize::height()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#a518f29c07d95774fdc18c7cc88c1b562).
    fn height(&self) -> c_float {
        unsafe { ffi::BSize_height(self.as_ptr()) }
    }
    /// The horizontal dimension.
    ///
    /// See [C++ `BSize::width()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#a4c78fadd3cc44c3460e88842cb90bd67).
    fn width(&self) -> c_float {
        unsafe { ffi::BSize_width(self.as_ptr()) }
    }
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn Height()
    /// Returns the height value of a BSize object as an int32.
    ///
    /// See [C++ `BSize::IntegerHeight()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#a6437f6b0f008a2cdb620de4f992c309c).
    fn integer_height(&self) -> i32 {
        unsafe { ffi::BSize_IntegerHeight(self.as_ptr()) }
    }
    /// Returns the width value of a BSize object as an int32.
    ///
    /// See [C++ `BSize::IntegerWidth()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#aaaafde7965ffc89603d63ea615667b0f).
    fn integer_width(&self) -> i32 {
        unsafe { ffi::BSize_IntegerWidth(self.as_ptr()) }
    }
    /// Checks if the BSize::height is not B_SIZE_UNSET.
    ///
    /// See [C++ `BSize::IsHeightSet()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#a5ad873454dda2b3f33046297e1d74172).
    fn is_height_set(&self) -> bool {
        unsafe { ffi::BSize_IsHeightSet(self.as_ptr()) }
    }
    /// Checks if the BSize::width is not B_SIZE_UNSET.
    ///
    /// See [C++ `BSize::IsWidthSet()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#a453861866d677ae6d3bf2e9ccc436179).
    fn is_width_set(&self) -> bool {
        unsafe { ffi::BSize_IsWidthSet(self.as_ptr()) }
    }
    /// Sets the width and height values of a BSize object.
    ///
    /// See [C++ `BSize::Set()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#ac5c881bb77144a1bb06c035298870c99).
    fn set(&self, width: c_float, height: c_float) {
        unsafe { ffi::BSize_Set(self.as_ptr(), width, height) }
    }
    /// Sets the height value of a BSize object.
    ///
    /// See [C++ `BSize::SetHeight()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#a33da506f5759c9651a61af61a766221c).
    fn set_height(&self, height: c_float) {
        unsafe { ffi::BSize_SetHeight(self.as_ptr(), height) }
    }
    /// Sets the width value of a BSize object.
    ///
    /// See [C++ `BSize::SetWidth()`'s documentation](https://www.haiku-os.org/docs/api/classBSize.html#a7272b3bdb3e42642bf437f6c7621fc3b).
    fn set_width(&self, width: c_float) {
        unsafe { ffi::BSize_SetWidth(self.as_ptr(), width) }
    }
    // BLOCKED: fn Width()
}

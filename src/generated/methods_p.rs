use super::*;


// BPoint
    /// This trait represents [C++ `BPoint` class](https://www.haiku-os.org/docs/api/classBPoint.html)'s methods and inheritance.
    ///
    /// See [`PointFromCpp`] documentation for the class usage.
pub trait PointMethods: RustBindingMethods {
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-1()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
    // NOT_SUPPORTED: fn ConstrainTo()
    /// Print the x and y coordinates to standard output.
    ///
    /// See [C++ `BPoint::PrintToStream()`'s documentation](https://www.haiku-os.org/docs/api/classBPoint.html#a18a683442d46d96f940caf72989c139e).
    fn print_to_stream(&self) {
        unsafe { ffi::BPoint_PrintToStream(self.as_ptr()) }
    }
    /// Sets the x and y coordinates of a BPoint object.
    ///
    /// See [C++ `BPoint::Set()`'s documentation](https://www.haiku-os.org/docs/api/classBPoint.html#a2c74b3de1e07a3e015d8aff2201f0d93).
    fn set(&self, x: c_float, y: c_float) {
        unsafe { ffi::BPoint_Set(self.as_ptr(), x, y) }
    }
}

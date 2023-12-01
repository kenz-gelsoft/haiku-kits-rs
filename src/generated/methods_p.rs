use super::*;


// BPoint
    /// This trait represents [C++ `BPoint` class](https://www.haiku-os.org/docs/api/classBPoint.html)'s methods and inheritance.
    ///
    /// See [`PointFromCpp`] documentation for the class usage.
pub trait PointMethods: RustBindingMethods {
    /// The horizontal coordinate.
    ///
    /// See [C++ `BPoint::x()`'s documentation](https://www.haiku-os.org/docs/api/classBPoint.html#ae8e68c02284caf91f82efb3410fd6708).
    fn x(&self) -> c_float {
        unsafe { ffi::BPoint_x(self.as_ptr()) }
    }
    /// The vertical coordinate.
    ///
    /// See [C++ `BPoint::y()`'s documentation](https://www.haiku-os.org/docs/api/classBPoint.html#a5ffb358a63973977830a85489829c737).
    fn y(&self) -> c_float {
        unsafe { ffi::BPoint_y(self.as_ptr()) }
    }
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-1()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
    /// Moves the BPoint so that it is contained within rect.
    ///
    /// See [C++ `BPoint::ConstrainTo()`'s documentation](https://www.haiku-os.org/docs/api/classBPoint.html#a81254fef5bb205f952e252cf5f07ab59).
    fn constrain_to<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::BPoint_ConstrainTo(self.as_ptr(), rect)
        }
    }
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

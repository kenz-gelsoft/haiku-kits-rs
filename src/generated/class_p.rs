use super::*;


// BPoint
binding! {
    /// A point on a two-dimensional Cartesian coordinate system.
    /// - [`Point`] represents a C++ `BPoint` class instance which your code has ownership, [`PointFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BPoint` class's documentation](https://www.haiku-os.org/docs/api/classBPoint.html) for more details.
    #[doc(alias = "BPoint")]
    #[doc(alias = "Point")]
    class Point
        = PointFromCpp<false>(BPoint) impl
        PointMethods
}
impl<const FROM_CPP: bool> PointFromCpp<FROM_CPP> {
    /// Initializes a BPoint object at the origin, (0, 0).
    ///
    /// See [C++ `BPoint::BPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBPoint.html#a00db170ab9b36aa841dd8a99efeb8e20).
    pub fn new() -> PointFromCpp<FROM_CPP> {
        unsafe { PointFromCpp(ffi::BPoint_new()) }
    }
    /// Initializes a BPoint object from another BPoint.
    ///
    /// See [C++ `BPoint::BPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBPoint.html#acd4b14896346e476bcf7d3e905001edd).
    pub fn new_with_point<P: PointMethods>(p: &P) -> PointFromCpp<FROM_CPP> {
        unsafe {
            let p = p.as_ptr();
            PointFromCpp(ffi::BPoint_new1(p))
        }
    }
    /// Initializes a BPoint object at the specified x and y coordinates.
    ///
    /// See [C++ `BPoint::BPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBPoint.html#a4dc55682abc94f59f13faddb51518397).
    pub fn new_with_float(x: c_float, y: c_float) -> PointFromCpp<FROM_CPP> {
        unsafe { PointFromCpp(ffi::BPoint_new2(x, y)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PointFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for PointFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BPoint_delete(self.0) }
        }
    }
}

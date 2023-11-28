use super::*;


// BRect
binding! {
    /// Defines a rectangular area aligned along pixel dimensions.
    /// - [`Rect`] represents a C++ `BRect` class instance which your code has ownership, [`RectFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BRect` class's documentation](https://www.haiku-os.org/docs/api/classBRect.html) for more details.
    #[doc(alias = "BRect")]
    #[doc(alias = "Rect")]
    class Rect
        = RectFromCpp<false>(BRect) impl
        RectMethods
}
impl<const FROM_CPP: bool> RectFromCpp<FROM_CPP> {
    /// Creates an empty BRect object with dimensions (0, 0, -1, -1).
    ///
    /// See [C++ `BRect::BRect()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#aec45286d0efd6227e3fa09e04b7a3baf).
    pub fn new() -> RectFromCpp<FROM_CPP> {
        unsafe { RectFromCpp(ffi::BRect_new()) }
    }
    /// Creates a new BRect object with its dimensions defined by the leftTop and rightBottom points.
    ///
    /// See [C++ `BRect::BRect()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a2351732f3198e9465eac7a69e60d04f2).
    pub fn new_with_point_point(left_top: *mut c_void, right_bottom: *mut c_void) -> RectFromCpp<FROM_CPP> {
        unsafe { RectFromCpp(ffi::BRect_new1(left_top, right_bottom)) }
    }
    /// Creates a new BRect object with its dimensions defined by the leftTop point and size.
    ///
    /// See [C++ `BRect::BRect()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a45d54a02a30734acf967f28d0c267508).
    pub fn new_with_point_size(left_top: *mut c_void, size: *mut c_void) -> RectFromCpp<FROM_CPP> {
        unsafe { RectFromCpp(ffi::BRect_new2(left_top, size)) }
    }
    /// Creates a new BRect object as a copy of other.
    ///
    /// See [C++ `BRect::BRect()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a0fdf1eb31002961b98f3d577dc4346a7).
    pub fn new_with_rect<R: RectMethods>(other: &R) -> RectFromCpp<FROM_CPP> {
        unsafe {
            let other = other.as_ptr();
            RectFromCpp(ffi::BRect_new3(other))
        }
    }
    /// Creates a new BRect object with the given dimensions.
    ///
    /// See [C++ `BRect::BRect()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a2049dc161cc78c7955a6302304faae66).
    pub fn new_with_float_float(left: c_float, top: c_float, right: c_float, bottom: c_float) -> RectFromCpp<FROM_CPP> {
        unsafe { RectFromCpp(ffi::BRect_new4(left, top, right, bottom)) }
    }
    /// Creates a new BRect object setting the left and top dimensions to 0 and setting the right and bottom dimensions to side - 1.
    ///
    /// See [C++ `BRect::BRect()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a4c40d50693fc042a4e3036bb7fd863e7).
    pub fn new_with_float(side: c_float) -> RectFromCpp<FROM_CPP> {
        unsafe { RectFromCpp(ffi::BRect_new5(side)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for RectFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for RectFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BRect_delete(self.0) }
        }
    }
}

use super::*;


// BRect
    /// This trait represents [C++ `BRect` class](https://www.haiku-os.org/docs/api/classBRect.html)'s methods and inheritance.
    ///
    /// See [`RectFromCpp`] documentation for the class usage.
pub trait RectMethods: RustBindingMethods {
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator&()
    // BLOCKED: fn operator|()
    /// Inset the BRect by the x and y coordinates of point.
    ///
    /// See [C++ `BRect::InsetBy()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#ad3b069e34687b65d0e6b4b083c47b59c).
    fn inset_by_point(&self, inset: *mut c_void) {
        unsafe { ffi::BRect_InsetBy(self.as_ptr(), inset) }
    }
    /// Inset the BRect by dx units in the horizontal direction and dy units in the vertical direction.
    ///
    /// See [C++ `BRect::InsetBy()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a343783f899a8501669458172f6949b35).
    fn inset_by_float(&self, dx: c_float, dy: c_float) {
        unsafe { ffi::BRect_InsetBy1(self.as_ptr(), dx, dy) }
    }
    /// Like BRect::InsetBy() but returns the transformed BRect.
    ///
    /// See [C++ `BRect::InsetBySelf()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a83c551c7c3ea2682103bdacf950045bd).
    fn inset_by_self_point(&self, inset: *mut c_void) -> &Self {
        unsafe { ffi::BRect_InsetBySelf(self.as_ptr(), inset); &self }
    }
    /// Like BRect::InsetBy() but returns the transformed BRect.
    ///
    /// See [C++ `BRect::InsetBySelf()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a6377eafe20d2d0e872082dac30eafbe4).
    fn inset_by_self_float(&self, dx: c_float, dy: c_float) -> &Self {
        unsafe { ffi::BRect_InsetBySelf1(self.as_ptr(), dx, dy); &self }
    }
    /// Like BRect::InsetBy() but returns a copy of the transformed BRect leaving the original unmodified.
    ///
    /// See [C++ `BRect::InsetByCopy()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a41ff92a5d0b62a5ba4543ce841b83f22).
    fn inset_by_copy_point(&self, inset: *mut c_void) -> Rect {
        unsafe { Rect::from_ptr(ffi::BRect_InsetByCopy(self.as_ptr(), inset)) }
    }
    /// Like BRect::InsetBy() but returns a copy of the transformed BRect leaving the original unmodified.
    ///
    /// See [C++ `BRect::InsetByCopy()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a509115fe2690096e857068dae20f09f0).
    fn inset_by_copy_float(&self, dx: c_float, dy: c_float) -> Rect {
        unsafe { Rect::from_ptr(ffi::BRect_InsetByCopy1(self.as_ptr(), dx, dy)) }
    }
    /// Moves the BRect horizontally by the x value of point and vertically by y value of point without changing the size of the rectangle.
    ///
    /// See [C++ `BRect::OffsetBy()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a9a8b7148025f839c387085535048fae2).
    fn offset_by_point(&self, delta: *mut c_void) {
        unsafe { ffi::BRect_OffsetBy(self.as_ptr(), delta) }
    }
    /// Moves the BRect horizontally by dx units and vertically by dy units point without changing the size of the rectangle.
    ///
    /// See [C++ `BRect::OffsetBy()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#aee260dabc7f5d10f117b916f11168429).
    fn offset_by_float(&self, dx: c_float, dy: c_float) {
        unsafe { ffi::BRect_OffsetBy1(self.as_ptr(), dx, dy) }
    }
    /// Move the BRect to the location specified by point.
    ///
    /// See [C++ `BRect::OffsetTo()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a1ca9ed8ce028d98c8a124295ecea41fe).
    fn offset_to_point(&self, offset: *mut c_void) {
        unsafe { ffi::BRect_OffsetTo(self.as_ptr(), offset) }
    }
    /// Move the BRect to the point specified by the given x and y coordinates.
    ///
    /// See [C++ `BRect::OffsetTo()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#afd443e9c51d991d386bd11b372e1fa69).
    fn offset_to_float(&self, x: c_float, y: c_float) {
        unsafe { ffi::BRect_OffsetTo1(self.as_ptr(), x, y) }
    }
    /// Like BRect::OffsetBy() but returns the translated BRect.
    ///
    /// See [C++ `BRect::OffsetBySelf()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a7fa352a96bb16046f0d5b0801ed91ba1).
    fn offset_by_self_point(&self, offset: *mut c_void) -> &Self {
        unsafe { ffi::BRect_OffsetBySelf(self.as_ptr(), offset); &self }
    }
    /// Like BRect::OffsetBy() but returns the translated BRect.
    ///
    /// See [C++ `BRect::OffsetBySelf()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a77f057d3ab457b177ef70ff726bda59d).
    fn offset_by_self_float(&self, dx: c_float, dy: c_float) -> &Self {
        unsafe { ffi::BRect_OffsetBySelf1(self.as_ptr(), dx, dy); &self }
    }
    /// Like BRect::OffsetBy() but returns a copy of the translated BRect leaving the original unmodified.
    ///
    /// See [C++ `BRect::OffsetByCopy()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#aa3c2f35396060f90675701f16ffc7c45).
    fn offset_by_copy_point(&self, offset: *mut c_void) -> Rect {
        unsafe { Rect::from_ptr(ffi::BRect_OffsetByCopy(self.as_ptr(), offset)) }
    }
    /// Like BRect::OffsetBy() but returns a copy of the translated BRect leaving the original unmodified.
    ///
    /// See [C++ `BRect::OffsetByCopy()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#ae5ab87965986017c658fc906fed1a388).
    fn offset_by_copy_float(&self, dx: c_float, dy: c_float) -> Rect {
        unsafe { Rect::from_ptr(ffi::BRect_OffsetByCopy1(self.as_ptr(), dx, dy)) }
    }
    /// Like BRect::OffsetTo() but returns the translated BRect.
    ///
    /// See [C++ `BRect::OffsetToSelf()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a2b8fa1ec48096e16de1c0a94efc6816e).
    fn offset_to_self_point(&self, offset: *mut c_void) -> &Self {
        unsafe { ffi::BRect_OffsetToSelf(self.as_ptr(), offset); &self }
    }
    /// Like BRect::OffsetTo() but returns the translated BRect.
    ///
    /// See [C++ `BRect::OffsetToSelf()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#ace358552bc769a21db704b00b11f9400).
    fn offset_to_self_float(&self, x: c_float, y: c_float) -> &Self {
        unsafe { ffi::BRect_OffsetToSelf1(self.as_ptr(), x, y); &self }
    }
    /// Like BRect::OffsetTo() but returns a copy of the translated BRect leaving the original unmodified.
    ///
    /// See [C++ `BRect::OffsetToCopy()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#ac1b3a5eab5aac9a852df175a406927c8).
    fn offset_to_copy_point(&self, offset: *mut c_void) -> Rect {
        unsafe { Rect::from_ptr(ffi::BRect_OffsetToCopy(self.as_ptr(), offset)) }
    }
    /// Like BRect::OffsetTo() but returns a copy of the translated BRect leaving the original unmodified.
    ///
    /// See [C++ `BRect::OffsetToCopy()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a9df225c7be4a68fffc865e19b2f10080).
    fn offset_to_copy_float(&self, x: c_float, y: c_float) -> Rect {
        unsafe { Rect::from_ptr(ffi::BRect_OffsetToCopy1(self.as_ptr(), x, y)) }
    }
    /// Returns whether or not the BRect contains point.
    ///
    /// See [C++ `BRect::Contains()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a6f5c37ccc41d49131df112d5a74d5200).
    fn contains_point(&self, point: *mut c_void) -> bool {
        unsafe { ffi::BRect_Contains(self.as_ptr(), point) }
    }
    /// Returns whether or not the BRect wholly contains rect.
    ///
    /// See [C++ `BRect::Contains()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a7e5268e764cdcd03ef7177780338c658).
    fn contains_rect(&self, rect: *mut c_void) -> bool {
        unsafe { ffi::BRect_Contains1(self.as_ptr(), rect) }
    }
    /// Returns the height of the rectangle.
    ///
    /// See [C++ `BRect::Height()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#aca81521a6170bdc66bf1cefa79ad3609).
    fn height(&self) -> c_float {
        unsafe { ffi::BRect_Height(self.as_ptr()) }
    }
    /// Returns The height of the rectangle rounded using ceil(bottom - top).
    ///
    /// See [C++ `BRect::IntegerHeight()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a9b163b3d07d27dd23130781bde483bd9).
    fn integer_height(&self) -> i32 {
        unsafe { ffi::BRect_IntegerHeight(self.as_ptr()) }
    }
    /// Returns The width of the rectangle rounded using ceil(right - left).
    ///
    /// See [C++ `BRect::IntegerWidth()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#ae52162fc1adc048605ec958c298417cd).
    fn integer_width(&self) -> i32 {
        unsafe { ffi::BRect_IntegerWidth(self.as_ptr()) }
    }
    /// Returns whether or not the BRect and rect intersect.
    ///
    /// See [C++ `BRect::Intersects()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a5f0bd9533e3a9b3aa73674e2f74cb7e3).
    fn intersects(&self, rect: *mut c_void) -> bool {
        unsafe { ffi::BRect_Intersects(self.as_ptr(), rect) }
    }
    /// Returns whether or not the BRect is valid.
    ///
    /// See [C++ `BRect::IsValid()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#af3e701c7a92ea7379943ce2cc1d66a79).
    fn is_valid(&self) -> bool {
        unsafe { ffi::BRect_IsValid(self.as_ptr()) }
    }
    /// Returns the left bottom point of the BRect.
    ///
    /// See [C++ `BRect::LeftBottom()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#aeea8c62fa651880f159793ae7da5054a).
    fn left_bottom(&self) -> Point {
        unsafe { Point::from_ptr(ffi::BRect_LeftBottom(self.as_ptr())) }
    }
    /// Returns the left top point of the BRect.
    ///
    /// See [C++ `BRect::LeftTop()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a39cb030731bbc8b0e82e04ed563d4340).
    fn left_top(&self) -> Point {
        unsafe { Point::from_ptr(ffi::BRect_LeftTop(self.as_ptr())) }
    }
    /// Prints the BRect dimensions to standard output.
    ///
    /// See [C++ `BRect::PrintToStream()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#ad2e5f6b30e591969f620a2e5cb8d3785).
    fn print_to_stream(&self) {
        unsafe { ffi::BRect_PrintToStream(self.as_ptr()) }
    }
    /// Returns the right bottom point of the BRect.
    ///
    /// See [C++ `BRect::RightBottom()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a7e52511bb18f8e22d1bdd7d9c01b6a39).
    fn right_bottom(&self) -> Point {
        unsafe { Point::from_ptr(ffi::BRect_RightBottom(self.as_ptr())) }
    }
    /// Returns the right top point of the BRect.
    ///
    /// See [C++ `BRect::RightTop()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#af66a48189605a69edf987720ad5fa635).
    fn right_top(&self) -> Point {
        unsafe { Point::from_ptr(ffi::BRect_RightTop(self.as_ptr())) }
    }
    /// Sets the dimensions of a BRect.
    ///
    /// See [C++ `BRect::Set()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a7ad2bdde464155de1bdf38db91bceca3).
    fn set(&self, left: c_float, top: c_float, right: c_float, bottom: c_float) {
        unsafe { ffi::BRect_Set(self.as_ptr(), left, top, right, bottom) }
    }
    /// Sets the left bottom point of the BRect.
    ///
    /// See [C++ `BRect::SetLeftBottom()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a8c2558f788131c37db92a0be041a46ec).
    fn set_left_bottom(&self, point: *const c_void) {
        unsafe { ffi::BRect_SetLeftBottom(self.as_ptr(), point) }
    }
    /// Sets the left top point of the BRect.
    ///
    /// See [C++ `BRect::SetLeftTop()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a304bd292bba6a3fa54c0aa7b54220290).
    fn set_left_top(&self, point: *const c_void) {
        unsafe { ffi::BRect_SetLeftTop(self.as_ptr(), point) }
    }
    /// Sets the right bottom point of the BRect.
    ///
    /// See [C++ `BRect::SetRightBottom()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#ae5c5c2389dab41f899217e9b81af7aad).
    fn set_right_bottom(&self, point: *const c_void) {
        unsafe { ffi::BRect_SetRightBottom(self.as_ptr(), point) }
    }
    /// Sets the right top point of the BRect.
    ///
    /// See [C++ `BRect::SetRightTop()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a4f1f445beee1e7048fe0dd7a4fd40d7c).
    fn set_right_top(&self, point: *const c_void) {
        unsafe { ffi::BRect_SetRightTop(self.as_ptr(), point) }
    }
    /// Returns the dimensions of the BRect.
    ///
    /// See [C++ `BRect::Size()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#a903279542b3a58bf10fd6d8b259f725b).
    fn size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::BRect_Size(self.as_ptr())) }
    }
    /// Returns the width of the rectangle.
    ///
    /// See [C++ `BRect::Width()`'s documentation](https://www.haiku-os.org/docs/api/classBRect.html#aaa68bc8c6101c25b021dcae20c01960a).
    fn width(&self) -> c_float {
        unsafe { ffi::BRect_Width(self.as_ptr()) }
    }
}

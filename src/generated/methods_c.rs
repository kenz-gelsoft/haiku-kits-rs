use super::*;

// BControl
/// This trait represents [C++ `BControl` class](https://www.haiku-os.org/docs/api/classBControl.html)'s methods and inheritance.
///
/// See [`ControlFromCpp`] documentation for the class usage.
pub trait ControlMethods: ViewMethods {
    // DTOR: fn ~BControl()
    /// Get the currently set bitmap for a specific state.
    ///
    /// See [C++ `BControl::IconBitmap()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#a186746c30c79d2645fb94301ef05dfff).
    fn icon_bitmap(&self, which: u32) -> *const c_void {
        unsafe { ffi::BControl_IconBitmap(self.as_ptr(), which) }
    }
    /// Gets whether or not the control is currently enabled.
    ///
    /// See [C++ `BControl::IsEnabled()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#a179de32505328b72911cd85bf2c3cfe8).
    fn is_enabled(&self) -> bool {
        unsafe { ffi::BControl_IsEnabled(self.as_ptr()) }
    }
    /// Gets the label of the control.
    ///
    /// See [C++ `BControl::Label()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#a0c345befac88d6af592ce2b3e7d7214e).
    fn label(&self) -> Option<&CStr> {
        unsafe { CStr::option_from(ffi::BControl_Label(self.as_ptr())) }
    }
    /// Enables or disables the control.
    ///
    /// See [C++ `BControl::SetEnabled()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#a54529bfa2d272ef17f1b790d27c7ff17).
    fn set_enabled(&self, enabled: bool) {
        unsafe { ffi::BControl_SetEnabled(self.as_ptr(), enabled) }
    }
    /// This convenience method is used to set the bitmaps for the standard states from a single bitmap.
    ///
    /// See [C++ `BControl::SetIcon()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#acf788aa66dd061c78256f357df691765).
    fn set_icon(&self, bitmap: *const c_void, flags: u32) -> status_t {
        unsafe { ffi::BControl_SetIcon(self.as_ptr(), bitmap, flags) }
    }
    /// Icon bitmaps for various states of the control (off, on, partially on, each enabled or disabled, plus up to 125 custom states) can be set individually.
    ///
    /// See [C++ `BControl::SetIconBitmap()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#a1017ff0cf130fa1246291a016b2a7f2b).
    fn set_icon_bitmap(&self, bitmap: *const c_void, which: u32, flags: u32) -> status_t {
        unsafe { ffi::BControl_SetIconBitmap(self.as_ptr(), bitmap, which, flags) }
    }
    /// Sets the label of the control.
    ///
    /// See [C++ `BControl::SetLabel()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#a2894bf781c41b3ab8a99417f098e71dc).
    fn set_label(&self, string: &str) {
        unsafe {
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            ffi::BControl_SetLabel(self.as_ptr(), string)
        }
    }
    /// Sets the value of the control.
    ///
    /// See [C++ `BControl::SetValue()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#a7841a122e7fa9ab5efb5217c562e763d).
    fn set_value(&self, value: i32) {
        unsafe { ffi::BControl_SetValue(self.as_ptr(), value) }
    }
    /// Gets the value of the control.
    ///
    /// See [C++ `BControl::Value()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#a759e9aebe200495c3773d2920e19e91f).
    fn value(&self) -> i32 {
        unsafe { ffi::BControl_Value(self.as_ptr()) }
    }
}

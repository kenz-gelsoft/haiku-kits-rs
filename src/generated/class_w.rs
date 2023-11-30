use super::*;


// BWindow
binding! {
    /// Window base class.
    /// - [`Window`] represents a C++ `BWindow` class instance which your code has ownership, [`WindowFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BWindow` class's documentation](https://www.haiku-os.org/docs/api/classBWindow.html) for more details.
    #[doc(alias = "BWindow")]
    #[doc(alias = "Window")]
    class Window
        = WindowFromCpp<false>(BWindow) impl
        WindowMethods,
        LooperMethods,
        HandlerMethods
        // ArchivableMethods
}
impl<const FROM_CPP: bool> WindowFromCpp<FROM_CPP> {
    /// Archive constructor.
    ///
    /// See [C++ `BWindow::BWindow()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a967856a612c3e7ad4d5d1f4970f744e3).
    pub fn new_with_message<M: MessageMethods>(archive: Option<&M>) -> WindowFromCpp<FROM_CPP> {
        unsafe {
            let archive = match archive {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowFromCpp(ffi::BWindow_new(archive))
        }
    }
    /// Creates a new BWindow object with the specified look and feel.
    ///
    /// See [C++ `BWindow::BWindow()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a92dfad4d2089ea9a4e8ca8154776e82d).
    pub fn new_with_rect_window_look<R: RectMethods>(frame: &R, title: &str, look: window_look, feel: window_feel, flags: u32, workspace: u32) -> WindowFromCpp<FROM_CPP> {
        unsafe {
            let frame = frame.as_ptr();
            let title = CString::from_vec_unchecked(title.into());
            let title = title.as_ptr();
            WindowFromCpp(ffi::BWindow_new1(frame, title, look, feel, flags, workspace))
        }
    }
    /// Creates a new BWindow object.
    ///
    /// See [C++ `BWindow::BWindow()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#afe03898c4cefc6b853f304c57afee533).
    pub fn new_with_rect_window_type<R: RectMethods>(frame: &R, title: &str, type_: window_type, flags: u32, workspace: u32) -> WindowFromCpp<FROM_CPP> {
        unsafe {
            let frame = frame.as_ptr();
            let title = CString::from_vec_unchecked(title.into());
            let title = title.as_ptr();
            WindowFromCpp(ffi::BWindow_new2(frame, title, type_, flags, workspace))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for WindowFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<WindowFromCpp<FROM_CPP>> for LooperFromCpp<FROM_CPP> {
    fn from(o: WindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<WindowFromCpp<FROM_CPP>> for HandlerFromCpp<FROM_CPP> {
    fn from(o: WindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<WindowFromCpp<FROM_CPP>> for ArchivableFromCpp<FROM_CPP> {
    fn from(o: WindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> ArchivableMethods for WindowFromCpp<FROM_CPP> {
    /// Creates a new BWindow object from the data message.
    ///
    /// See [C++ `BWindow::Instantiate()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#af03c3109307589d67a1888a26f516f3f).
    fn instantiate<M: MessageMethods>(archive: Option<&M>) -> Option<ArchivableFromCpp<true>> {
        unsafe {
            let archive = match archive {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Archivable::option_from(ffi::BWindow_Instantiate(archive))
        }
    }
}

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
    pub fn new(archive: *mut c_void) -> WindowFromCpp<FROM_CPP> {
        unsafe { WindowFromCpp(ffi::BWindow_new(archive)) }
    }
    // NOT_SUPPORTED: fn BWindow1()
    // NOT_SUPPORTED: fn BWindow2()
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
impl<const FROM_CPP: bool> Drop for WindowFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BArchivable_delete(self.0) }
        }
    }
}
impl<const FROM_CPP: bool> ArchivableMethods for WindowFromCpp<FROM_CPP> {
    /// Creates a new BWindow object from the data message.
    ///
    /// See [C++ `BWindow::Instantiate()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#af03c3109307589d67a1888a26f516f3f).
    fn instantiate(archive: *mut c_void) -> Option<ArchivableFromCpp<true>> {
        unsafe { Archivable::option_from(ffi::BWindow_Instantiate(archive)) }
    }
}

use super::*;


// BApplication
    /// This trait represents [C++ `BApplication` class](url/to/classBApplication)'s methods and inheritance.
    ///
    /// See [`ApplicationFromCpp`] documentation for the class usage.
pub trait ApplicationMethods: ooperMethods {
    /// Hook method that's invoked when the BApplication receives a B_READY_TO_RUN message.
    ///
    /// See [C++ `BApplication::ReadyToRun()`'s documentation](url/to/classBApplication#af912cef601090a89cc20cb3a7af48315).
    fn ready_to_run(&self) {
        unsafe { ffi::BApplication_ReadyToRun(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn ArgvReceived()
    /// Hook method that gets invoked when the application receives B_APP_ACTIVATED message.
    ///
    /// See [C++ `BApplication::AppActivated()`'s documentation](url/to/classBApplication#a71e7db8bc9e4f34137bcd4c5e3ed6a16).
    fn app_activated(&self, active: bool) {
        unsafe { ffi::BApplication_AppActivated(self.as_ptr(), active) }
    }
    /// Hook method that gets invoked when the application receives a B_REFS_RECEIVED message.
    ///
    /// See [C++ `BApplication::RefsReceived()`'s documentation](url/to/classBApplication#a5fae9740458d9aec66f3b1d5c50fae87).
    fn refs_received(&self, message: *mut c_void) {
        unsafe { ffi::BApplication_RefsReceived(self.as_ptr(), message) }
    }
    /// Hook method that gets invoked when the BApplication receives a B_ABOUT_REQUESTED message.
    ///
    /// See [C++ `BApplication::AboutRequested()`'s documentation](url/to/classBApplication#a47d9b29407642805b64f0478a21e5895).
    fn about_requested(&self) {
        unsafe { ffi::BApplication_AboutRequested(self.as_ptr()) }
    }
    /// Hook method that gets invoked when the BApplication receives a B_PULSE message.
    ///
    /// See [C++ `BApplication::Pulse()`'s documentation](url/to/classBApplication#abf9de9bc0de3a8c504a4174f34ed82b4).
    fn pulse(&self) {
        unsafe { ffi::BApplication_Pulse(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetPulseRate()
    /// Restores the cursor.
    ///
    /// See [C++ `BApplication::ShowCursor()`'s documentation](url/to/classBApplication#ad21286f2cae75ee360d022fbacc8a21d).
    fn show_cursor(&self) {
        unsafe { ffi::BApplication_ShowCursor(self.as_ptr()) }
    }
    /// Hides the cursor from the screen.
    ///
    /// See [C++ `BApplication::HideCursor()`'s documentation](url/to/classBApplication#a82504dfa192861bbf322e81550e53643).
    fn hide_cursor(&self) {
        unsafe { ffi::BApplication_HideCursor(self.as_ptr()) }
    }
    /// Hides the cursor until the mouse is moved.
    ///
    /// See [C++ `BApplication::ObscureCursor()`'s documentation](url/to/classBApplication#a206c888fb8198a76c3b63668aadab0af).
    fn obscure_cursor(&self) {
        unsafe { ffi::BApplication_ObscureCursor(self.as_ptr()) }
    }
    /// Returns whether or not the cursor is hidden.
    ///
    /// See [C++ `BApplication::IsCursorHidden()`'s documentation](url/to/classBApplication#a3503caddf7e23970a311b07d5db7576a).
    fn is_cursor_hidden(&self) -> bool {
        unsafe { ffi::BApplication_IsCursorHidden(self.as_ptr()) }
    }
    /// Sets the cursor to be used when the application is active.
    ///
    /// See [C++ `BApplication::SetCursor()`'s documentation](url/to/classBApplication#aa4262e1879bcdd8f82af94dab6d4650b).
    fn set_cursor_void(&self, cursor: *const c_void) {
        unsafe { ffi::BApplication_SetCursor(self.as_ptr(), cursor) }
    }
    /// Sets the cursor to be used when the application is active with sync immediately option.
    ///
    /// See [C++ `BApplication::SetCursor()`'s documentation](url/to/classBApplication#a08e75088c2f6396dfc9fc8c01a9c0545).
    fn set_cursor_cursor(&self, cursor: *const c_void, sync: bool) {
        unsafe { ffi::BApplication_SetCursor1(self.as_ptr(), cursor, sync) }
    }
    // NOT_SUPPORTED: fn CountWindows()
    // NOT_SUPPORTED: fn WindowAt()
    // NOT_SUPPORTED: fn CountLoopers()
    // NOT_SUPPORTED: fn LooperAt()
    /// Returns whether or not the application is in the process of launching.
    ///
    /// See [C++ `BApplication::IsLaunching()`'s documentation](url/to/classBApplication#a49c3d8c8521a1b931f45fdcbaedb3f2d).
    fn is_launching(&self) -> bool {
        unsafe { ffi::BApplication_IsLaunching(self.as_ptr()) }
    }
    /// Returns the signature of the Application.
    ///
    /// See [C++ `BApplication::Signature()`'s documentation](url/to/classBApplication#a9f22e1a76c3a742cc9139e947e2307b3).
    fn signature(&self) -> *const c_void {
        unsafe { ffi::BApplication_Signature(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAppInfo()
    /// Returns a BResources object for the application.
    ///
    /// See [C++ `BApplication::AppResources()`'s documentation](url/to/classBApplication#a1aef5ac43f00eed24d6501b7afb50f47).
    fn app_resources() -> *mut c_void {
        unsafe { ffi::BApplication_AppResources() }
    }
    // NOT_SUPPORTED: fn RegisterLooper()
    // NOT_SUPPORTED: fn UnregisterLooper()
    // DTOR: fn ~BApplication()
    // NOT_SUPPORTED: fn InitCheck()
}

// BArchivable
    /// This trait represents [C++ `BArchivable` class](url/to/classBArchivable)'s methods and inheritance.
    ///
    /// See [`ArchivableFromCpp`] documentation for the class usage.
pub trait ArchivableMethods: WxRustMethods {
    // DTOR: fn ~BArchivable()
    // NOT_SUPPORTED: fn AllArchived()
    // NOT_SUPPORTED: fn AllUnarchived()
    // NOT_SUPPORTED: fn Archive()
    // NOT_SUPPORTED: fn Perform()
    /// Static member to restore objects from messages.
    ///
    /// See [C++ `BArchivable::Instantiate()`'s documentation](url/to/classBArchivable#a04efcb17fa2a64a776923cc12303efcd).
    fn instantiate(archive: *mut c_void) -> Option<ArchivableFromCpp<true>> {
        unsafe { Archivable::option_from(ffi::BArchivable_Instantiate(archive)) }
    }
}

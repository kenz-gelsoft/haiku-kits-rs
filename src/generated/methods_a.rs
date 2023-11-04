use super::*;


// BApplication
    /// This trait represents [C++ `BApplication` class](https://www.haiku-os.org/docs/api/classBApplication.html)'s methods and inheritance.
    ///
    /// See [`ApplicationFromCpp`] documentation for the class usage.
pub trait ApplicationMethods: LooperMethods {
    /// Hook method that's invoked when the BApplication receives a B_READY_TO_RUN message.
    ///
    /// See [C++ `BApplication::ReadyToRun()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#af912cef601090a89cc20cb3a7af48315).
    fn ready_to_run(&self) {
        unsafe { ffi::BApplication_ReadyToRun(self.as_ptr()) }
    }
    /// Hook method that gets invoked when the application receives a B_ARGV_RECEIVED message.
    ///
    /// See [C++ `BApplication::ArgvReceived()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a0826684edce56baa7a31c89c97a1d161).
    fn argv_received(&self, argc: i32, argv: *mut c_void) {
        unsafe { ffi::BApplication_ArgvReceived(self.as_ptr(), argc, argv) }
    }
    /// Hook method that gets invoked when the application receives B_APP_ACTIVATED message.
    ///
    /// See [C++ `BApplication::AppActivated()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a71e7db8bc9e4f34137bcd4c5e3ed6a16).
    fn app_activated(&self, active: bool) {
        unsafe { ffi::BApplication_AppActivated(self.as_ptr(), active) }
    }
    /// Hook method that gets invoked when the application receives a B_REFS_RECEIVED message.
    ///
    /// See [C++ `BApplication::RefsReceived()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a5fae9740458d9aec66f3b1d5c50fae87).
    fn refs_received(&self, message: *mut c_void) {
        unsafe { ffi::BApplication_RefsReceived(self.as_ptr(), message) }
    }
    /// Hook method that gets invoked when the BApplication receives a B_ABOUT_REQUESTED message.
    ///
    /// See [C++ `BApplication::AboutRequested()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a47d9b29407642805b64f0478a21e5895).
    fn about_requested(&self) {
        unsafe { ffi::BApplication_AboutRequested(self.as_ptr()) }
    }
    /// Hook method that gets invoked when the BApplication receives a B_PULSE message.
    ///
    /// See [C++ `BApplication::Pulse()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#abf9de9bc0de3a8c504a4174f34ed82b4).
    fn pulse(&self) {
        unsafe { ffi::BApplication_Pulse(self.as_ptr()) }
    }
    /// Sets the interval that the B_PULSE messages are sent.
    ///
    /// See [C++ `BApplication::SetPulseRate()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a565e7ede16cd6a0e7df384c3c97ba860).
    fn set_pulse_rate(&self, rate: i64) {
        unsafe { ffi::BApplication_SetPulseRate(self.as_ptr(), rate) }
    }
    /// Restores the cursor.
    ///
    /// See [C++ `BApplication::ShowCursor()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#ad21286f2cae75ee360d022fbacc8a21d).
    fn show_cursor(&self) {
        unsafe { ffi::BApplication_ShowCursor(self.as_ptr()) }
    }
    /// Hides the cursor from the screen.
    ///
    /// See [C++ `BApplication::HideCursor()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a82504dfa192861bbf322e81550e53643).
    fn hide_cursor(&self) {
        unsafe { ffi::BApplication_HideCursor(self.as_ptr()) }
    }
    /// Hides the cursor until the mouse is moved.
    ///
    /// See [C++ `BApplication::ObscureCursor()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a206c888fb8198a76c3b63668aadab0af).
    fn obscure_cursor(&self) {
        unsafe { ffi::BApplication_ObscureCursor(self.as_ptr()) }
    }
    /// Returns whether or not the cursor is hidden.
    ///
    /// See [C++ `BApplication::IsCursorHidden()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a3503caddf7e23970a311b07d5db7576a).
    fn is_cursor_hidden(&self) -> bool {
        unsafe { ffi::BApplication_IsCursorHidden(self.as_ptr()) }
    }
    /// Sets the cursor to be used when the application is active.
    ///
    /// See [C++ `BApplication::SetCursor()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#aa4262e1879bcdd8f82af94dab6d4650b).
    fn set_cursor_void(&self, cursor: *const c_void) {
        unsafe { ffi::BApplication_SetCursor(self.as_ptr(), cursor) }
    }
    /// Sets the cursor to be used when the application is active with sync immediately option.
    ///
    /// See [C++ `BApplication::SetCursor()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a08e75088c2f6396dfc9fc8c01a9c0545).
    fn set_cursor_cursor(&self, cursor: *const c_void, sync: bool) {
        unsafe { ffi::BApplication_SetCursor1(self.as_ptr(), cursor, sync) }
    }
    /// Returns the number of windows created by the application.
    ///
    /// See [C++ `BApplication::CountWindows()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a0cc9fc7396ac3717b238d61ab1f4b82b).
    fn count_windows(&self) -> i32 {
        unsafe { ffi::BApplication_CountWindows(self.as_ptr()) }
    }
    /// Returns the BWindow object at the specified index in the application's window list.
    ///
    /// See [C++ `BApplication::WindowAt()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a8850d9542d628e11d91a4fd25279b65d).
    fn window_at(&self, index: i32) -> *mut c_void {
        unsafe { ffi::BApplication_WindowAt(self.as_ptr(), index) }
    }
    /// Returns the number of BLoopers created by the application.
    ///
    /// See [C++ `BApplication::CountLoopers()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#aa7ae6e5ae95ce7befab6e0022a2ec983).
    fn count_loopers(&self) -> i32 {
        unsafe { ffi::BApplication_CountLoopers(self.as_ptr()) }
    }
    /// Returns the BLooper object at the specified index in the application's looper list.
    ///
    /// See [C++ `BApplication::LooperAt()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a68827a127ca45f4c421e3b30ca9500bf).
    fn looper_at(&self, index: i32) -> Option<LooperFromCpp<true>> {
        unsafe { Looper::option_from(ffi::BApplication_LooperAt(self.as_ptr(), index)) }
    }
    /// Returns whether or not the application is in the process of launching.
    ///
    /// See [C++ `BApplication::IsLaunching()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a49c3d8c8521a1b931f45fdcbaedb3f2d).
    fn is_launching(&self) -> bool {
        unsafe { ffi::BApplication_IsLaunching(self.as_ptr()) }
    }
    /// Returns the signature of the Application.
    ///
    /// See [C++ `BApplication::Signature()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a9f22e1a76c3a742cc9139e947e2307b3).
    fn signature(&self) -> *const c_void {
        unsafe { ffi::BApplication_Signature(self.as_ptr()) }
    }
    /// Fills out the info parameter with information about the application.
    ///
    /// See [C++ `BApplication::GetAppInfo()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a2b27a6149b1bf53e71a8d83382708308).
    fn get_app_info(&self, info: *mut c_void) -> i32 {
        unsafe { ffi::BApplication_GetAppInfo(self.as_ptr(), info) }
    }
    /// Returns a BResources object for the application.
    ///
    /// See [C++ `BApplication::AppResources()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a1aef5ac43f00eed24d6501b7afb50f47).
    fn app_resources() -> *mut c_void {
        unsafe { ffi::BApplication_AppResources() }
    }
    /// Register a looper to quit when the application quits.
    ///
    /// See [C++ `BApplication::RegisterLooper()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a2b46141e108da1167be0d3427ca37716).
    fn register_looper<L: LooperMethods>(&self, looper: Option<&L>) -> i32 {
        unsafe {
            let looper = match looper {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BApplication_RegisterLooper(self.as_ptr(), looper)
        }
    }
    /// Remove a previously registered Looper from the quit-list.
    ///
    /// See [C++ `BApplication::UnregisterLooper()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#aac40a7828ef8cc9c65f65a30b14594f6).
    fn unregister_looper<L: LooperMethods>(&self, looper: Option<&L>) -> i32 {
        unsafe {
            let looper = match looper {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BApplication_UnregisterLooper(self.as_ptr(), looper)
        }
    }
    // DTOR: fn ~BApplication()
    /// Returns the status of the constructor.
    ///
    /// See [C++ `BApplication::InitCheck()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#aef81a1f9e59c305f2af7a7ad8e240ae0).
    fn init_check(&self) -> i32 {
        unsafe { ffi::BApplication_InitCheck(self.as_ptr()) }
    }
}

// BArchivable
    /// This trait represents [C++ `BArchivable` class](https://www.haiku-os.org/docs/api/classBArchivable.html)'s methods and inheritance.
    ///
    /// See [`ArchivableFromCpp`] documentation for the class usage.
pub trait ArchivableMethods: _WxRustMethods {
    // DTOR: fn ~BArchivable()
    /// Method relating to the use of BArchiver.
    ///
    /// See [C++ `BArchivable::AllArchived()`'s documentation](https://www.haiku-os.org/docs/api/classBArchivable.html#a4075315c407443a3b0dbb3d7551b53c9).
    fn all_archived(&self, archive: *mut c_void) -> i32 {
        unsafe { ffi::BArchivable_AllArchived(self.as_ptr(), archive) }
    }
    /// Method relating to the use of BUnarchiver.
    ///
    /// See [C++ `BArchivable::AllUnarchived()`'s documentation](https://www.haiku-os.org/docs/api/classBArchivable.html#a2b1d74c147ea18e4b4bfacd42f11e0d2).
    fn all_unarchived(&self, archive: *const c_void) -> i32 {
        unsafe { ffi::BArchivable_AllUnarchived(self.as_ptr(), archive) }
    }
    /// Archive the object into a BMessage.
    ///
    /// See [C++ `BArchivable::Archive()`'s documentation](https://www.haiku-os.org/docs/api/classBArchivable.html#a051c5263dd1a75dcf28787b60825dc44).
    fn archive(&self, into: *mut c_void, deep: bool) -> i32 {
        unsafe { ffi::BArchivable_Archive(self.as_ptr(), into, deep) }
    }
    /// Perform some action (Internal method defined for binary compatibility purposes).
    ///
    /// See [C++ `BArchivable::Perform()`'s documentation](https://www.haiku-os.org/docs/api/classBArchivable.html#a827fa92febcf925f07545aa9777df9de).
    fn perform(&self, d: u32, arg: *mut c_void) -> i32 {
        unsafe { ffi::BArchivable_Perform(self.as_ptr(), d, arg) }
    }
    /// Static member to restore objects from messages.
    ///
    /// See [C++ `BArchivable::Instantiate()`'s documentation](https://www.haiku-os.org/docs/api/classBArchivable.html#a04efcb17fa2a64a776923cc12303efcd).
    fn instantiate(archive: *mut c_void) -> Option<ArchivableFromCpp<true>> {
        unsafe { Archivable::option_from(ffi::BArchivable_Instantiate(archive)) }
    }
}

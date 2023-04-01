use super::*;


// BHandler
    /// This trait represents [C++ `BHandler` class](https://www.haiku-os.org/docs/api/classBHandler.html)'s methods and inheritance.
    ///
    /// See [`HandlerFromCpp`] documentation for the class usage.
pub trait HandlerMethods: ArchivableMethods {
    /// Handle message that has been received by the associated looper.
    ///
    /// See [C++ `BHandler::MessageReceived()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#aeecda5017e0081db617f23bbff71fb53).
    fn message_received(&self, message: *mut c_void) {
        unsafe { ffi::BHandler_MessageReceived(self.as_ptr(), message) }
    }
    /// Return a pointer to the looper that this handler is associated with.
    ///
    /// See [C++ `BHandler::Looper()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#ac8e949959b12e602e3b0c4f023c60e6c).
    fn looper(&self) -> Option<LooperFromCpp<true>> {
        unsafe { Looper::option_from(ffi::BHandler_Looper(self.as_ptr())) }
    }
    /// Set or change the name of this handler.
    ///
    /// See [C++ `BHandler::SetName()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#abf861126df4b6e71b9261a99da4ad0eb).
    fn set_name(&self, name: *const c_void) {
        unsafe { ffi::BHandler_SetName(self.as_ptr(), name) }
    }
    /// Return the name of this handler.
    ///
    /// See [C++ `BHandler::Name()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a40b246ac272e09b2f641d1290be21200).
    fn name(&self) -> *const c_void {
        unsafe { ffi::BHandler_Name(self.as_ptr()) }
    }
    /// Set the next handler in the chain that the message is passed on to if this handler cannot process it.
    ///
    /// See [C++ `BHandler::SetNextHandler()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a02f78779c8141987d6030e73c22e734a).
    fn set_next_handler<H: HandlerMethods>(&self, handler: Option<&H>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BHandler_SetNextHandler(self.as_ptr(), handler)
        }
    }
    /// Return the next hander in the chain to which the message is passed on.
    ///
    /// See [C++ `BHandler::NextHandler()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a92855d8f88f6072e7225bde292feaa38).
    fn next_handler(&self) -> Option<HandlerFromCpp<true>> {
        unsafe { Handler::option_from(ffi::BHandler_NextHandler(self.as_ptr())) }
    }
    /// Add filter as a prerequisite to this handler.
    ///
    /// See [C++ `BHandler::AddFilter()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a862bea548363faac6f883b348cbed838).
    fn add_filter(&self, filter: *mut c_void) {
        unsafe { ffi::BHandler_AddFilter(self.as_ptr(), filter) }
    }
    /// Remove filter from the filter list.
    ///
    /// See [C++ `BHandler::RemoveFilter()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a2b608135b27f283802648e7c6f8cdc83).
    fn remove_filter(&self, filter: *mut c_void) -> bool {
        unsafe { ffi::BHandler_RemoveFilter(self.as_ptr(), filter) }
    }
    /// Set the internal list of filters to filters.
    ///
    /// See [C++ `BHandler::SetFilterList()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a503344dc801858e1f5f419be3919e9cf).
    fn set_filter_list(&self, filters: *mut c_void) {
        unsafe { ffi::BHandler_SetFilterList(self.as_ptr(), filters) }
    }
    /// Return a pointer to the list of filters.
    ///
    /// See [C++ `BHandler::FilterList()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a909662a049856f6863a934cf18161554).
    fn filter_list(&self) -> *mut c_void {
        unsafe { ffi::BHandler_FilterList(self.as_ptr()) }
    }
    /// Lock the looper associated with this handler.
    ///
    /// See [C++ `BHandler::LockLooper()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a9d64bf66b4914918e24390d117e83477).
    fn lock_looper(&self) -> bool {
        unsafe { ffi::BHandler_LockLooper(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LockLooperWithTimeout()
    /// Unlock the looper.
    ///
    /// See [C++ `BHandler::UnlockLooper()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#ab131af7eb7aa2b9eea3a11ac1bfd4c03).
    fn unlock_looper(&self) {
        unsafe { ffi::BHandler_UnlockLooper(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn ResolveSpecifier()
    // NOT_SUPPORTED: fn GetSupportedSuites()
    // NOT_SUPPORTED: fn StartWatching()
    // NOT_SUPPORTED: fn StartWatchingAll()
    // NOT_SUPPORTED: fn StopWatching()
    // NOT_SUPPORTED: fn StopWatchingAll()
    // NOT_SUPPORTED: fn StartWatching1()
    // NOT_SUPPORTED: fn StartWatchingAll1()
    // NOT_SUPPORTED: fn StopWatching1()
    // NOT_SUPPORTED: fn StopWatchingAll1()
    // NOT_SUPPORTED: fn SendNotices()
    /// Check if there are any observers watching this handler.
    ///
    /// See [C++ `BHandler::IsWatched()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a581b84f0f067afa88768ce6a0c07f59f).
    fn is_watched(&self) -> bool {
        unsafe { ffi::BHandler_IsWatched(self.as_ptr()) }
    }
    // DTOR: fn ~BHandler()
}

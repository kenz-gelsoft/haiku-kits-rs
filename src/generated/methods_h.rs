use super::*;


// BHandler
    /// This trait represents [C++ `BHandler` class](https://www.haiku-os.org/docs/api/classBHandler.html)'s methods and inheritance.
    ///
    /// See [`HandlerFromCpp`] documentation for the class usage.
pub trait HandlerMethods: ArchivableMethods {
    /// Handle message that has been received by the associated looper.
    ///
    /// See [C++ `BHandler::MessageReceived()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#aeecda5017e0081db617f23bbff71fb53).
    fn message_received<M: MessageMethods>(&self, message: Option<&M>) {
        unsafe {
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BHandler_MessageReceived(self.as_ptr(), message)
        }
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
    fn set_name(&self, name: &str) {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BHandler_SetName(self.as_ptr(), name)
        }
    }
    /// Return the name of this handler.
    ///
    /// See [C++ `BHandler::Name()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a40b246ac272e09b2f641d1290be21200).
    fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::BHandler_Name(self.as_ptr())) }
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
    /// Lock the looper associated with this handler, with a time out value.
    ///
    /// See [C++ `BHandler::LockLooperWithTimeout()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#afa0099ff6defcb6d64e3ee1c2fb47afe).
    fn lock_looper_with_timeout(&self, timeout: bigtime_t) -> status_t {
        unsafe { ffi::BHandler_LockLooperWithTimeout(self.as_ptr(), timeout) }
    }
    /// Unlock the looper.
    ///
    /// See [C++ `BHandler::UnlockLooper()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#ab131af7eb7aa2b9eea3a11ac1bfd4c03).
    fn unlock_looper(&self) {
        unsafe { ffi::BHandler_UnlockLooper(self.as_ptr()) }
    }
    /// Determine the proper handler for a scripting message.
    ///
    /// See [C++ `BHandler::ResolveSpecifier()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a76439ffaf84e65232698d2a4a3317d22).
    fn resolve_specifier<M: MessageMethods, M2: MessageMethods>(&self, message: Option<&M>, index: i32, specifier: Option<&M2>, what: i32, property: &str) -> Option<HandlerFromCpp<true>> {
        unsafe {
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let specifier = match specifier {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let property = CString::from_vec_unchecked(property.into());
            let property = property.as_ptr();
            Handler::option_from(ffi::BHandler_ResolveSpecifier(self.as_ptr(), message, index, specifier, what, property))
        }
    }
    /// Reports the suites of messages and specifiers that derived classes understand.
    ///
    /// See [C++ `BHandler::GetSupportedSuites()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#acf34435dada239f411e8e034e0ea36b5).
    fn get_supported_suites<M: MessageMethods>(&self, data: Option<&M>) -> status_t {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BHandler_GetSupportedSuites(self.as_ptr(), data)
        }
    }
    // NOT_SUPPORTED: fn StartWatching()
    // NOT_SUPPORTED: fn StartWatchingAll()
    // NOT_SUPPORTED: fn StopWatching()
    // NOT_SUPPORTED: fn StopWatchingAll()
    /// Subscribe an observer for a specific state change of this handler.
    ///
    /// See [C++ `BHandler::StartWatching()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a20713ff6ee9df49a014f391374eaf689).
    fn start_watching<H: HandlerMethods>(&self, observer: Option<&H>, what: u32) -> status_t {
        unsafe {
            let observer = match observer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BHandler_StartWatching1(self.as_ptr(), observer, what)
        }
    }
    /// Subscribe an observer for a all state changes.
    ///
    /// See [C++ `BHandler::StartWatchingAll()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a31927c51d89e0e3b3bf609a786ee6c3b).
    fn start_watching_all<H: HandlerMethods>(&self, observer: Option<&H>) -> status_t {
        unsafe {
            let observer = match observer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BHandler_StartWatchingAll1(self.as_ptr(), observer)
        }
    }
    /// Unsubscribe an observer from watching a specific state.
    ///
    /// See [C++ `BHandler::StopWatching()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#ad3544be491270f856a0af8d36ce02d78).
    fn stop_watching<H: HandlerMethods>(&self, observer: Option<&H>, what: u32) -> status_t {
        unsafe {
            let observer = match observer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BHandler_StopWatching1(self.as_ptr(), observer, what)
        }
    }
    /// Unsubscribe an observer from watching all states.
    ///
    /// See [C++ `BHandler::StopWatchingAll()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a8b9a424ce63f5932666094b6eadf10cf).
    fn stop_watching_all<H: HandlerMethods>(&self, observer: Option<&H>) -> status_t {
        unsafe {
            let observer = match observer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BHandler_StopWatchingAll1(self.as_ptr(), observer)
        }
    }
    /// Emit a state change to the observers.
    ///
    /// See [C++ `BHandler::SendNotices()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a71bf72dc17a64bcd42656722271a9e0c).
    fn send_notices<M: MessageMethods>(&self, what: u32, notice: Option<&M>) {
        unsafe {
            let notice = match notice {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BHandler_SendNotices(self.as_ptr(), what, notice)
        }
    }
    /// Check if there are any observers watching this handler.
    ///
    /// See [C++ `BHandler::IsWatched()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a581b84f0f067afa88768ce6a0c07f59f).
    fn is_watched(&self) -> bool {
        unsafe { ffi::BHandler_IsWatched(self.as_ptr()) }
    }
    // DTOR: fn ~BHandler()
}

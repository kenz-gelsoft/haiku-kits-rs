use super::*;


// BLooper
    /// This trait represents [C++ `BLooper` class](url/to/classBLooper)'s methods and inheritance.
    ///
    /// See [`LooperFromCpp`] documentation for the class usage.
pub trait LooperMethods: andlerMethods {
    // NOT_SUPPORTED: fn PostMessage()
    // NOT_SUPPORTED: fn PostMessage1()
    // NOT_SUPPORTED: fn PostMessage2()
    // NOT_SUPPORTED: fn PostMessage3()
    /// Dispatch a message to a handler. Override if there are messages that you want to catch before they are sent to the handlers.
    ///
    /// See [C++ `BLooper::DispatchMessage()`'s documentation](url/to/classBLooper#add21ca8765c67b0dbf95b8f0361afa73).
    fn dispatch_message<H: HandlerMethods>(&self, message: *mut c_void, handler: Option<&H>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_DispatchMessage(self.as_ptr(), message, handler)
        }
    }
    /// Retrieve the current message.
    ///
    /// See [C++ `BLooper::CurrentMessage()`'s documentation](url/to/classBLooper#a6d244af065c4a12ea795bbbc7bb20e07).
    fn current_message(&self) -> *mut c_void {
        unsafe { ffi::BLooper_CurrentMessage(self.as_ptr()) }
    }
    /// Get ownership of the message currently being processed.
    ///
    /// See [C++ `BLooper::DetachCurrentMessage()`'s documentation](url/to/classBLooper#a7c8f05bcc354bc1d53026417417120e3).
    fn detach_current_message(&self) -> *mut c_void {
        unsafe { ffi::BLooper_DetachCurrentMessage(self.as_ptr()) }
    }
    /// Internal method to support single-threaded GUI toolkits.
    ///
    /// See [C++ `BLooper::DispatchExternalMessage()`'s documentation](url/to/classBLooper#a3a2d3773466de19a6c117f3a04861b11).
    fn dispatch_external_message<H: HandlerMethods>(&self, message: *mut c_void, handler: Option<&H>, _detached: *mut c_void) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_DispatchExternalMessage(self.as_ptr(), message, handler, _detached)
        }
    }
    /// Get a pointer to the internal message queue of this looper.
    ///
    /// See [C++ `BLooper::MessageQueue()`'s documentation](url/to/classBLooper#ad0643973ceda5cca540c4684a9f670ac).
    fn message_queue(&self) -> *mut c_void {
        unsafe { ffi::BLooper_MessageQueue(self.as_ptr()) }
    }
    /// Check if there is a message waiting.
    ///
    /// See [C++ `BLooper::IsMessageWaiting()`'s documentation](url/to/classBLooper#a27fd22b9eeaeadbeff1e973ed5242a64).
    fn is_message_waiting(&self) -> bool {
        unsafe { ffi::BLooper_IsMessageWaiting(self.as_ptr()) }
    }
    /// Associate a handler to this looper.
    ///
    /// See [C++ `BLooper::AddHandler()`'s documentation](url/to/classBLooper#a484be74814014e3c48c1a16f44e34074).
    fn add_handler<H: HandlerMethods>(&self, handler: Option<&H>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_AddHandler(self.as_ptr(), handler)
        }
    }
    /// Disassociate a handler from this looper.
    ///
    /// See [C++ `BLooper::RemoveHandler()`'s documentation](url/to/classBLooper#a5af57140bf018388a2e64343f46ac330).
    fn remove_handler<H: HandlerMethods>(&self, handler: Option<&H>) -> bool {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_RemoveHandler(self.as_ptr(), handler)
        }
    }
    // NOT_SUPPORTED: fn CountHandlers()
    // NOT_SUPPORTED: fn HandlerAt()
    // NOT_SUPPORTED: fn IndexOf()
    /// Get the preferred handler.
    ///
    /// See [C++ `BLooper::PreferredHandler()`'s documentation](url/to/classBLooper#af1a4af6872abc40a887bfcabd55aff98).
    fn preferred_handler(&self) -> Option<HandlerFromCpp<true>> {
        unsafe { Handler::option_from(ffi::BLooper_PreferredHandler(self.as_ptr())) }
    }
    /// Set a preferred handler.
    ///
    /// See [C++ `BLooper::SetPreferredHandler()`'s documentation](url/to/classBLooper#a07c6e5c65354b48465fde2b0b44e22c6).
    fn set_preferred_handler<H: HandlerMethods>(&self, handler: Option<&H>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_SetPreferredHandler(self.as_ptr(), handler)
        }
    }
    // NOT_SUPPORTED: fn Run()
    /// Run the event loop in the current thread.
    ///
    /// See [C++ `BLooper::Loop()`'s documentation](url/to/classBLooper#a5cb1d2eb7640fdcbc1085ad19583691d).
    fn loop(&self) {
        unsafe { ffi::BLooper_Loop(self.as_ptr()) }
    }
    /// Hook method that is called after a B_QUIT_REQUESTED message.
    ///
    /// See [C++ `BLooper::Quit()`'s documentation](url/to/classBLooper#a10d5a4873f14fd247890a6dfc9b8b949).
    fn quit(&self) {
        unsafe { ffi::BLooper_Quit(self.as_ptr()) }
    }
    /// Hook method that is called during a B_QUIT_REQUESTED message.
    ///
    /// See [C++ `BLooper::QuitRequested()`'s documentation](url/to/classBLooper#aff7aaf066538383f0e0ea28d18eb8863).
    fn quit_requested(&self) -> bool {
        unsafe { ffi::BLooper_QuitRequested(self.as_ptr()) }
    }
    /// Lock the looper.
    ///
    /// See [C++ `BLooper::Lock()`'s documentation](url/to/classBLooper#a757a3e406ea3b7bb9a1ad16d04689b1d).
    fn lock(&self) -> bool {
        unsafe { ffi::BLooper_Lock(self.as_ptr()) }
    }
    /// Unlock a locked looper.
    ///
    /// See [C++ `BLooper::Unlock()`'s documentation](url/to/classBLooper#ac84ea15405640082b9fda58f6672467c).
    fn unlock(&self) {
        unsafe { ffi::BLooper_Unlock(self.as_ptr()) }
    }
    /// Check if a looper is locked.
    ///
    /// See [C++ `BLooper::IsLocked()`'s documentation](url/to/classBLooper#a525d6677c2fc76a84c2d48853dad142a).
    fn is_locked(&self) -> bool {
        unsafe { ffi::BLooper_IsLocked(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LockWithTimeout()
    // NOT_SUPPORTED: fn Thread()
    // NOT_SUPPORTED: fn Team()
    // NOT_SUPPORTED: fn LooperForThread()
    // NOT_SUPPORTED: fn LockingThread()
    // NOT_SUPPORTED: fn CountLocks()
    // NOT_SUPPORTED: fn CountLockRequests()
    // NOT_SUPPORTED: fn Sem()
    /// Add a common filter to the list of filters that are applied to all incoming messages.
    ///
    /// See [C++ `BLooper::AddCommonFilter()`'s documentation](url/to/classBLooper#a25adc41bdfd741e9c82d2b469a5086b0).
    fn add_common_filter(&self, filter: *mut c_void) {
        unsafe { ffi::BLooper_AddCommonFilter(self.as_ptr(), filter) }
    }
    /// Remove a filter from the common message filter list.
    ///
    /// See [C++ `BLooper::RemoveCommonFilter()`'s documentation](url/to/classBLooper#ad2233fa143e9deff912bff39eebac4a2).
    fn remove_common_filter(&self, filter: *mut c_void) -> bool {
        unsafe { ffi::BLooper_RemoveCommonFilter(self.as_ptr(), filter) }
    }
    /// Set a new list of filters that need to be applied to all incoming messages.
    ///
    /// See [C++ `BLooper::SetCommonFilterList()`'s documentation](url/to/classBLooper#a9b39e686873763cc6491c91f77b177b7).
    fn set_common_filter_list(&self, filters: *mut c_void) {
        unsafe { ffi::BLooper_SetCommonFilterList(self.as_ptr(), filters) }
    }
    /// Return a list of filters applied to all incoming messages.
    ///
    /// See [C++ `BLooper::CommonFilterList()`'s documentation](url/to/classBLooper#ad64f1a98c1cce8087e7d492860461a62).
    fn common_filter_list(&self) -> *mut c_void {
        unsafe { ffi::BLooper_CommonFilterList(self.as_ptr()) }
    }
    // DTOR: fn ~BLooper()
}

use super::*;

// BLooper
/// This trait represents [C++ `BLooper` class](https://www.haiku-os.org/docs/api/classBLooper.html)'s methods and inheritance.
///
/// See [`LooperFromCpp`] documentation for the class usage.
pub trait LooperMethods: HandlerMethods {
    /// Post a message with the command as what identifier to this looper.
    ///
    /// See [C++ `BLooper::PostMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a0de6737bfbf8a8b4913adc8c74bb544e).
    fn post_message_uint32(&self, command: u32) -> status_t {
        unsafe { ffi::BLooper_PostMessage(self.as_ptr(), command) }
    }
    /// Post a message to this looper.
    ///
    /// See [C++ `BLooper::PostMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#ae79a7818ce950d8edcd238f7948df020).
    fn post_message_message<M: MessageMethods>(&self, message: Option<&M>) -> status_t {
        unsafe {
            let message = match &message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_PostMessage1(self.as_ptr(), message)
        }
    }
    /// Sends a message with command what identifier to the handler associated with this looper. A response may be sent to the replyTo handler asynchronously.
    ///
    /// See [C++ `BLooper::PostMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a2dc16ecf211eb7e32eaa4b08863e856d).
    fn post_message_uint32_handler<H: HandlerMethods, H2: HandlerMethods>(
        &self,
        command: u32,
        handler: Option<&H>,
        reply_to: Option<&H2>,
    ) -> status_t {
        unsafe {
            let handler = match &handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let reply_to = match &reply_to {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_PostMessage2(self.as_ptr(), command, handler, reply_to)
        }
    }
    /// Send a message to the handler associated with this looper. A response may be sent to the replyTo handler asynchronously.
    ///
    /// See [C++ `BLooper::PostMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#ac75eed80e72b236650f19b4015de6e99).
    fn post_message_message_handler<M: MessageMethods, H: HandlerMethods, H2: HandlerMethods>(
        &self,
        message: Option<&M>,
        handler: Option<&H>,
        reply_to: Option<&H2>,
    ) -> status_t {
        unsafe {
            let message = match &message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let handler = match &handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let reply_to = match &reply_to {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_PostMessage3(self.as_ptr(), message, handler, reply_to)
        }
    }
    /// Dispatch a message to a handler. Override if there are messages that you want to catch before they are sent to the handlers.
    ///
    /// See [C++ `BLooper::DispatchMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#add21ca8765c67b0dbf95b8f0361afa73).
    fn dispatch_message<M: MessageMethods, H: HandlerMethods>(
        &self,
        message: Option<&M>,
        handler: Option<&H>,
    ) {
        unsafe {
            let message = match &message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let handler = match &handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_DispatchMessage(self.as_ptr(), message, handler)
        }
    }
    /// Retrieve the current message.
    ///
    /// See [C++ `BLooper::CurrentMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a6d244af065c4a12ea795bbbc7bb20e07).
    fn current_message(&self) -> Option<MessageFromCpp<true>> {
        unsafe { Message::option_from(ffi::BLooper_CurrentMessage(self.as_ptr())) }
    }
    /// Get ownership of the message currently being processed.
    ///
    /// See [C++ `BLooper::DetachCurrentMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a7c8f05bcc354bc1d53026417417120e3).
    fn detach_current_message(&self) -> Option<MessageFromCpp<true>> {
        unsafe { Message::option_from(ffi::BLooper_DetachCurrentMessage(self.as_ptr())) }
    }
    /// Internal method to support single-threaded GUI toolkits.
    ///
    /// See [C++ `BLooper::DispatchExternalMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a3a2d3773466de19a6c117f3a04861b11).
    fn dispatch_external_message<M: MessageMethods, H: HandlerMethods>(
        &self,
        message: Option<&M>,
        handler: Option<&H>,
        _detached: *mut c_void,
    ) {
        unsafe {
            let message = match &message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let handler = match &handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_DispatchExternalMessage(self.as_ptr(), message, handler, _detached)
        }
    }
    /// Get a pointer to the internal message queue of this looper.
    ///
    /// See [C++ `BLooper::MessageQueue()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#ad0643973ceda5cca540c4684a9f670ac).
    fn message_queue(&self) -> *mut c_void {
        unsafe { ffi::BLooper_MessageQueue(self.as_ptr()) }
    }
    /// Check if there is a message waiting.
    ///
    /// See [C++ `BLooper::IsMessageWaiting()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a27fd22b9eeaeadbeff1e973ed5242a64).
    fn is_message_waiting(&self) -> bool {
        unsafe { ffi::BLooper_IsMessageWaiting(self.as_ptr()) }
    }
    /// Associate a handler to this looper.
    ///
    /// See [C++ `BLooper::AddHandler()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a484be74814014e3c48c1a16f44e34074).
    fn add_handler<H: HandlerMethods>(&self, handler: Option<&H>) {
        unsafe {
            let handler = match &handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_AddHandler(self.as_ptr(), handler)
        }
    }
    /// Disassociate a handler from this looper.
    ///
    /// See [C++ `BLooper::RemoveHandler()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a5af57140bf018388a2e64343f46ac330).
    fn remove_handler<H: HandlerMethods>(&self, handler: Option<&H>) -> bool {
        unsafe {
            let handler = match &handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_RemoveHandler(self.as_ptr(), handler)
        }
    }
    /// Get the number of handlers associated with this looper.
    ///
    /// See [C++ `BLooper::CountHandlers()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a6179629238d0be92d7a89503be24d4e3).
    fn count_handlers(&self) -> i32 {
        unsafe { ffi::BLooper_CountHandlers(self.as_ptr()) }
    }
    /// Get the handler at an index of the list of associated handlers.
    ///
    /// See [C++ `BLooper::HandlerAt()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a6976132123edacf4b3a2831eadf5f4bf).
    fn handler_at(&self, index: i32) -> Option<HandlerFromCpp<true>> {
        unsafe { Handler::option_from(ffi::BLooper_HandlerAt(self.as_ptr(), index)) }
    }
    /// Get the index of the handler that is in the associated handler list.
    ///
    /// See [C++ `BLooper::IndexOf()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a3061534f6d2f3186efca4cddd19f378f).
    fn index_of<H: HandlerMethods>(&self, handler: Option<&H>) -> i32 {
        unsafe {
            let handler = match &handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_IndexOf(self.as_ptr(), handler)
        }
    }
    /// Get the preferred handler.
    ///
    /// See [C++ `BLooper::PreferredHandler()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#af1a4af6872abc40a887bfcabd55aff98).
    fn preferred_handler(&self) -> Option<HandlerFromCpp<true>> {
        unsafe { Handler::option_from(ffi::BLooper_PreferredHandler(self.as_ptr())) }
    }
    /// Set a preferred handler.
    ///
    /// See [C++ `BLooper::SetPreferredHandler()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a07c6e5c65354b48465fde2b0b44e22c6).
    fn set_preferred_handler<H: HandlerMethods>(&self, handler: Option<&H>) {
        unsafe {
            let handler = match &handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BLooper_SetPreferredHandler(self.as_ptr(), handler)
        }
    }
    /// Start the event loop.
    ///
    /// See [C++ `BLooper::Run()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a88bb85410e0c4124bdc6f16168df4811).
    fn run(&self) -> thread_id {
        unsafe { ffi::BLooper_Run(self.as_ptr()) }
    }
    /// Run the event loop in the current thread.
    ///
    /// See [C++ `BLooper::Loop()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a5cb1d2eb7640fdcbc1085ad19583691d).
    fn loop_(&self) {
        unsafe { ffi::BLooper_Loop(self.as_ptr()) }
    }
    /// Hook method that is called after a B_QUIT_REQUESTED message.
    ///
    /// See [C++ `BLooper::Quit()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a10d5a4873f14fd247890a6dfc9b8b949).
    fn quit(&self) {
        unsafe { ffi::BLooper_Quit(self.as_ptr()) }
    }
    /// Hook method that is called during a B_QUIT_REQUESTED message.
    ///
    /// See [C++ `BLooper::QuitRequested()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#aff7aaf066538383f0e0ea28d18eb8863).
    fn quit_requested(&self) -> bool {
        unsafe { ffi::BLooper_QuitRequested(self.as_ptr()) }
    }
    /// Lock the looper.
    ///
    /// See [C++ `BLooper::Lock()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a757a3e406ea3b7bb9a1ad16d04689b1d).
    fn lock(&self) -> bool {
        unsafe { ffi::BLooper_Lock(self.as_ptr()) }
    }
    /// Unlock a locked looper.
    ///
    /// See [C++ `BLooper::Unlock()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#ac84ea15405640082b9fda58f6672467c).
    fn unlock(&self) {
        unsafe { ffi::BLooper_Unlock(self.as_ptr()) }
    }
    /// Check if a looper is locked.
    ///
    /// See [C++ `BLooper::IsLocked()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a525d6677c2fc76a84c2d48853dad142a).
    fn is_locked(&self) -> bool {
        unsafe { ffi::BLooper_IsLocked(self.as_ptr()) }
    }
    /// Lock a looper with a timeout.
    ///
    /// See [C++ `BLooper::LockWithTimeout()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a734cdb06bfe92efdea24528c9b43bfc1).
    fn lock_with_timeout(&self, timeout: bigtime_t) -> status_t {
        unsafe { ffi::BLooper_LockWithTimeout(self.as_ptr(), timeout) }
    }
    /// Return the thread id of the internal message looper thread.
    ///
    /// See [C++ `BLooper::Thread()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a257d396d04d128edf7a0c9669ead36c0).
    fn thread(&self) -> thread_id {
        unsafe { ffi::BLooper_Thread(self.as_ptr()) }
    }
    /// Return the team id in which this looper exists.
    ///
    /// See [C++ `BLooper::Team()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#afeb7a4900f26e7746d6c8bdb5cf684f3).
    fn team(&self) -> team_id {
        unsafe { ffi::BLooper_Team(self.as_ptr()) }
    }
    /// Static method to retrieve a BLooper for a specified thread.
    ///
    /// See [C++ `BLooper::LooperForThread()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#acf16bb9628e7dbbace3920ebeff6dc34).
    fn looper_for_thread(thread: thread_id) -> Option<LooperFromCpp<true>> {
        unsafe { Looper::option_from(ffi::BLooper_LooperForThread(thread)) }
    }
    /// Return the thread id of the thread that currently holds the lock.
    ///
    /// See [C++ `BLooper::LockingThread()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a1d5d9dbc9d890ed1329b69fbfe9254b3).
    fn locking_thread(&self) -> thread_id {
        unsafe { ffi::BLooper_LockingThread(self.as_ptr()) }
    }
    /// Return the number of recursive locks that are currently being held on this looper.
    ///
    /// See [C++ `BLooper::CountLocks()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a4eec703acd9bd7fe9a455af0f81e08f9).
    fn count_locks(&self) -> i32 {
        unsafe { ffi::BLooper_CountLocks(self.as_ptr()) }
    }
    /// Return the number of pending locks.
    ///
    /// See [C++ `BLooper::CountLockRequests()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a64e7b89294df1f7b614d1267f870ecd9).
    fn count_lock_requests(&self) -> i32 {
        unsafe { ffi::BLooper_CountLockRequests(self.as_ptr()) }
    }
    /// Return the id of the semaphore that is used to lock this looper.
    ///
    /// See [C++ `BLooper::Sem()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#aa96df67561b0ce1428ae45148c21e01f).
    fn sem(&self) -> sem_id {
        unsafe { ffi::BLooper_Sem(self.as_ptr()) }
    }
    /// Add a common filter to the list of filters that are applied to all incoming messages.
    ///
    /// See [C++ `BLooper::AddCommonFilter()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a25adc41bdfd741e9c82d2b469a5086b0).
    fn add_common_filter(&self, filter: *mut c_void) {
        unsafe { ffi::BLooper_AddCommonFilter(self.as_ptr(), filter) }
    }
    /// Remove a filter from the common message filter list.
    ///
    /// See [C++ `BLooper::RemoveCommonFilter()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#ad2233fa143e9deff912bff39eebac4a2).
    fn remove_common_filter(&self, filter: *mut c_void) -> bool {
        unsafe { ffi::BLooper_RemoveCommonFilter(self.as_ptr(), filter) }
    }
    /// Set a new list of filters that need to be applied to all incoming messages.
    ///
    /// See [C++ `BLooper::SetCommonFilterList()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a9b39e686873763cc6491c91f77b177b7).
    fn set_common_filter_list(&self, filters: *mut c_void) {
        unsafe { ffi::BLooper_SetCommonFilterList(self.as_ptr(), filters) }
    }
    /// Return a list of filters applied to all incoming messages.
    ///
    /// See [C++ `BLooper::CommonFilterList()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#ad64f1a98c1cce8087e7d492860461a62).
    fn common_filter_list(&self) -> *mut c_void {
        unsafe { ffi::BLooper_CommonFilterList(self.as_ptr()) }
    }
    // DTOR: fn ~BLooper()
}

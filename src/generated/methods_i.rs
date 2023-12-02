use super::*;

// BInvoker
/// This trait represents [C++ `BInvoker` class](https://www.haiku-os.org/docs/api/classBInvoker.html)'s methods and inheritance.
///
/// See [`InvokerFromCpp`] documentation for the class usage.
pub trait InvokerMethods: RustBindingMethods {
    fn as_invoker(&self) -> *mut c_void {
        unsafe { self.as_ptr() }
    }
    // DTOR: fn ~BInvoker()
    /// Returns the message's what data member.
    ///
    /// See [C++ `BInvoker::Command()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#a6eb323ab957a5ee99e8db2b2fb0e9800).
    fn command(&self) -> u32 {
        unsafe { ffi::BInvoker_Command(self.as_invoker()) }
    }
    /// Returns the previously set reply handler or NULL if not set.
    ///
    /// See [C++ `BInvoker::HandlerForReply()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#a710e78abc03eb29e9e25d11d34c874cc).
    fn handler_for_reply(&self) -> Option<HandlerFromCpp<true>> {
        unsafe { Handler::option_from(ffi::BInvoker_HandlerForReply(self.as_invoker())) }
    }
    /// Sends the message to the invoker's target.
    ///
    /// See [C++ `BInvoker::Invoke()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#ad4eb9ad4b3b8286f2a155ad7f087511c).
    fn invoke<M: MessageMethods>(&self, message: Option<&M>) -> status_t {
        unsafe {
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BInvoker_Invoke(self.as_invoker(), message)
        }
    }
    /// Sends the message to its target, using the notification code specified by kind.
    ///
    /// See [C++ `BInvoker::InvokeNotify()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#ab009fe9c6a1f64798c614612e8c3db20).
    fn invoke_notify<M: MessageMethods>(&self, message: Option<&M>, kind: u32) -> status_t {
        unsafe {
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BInvoker_InvokeNotify(self.as_invoker(), message, kind)
        }
    }
    /// Returns whether or not the invoker and its target belong to the same team.
    ///
    /// See [C++ `BInvoker::IsTargetLocal()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#a17470932a103267815f01d0df9e891e9).
    fn is_target_local(&self) -> bool {
        unsafe { ffi::BInvoker_IsTargetLocal(self.as_invoker()) }
    }
    /// Returns a pointer to the invoker's message object.
    ///
    /// See [C++ `BInvoker::Message()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#ab0b73729de282c93b4da51e6b9dbc170).
    fn message(&self) -> Option<MessageFromCpp<true>> {
        unsafe { Message::option_from(ffi::BInvoker_Message(self.as_invoker())) }
    }
    // NOT_SUPPORTED: fn Messenger()
    /// Sets the BHandler object responsible for handling reply messages.
    ///
    /// See [C++ `BInvoker::SetHandlerForReply()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#a9983897bef828eea70f574f9571b8e85).
    fn set_handler_for_reply<H: HandlerMethods>(&self, handler: Option<&H>) -> status_t {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BInvoker_SetHandlerForReply(self.as_invoker(), handler)
        }
    }
    /// Assigns message to the invoker, deleting any previously assigned message.
    ///
    /// See [C++ `BInvoker::SetMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#a42fae8f984af3765c1a6af404dc36816).
    fn set_message<M: MessageMethods>(&self, message: Option<&M>) -> status_t {
        unsafe {
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BInvoker_SetMessage(self.as_invoker(), message)
        }
    }
    // NOT_SUPPORTED: fn SetTarget()
    /// Sets the target to either a local handler or as the preferred handler of a local looper.
    ///
    /// See [C++ `BInvoker::SetTarget()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#ac857ceb882d95a5826447dc12dd06c74).
    fn set_target<H: HandlerMethods, L: LooperMethods>(
        &self,
        handler: Option<&H>,
        looper: Option<&L>,
    ) -> status_t {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let looper = match looper {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BInvoker_SetTarget1(self.as_invoker(), handler, looper)
        }
    }
    /// Sets the timeout to use when sending the message to the target.
    ///
    /// See [C++ `BInvoker::SetTimeout()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#ab3c4f55b01f3775832871944b5c943de).
    fn set_timeout(&self, timeout: bigtime_t) -> status_t {
        unsafe { ffi::BInvoker_SetTimeout(self.as_invoker(), timeout) }
    }
    /// Invoke BMessenger::Target() on the internal messenger.
    ///
    /// See [C++ `BInvoker::Target()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#aba5a469f2e0c94c08553ebe59c238cc8).
    fn target<L: LooperMethods>(&self, _looper: Option<&L>) -> Option<HandlerFromCpp<true>> {
        unsafe {
            let _looper = match _looper {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Handler::option_from(ffi::BInvoker_Target(self.as_invoker(), _looper))
        }
    }
    /// Returns the current timeout value.
    ///
    /// See [C++ `BInvoker::Timeout()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#af2632aee4882b11ed1bc34ce55ef4bbe).
    fn timeout(&self) -> bigtime_t {
        unsafe { ffi::BInvoker_Timeout(self.as_invoker()) }
    }
}

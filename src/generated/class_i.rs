use super::*;

// BInvoker
binding! {
    /// An object that can be "invoked" to send a message to a BHandler.
    /// - [`Invoker`] represents a C++ `BInvoker` class instance which your code has ownership, [`InvokerFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BInvoker` class's documentation](https://www.haiku-os.org/docs/api/classBInvoker.html) for more details.
    #[doc(alias = "BInvoker")]
    #[doc(alias = "Invoker")]
    class Invoker
        = InvokerFromCpp<false>(BInvoker) impl
        InvokerMethods
}
impl<const FROM_CPP: bool> InvokerFromCpp<FROM_CPP> {
    /// Initializes a BInvoker without a message or target.
    ///
    /// See [C++ `BInvoker::BInvoker()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#ad03149db91cd6460f12adb72780a7080).
    pub fn new() -> InvokerFromCpp<FROM_CPP> {
        unsafe { InvokerFromCpp(ffi::BInvoker_new()) }
    }
    // NOT_SUPPORTED: fn BInvoker1()
    /// Initializes the BInvoker with message and sets the target to either a local handler or as the preferred handler of a local looper where the message is sent when Invoke() is called.
    ///
    /// See [C++ `BInvoker::BInvoker()`'s documentation](https://www.haiku-os.org/docs/api/classBInvoker.html#a6405334f1589c7a9fc3604ff8a4c8c83).
    pub fn new_with_message<M: MessageMethods, H: HandlerMethods, L: LooperMethods>(
        message: Option<&M>,
        handler: Option<&H>,
        looper: Option<&L>,
    ) -> InvokerFromCpp<FROM_CPP> {
        unsafe {
            let message = match &message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let handler = match &handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let looper = match &looper {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            InvokerFromCpp(ffi::BInvoker_new2(message, handler, looper))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for InvokerFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for InvokerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BInvoker_delete(self.0) }
        }
    }
}

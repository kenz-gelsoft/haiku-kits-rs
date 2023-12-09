use super::*;

// BHandler
binding! {
    /// Handles messages that are passed on by a BLooper.
    /// - [`Handler`] represents a C++ `BHandler` class instance which your code has ownership, [`HandlerFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BHandler` class's documentation](https://www.haiku-os.org/docs/api/classBHandler.html) for more details.
    #[doc(alias = "BHandler")]
    #[doc(alias = "Handler")]
    class Handler
        = HandlerFromCpp<false>(BHandler) impl
        HandlerMethods
        // ArchivableMethods
}
impl<const FROM_CPP: bool> HandlerFromCpp<FROM_CPP> {
    /// Construct a handler from an archived message.
    ///
    /// See [C++ `BHandler::BHandler()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#add8fa081a7bb8633581e78777b215d0b).
    pub fn new_with_message<M: MessageMethods>(data: Option<&M>) -> HandlerFromCpp<FROM_CPP> {
        unsafe {
            let data = match &data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            HandlerFromCpp(ffi::BHandler_new(data))
        }
    }
    /// Construct a new handler with a name.
    ///
    /// See [C++ `BHandler::BHandler()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a2142e21fe781b24f914ec2086e5f05b7).
    pub fn new_with_str(name: Option<&str>) -> HandlerFromCpp<FROM_CPP> {
        unsafe {
            let name = match name {
                Some(s) => Some(CString::from_vec_unchecked(s.into())),
                None => None,
            };
            let name = match &name {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            HandlerFromCpp(ffi::BHandler_new1(name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HandlerFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<HandlerFromCpp<FROM_CPP>> for ArchivableFromCpp<FROM_CPP> {
    fn from(o: HandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for HandlerFromCpp<FROM_CPP> {
    fn dynamic_cast<T: DynamicCast>(from: &T) -> Option<Self::CppManaged> {
        unsafe { Self::CppManaged::option_from(ffi::BHandler_dynamic_cast(from.as_ptr())) }
    }
}
impl<const FROM_CPP: bool> Drop for HandlerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BArchivable_delete(self.0) }
        }
    }
}
impl<const FROM_CPP: bool> ArchivableMethods for HandlerFromCpp<FROM_CPP> {
    /// Static method to instantiate a handler from an archived message.
    ///
    /// See [C++ `BHandler::Instantiate()`'s documentation](https://www.haiku-os.org/docs/api/classBHandler.html#a0c23aeb48d578525f81ba6d47f968528).
    fn instantiate<M: MessageMethods>(data: Option<&M>) -> Option<ArchivableFromCpp<true>> {
        unsafe {
            let data = match &data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Archivable::option_from(ffi::BHandler_Instantiate(data))
        }
    }
}

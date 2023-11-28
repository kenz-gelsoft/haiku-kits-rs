use super::*;


// BLooper
binding! {
    /// Receive and process messages in a separate thread.
    /// - [`Looper`] represents a C++ `BLooper` class instance which your code has ownership, [`LooperFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BLooper` class's documentation](https://www.haiku-os.org/docs/api/classBLooper.html) for more details.
    #[doc(alias = "BLooper")]
    #[doc(alias = "Looper")]
    class Looper
        = LooperFromCpp<false>(BLooper) impl
        LooperMethods,
        HandlerMethods
        // ArchivableMethods
}
impl<const FROM_CPP: bool> LooperFromCpp<FROM_CPP> {
    /// Construct a looper from an archived message.
    ///
    /// See [C++ `BLooper::BLooper()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#aad314758fd652fb48d61bcccab8b6ae3).
    pub fn new_with_message<M: MessageMethods>(data: Option<&M>) -> LooperFromCpp<FROM_CPP> {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            LooperFromCpp(ffi::BLooper_new(data))
        }
    }
    /// Construct a new BLooper with a priority and an capacity.
    ///
    /// See [C++ `BLooper::BLooper()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#a33fa84a6ed383e5a897d11380d72ce38).
    pub fn new_with_str(name: &str, priority: i32, port_capacity: i32) -> LooperFromCpp<FROM_CPP> {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            LooperFromCpp(ffi::BLooper_new1(name, priority, port_capacity))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for LooperFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<LooperFromCpp<FROM_CPP>> for HandlerFromCpp<FROM_CPP> {
    fn from(o: LooperFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<LooperFromCpp<FROM_CPP>> for ArchivableFromCpp<FROM_CPP> {
    fn from(o: LooperFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for LooperFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BArchivable_delete(self.0) }
        }
    }
}
impl<const FROM_CPP: bool> ArchivableMethods for LooperFromCpp<FROM_CPP> {
    /// Static method to instantiate a looper from an archived message.
    ///
    /// See [C++ `BLooper::Instantiate()`'s documentation](https://www.haiku-os.org/docs/api/classBLooper.html#aee61314ab77c54a64f8122440189b73a).
    fn instantiate<M: MessageMethods>(data: Option<&M>) -> Option<ArchivableFromCpp<true>> {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Archivable::option_from(ffi::BLooper_Instantiate(data))
        }
    }
}

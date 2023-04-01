use super::*;


// BLooper
wxwidgets! {
    /// Receive and process messages in a separate thread.
    /// - [`Looper`] represents a C++ `BLooper` class instance which your code has ownership, [`LooperFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BLooper` class's documentation](url/to/classBLooper) for more details.
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
    /// See [C++ `BLooper::BLooper()`'s documentation](url/to/classBLooper#aad314758fd652fb48d61bcccab8b6ae3).
    pub fn new(data: *mut c_void) -> LooperFromCpp<FROM_CPP> {
        unsafe { LooperFromCpp(ffi::BLooper_new(data)) }
    }
    // NOT_SUPPORTED: fn BLooper1()
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
impl<const FROM_CPP: bool> DynamicCast for LooperFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::BLooper_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for LooperFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BLooper_delete(self.0) }
        }
    }
}
impl<const FROM_CPP: bool> ArchivableMethods for LooperFromCpp<FROM_CPP> {
    /// Static method to instantiate a looper from an archived message.
    ///
    /// See [C++ `BLooper::Instantiate()`'s documentation](url/to/classBLooper#aee61314ab77c54a64f8122440189b73a).
    fn instantiate(data: *mut c_void) -> Option<ArchivableFromCpp<true>> {
        unsafe { Archivable::option_from(ffi::BLooper_Instantiate(data)) }
    }
}

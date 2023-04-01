use super::*;


// BApplication
wxwidgets! {
    /// A container object for an application.
    /// - [`Application`] represents a C++ `BApplication` class instance which your code has ownership, [`ApplicationFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BApplication` class's documentation](https://www.haiku-os.org/docs/api/classBApplication.html) for more details.
    #[doc(alias = "BApplication")]
    #[doc(alias = "Application")]
    class Application
        = ApplicationFromCpp<false>(BApplication) impl
        ApplicationMethods,
        LooperMethods,
        HandlerMethods
        // ArchivableMethods
}
impl<const FROM_CPP: bool> ApplicationFromCpp<FROM_CPP> {
    /// Initialize a BApplication object from a message.
    ///
    /// See [C++ `BApplication::BApplication()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#ae76219f7c7c91b14739e94c608f7349c).
    pub fn new_with_message(data: *mut c_void) -> ApplicationFromCpp<FROM_CPP> {
        unsafe { ApplicationFromCpp(ffi::BApplication_new(data)) }
    }
    /// Initialize a BApplication with the passed in signature.
    ///
    /// See [C++ `BApplication::BApplication()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a43b0671b327a0883720c6cc18d908149).
    pub fn new_with_char(signature: *const c_void) -> ApplicationFromCpp<FROM_CPP> {
        unsafe { ApplicationFromCpp(ffi::BApplication_new1(signature)) }
    }
    /// Initialize a BApplication with the passed in signature and a pointer to an error message.
    ///
    /// See [C++ `BApplication::BApplication()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a998f767f4ddcbb5588455c8b63e08f74).
    pub fn new_with_char_status_t(signature: *const c_void, error: *mut c_void) -> ApplicationFromCpp<FROM_CPP> {
        unsafe { ApplicationFromCpp(ffi::BApplication_new2(signature, error)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ApplicationFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ApplicationFromCpp<FROM_CPP>> for LooperFromCpp<FROM_CPP> {
    fn from(o: ApplicationFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ApplicationFromCpp<FROM_CPP>> for HandlerFromCpp<FROM_CPP> {
    fn from(o: ApplicationFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ApplicationFromCpp<FROM_CPP>> for ArchivableFromCpp<FROM_CPP> {
    fn from(o: ApplicationFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ApplicationFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::BApplication_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ApplicationFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BApplication_delete(self.0) }
        }
    }
}
impl<const FROM_CPP: bool> ArchivableMethods for ApplicationFromCpp<FROM_CPP> {
    /// Restores the BApplication object from a BMessage.
    ///
    /// See [C++ `BApplication::Instantiate()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#aa2814aceefbe18ac62814157f53a07c0).
    fn instantiate(data: *mut c_void) -> Option<ArchivableFromCpp<true>> {
        unsafe { Archivable::option_from(ffi::BApplication_Instantiate(data)) }
    }
}

// BArchivable
wxwidgets! {
    /// Interface for objects that can be archived into a BMessage.
    /// - [`Archivable`] represents a C++ `BArchivable` class instance which your code has ownership, [`ArchivableFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BArchivable` class's documentation](https://www.haiku-os.org/docs/api/classBArchivable.html) for more details.
    #[doc(alias = "BArchivable")]
    #[doc(alias = "Archivable")]
    class Archivable
        = ArchivableFromCpp<false>(BArchivable) impl
        ArchivableMethods
}
impl<const FROM_CPP: bool> ArchivableFromCpp<FROM_CPP> {
    /// Constructor. Does nothing.
    ///
    /// See [C++ `BArchivable::BArchivable()`'s documentation](https://www.haiku-os.org/docs/api/classBArchivable.html#a66e6b947f092d32f287913a8c253290f).
    pub fn new() -> ArchivableFromCpp<FROM_CPP> {
        unsafe { ArchivableFromCpp(ffi::BArchivable_new()) }
    }
    /// Constructor. Does important behind-the-scenes work in the unarchiving process.
    ///
    /// See [C++ `BArchivable::BArchivable()`'s documentation](https://www.haiku-os.org/docs/api/classBArchivable.html#aed4f0566f9659b03912a859077c27c89).
    pub fn new_with_message(from: *mut c_void) -> ArchivableFromCpp<FROM_CPP> {
        unsafe { ArchivableFromCpp(ffi::BArchivable_new1(from)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ArchivableFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for ArchivableFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BArchivable_delete(self.0) }
        }
    }
}
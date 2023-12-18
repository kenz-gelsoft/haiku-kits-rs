use super::*;

// BApplication
binding! {
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
    pub fn new_with_message<M: MessageMethods>(data: Option<&M>) -> ApplicationFromCpp<FROM_CPP> {
        unsafe {
            let data = match &data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ApplicationFromCpp(ffi::BApplication_new(data))
        }
    }
    /// Initialize a BApplication with the passed in signature.
    ///
    /// See [C++ `BApplication::BApplication()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a43b0671b327a0883720c6cc18d908149).
    pub fn new_with_str(signature: Option<&str>) -> ApplicationFromCpp<FROM_CPP> {
        unsafe {
            let signature = match signature {
                Some(s) => Some(CString::from_vec_unchecked(s.into())),
                None => None,
            };
            let signature = match &signature {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ApplicationFromCpp(ffi::BApplication_new1(signature))
        }
    }
    /// Initialize a BApplication with the passed in signature and a pointer to an error message.
    ///
    /// See [C++ `BApplication::BApplication()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#a998f767f4ddcbb5588455c8b63e08f74).
    pub fn new_with_str_status_t(
        signature: Option<&str>,
        error: *mut c_void,
    ) -> ApplicationFromCpp<FROM_CPP> {
        unsafe {
            let signature = match signature {
                Some(s) => Some(CString::from_vec_unchecked(s.into())),
                None => None,
            };
            let signature = match &signature {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ApplicationFromCpp(ffi::BApplication_new2(signature, error))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ApplicationFromCpp<FROM_CPP> {
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
    fn dynamic_cast<T: DynamicCast>(from: &T) -> Option<Self::CppManaged> {
        unsafe { Self::CppManaged::option_from(ffi::BApplication_dynamic_cast(from.as_ptr())) }
    }
}
impl<const FROM_CPP: bool> ArchivableMethods for ApplicationFromCpp<FROM_CPP> {
    /// Restores the BApplication object from a BMessage.
    ///
    /// See [C++ `BApplication::Instantiate()`'s documentation](https://www.haiku-os.org/docs/api/classBApplication.html#aa2814aceefbe18ac62814157f53a07c0).
    fn instantiate<M: MessageMethods>(data: Option<&M>) -> Option<ArchivableFromCpp<true>> {
        unsafe {
            let data = match &data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Archivable::option_from(ffi::BApplication_Instantiate(data))
        }
    }
}

// BArchivable
binding! {
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
    pub fn new_with_message<M: MessageMethods>(from: Option<&M>) -> ArchivableFromCpp<FROM_CPP> {
        unsafe {
            let from = match &from {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ArchivableFromCpp(ffi::BArchivable_new1(from))
        }
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
impl<const FROM_CPP: bool> DynamicCast for ArchivableFromCpp<FROM_CPP> {
    fn dynamic_cast<T: DynamicCast>(from: &T) -> Option<Self::CppManaged> {
        unsafe { Self::CppManaged::option_from(ffi::BArchivable_dynamic_cast(from.as_ptr())) }
    }
}

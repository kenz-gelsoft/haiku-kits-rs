use super::*;

// BView
binding! {
    /// View base class.
    /// - [`View`] represents a C++ `BView` class instance which your code has ownership, [`ViewFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BView` class's documentation](https://www.haiku-os.org/docs/api/classBView.html) for more details.
    #[doc(alias = "BView")]
    #[doc(alias = "View")]
    class View
        = ViewFromCpp<false>(BView) impl
        ViewMethods,
        HandlerMethods
        // ArchivableMethods
}
impl<const FROM_CPP: bool> ViewFromCpp<FROM_CPP> {
    /// Archive constructor.
    ///
    /// See [C++ `BView::BView()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae982fac35c278cc64990c506efe3965e).
    pub fn new_with_message<M: MessageMethods>(archive: Option<&M>) -> ViewFromCpp<FROM_CPP> {
        unsafe {
            let archive = match &archive {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ViewFromCpp(ffi::BView_new(archive))
        }
    }
    /// Standard constructor.
    ///
    /// See [C++ `BView::BView()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae3a1b2a66dc19690e766623be65c37e2).
    pub fn new_with_rect<R: RectMethods>(
        frame: &R,
        name: Option<&str>,
        resizing_mode: u32,
        flags: u32,
    ) -> ViewFromCpp<FROM_CPP> {
        unsafe {
            let frame = frame.as_ptr();
            let name = match name {
                Some(s) => Some(CString::from_vec_unchecked(s.into())),
                None => None,
            };
            let name = match &name {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ViewFromCpp(ffi::BView_new1(frame, name, resizing_mode, flags))
        }
    }
    /// Layout constructor.
    ///
    /// See [C++ `BView::BView()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ab148f59c41c9a22ced87bf7a0221ed62).
    pub fn new_with_str(
        name: Option<&str>,
        flags: u32,
        layout: *mut c_void,
    ) -> ViewFromCpp<FROM_CPP> {
        unsafe {
            let name = match name {
                Some(s) => Some(CString::from_vec_unchecked(s.into())),
                None => None,
            };
            let name = match &name {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ViewFromCpp(ffi::BView_new2(name, flags, layout))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ViewFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ViewFromCpp<FROM_CPP>> for HandlerFromCpp<FROM_CPP> {
    fn from(o: ViewFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ViewFromCpp<FROM_CPP>> for ArchivableFromCpp<FROM_CPP> {
    fn from(o: ViewFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ViewFromCpp<FROM_CPP> {
    fn dynamic_cast<T: DynamicCast>(from: &T) -> Option<Self::CppManaged> {
        unsafe { Self::CppManaged::option_from(ffi::BView_dynamic_cast(from.as_ptr())) }
    }
}
impl<const FROM_CPP: bool> Drop for ViewFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BArchivable_delete(self.0) }
        }
    }
}
impl<const FROM_CPP: bool> ArchivableMethods for ViewFromCpp<FROM_CPP> {
    /// Creates a new BView object from the data message.
    ///
    /// See [C++ `BView::Instantiate()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a730a46bedfe8db2176e4c331330e7924).
    fn instantiate<M: MessageMethods>(archive: Option<&M>) -> Option<ArchivableFromCpp<true>> {
        unsafe {
            let archive = match &archive {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Archivable::option_from(ffi::BView_Instantiate(archive))
        }
    }
}

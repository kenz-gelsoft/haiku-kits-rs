use super::*;

// BControl
binding! {
    /// BControl is the base class for user-event handling objects.
    /// - [`Control`] represents a C++ `BControl` class instance which your code has ownership, [`ControlFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BControl` class's documentation](https://www.haiku-os.org/docs/api/classBControl.html) for more details.
    #[doc(alias = "BControl")]
    #[doc(alias = "Control")]
    class Control
        = ControlFromCpp<false>(BControl) impl
        ControlMethods,
        ViewMethods,
        HandlerMethods
        // ArchivableMethods
}
impl<const FROM_CPP: bool> ControlFromCpp<FROM_CPP> {
    /// Creates a new BControl object from an data message.
    ///
    /// See [C++ `BControl::BControl()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#a1224f952eb63f1b0d338720b88fdf220).
    pub fn new_with_message<M: MessageMethods>(data: Option<&M>) -> ControlFromCpp<FROM_CPP> {
        unsafe {
            let data = match &data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ControlFromCpp(ffi::BControl_new(data))
        }
    }
    /// Construct a control in the frame with a name, label, model message, resizingMode, and creation flags.
    ///
    /// See [C++ `BControl::BControl()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#ae2bd77f0349ca0197b7599393a6ae3ca).
    pub fn new_with_rect<R: RectMethods, M: MessageMethods>(
        frame: &R,
        name: Option<&str>,
        label: Option<&str>,
        message: Option<&M>,
        resizing_mode: u32,
        flags: u32,
    ) -> ControlFromCpp<FROM_CPP> {
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
            let label = match label {
                Some(s) => Some(CString::from_vec_unchecked(s.into())),
                None => None,
            };
            let label = match &label {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = match &message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ControlFromCpp(ffi::BControl_new1(
                frame,
                name,
                label,
                message,
                resizing_mode,
                flags,
            ))
        }
    }
    /// Construct a control with a name, label, model message, and creation flags suitable for use with the Layout API.
    ///
    /// See [C++ `BControl::BControl()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#a6243ab82eecb7ac3a5c35592a057845d).
    pub fn new_with_str<M: MessageMethods>(
        name: Option<&str>,
        label: Option<&str>,
        message: Option<&M>,
        flags: u32,
    ) -> ControlFromCpp<FROM_CPP> {
        unsafe {
            let name = match name {
                Some(s) => Some(CString::from_vec_unchecked(s.into())),
                None => None,
            };
            let name = match &name {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = match label {
                Some(s) => Some(CString::from_vec_unchecked(s.into())),
                None => None,
            };
            let label = match &label {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = match &message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ControlFromCpp(ffi::BControl_new2(name, label, message, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ControlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ControlFromCpp<FROM_CPP>> for ViewFromCpp<FROM_CPP> {
    fn from(o: ControlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ControlFromCpp<FROM_CPP>> for HandlerFromCpp<FROM_CPP> {
    fn from(o: ControlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ControlFromCpp<FROM_CPP>> for ArchivableFromCpp<FROM_CPP> {
    fn from(o: ControlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ControlFromCpp<FROM_CPP> {
    fn dynamic_cast<T: DynamicCast>(from: &T) -> Option<Self::CppManaged> {
        unsafe { Self::CppManaged::option_from(ffi::BControl_dynamic_cast(from.as_ptr())) }
    }
}
// Mix-in(s) to BControl
impl<const FROM_CPP: bool> InvokerMethods for ControlFromCpp<FROM_CPP> {
    fn as_invoker(&self) -> *mut c_void {
        unsafe { ffi::BControl_AsInvoker(self.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> ArchivableMethods for ControlFromCpp<FROM_CPP> {
    /// Creates a new object from an data.
    ///
    /// See [C++ `BControl::Instantiate()`'s documentation](https://www.haiku-os.org/docs/api/classBControl.html#ad5ba32b4f839a5a9bc60fcf037e23846).
    fn instantiate<M: MessageMethods>(data: Option<&M>) -> Option<ArchivableFromCpp<true>> {
        unsafe {
            let data = match &data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Archivable::option_from(ffi::BControl_Instantiate(data))
        }
    }
}

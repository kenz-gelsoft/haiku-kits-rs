use super::*;

// BButton
binding! {
    /// A control used to initiate an action.
    /// - [`Button`] represents a C++ `BButton` class instance which your code has ownership, [`ButtonFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BButton` class's documentation](https://www.haiku-os.org/docs/api/classBButton.html) for more details.
    #[doc(alias = "BButton")]
    #[doc(alias = "Button")]
    class Button
        = ButtonFromCpp<false>(BButton) impl
        ButtonMethods,
        ControlMethods,
        ViewMethods,
        HandlerMethods
        // ArchivableMethods
}
impl<const FROM_CPP: bool> ButtonFromCpp<FROM_CPP> {
    //  ENUM: BBehavior
    pub const B_BUTTON_BEHAVIOR: c_int = 0;
    pub const B_TOGGLE_BEHAVIOR: c_int = 0 + 1;
    pub const B_POP_UP_BEHAVIOR: c_int = 0 + 2;

    /// Constructs a BButton object from an data message.
    ///
    /// See [C++ `BButton::BButton()`'s documentation](https://www.haiku-os.org/docs/api/classBButton.html#a1e4e191108e817e0c4f666e8c06b5460).
    pub fn new_with_message<M: MessageMethods>(data: Option<&M>) -> ButtonFromCpp<FROM_CPP> {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ButtonFromCpp(ffi::BButton_new(data))
        }
    }
    /// Creates and initializes a BButton control.
    ///
    /// See [C++ `BButton::BButton()`'s documentation](https://www.haiku-os.org/docs/api/classBButton.html#abc54e951f00a41e93c68487808f1605c).
    pub fn new_with_rect<R: RectMethods, M: MessageMethods>(
        frame: &R,
        name: &str,
        label: &str,
        message: Option<&M>,
        resizing_mode: u32,
        flags: u32,
    ) -> ButtonFromCpp<FROM_CPP> {
        unsafe {
            let frame = frame.as_ptr();
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let label = CString::from_vec_unchecked(label.into());
            let label = label.as_ptr();
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ButtonFromCpp(ffi::BButton_new1(
                frame,
                name,
                label,
                message,
                resizing_mode,
                flags,
            ))
        }
    }
    /// Creates and initializes a BButton control.
    ///
    /// See [C++ `BButton::BButton()`'s documentation](https://www.haiku-os.org/docs/api/classBButton.html#a8fd8464e0b2288e0e5451bc683d7d5b0).
    pub fn new_with_str_message<M: MessageMethods>(
        label: &str,
        message: Option<&M>,
    ) -> ButtonFromCpp<FROM_CPP> {
        unsafe {
            let label = CString::from_vec_unchecked(label.into());
            let label = label.as_ptr();
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ButtonFromCpp(ffi::BButton_new2(label, message))
        }
    }
    /// Creates and initializes a BButton control.
    ///
    /// See [C++ `BButton::BButton()`'s documentation](https://www.haiku-os.org/docs/api/classBButton.html#a454f793b3e781dbc681add9d35e29705).
    pub fn new_with_str_str<M: MessageMethods>(
        name: &str,
        label: &str,
        message: Option<&M>,
        flags: u32,
    ) -> ButtonFromCpp<FROM_CPP> {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let label = CString::from_vec_unchecked(label.into());
            let label = label.as_ptr();
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ButtonFromCpp(ffi::BButton_new3(name, label, message, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ButtonFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ButtonFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ButtonFromCpp<FROM_CPP>> for ViewFromCpp<FROM_CPP> {
    fn from(o: ButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ButtonFromCpp<FROM_CPP>> for HandlerFromCpp<FROM_CPP> {
    fn from(o: ButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ButtonFromCpp<FROM_CPP>> for ArchivableFromCpp<FROM_CPP> {
    fn from(o: ButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for ButtonFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BArchivable_delete(self.0) }
        }
    }
}
// Mix-in(s) to BButton
impl<const FROM_CPP: bool> InvokerMethods for ButtonFromCpp<FROM_CPP> {
    fn as_invoker(&self) -> *mut c_void {
        unsafe { ffi::BButton_AsInvoker(self.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> ArchivableMethods for ButtonFromCpp<FROM_CPP> {
    /// Creates a new BButton object from the archive message.
    ///
    /// See [C++ `BButton::Instantiate()`'s documentation](https://www.haiku-os.org/docs/api/classBButton.html#a578835166502d128d1e038602f5f9c6d).
    fn instantiate<M: MessageMethods>(data: Option<&M>) -> Option<ArchivableFromCpp<true>> {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Archivable::option_from(ffi::BButton_Instantiate(data))
        }
    }
}

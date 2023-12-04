use super::*;

// BButton
/// This trait represents [C++ `BButton` class](https://www.haiku-os.org/docs/api/classBButton.html)'s methods and inheritance.
///
/// See [`ButtonFromCpp`] documentation for the class usage.
pub trait ButtonMethods: ControlMethods {
    // DTOR: fn ~BButton()
    // NOT_SUPPORTED: fn Behavior()
    /// Returns whether or not the button is the default button on the window, i.e. whether or not it responds to the Enter key.
    ///
    /// See [C++ `BButton::IsDefault()`'s documentation](https://www.haiku-os.org/docs/api/classBButton.html#ab8a01256f309f3e75fd6c7d0564d233e).
    fn is_default(&self) -> bool {
        unsafe { ffi::BButton_IsDefault(self.as_ptr()) }
    }
    /// Returns whether or not the button is flat or not.
    ///
    /// See [C++ `BButton::IsFlat()`'s documentation](https://www.haiku-os.org/docs/api/classBButton.html#ae38d8f3e62901af40b18f1574d60346c).
    fn is_flat(&self) -> bool {
        unsafe { ffi::BButton_IsFlat(self.as_ptr()) }
    }
    /// Make the BButton the default button i.e. it will be activated when the user pushes the Enter key.
    ///
    /// See [C++ `BButton::MakeDefault()`'s documentation](https://www.haiku-os.org/docs/api/classBButton.html#a42436301d7a3e6b5e07bc1608463ac47).
    fn make_default(&self, flag: bool) {
        unsafe { ffi::BButton_MakeDefault(self.as_ptr(), flag) }
    }
    /// Returns the message sent to the button's target when the pop-up marker is selected using B_POP_UP_BEHAVIOR.
    ///
    /// See [C++ `BButton::PopUpMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBButton.html#a7ee68788feb529301ea9ccacd30fa8f1).
    fn pop_up_message(&self) -> Option<MessageFromCpp<true>> {
        unsafe { Message::option_from(ffi::BButton_PopUpMessage(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn SetBehavior()
    /// Sets or unsets the button to be flat.
    ///
    /// See [C++ `BButton::SetFlat()`'s documentation](https://www.haiku-os.org/docs/api/classBButton.html#a9f9ead66b7c99f1dbf96bc275a6f4290).
    fn set_flat(&self, flat: bool) {
        unsafe { ffi::BButton_SetFlat(self.as_ptr(), flat) }
    }
    /// Sets the message sent to the button's target when the pop-up marker is selected using B_POP_UP_BEHAVIOR.
    ///
    /// See [C++ `BButton::SetPopUpMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBButton.html#a42f542a5b3b6d9a2a7d740d468cee0eb).
    fn set_pop_up_message<M: MessageMethods>(&self, message: Option<&M>) {
        unsafe {
            let message = match &message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BButton_SetPopUpMessage(self.as_ptr(), message)
        }
    }
}

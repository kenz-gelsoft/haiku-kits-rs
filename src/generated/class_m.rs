use super::*;

// BMessage
binding! {
    /// A container that can be send and received using the Haiku messaging subsystem.
    /// - [`Message`] represents a C++ `BMessage` class instance which your code has ownership, [`MessageFromCpp`]`<true>` represents one which don't own.
    /// - See [C++ `BMessage` class's documentation](https://www.haiku-os.org/docs/api/classBMessage.html) for more details.
    #[doc(alias = "BMessage")]
    #[doc(alias = "Message")]
    class Message
        = MessageFromCpp<false>(BMessage) impl
        MessageMethods
}
impl<const FROM_CPP: bool> MessageFromCpp<FROM_CPP> {
    //  ENUM: @4
    pub const sNumReplyPorts: c_int = 3;

    /// Construct an empty message, without any data members and with a what constant set to 0.
    ///
    /// See [C++ `BMessage::BMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a2d0749c439c93a1ca938378b6c890d07).
    pub fn new() -> MessageFromCpp<FROM_CPP> {
        unsafe { MessageFromCpp(ffi::BMessage_new()) }
    }
    /// Construct a new message that is a copy of another message.
    ///
    /// See [C++ `BMessage::BMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a6ab8aaec65db99134ea76790884c3af7).
    pub fn new_with_message<M: MessageMethods>(other: &M) -> MessageFromCpp<FROM_CPP> {
        unsafe {
            let other = other.as_ptr();
            MessageFromCpp(ffi::BMessage_new1(other))
        }
    }
    /// Construct an empty message with the what member set to the specified value.
    ///
    /// See [C++ `BMessage::BMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ae56247b7b613c0b1b626d1ea57f5beab).
    pub fn new_with_uint32(what: u32) -> MessageFromCpp<FROM_CPP> {
        unsafe { MessageFromCpp(ffi::BMessage_new2(what)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MessageFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for MessageFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::BMessage_delete(self.0) }
        }
    }
}

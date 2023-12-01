use super::*;


// BMessage
    /// This trait represents [C++ `BMessage` class](https://www.haiku-os.org/docs/api/classBMessage.html)'s methods and inheritance.
    ///
    /// See [`MessageFromCpp`] documentation for the class usage.
pub trait MessageMethods: RustBindingMethods {
    /// A 4-byte constant that determines the type of message.
    ///
    /// See [C++ `BMessage::what()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ad074faab0857f51fba778f0d8558df1e).
    fn what(&self) -> u32 {
        unsafe { ffi::BMessage_what(self.as_ptr()) }
    }
    /// A 4-byte constant that determines the type of message.
    ///
    /// See [C++ `BMessage::set_what()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ad074faab0857f51fba778f0d8558df1e).
    fn set_what(&self, what: u32) {
        unsafe { ffi::BMessage_set_what(self.as_ptr(), what) }
    }
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator new()
    // BLOCKED: fn operator new1()
    // BLOCKED: fn operator new2()
    // BLOCKED: fn operator delete()
    // NOT_SUPPORTED: fn GetInfo()
    /// Retrieve the type and the number of data items in this message that are associated with a name.
    ///
    /// See [C++ `BMessage::GetInfo()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a525991f82f8574bd6eb68796568eb340).
    fn get_info_int32(&self, name: &str, type_found: *mut c_void, count_found: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetInfo1(self.as_ptr(), name, type_found, count_found)
        }
    }
    /// Retrieve the type and whether or not the size of the data is fixed associated with a name.
    ///
    /// See [C++ `BMessage::GetInfo()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a6c8080ba2bdff134fd20e923145e4200).
    fn get_info_bool(&self, name: &str, type_found: *mut c_void, fixed_size: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetInfo2(self.as_ptr(), name, type_found, fixed_size)
        }
    }
    /// Retrieve the type and whether or not the size of the data is fixed associated with a name.
    ///
    /// See [C++ `BMessage::GetInfo()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a055eb9be666ebf7adad3880083606bb3).
    fn get_info_int32_bool(&self, name: &str, type_found: *mut c_void, count_found: *mut c_void, fixed_size: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetInfo3(self.as_ptr(), name, type_found, count_found, fixed_size)
        }
    }
    // NOT_SUPPORTED: fn CountNames()
    /// Check if the message has data members.
    ///
    /// See [C++ `BMessage::IsEmpty()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a773590d77b8d751ce9249e6acf3a463b).
    fn is_empty(&self) -> bool {
        unsafe { ffi::BMessage_IsEmpty(self.as_ptr()) }
    }
    /// Check if the message is a system message.
    ///
    /// See [C++ `BMessage::IsSystem()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a74a596ec83515e50217ab478f302bd99).
    fn is_system(&self) -> bool {
        unsafe { ffi::BMessage_IsSystem(self.as_ptr()) }
    }
    /// Check if the message is a reply to a (previous) message.
    ///
    /// See [C++ `BMessage::IsReply()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a01c7dd891b3b05eb7f458a621c5181d4).
    fn is_reply(&self) -> bool {
        unsafe { ffi::BMessage_IsReply(self.as_ptr()) }
    }
    /// Print the message to the standard output.
    ///
    /// See [C++ `BMessage::PrintToStream()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a6aa2006d1e5c6fcba3f5c6e8467a50a1).
    fn print_to_stream(&self) {
        unsafe { ffi::BMessage_PrintToStream(self.as_ptr()) }
    }
    /// Rename a data label.
    ///
    /// See [C++ `BMessage::Rename()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a2191de782a8ccef0252007d87c8f379c).
    fn rename(&self, old_entry: &str, new_entry: &str) -> status_t {
        unsafe {
            let old_entry = CString::from_vec_unchecked(old_entry.into());
            let old_entry = old_entry.as_ptr();
            let new_entry = CString::from_vec_unchecked(new_entry.into());
            let new_entry = new_entry.as_ptr();
            ffi::BMessage_Rename(self.as_ptr(), old_entry, new_entry)
        }
    }
    /// Check if this message was delivered through the delivery methods.
    ///
    /// See [C++ `BMessage::WasDelivered()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aaddd7356ffcac4ab4d6a26e8d393e12a).
    fn was_delivered(&self) -> bool {
        unsafe { ffi::BMessage_WasDelivered(self.as_ptr()) }
    }
    /// Check if the sender expects a reply.
    ///
    /// See [C++ `BMessage::IsSourceWaiting()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#acd51918e26e998067ee6412a55068abc).
    fn is_source_waiting(&self) -> bool {
        unsafe { ffi::BMessage_IsSourceWaiting(self.as_ptr()) }
    }
    /// Check if the message is sent by another application.
    ///
    /// See [C++ `BMessage::IsSourceRemote()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a3735f85fdf75af5309d0e61279d452c3).
    fn is_source_remote(&self) -> bool {
        unsafe { ffi::BMessage_IsSourceRemote(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn ReturnAddress()
    /// Get the message to which this message is a reply.
    ///
    /// See [C++ `BMessage::Previous()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a57c84d02e54ef19fd1516d9bf7e3fe45).
    fn previous(&self) -> Option<MessageFromCpp<true>> {
        unsafe { Message::option_from(ffi::BMessage_Previous(self.as_ptr())) }
    }
    /// Check if the message was delivered through 'drag and drop'.
    ///
    /// See [C++ `BMessage::WasDropped()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac66a35a4ef389a02bc36ace2cddb073e).
    fn was_dropped(&self) -> bool {
        unsafe { ffi::BMessage_WasDropped(self.as_ptr()) }
    }
    /// Get the coordinates of the drop point of the message.
    ///
    /// See [C++ `BMessage::DropPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a6785e345338000b685a3b5b182cc993a).
    fn drop_point<P: PointMethods>(&self, offset: Option<&P>) -> Point {
        unsafe {
            let offset = match offset {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Point::from_ptr(ffi::BMessage_DropPoint(self.as_ptr(), offset))
        }
    }
    /// Asynchronously send a reply to this message.
    ///
    /// See [C++ `BMessage::SendReply()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aad1eb085346e2c91133e372e7924637c).
    fn send_reply_uint32_handler<H: HandlerMethods>(&self, command: u32, reply_to: Option<&H>) -> status_t {
        unsafe {
            let reply_to = match reply_to {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_SendReply(self.as_ptr(), command, reply_to)
        }
    }
    /// Asynchronously send a reply to this message.
    ///
    /// See [C++ `BMessage::SendReply()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a08a2f161642de20dc483afb0ac4db167).
    fn send_reply_message_handler<M: MessageMethods, H: HandlerMethods>(&self, reply: Option<&M>, reply_to: Option<&H>, timeout: bigtime_t) -> status_t {
        unsafe {
            let reply = match reply {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let reply_to = match reply_to {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_SendReply1(self.as_ptr(), reply, reply_to, timeout)
        }
    }
    // NOT_SUPPORTED: fn SendReply2()
    /// Synchronously send a reply to this message, and wait for a reply back.
    ///
    /// See [C++ `BMessage::SendReply()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a039a8ff8563bbc58c54a50ca417ac75e).
    fn send_reply_uint32_message<M: MessageMethods>(&self, command: u32, reply_to_reply: Option<&M>) -> status_t {
        unsafe {
            let reply_to_reply = match reply_to_reply {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_SendReply3(self.as_ptr(), command, reply_to_reply)
        }
    }
    /// Synchronously send a reply to this message, and wait for a reply back.
    ///
    /// See [C++ `BMessage::SendReply()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a5461fc22f8bf044d21fd5429b7977213).
    fn send_reply_message_message<M: MessageMethods, M2: MessageMethods>(&self, reply: Option<&M>, reply_to_reply: Option<&M2>, send_timeout: bigtime_t, reply_timeout: bigtime_t) -> status_t {
        unsafe {
            let reply = match reply {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let reply_to_reply = match reply_to_reply {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_SendReply4(self.as_ptr(), reply, reply_to_reply, send_timeout, reply_timeout)
        }
    }
    // NOT_SUPPORTED: fn FlattenedSize()
    // NOT_SUPPORTED: fn Flatten()
    /// Flatten the message to a stream.
    ///
    /// See [C++ `BMessage::Flatten()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ae0b47fbff10bc50153144b178fc1537d).
    fn flatten(&self, stream: *mut c_void, size: *mut c_void) -> status_t {
        unsafe { ffi::BMessage_Flatten1(self.as_ptr(), stream, size) }
    }
    /// Unflatten a message from a buffer and put it into the current object.
    ///
    /// See [C++ `BMessage::Unflatten()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a6704fc8df822fc366b616c3abf36ae9f).
    fn unflatten_str(&self, flat_buffer: &str) -> status_t {
        unsafe {
            let flat_buffer = CString::from_vec_unchecked(flat_buffer.into());
            let flat_buffer = flat_buffer.as_ptr();
            ffi::BMessage_Unflatten(self.as_ptr(), flat_buffer)
        }
    }
    /// Unflatten a message from a stream and put it into the current object.
    ///
    /// See [C++ `BMessage::Unflatten()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a7ded81e10fed9a25b1a22934e04928dc).
    fn unflatten_dataio(&self, stream: *mut c_void) -> status_t {
        unsafe { ffi::BMessage_Unflatten1(self.as_ptr(), stream) }
    }
    /// Undocumented.
    ///
    /// See [C++ `BMessage::AddSpecifier()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a8f6efbc2f8c4faf38d1a6dab304b8880).
    fn add_specifier_str(&self, property: &str) -> status_t {
        unsafe {
            let property = CString::from_vec_unchecked(property.into());
            let property = property.as_ptr();
            ffi::BMessage_AddSpecifier(self.as_ptr(), property)
        }
    }
    /// Undocumented.
    ///
    /// See [C++ `BMessage::AddSpecifier()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a1e904ab6672cd0b29007fa9859ffe685).
    fn add_specifier_str_int32(&self, property: &str, index: i32) -> status_t {
        unsafe {
            let property = CString::from_vec_unchecked(property.into());
            let property = property.as_ptr();
            ffi::BMessage_AddSpecifier1(self.as_ptr(), property, index)
        }
    }
    /// Undocumented.
    ///
    /// See [C++ `BMessage::AddSpecifier()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a1ae990808fff37361f1abe3688dbb181).
    fn add_specifier_str_int32_int32(&self, property: &str, index: i32, range: i32) -> status_t {
        unsafe {
            let property = CString::from_vec_unchecked(property.into());
            let property = property.as_ptr();
            ffi::BMessage_AddSpecifier2(self.as_ptr(), property, index, range)
        }
    }
    /// Undocumented.
    ///
    /// See [C++ `BMessage::AddSpecifier()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a3ba4e7e99c78281226c2befb8912e327).
    fn add_specifier_str_str(&self, property: &str, name: &str) -> status_t {
        unsafe {
            let property = CString::from_vec_unchecked(property.into());
            let property = property.as_ptr();
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddSpecifier3(self.as_ptr(), property, name)
        }
    }
    /// Undocumented.
    ///
    /// See [C++ `BMessage::AddSpecifier()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a29406ecff2333e64d28850548a3b463c).
    fn add_specifier_message<M: MessageMethods>(&self, specifier: Option<&M>) -> status_t {
        unsafe {
            let specifier = match specifier {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_AddSpecifier4(self.as_ptr(), specifier)
        }
    }
    /// Undocumented.
    ///
    /// See [C++ `BMessage::SetCurrentSpecifier()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aa4bcec94162df7498208a1136c0f05ba).
    fn set_current_specifier(&self, index: i32) -> status_t {
        unsafe { ffi::BMessage_SetCurrentSpecifier(self.as_ptr(), index) }
    }
    /// Undocumented.
    ///
    /// See [C++ `BMessage::GetCurrentSpecifier()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a641086c9321e56195843a3224bb0f11d).
    fn get_current_specifier<M: MessageMethods>(&self, index: *mut c_void, specifier: Option<&M>, what: *mut c_void, property: &str) -> status_t {
        unsafe {
            let specifier = match specifier {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let property = CString::from_vec_unchecked(property.into());
            let property = property.as_ptr();
            ffi::BMessage_GetCurrentSpecifier(self.as_ptr(), index, specifier, what, property)
        }
    }
    /// Undocumented.
    ///
    /// See [C++ `BMessage::HasSpecifiers()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a70f59c7377d4d67f47feb212b73c8b25).
    fn has_specifiers(&self) -> bool {
        unsafe { ffi::BMessage_HasSpecifiers(self.as_ptr()) }
    }
    /// Undocumented.
    ///
    /// See [C++ `BMessage::PopSpecifier()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aeab6b578828566453d03152bfb446e1a).
    fn pop_specifier(&self) -> status_t {
        unsafe { ffi::BMessage_PopSpecifier(self.as_ptr()) }
    }
    /// Convenience method to add a BAlignment to the label name.
    ///
    /// See [C++ `BMessage::AddAlignment()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4fa4e6bcc354814dc17b61532480837d).
    fn add_alignment(&self, name: &str, alignment: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddAlignment(self.as_ptr(), name, alignment)
        }
    }
    /// Convenience method to add a BRect to the label name.
    ///
    /// See [C++ `BMessage::AddRect()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a42d9e94fb88d457699d193eb5094a24c).
    fn add_rect<R: RectMethods>(&self, name: &str, rect: &R) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let rect = rect.as_ptr();
            ffi::BMessage_AddRect(self.as_ptr(), name, rect)
        }
    }
    /// Convenience method to add a BPoint to the label name.
    ///
    /// See [C++ `BMessage::AddPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#afde5dc7d76c57e5d062de58954fb2548).
    fn add_point<P: PointMethods>(&self, name: &str, point: &P) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let point = point.as_ptr();
            ffi::BMessage_AddPoint(self.as_ptr(), name, point)
        }
    }
    /// Convenience method to add a BSize to the label name.
    ///
    /// See [C++ `BMessage::AddSize()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a37d4771d6d726a439e0c0f36943d9e52).
    fn add_size<S: SizeMethods>(&self, name: &str, size: &S) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let size = size.as_ptr();
            ffi::BMessage_AddSize(self.as_ptr(), name, size)
        }
    }
    /// Convenience method to add a C-string to the label name.
    ///
    /// See [C++ `BMessage::AddString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a342311ccdf68206c4b879bcd0f2d6e83).
    fn add_string_str(&self, name: &str, string: &str) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            ffi::BMessage_AddString(self.as_ptr(), name, string)
        }
    }
    /// Convenience method to add a BString to the label name.
    ///
    /// See [C++ `BMessage::AddString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a76d63dc8621996c05a8c2b8186b28efb).
    fn add_string_string(&self, name: &str, string: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddString1(self.as_ptr(), name, string)
        }
    }
    /// Convenience method to add list of strings to the label name.
    ///
    /// See [C++ `BMessage::AddStrings()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4b3643ef27b1a9edffceac01f9d2ae89).
    fn add_strings(&self, name: &str, list: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddStrings(self.as_ptr(), name, list)
        }
    }
    /// Convenience method to add an int8 to the label name.
    ///
    /// See [C++ `BMessage::AddInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a224c55f626ae3a4552d06788539c7d60).
    fn add_int8(&self, name: &str, value: i8) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddInt8(self.as_ptr(), name, value)
        }
    }
    /// Convenience method to add an uint8 to the label name.
    ///
    /// See [C++ `BMessage::AddUInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a98bbc2ca6c4fcb26bcd162c06ea0f46f).
    fn add_u_int8(&self, name: &str, value: u8) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddUInt8(self.as_ptr(), name, value)
        }
    }
    /// Convenience method to add an int16 to the label name.
    ///
    /// See [C++ `BMessage::AddInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a8e865679288281553721414aa17a7fea).
    fn add_int16(&self, name: &str, value: i16) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddInt16(self.as_ptr(), name, value)
        }
    }
    /// Convenience method to add an uint16 to the label name.
    ///
    /// See [C++ `BMessage::AddUInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aa43cedc7a1d6ecdc26041e6657c9288d).
    fn add_u_int16(&self, name: &str, value: u16) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddUInt16(self.as_ptr(), name, value)
        }
    }
    /// Convenience method to add an int32 to the label name.
    ///
    /// See [C++ `BMessage::AddInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aa4d9d24d521c499aefd512b18d1e38c5).
    fn add_int32(&self, name: &str, value: i32) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddInt32(self.as_ptr(), name, value)
        }
    }
    /// Convenience method to add an uint32 to the label name.
    ///
    /// See [C++ `BMessage::AddUInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#acc2a562d501b88ce8fe55ae4017d3bb6).
    fn add_u_int32(&self, name: &str, value: u32) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddUInt32(self.as_ptr(), name, value)
        }
    }
    /// Convenience method to add an int64 to the label name.
    ///
    /// See [C++ `BMessage::AddInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ad172f4c606396d8457a686fb16d0be31).
    fn add_int64(&self, name: &str, value: i64) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddInt64(self.as_ptr(), name, value)
        }
    }
    /// Convenience method to add an uint64 to the label name.
    ///
    /// See [C++ `BMessage::AddUInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a1c6b3d4ee371ccb4ddfc6c0fd1dc8e58).
    fn add_u_int64(&self, name: &str, value: u64) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddUInt64(self.as_ptr(), name, value)
        }
    }
    /// Convenience method to add a bool to the label name.
    ///
    /// See [C++ `BMessage::AddBool()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a2976642c2259ebad33704d1b127bae60).
    fn add_bool(&self, name: &str, value: bool) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddBool(self.as_ptr(), name, value)
        }
    }
    /// Convenience method to add a float to the label name.
    ///
    /// See [C++ `BMessage::AddFloat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#acdffd573b491074deef980df9f7f6e07).
    fn add_float(&self, name: &str, value: c_float) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddFloat(self.as_ptr(), name, value)
        }
    }
    /// Convenience method to add a double to the label name.
    ///
    /// See [C++ `BMessage::AddDouble()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a77a042d689d478572e23e87c3ad9982f).
    fn add_double(&self, name: &str, value: c_double) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddDouble(self.as_ptr(), name, value)
        }
    }
    // NOT_SUPPORTED: fn AddColor()
    /// Convenience method to add a pointer to the label name.
    ///
    /// See [C++ `BMessage::AddPointer()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a3dc68e69cb8da2ca0f7512e42c4393fe).
    fn add_pointer(&self, name: &str, pointer: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddPointer(self.as_ptr(), name, pointer)
        }
    }
    // NOT_SUPPORTED: fn AddMessenger()
    /// Convenience method to add an entry_ref to the label name.
    ///
    /// See [C++ `BMessage::AddRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a8e67d1398ecbfc5c75ab90ee2e602af5).
    fn add_ref(&self, name: &str, ref_: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddRef(self.as_ptr(), name, ref_)
        }
    }
    /// Convenience method to add a node_ref with the label name.
    ///
    /// See [C++ `BMessage::AddNodeRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4e486773f6adca4610d01d96c0bd6d23).
    fn add_node_ref(&self, name: &str, ref_: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddNodeRef(self.as_ptr(), name, ref_)
        }
    }
    /// Convenience method to add a message to the label name.
    ///
    /// See [C++ `BMessage::AddMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a421a8dee0e636fe619e9ebf561aafa0a).
    fn add_message<M: MessageMethods>(&self, name: &str, message: Option<&M>) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_AddMessage(self.as_ptr(), name, message)
        }
    }
    // BLOCKED: fn AddFlat()
    /// Convenience method to add a flattenable to the label name.
    ///
    /// See [C++ `BMessage::AddFlat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac04c2914f73813a766c5644765a506a4).
    fn add_flat(&self, name: &str, object: *const c_void, count: i32) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_AddFlat1(self.as_ptr(), name, object, count)
        }
    }
    // NOT_SUPPORTED: fn AddData()
    /// Append the data of another message to this message.
    ///
    /// See [C++ `BMessage::Append()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac2441b6ef13598fb8289c392b6588882).
    fn append<M: MessageMethods>(&self, message: &M) -> status_t {
        unsafe {
            let message = message.as_ptr();
            ffi::BMessage_Append(self.as_ptr(), message)
        }
    }
    /// Remove data associated with name at a specified index.
    ///
    /// See [C++ `BMessage::RemoveData()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a72ce9596ebe74ce6e995705304b7fc7d).
    fn remove_data(&self, name: &str, index: i32) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_RemoveData(self.as_ptr(), name, index)
        }
    }
    /// Remove all data associated with a name.
    ///
    /// See [C++ `BMessage::RemoveName()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a5db9b5a9ddc0b04173afb8dbc1c69e5c).
    fn remove_name(&self, name: &str) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_RemoveName(self.as_ptr(), name)
        }
    }
    /// Clear all data and metadata in this message.
    ///
    /// See [C++ `BMessage::MakeEmpty()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#abac76b537c88ce7f1a127fa6425c3279).
    fn make_empty(&self) -> status_t {
        unsafe { ffi::BMessage_MakeEmpty(self.as_ptr()) }
    }
    /// Find an alignment at the label name.
    ///
    /// See [C++ `BMessage::FindAlignment()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#abf81526338001abe288b5831fcb88874).
    fn find_alignment_alignment(&self, name: &str, alignment: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindAlignment(self.as_ptr(), name, alignment)
        }
    }
    /// Find an alignment at the label name at an index.
    ///
    /// See [C++ `BMessage::FindAlignment()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac124217fb122b3e012ee431fb3cbd68f).
    fn find_alignment_int32(&self, name: &str, index: i32, alignment: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindAlignment1(self.as_ptr(), name, index, alignment)
        }
    }
    /// Find a rectangle at the label name.
    ///
    /// See [C++ `BMessage::FindRect()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a84e6020b903936f61538e33988792956).
    fn find_rect_rect<R: RectMethods>(&self, name: &str, rect: Option<&R>) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_FindRect(self.as_ptr(), name, rect)
        }
    }
    /// Find a rectangle at the label name at an index.
    ///
    /// See [C++ `BMessage::FindRect()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#abc4398bca7420cc74c4d5cc76ccba018).
    fn find_rect_int32_rect<R: RectMethods>(&self, name: &str, index: i32, rect: Option<&R>) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_FindRect1(self.as_ptr(), name, index, rect)
        }
    }
    /// Find a point at the label name.
    ///
    /// See [C++ `BMessage::FindPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a88507a5c41974ab5c13dd5da800a68d7).
    fn find_point_point<P: PointMethods>(&self, name: &str, point: Option<&P>) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let point = match point {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_FindPoint(self.as_ptr(), name, point)
        }
    }
    /// Find a point at the label name at an index.
    ///
    /// See [C++ `BMessage::FindPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a9a53e98aa4593529103051dbcd0186d4).
    fn find_point_int32_point<P: PointMethods>(&self, name: &str, index: i32, point: Option<&P>) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let point = match point {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_FindPoint1(self.as_ptr(), name, index, point)
        }
    }
    /// Find a size at the label name.
    ///
    /// See [C++ `BMessage::FindSize()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a463777b0384a8ee580709c62dfbfa2ca).
    fn find_size_size<S: SizeMethods>(&self, name: &str, size: Option<&S>) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let size = match size {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_FindSize(self.as_ptr(), name, size)
        }
    }
    /// Find a size at the label name at an index.
    ///
    /// See [C++ `BMessage::FindSize()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a769dcfc011f91c8372512f7396e6e59e).
    fn find_size_int32<S: SizeMethods>(&self, name: &str, index: i32, size: Option<&S>) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let size = match size {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_FindSize1(self.as_ptr(), name, index, size)
        }
    }
    /// Find a string at the label name.
    ///
    /// See [C++ `BMessage::FindString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac2a5b50960210666f57952b3a050530d).
    fn find_string_str(&self, name: &str, string: &str) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            ffi::BMessage_FindString(self.as_ptr(), name, string)
        }
    }
    /// Find a string at the label name at an index.
    ///
    /// See [C++ `BMessage::FindString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4370bfd828545100484adaf51a38df47).
    fn find_string_int32_str(&self, name: &str, index: i32, string: &str) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            ffi::BMessage_FindString1(self.as_ptr(), name, index, string)
        }
    }
    /// Find a string at the label name.
    ///
    /// See [C++ `BMessage::FindString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a2930978f62db0238b780d67a1e74e69b).
    fn find_string_string(&self, name: &str, string: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindString2(self.as_ptr(), name, string)
        }
    }
    /// Find a string at the label name at an index.
    ///
    /// See [C++ `BMessage::FindString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a35f163b19ec1862a2f117d1d1ac3ef46).
    fn find_string_int32_string(&self, name: &str, index: i32, string: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindString3(self.as_ptr(), name, index, string)
        }
    }
    /// Find all the strings at the label name.
    ///
    /// See [C++ `BMessage::FindStrings()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a1a482768af8c3361ae84f5458d674175).
    fn find_strings(&self, name: &str, list: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindStrings(self.as_ptr(), name, list)
        }
    }
    /// Find an integer at the label name.
    ///
    /// See [C++ `BMessage::FindInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a757685e8e85ac38f882e28bd281e9f9d).
    fn find_int8_int8(&self, name: &str, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindInt8(self.as_ptr(), name, value)
        }
    }
    /// Find an integer at the label name at an index.
    ///
    /// See [C++ `BMessage::FindInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aed840d1a802546534eaa95e5f1247a53).
    fn find_int8_int32_int8(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindInt81(self.as_ptr(), name, index, value)
        }
    }
    /// Find an integer at the label name.
    ///
    /// See [C++ `BMessage::FindUInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a37170512c52bac977fb81e7c9d44ef95).
    fn find_u_int8_uint8(&self, name: &str, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindUInt8(self.as_ptr(), name, value)
        }
    }
    /// Find an integer at the label name at an index.
    ///
    /// See [C++ `BMessage::FindUInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ab5ce6415eb074f94f1f8dc59f8c3985e).
    fn find_u_int8_int32(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindUInt81(self.as_ptr(), name, index, value)
        }
    }
    /// Find an integer at the label name.
    ///
    /// See [C++ `BMessage::FindInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4dd27c3a8fc6e2e0366d77254b2095e6).
    fn find_int16_int16(&self, name: &str, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindInt16(self.as_ptr(), name, value)
        }
    }
    /// Find an integer at the label name at an index.
    ///
    /// See [C++ `BMessage::FindInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#afedef75ac1e9c5a1d3f069b641975041).
    fn find_int16_int32_int16(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindInt161(self.as_ptr(), name, index, value)
        }
    }
    /// Find an integer at the label name.
    ///
    /// See [C++ `BMessage::FindUInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a5653c46ede97ff7667d0e823674b5953).
    fn find_u_int16_uint16(&self, name: &str, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindUInt16(self.as_ptr(), name, value)
        }
    }
    /// Find an integer at the label name at an index.
    ///
    /// See [C++ `BMessage::FindUInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a761a6bb39440a15176c5dc600d7b7041).
    fn find_u_int16_int32(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindUInt161(self.as_ptr(), name, index, value)
        }
    }
    // BLOCKED: fn FindInt32()
    /// Find an integer at the label name at an index.
    ///
    /// See [C++ `BMessage::FindInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#afce55a9ef851c34fa07d21bcf9e2f1f2).
    fn find_int32(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindInt321(self.as_ptr(), name, index, value)
        }
    }
    /// Find an integer at the label name.
    ///
    /// See [C++ `BMessage::FindUInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a486a2138fb0edc7bf17b1a9676d5ecde).
    fn find_u_int32_uint32(&self, name: &str, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindUInt32(self.as_ptr(), name, value)
        }
    }
    /// Find an integer at the label name at an index.
    ///
    /// See [C++ `BMessage::FindUInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a10e79ec598190c30d3d07c46c297eacf).
    fn find_u_int32_int32(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindUInt321(self.as_ptr(), name, index, value)
        }
    }
    /// Find an integer at the label name.
    ///
    /// See [C++ `BMessage::FindInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aedbef710d7c8e61cd4ff93efbb32889a).
    fn find_int64_int64(&self, name: &str, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindInt64(self.as_ptr(), name, value)
        }
    }
    /// Find an integer at the label name at an index.
    ///
    /// See [C++ `BMessage::FindInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ab3eef33595e98a3df56d18ade5f41a98).
    fn find_int64_int32_int64(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindInt641(self.as_ptr(), name, index, value)
        }
    }
    /// Find an integer at the label name.
    ///
    /// See [C++ `BMessage::FindUInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a20f4b14c6c45c4a24a3cbeea4347f79a).
    fn find_u_int64_uint64(&self, name: &str, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindUInt64(self.as_ptr(), name, value)
        }
    }
    /// Find an integer at the label name at an index.
    ///
    /// See [C++ `BMessage::FindUInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#abd21bedb93952f87cd12bf50c89d8612).
    fn find_u_int64_int32(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindUInt641(self.as_ptr(), name, index, value)
        }
    }
    /// Find a boolean at the label name.
    ///
    /// See [C++ `BMessage::FindBool()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a886d79a7c580d3563940333cfa7f3870).
    fn find_bool_bool(&self, name: &str, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindBool(self.as_ptr(), name, value)
        }
    }
    /// Find a boolean at the label name at an index.
    ///
    /// See [C++ `BMessage::FindBool()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a99405de493bf7e49ce033ef8cac8825b).
    fn find_bool_int32_bool(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindBool1(self.as_ptr(), name, index, value)
        }
    }
    /// Find a float at the label name.
    ///
    /// See [C++ `BMessage::FindFloat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac26f15d78b69e330f9cb933db8c0500b).
    fn find_float_float(&self, name: &str, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindFloat(self.as_ptr(), name, value)
        }
    }
    /// Find a float at the label name at an index.
    ///
    /// See [C++ `BMessage::FindFloat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#af5d62fe0dba7074556b877a52d1cecb2).
    fn find_float_int32_float(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindFloat1(self.as_ptr(), name, index, value)
        }
    }
    /// Find a double at the label name.
    ///
    /// See [C++ `BMessage::FindDouble()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a32672932ca24ee53b3642bf9d25f709d).
    fn find_double_double(&self, name: &str, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindDouble(self.as_ptr(), name, value)
        }
    }
    /// Find a double at the label name at an index.
    ///
    /// See [C++ `BMessage::FindDouble()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac332368de00ff51eb06097e134244e5a).
    fn find_double_int32_double(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindDouble1(self.as_ptr(), name, index, value)
        }
    }
    /// Find a color with the label name.
    ///
    /// See [C++ `BMessage::FindColor()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a8980e233e8afb3f302dc5218483a5456).
    fn find_color_rgb_color(&self, name: &str, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindColor(self.as_ptr(), name, value)
        }
    }
    /// Find a color at the label name at an index.
    ///
    /// See [C++ `BMessage::FindColor()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a6e05596bbf62217b061e54a18fa51a72).
    fn find_color_int32(&self, name: &str, index: i32, value: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindColor1(self.as_ptr(), name, index, value)
        }
    }
    /// Find a pointer at the label name.
    ///
    /// See [C++ `BMessage::FindPointer()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aee05fc5fe7053a1d85deb3169742e05a).
    fn find_pointer_void(&self, name: &str, pointer: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindPointer(self.as_ptr(), name, pointer)
        }
    }
    /// Find a pointer at the label name at an index.
    ///
    /// See [C++ `BMessage::FindPointer()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4de3e5f0cb4d25da0e00b13e33d28a66).
    fn find_pointer_int32(&self, name: &str, index: i32, pointer: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindPointer1(self.as_ptr(), name, index, pointer)
        }
    }
    /// Find a messenger at the label name.
    ///
    /// See [C++ `BMessage::FindMessenger()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a1b01edbdb7740ad1a06b2e81366fd47d).
    fn find_messenger_messenger(&self, name: &str, messenger: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindMessenger(self.as_ptr(), name, messenger)
        }
    }
    /// Find a messenger at the label name at an index.
    ///
    /// See [C++ `BMessage::FindMessenger()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a87a4b0591e87332366a99d09462d9be8).
    fn find_messenger_int32(&self, name: &str, index: i32, messenger: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindMessenger1(self.as_ptr(), name, index, messenger)
        }
    }
    /// Find a reference to a file at the label name.
    ///
    /// See [C++ `BMessage::FindRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a85035f98a61911e973af9a8821b5e254).
    fn find_ref_entry_ref(&self, name: &str, ref_: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindRef(self.as_ptr(), name, ref_)
        }
    }
    /// Find a reference to a file at the label name at an index.
    ///
    /// See [C++ `BMessage::FindRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4b1c2190d9f3742c86edc72c3c49eeca).
    fn find_ref_int32(&self, name: &str, index: i32, ref_: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindRef1(self.as_ptr(), name, index, ref_)
        }
    }
    /// Find a reference to a node at the label name.
    ///
    /// See [C++ `BMessage::FindNodeRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#acfb1819b93db7eadd0cabcd1cf1d45ec).
    fn find_node_ref_node_ref(&self, name: &str, ref_: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindNodeRef(self.as_ptr(), name, ref_)
        }
    }
    /// Find a reference to a node at the label name at an index.
    ///
    /// See [C++ `BMessage::FindNodeRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4a4b0e9e0a52151ec8d1df979925857a).
    fn find_node_ref_int32(&self, name: &str, index: i32, ref_: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindNodeRef1(self.as_ptr(), name, index, ref_)
        }
    }
    /// Find a message at the label name.
    ///
    /// See [C++ `BMessage::FindMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a8286c1590191f37641ac6916aaf6418b).
    fn find_message_message<M: MessageMethods>(&self, name: &str, message: Option<&M>) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_FindMessage(self.as_ptr(), name, message)
        }
    }
    /// Find a message at the label name at an index.
    ///
    /// See [C++ `BMessage::FindMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aecdfe05be151ea5a978225152486d44a).
    fn find_message_int32<M: MessageMethods>(&self, name: &str, index: i32, message: Option<&M>) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_FindMessage1(self.as_ptr(), name, index, message)
        }
    }
    /// Find a flattened object at the label name.
    ///
    /// See [C++ `BMessage::FindFlat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ae910af356e0732edaa09260e47e1bbd0).
    fn find_flat_flattenable(&self, name: &str, object: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindFlat(self.as_ptr(), name, object)
        }
    }
    /// Find a flattened object at the label name at an index.
    ///
    /// See [C++ `BMessage::FindFlat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a017c619934572443bf1ac5161dcdb2c2).
    fn find_flat_int32(&self, name: &str, index: i32, object: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindFlat1(self.as_ptr(), name, index, object)
        }
    }
    // NOT_SUPPORTED: fn FindData()
    // NOT_SUPPORTED: fn FindData1()
    /// Replace an alignment at the label name.
    ///
    /// See [C++ `BMessage::ReplaceAlignment()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a7613f4cdd1154b3e70771fdc9a4e3e04).
    fn replace_alignment_alignment(&self, name: &str, alignment: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceAlignment(self.as_ptr(), name, alignment)
        }
    }
    /// Replace an alignment at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceAlignment()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ae6f32ad46aa14db07c9d324656daf871).
    fn replace_alignment_int32(&self, name: &str, index: i32, alignment: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceAlignment1(self.as_ptr(), name, index, alignment)
        }
    }
    /// Replace a rectangle at the label name.
    ///
    /// See [C++ `BMessage::ReplaceRect()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aa24fb4e8fbc39011d1b7fb55c7f752c1).
    fn replace_rect_rect<R: RectMethods>(&self, name: &str, rect: &R) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let rect = rect.as_ptr();
            ffi::BMessage_ReplaceRect(self.as_ptr(), name, rect)
        }
    }
    /// Replace a rectangle at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceRect()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a2e38c3d43259b9157cc5c8647030dac0).
    fn replace_rect_int32<R: RectMethods>(&self, name: &str, index: i32, rect: &R) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let rect = rect.as_ptr();
            ffi::BMessage_ReplaceRect1(self.as_ptr(), name, index, rect)
        }
    }
    /// Replace a point at the label name.
    ///
    /// See [C++ `BMessage::ReplacePoint()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac1b948f0c5867720173ef96450201859).
    fn replace_point_point<P: PointMethods>(&self, name: &str, a_point: &P) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let a_point = a_point.as_ptr();
            ffi::BMessage_ReplacePoint(self.as_ptr(), name, a_point)
        }
    }
    /// Replace a point at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplacePoint()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4897600b32e200802cf7c5d397f7e8cf).
    fn replace_point_int32<P: PointMethods>(&self, name: &str, index: i32, a_point: &P) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let a_point = a_point.as_ptr();
            ffi::BMessage_ReplacePoint1(self.as_ptr(), name, index, a_point)
        }
    }
    /// Replace a size at the label name.
    ///
    /// See [C++ `BMessage::ReplaceSize()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a440d37083df7c8845d3337cc8c9d695a).
    fn replace_size_size<S: SizeMethods>(&self, name: &str, a_size: &S) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let a_size = a_size.as_ptr();
            ffi::BMessage_ReplaceSize(self.as_ptr(), name, a_size)
        }
    }
    /// Replace a size at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceSize()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a9ff6d6940725e22e2fab190bf42a1659).
    fn replace_size_int32<S: SizeMethods>(&self, name: &str, index: i32, a_size: &S) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let a_size = a_size.as_ptr();
            ffi::BMessage_ReplaceSize1(self.as_ptr(), name, index, a_size)
        }
    }
    /// Replace a string at the label name.
    ///
    /// See [C++ `BMessage::ReplaceString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a3a606679aa72f7530034994f9cf4ad32).
    fn replace_string_str(&self, name: &str, string: &str) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            ffi::BMessage_ReplaceString(self.as_ptr(), name, string)
        }
    }
    /// Replace a string at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aceeabb6ee480223b56d58e42a87ba7f5).
    fn replace_string_int32_str(&self, name: &str, index: i32, string: &str) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            ffi::BMessage_ReplaceString1(self.as_ptr(), name, index, string)
        }
    }
    /// Replace a string at the label name.
    ///
    /// See [C++ `BMessage::ReplaceString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a15b2ce4ff50f420a64422545079af452).
    fn replace_string_string(&self, name: &str, string: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceString2(self.as_ptr(), name, string)
        }
    }
    /// Replace a string at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a59a40f04a1fc5b0fe523a6e025d9860e).
    fn replace_string_int32_string(&self, name: &str, index: i32, string: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceString3(self.as_ptr(), name, index, string)
        }
    }
    /// Replace an integer at the label name.
    ///
    /// See [C++ `BMessage::ReplaceInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a55f909d237c7c1abfca5f918a8a5ffb9).
    fn replace_int8_int8(&self, name: &str, value: i8) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceInt8(self.as_ptr(), name, value)
        }
    }
    /// Replace an integer at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a8ae3786000645416e1911fe2b618c432).
    fn replace_int8_int32(&self, name: &str, index: i32, value: i8) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceInt81(self.as_ptr(), name, index, value)
        }
    }
    /// Replace an integer at the label name.
    ///
    /// See [C++ `BMessage::ReplaceUInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aee573704ea412f6b53e1904e22e65454).
    fn replace_u_int8_uint8(&self, name: &str, value: u8) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceUInt8(self.as_ptr(), name, value)
        }
    }
    /// Replace an integer at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceUInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aff3cffe95f04bf5576393f65bb378a1c).
    fn replace_u_int8_int32(&self, name: &str, index: i32, value: u8) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceUInt81(self.as_ptr(), name, index, value)
        }
    }
    /// Replace an integer at the label name.
    ///
    /// See [C++ `BMessage::ReplaceInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a525deaa699950007c8fdab78c82ada32).
    fn replace_int16_int16(&self, name: &str, value: i16) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceInt16(self.as_ptr(), name, value)
        }
    }
    /// Replace an integer at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a05b21498414818bc51e76451a26a143f).
    fn replace_int16_int32(&self, name: &str, index: i32, value: i16) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceInt161(self.as_ptr(), name, index, value)
        }
    }
    /// Replace an integer at the label name.
    ///
    /// See [C++ `BMessage::ReplaceUInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a0029cb535e97ed753ee9529c646d763b).
    fn replace_u_int16_uint16(&self, name: &str, value: u16) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceUInt16(self.as_ptr(), name, value)
        }
    }
    /// Replace an integer at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceUInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ab5d7606278375f280ab402b9f7e388be).
    fn replace_u_int16_int32(&self, name: &str, index: i32, value: u16) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceUInt161(self.as_ptr(), name, index, value)
        }
    }
    /// Replace an integer at the label name.
    ///
    /// See [C++ `BMessage::ReplaceInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ad7b41f02b8a298bcd6699e972b88c82e).
    fn replace_int32(&self, name: &str, value: i32) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceInt32(self.as_ptr(), name, value)
        }
    }
    /// Replace an integer at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ae1fe7f9855edb57e186cac617569cb79).
    fn replace_int32_int32(&self, name: &str, index: i32, value: i32) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceInt321(self.as_ptr(), name, index, value)
        }
    }
    /// Replace an integer at the label name.
    ///
    /// See [C++ `BMessage::ReplaceUInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#afdfe554348e5e672a1a358f3b944d63e).
    fn replace_u_int32_uint32(&self, name: &str, value: u32) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceUInt32(self.as_ptr(), name, value)
        }
    }
    /// Replace an integer at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceUInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a8bd5a2123e4097377f33d12aca43b615).
    fn replace_u_int32_int32(&self, name: &str, index: i32, value: u32) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceUInt321(self.as_ptr(), name, index, value)
        }
    }
    /// Replace an integer at the label name.
    ///
    /// See [C++ `BMessage::ReplaceInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a97220953786ff1cbb286e7e1113510ff).
    fn replace_int64_int64(&self, name: &str, value: i64) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceInt64(self.as_ptr(), name, value)
        }
    }
    /// Replace an integer at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac92d2b43a1dbc3e6591e0ddb03880e9b).
    fn replace_int64_int32(&self, name: &str, index: i32, value: i64) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceInt641(self.as_ptr(), name, index, value)
        }
    }
    /// Replace an integer at the label name.
    ///
    /// See [C++ `BMessage::ReplaceUInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ad906dc7124f3dab9f41ca94f678c29d4).
    fn replace_u_int64_uint64(&self, name: &str, value: u64) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceUInt64(self.as_ptr(), name, value)
        }
    }
    /// Replace an integer at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceUInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a66c1638d29c7474762e7ce3799785ffa).
    fn replace_u_int64_int32(&self, name: &str, index: i32, value: u64) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceUInt641(self.as_ptr(), name, index, value)
        }
    }
    /// Replace a boolean at the label name.
    ///
    /// See [C++ `BMessage::ReplaceBool()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a64357e8743595d49dd53975e3f534a3f).
    fn replace_bool_bool(&self, name: &str, a_boolean: bool) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceBool(self.as_ptr(), name, a_boolean)
        }
    }
    /// Replace a boolean at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceBool()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#af6bfc87ac8fe3f470b8d73a01dca388a).
    fn replace_bool_int32(&self, name: &str, index: i32, value: bool) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceBool1(self.as_ptr(), name, index, value)
        }
    }
    /// Replace a float at the label name.
    ///
    /// See [C++ `BMessage::ReplaceFloat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ab0c54a885d2cf1826dcb2fa53cfd0175).
    fn replace_float_float(&self, name: &str, value: c_float) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceFloat(self.as_ptr(), name, value)
        }
    }
    /// Replace a float at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceFloat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ad081e12c8b593d03c7fbee4ea61c8f34).
    fn replace_float_int32(&self, name: &str, index: i32, value: c_float) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceFloat1(self.as_ptr(), name, index, value)
        }
    }
    /// Replace a double at the label name.
    ///
    /// See [C++ `BMessage::ReplaceDouble()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a023f8e3aa932cd312acd5653ecebaca4).
    fn replace_double_double(&self, name: &str, value: c_double) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceDouble(self.as_ptr(), name, value)
        }
    }
    /// Replace a double at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceDouble()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aee50e26caa09886d0cdcbc753a91317f).
    fn replace_double_int32(&self, name: &str, index: i32, value: c_double) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceDouble1(self.as_ptr(), name, index, value)
        }
    }
    // NOT_SUPPORTED: fn ReplaceColor()
    // NOT_SUPPORTED: fn ReplaceColor1()
    /// Replace a pointer at the label name.
    ///
    /// See [C++ `BMessage::ReplacePointer()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aaa543367787ed528566a4f629e78f60b).
    fn replace_pointer_void(&self, name: &str, pointer: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplacePointer(self.as_ptr(), name, pointer)
        }
    }
    /// Replace a pointer at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplacePointer()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a77be90c4ecd92ca28cea4dbc46f8e8a2).
    fn replace_pointer_int32(&self, name: &str, index: i32, pointer: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplacePointer1(self.as_ptr(), name, index, pointer)
        }
    }
    // NOT_SUPPORTED: fn ReplaceMessenger()
    // NOT_SUPPORTED: fn ReplaceMessenger1()
    /// Replace a reference to a file at the label name.
    ///
    /// See [C++ `BMessage::ReplaceRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a44077e7618f42a388db207724075c0a6).
    fn replace_ref_entry_ref(&self, name: &str, ref_: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceRef(self.as_ptr(), name, ref_)
        }
    }
    /// Replace a reference to a file at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#acb93b69f6693499b1886173a4900b8c7).
    fn replace_ref_int32(&self, name: &str, index: i32, ref_: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceRef1(self.as_ptr(), name, index, ref_)
        }
    }
    /// Replace a reference to a node at the label name.
    ///
    /// See [C++ `BMessage::ReplaceNodeRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a57fb9d5c2b6aafc45932e5a10f1747a7).
    fn replace_node_ref_node_ref(&self, name: &str, ref_: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceNodeRef(self.as_ptr(), name, ref_)
        }
    }
    /// Replace a reference to a node at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceNodeRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4749eff1c0bfe6a258570536033dd698).
    fn replace_node_ref_int32(&self, name: &str, index: i32, ref_: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceNodeRef1(self.as_ptr(), name, index, ref_)
        }
    }
    /// Replace a message at the label name.
    ///
    /// See [C++ `BMessage::ReplaceMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a7c9f522c31ffee206e1df06aa8246f30).
    fn replace_message_message<M: MessageMethods>(&self, name: &str, message: Option<&M>) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_ReplaceMessage(self.as_ptr(), name, message)
        }
    }
    /// Replace a message at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#afc72518025d6c772e82acafad8669bc3).
    fn replace_message_int32<M: MessageMethods>(&self, name: &str, index: i32, message: Option<&M>) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BMessage_ReplaceMessage1(self.as_ptr(), name, index, message)
        }
    }
    /// Replace a flattened object at the label name.
    ///
    /// See [C++ `BMessage::ReplaceFlat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aaa8c8f67ea3314d93f39e1618b19621e).
    fn replace_flat_flattenable(&self, name: &str, object: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceFlat(self.as_ptr(), name, object)
        }
    }
    /// Replace a flattened object at the label name at a specified index.
    ///
    /// See [C++ `BMessage::ReplaceFlat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a32f79d59881b7bf9df0f1ab819fd9390).
    fn replace_flat_int32(&self, name: &str, index: i32, object: *mut c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_ReplaceFlat1(self.as_ptr(), name, index, object)
        }
    }
    // NOT_SUPPORTED: fn ReplaceData()
    // NOT_SUPPORTED: fn ReplaceData1()
    /// Experimental method to compare two messages.
    ///
    /// See [C++ `BMessage::HasSameData()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a6cfbd78a1801c5b199fd2b9c20f1edc0).
    fn has_same_data<M: MessageMethods>(&self, other: &M, ignore_field_order: bool, deep: bool) -> bool {
        unsafe {
            let other = other.as_ptr();
            ffi::BMessage_HasSameData(self.as_ptr(), other, ignore_field_order, deep)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasAlignment()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#adda48b9629ff85f0b6f5c14de1fb3089).
    fn has_alignment(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasAlignment(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasRect()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4470bc9810d3157abf300958299b3a8e).
    fn has_rect(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasRect(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac273669ad82b047d7df59f5d96789b36).
    fn has_point(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasPoint(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasSize()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ae196d8af4c0205d1a3f68b300fede30d).
    fn has_size(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasSize(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a21593fb0e2ba2d554f5e73aa69682509).
    fn has_string(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasString(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac4c6e7e0e7ccd94119ca7b1fb8a95ab0).
    fn has_int8(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasInt8(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasUInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a6aa77ba54a0aeb5e368b1ae0b2ebcc0e).
    fn has_u_int8(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasUInt8(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#afc15a565ef75a9acc96b5e7fc5f8a99f).
    fn has_int16(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasInt16(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasUInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a375487bace52f5629d38a90ab82e3501).
    fn has_u_int16(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasUInt16(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a3b8dc2a6990cca27044c581e960c4815).
    fn has_int32(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasInt32(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasUInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a61edd5753b91bef212a6d20618fc2115).
    fn has_u_int32(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasUInt32(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a5298eb19cf9387a592cf348804bbb543).
    fn has_int64(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasInt64(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasUInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4718b00776b779d72f48cfcb044d09c7).
    fn has_u_int64(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasUInt64(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasBool()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#af83659da0a00c4652bc410868e10d3e3).
    fn has_bool(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasBool(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasFloat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4ed1ab46d69555bed8656b88cfbedf80).
    fn has_float(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasFloat(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasDouble()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a10cc5f06813d7d11df126e351b43c19e).
    fn has_double(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasDouble(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasColor()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aa17b02befd19088e560ff218b69b4336).
    fn has_color(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasColor(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasPointer()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aa573c41ac9b8882a1f6c5ade73d6a090).
    fn has_pointer(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasPointer(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasMessenger()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aea46bfdce87d5de4344e3aa6f45aa7ef).
    fn has_messenger(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasMessenger(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a963c126685bfc3e5f8e1240a024fedfa).
    fn has_ref(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasRef(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasNodeRef()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a0284024009482818cbc57970ab51ceff).
    fn has_node_ref(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasNodeRef(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#af9f8c20181a4f95e6a4fbadddb2d39c7).
    fn has_message(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasMessage(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasFlat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a74043f885d59434afca9368419f5608a).
    fn has_flat_flattenable(&self, name: &str, object: *const c_void) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasFlat(self.as_ptr(), name, object)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::HasFlat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a8dfa876ddb1775a21f3cbfe92db599c2).
    fn has_flat_int32(&self, name: &str, n: i32, object: *const c_void) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_HasFlat1(self.as_ptr(), name, n, object)
        }
    }
    // NOT_SUPPORTED: fn HasData()
    /// Deprecated.
    ///
    /// See [C++ `BMessage::FindRect()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a9713057fc44d53b1771ff071341482ac).
    fn find_rect_int32(&self, name: &str, n: i32) -> Rect {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            Rect::from_ptr(ffi::BMessage_FindRect2(self.as_ptr(), name, n))
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::FindPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a813615a6d10f5ec02573a45f14c1241c).
    fn find_point_int32(&self, name: &str, n: i32) -> Point {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            Point::from_ptr(ffi::BMessage_FindPoint2(self.as_ptr(), name, n))
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::FindString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a36103667f4d215f1a06add5b381c2037).
    fn find_string_int32(&self, name: &str, n: i32) -> &CStr {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            CStr::from_ptr(ffi::BMessage_FindString4(self.as_ptr(), name, n))
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::FindInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a941d54ffe9d43bc4f06b0f51b05fc9a4).
    fn find_int8_int32(&self, name: &str, n: i32) -> i8 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindInt82(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::FindInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a7fb486ea27ffbf2711e2290bb65ab106).
    fn find_int16_int32(&self, name: &str, n: i32) -> i16 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindInt162(self.as_ptr(), name, n)
        }
    }
    // BLOCKED: fn FindInt322()
    /// Deprecated.
    ///
    /// See [C++ `BMessage::FindInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac098297494e5013484455204339a17d7).
    fn find_int64_int32(&self, name: &str, n: i32) -> i64 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindInt642(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::FindBool()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#acbb4500a7a1118cd1ee554fbc726d19f).
    fn find_bool_int32(&self, name: &str, n: i32) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindBool2(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::FindFloat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a6d647b74485e51f9831b7e18e455b55a).
    fn find_float_int32(&self, name: &str, n: i32) -> c_float {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindFloat2(self.as_ptr(), name, n)
        }
    }
    /// Deprecated.
    ///
    /// See [C++ `BMessage::FindDouble()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aa58f382932f2bfdcdb5f3a390c8312de).
    fn find_double_int32(&self, name: &str, n: i32) -> c_double {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_FindDouble2(self.as_ptr(), name, n)
        }
    }
    /// Return the boolean value from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetBool()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aff0242cacf8613b9e58dde7871c4c21c).
    fn get_bool_bool(&self, name: &str, default_value: bool) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetBool(self.as_ptr(), name, default_value)
        }
    }
    /// Return the boolean value from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetBool()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a7de758471bcc9e642064b350c32459c7).
    fn get_bool_int32(&self, name: &str, index: i32, default_value: bool) -> bool {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetBool1(self.as_ptr(), name, index, default_value)
        }
    }
    /// Return the int8 value from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a822fcf4b8182566bfaebb8dd04cd2b97).
    fn get_int8_int8(&self, name: &str, default_value: i8) -> i8 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetInt8(self.as_ptr(), name, default_value)
        }
    }
    /// Return the int8 value from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aad10c23206e76e694eb6d8ee90ae7710).
    fn get_int8_int32(&self, name: &str, index: i32, default_value: i8) -> i8 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetInt81(self.as_ptr(), name, index, default_value)
        }
    }
    /// Return the uint8 value from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetUInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a146182cecba5ad8a802d0c60627b695a).
    fn get_u_int8_uint8(&self, name: &str, default_value: u8) -> u8 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetUInt8(self.as_ptr(), name, default_value)
        }
    }
    /// Return the uint8 message from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetUInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#af8f4251e140340b254a26f62fe3df880).
    fn get_u_int8_int32(&self, name: &str, index: i32, default_value: u8) -> u8 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetUInt81(self.as_ptr(), name, index, default_value)
        }
    }
    /// Return the int16 value from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aafc68a78c8b550fec01c4129683520b2).
    fn get_int16_int16(&self, name: &str, default_value: i16) -> i16 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetInt16(self.as_ptr(), name, default_value)
        }
    }
    /// Return the int16 value from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#afd2db75cacae2f4b44f93c5ea3fb0e0f).
    fn get_int16_int32(&self, name: &str, index: i32, default_value: i16) -> i16 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetInt161(self.as_ptr(), name, index, default_value)
        }
    }
    /// Return the uint16 value from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetUInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a20d02e3b2687cf4d884d5e308c5a55cb).
    fn get_u_int16_uint16(&self, name: &str, default_value: u16) -> u16 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetUInt16(self.as_ptr(), name, default_value)
        }
    }
    /// Return the uint16 value from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetUInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a9a3612c486e40bc08800f72e48d2b214).
    fn get_u_int16_int32(&self, name: &str, index: i32, default_value: u16) -> u16 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetUInt161(self.as_ptr(), name, index, default_value)
        }
    }
    /// Return the int32 value from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a526ebda92481d1c0da2786b93253efd1).
    fn get_int32(&self, name: &str, default_value: i32) -> i32 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetInt32(self.as_ptr(), name, default_value)
        }
    }
    /// Return the int32 value from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a1da13cf9b5913f1fcd6c8835f440b5f6).
    fn get_int32_int32(&self, name: &str, index: i32, default_value: i32) -> i32 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetInt321(self.as_ptr(), name, index, default_value)
        }
    }
    /// Return the uint32 value from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetUInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#aa90efd88a6a15fef748c62c5c46f2f65).
    fn get_u_int32_uint32(&self, name: &str, default_value: u32) -> u32 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetUInt32(self.as_ptr(), name, default_value)
        }
    }
    /// Return the uint32 value from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetUInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a771c61eb89e5952e0fc890bb586365a1).
    fn get_u_int32_int32(&self, name: &str, index: i32, default_value: u32) -> u32 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetUInt321(self.as_ptr(), name, index, default_value)
        }
    }
    /// Return the int64 value from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a2bb8e387b504dfe4ff47b7860ead1be9).
    fn get_int64_int64(&self, name: &str, default_value: i64) -> i64 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetInt64(self.as_ptr(), name, default_value)
        }
    }
    /// Return the int64 value from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a00e136d7bdfa978b2376ae993b200e64).
    fn get_int64_int32(&self, name: &str, index: i32, default_value: i64) -> i64 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetInt641(self.as_ptr(), name, index, default_value)
        }
    }
    /// Return the uint64 value from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetUInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a4d8f178f02212949cdd497c392ff6f13).
    fn get_u_int64_uint64(&self, name: &str, default_value: u64) -> u64 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetUInt64(self.as_ptr(), name, default_value)
        }
    }
    /// Return the uint64 value from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetUInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a9dce5b9af2a32cc3b1f185b3521ee61a).
    fn get_u_int64_int32(&self, name: &str, index: i32, default_value: u64) -> u64 {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetUInt641(self.as_ptr(), name, index, default_value)
        }
    }
    /// Return the float value from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetFloat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a2f5426192ec8ca235dd9f936510c4e93).
    fn get_float_float(&self, name: &str, default_value: c_float) -> c_float {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetFloat(self.as_ptr(), name, default_value)
        }
    }
    /// Return the float value from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetFloat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ad60572e86e9e605b27afcc50efcb1d24).
    fn get_float_int32(&self, name: &str, index: i32, default_value: c_float) -> c_float {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetFloat1(self.as_ptr(), name, index, default_value)
        }
    }
    /// Return the double value from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetDouble()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a018eed03b3b69e4248518e2a7c4a1c03).
    fn get_double_double(&self, name: &str, default_value: c_double) -> c_double {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetDouble(self.as_ptr(), name, default_value)
        }
    }
    /// Return the double value from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetDouble()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#af0486856061e17452614c6ece2cfab64).
    fn get_double_int32(&self, name: &str, index: i32, default_value: c_double) -> c_double {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetDouble1(self.as_ptr(), name, index, default_value)
        }
    }
    // NOT_SUPPORTED: fn GetColor()
    // NOT_SUPPORTED: fn GetColor1()
    /// Return the pointer type from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetPointer()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a817c8b7db104a1525ad15616bd06e284).
    fn get_pointer_int32(&self, name: &str, index: i32, default_value: *const c_void) -> *const c_void {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetPointer(self.as_ptr(), name, index, default_value)
        }
    }
    /// Return the pointer type from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetPointer()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a1141c4e98a497041cd46e2fce31ff880).
    fn get_pointer_void(&self, name: &str, default_value: *const c_void) -> *const c_void {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_GetPointer1(self.as_ptr(), name, default_value)
        }
    }
    /// Return the string from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#acf98ba0c69ac6dd433153b6e3e06c64f).
    fn get_string_str(&self, name: &str, default_value: &str) -> &CStr {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let default_value = CString::from_vec_unchecked(default_value.into());
            let default_value = default_value.as_ptr();
            CStr::from_ptr(ffi::BMessage_GetString(self.as_ptr(), name, default_value))
        }
    }
    /// Return the string from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a6e65f922253e4ab511507e015ce023f8).
    fn get_string_int32(&self, name: &str, index: i32, default_value: &str) -> &CStr {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let default_value = CString::from_vec_unchecked(default_value.into());
            let default_value = default_value.as_ptr();
            CStr::from_ptr(ffi::BMessage_GetString1(self.as_ptr(), name, index, default_value))
        }
    }
    // NOT_SUPPORTED: fn GetAlignment()
    // NOT_SUPPORTED: fn GetAlignment1()
    /// Return the BRect object from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetRect()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac830b029ae38dce1f6a2f823f289f222).
    fn get_rect_int32<R: RectMethods>(&self, name: &str, index: i32, default_value: &R) -> Rect {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let default_value = default_value.as_ptr();
            Rect::from_ptr(ffi::BMessage_GetRect(self.as_ptr(), name, index, default_value))
        }
    }
    /// Return the BRect object from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetRect()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ab813849c1d23d484fff039204198e6f6).
    fn get_rect_rect<R: RectMethods>(&self, name: &str, default_value: &R) -> Rect {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let default_value = default_value.as_ptr();
            Rect::from_ptr(ffi::BMessage_GetRect1(self.as_ptr(), name, default_value))
        }
    }
    /// Return the BPoint object from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a46167045bba07334db6acaa3c37e0498).
    fn get_point_int32<P: PointMethods>(&self, name: &str, index: i32, default_value: &P) -> Point {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let default_value = default_value.as_ptr();
            Point::from_ptr(ffi::BMessage_GetPoint(self.as_ptr(), name, index, default_value))
        }
    }
    /// Return the BPoint object from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ad68b28e3f6a38b8d832f2277c8be9372).
    fn get_point_point<P: PointMethods>(&self, name: &str, default_value: &P) -> Point {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let default_value = default_value.as_ptr();
            Point::from_ptr(ffi::BMessage_GetPoint1(self.as_ptr(), name, default_value))
        }
    }
    /// Return the BSize object from message with name and index, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetSize()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ae88c4239718d5a5ad14162e03889db9f).
    fn get_size_int32<S: SizeMethods>(&self, name: &str, index: i32, default_value: &S) -> Size {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let default_value = default_value.as_ptr();
            Size::from_ptr(ffi::BMessage_GetSize(self.as_ptr(), name, index, default_value))
        }
    }
    /// Return the BSize object from message with name, or defaultValue if not found.
    ///
    /// See [C++ `BMessage::GetSize()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a8e61a348444f29b08027d99eb5042ac8).
    fn get_size_size<S: SizeMethods>(&self, name: &str, default_value: &S) -> Size {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let default_value = default_value.as_ptr();
            Size::from_ptr(ffi::BMessage_GetSize1(self.as_ptr(), name, default_value))
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetBool()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a60e079b535d604425d79d21f6c2ff331).
    fn set_bool(&self, name: &str, value: bool) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetBool(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a43dd2dd4ef948377bd1c208e8fb1397a).
    fn set_int8(&self, name: &str, value: i8) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetInt8(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetUInt8()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a2b2c442855633f18d2575f081c426cfb).
    fn set_u_int8(&self, name: &str, value: u8) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetUInt8(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a8572ce07a58fb1d08e21bf10aefd3f61).
    fn set_int16(&self, name: &str, value: i16) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetInt16(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetUInt16()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#adc67eb8a3f7f7b25d0581bf839d8f994).
    fn set_u_int16(&self, name: &str, value: u16) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetUInt16(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ad3912ab5d568e4035ffb12aa74feda3f).
    fn set_int32(&self, name: &str, value: i32) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetInt32(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetUInt32()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a036ff5c9b6a87427fc0682f0afe0fc2d).
    fn set_u_int32(&self, name: &str, value: u32) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetUInt32(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a488e4670f9d00491385a5a85f42c67d9).
    fn set_int64(&self, name: &str, value: i64) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetInt64(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetUInt64()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a39ea15b60e3551cf3f81af7ff2e8a6ee).
    fn set_u_int64(&self, name: &str, value: u64) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetUInt64(self.as_ptr(), name, value)
        }
    }
    // NOT_SUPPORTED: fn SetColor()
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetPointer()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a7d2a78283d63010daf55315eae6983da).
    fn set_pointer(&self, name: &str, value: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetPointer(self.as_ptr(), name, value)
        }
    }
    /// Set the string with at the label name to string.
    ///
    /// See [C++ `BMessage::SetString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a7bbb5edf6eaec7d8ead471063ba05720).
    fn set_string_str(&self, name: &str, string: &str) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            ffi::BMessage_SetString(self.as_ptr(), name, string)
        }
    }
    /// Set the string with at the label name to string.
    ///
    /// See [C++ `BMessage::SetString()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a3581bea5b303469e21c2dbc0c77d0682).
    fn set_string_string(&self, name: &str, string: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetString1(self.as_ptr(), name, string)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetFloat()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a70bada6ab725e3c49d6be3f7b9cd32b0).
    fn set_float(&self, name: &str, value: c_float) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetFloat(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetDouble()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#ac63da648d43ba6f4ac4b6b78ce20fc14).
    fn set_double(&self, name: &str, value: c_double) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetDouble(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetAlignment()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a68d1c8c3faf0425085dd4c7368115445).
    fn set_alignment(&self, name: &str, value: *const c_void) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            ffi::BMessage_SetAlignment(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetPoint()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a5c573737d146370197cbbac1d0b85a12).
    fn set_point<P: PointMethods>(&self, name: &str, value: &P) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let value = value.as_ptr();
            ffi::BMessage_SetPoint(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetRect()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a1f6691ff69fe9d9027179d8f2a3c8165).
    fn set_rect<R: RectMethods>(&self, name: &str, value: &R) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let value = value.as_ptr();
            ffi::BMessage_SetRect(self.as_ptr(), name, value)
        }
    }
    /// Set the data with at the label name to value.
    ///
    /// See [C++ `BMessage::SetSize()`'s documentation](https://www.haiku-os.org/docs/api/classBMessage.html#a6981d8f08fd09740c20d1ba96cd5b85f).
    fn set_size<S: SizeMethods>(&self, name: &str, value: &S) -> status_t {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            let value = value.as_ptr();
            ffi::BMessage_SetSize(self.as_ptr(), name, value)
        }
    }
    // NOT_SUPPORTED: fn SetData()
    // DTOR: fn ~BMessage()
}

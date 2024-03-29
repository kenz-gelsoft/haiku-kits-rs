use super::*;

extern "C" {

    // BMessage
    pub fn BMessage_delete(self_: *mut c_void);
    pub fn BMessage_what(self_: *mut c_void) -> u32;
    pub fn BMessage_set_what(self_: *mut c_void, what: u32);
    // BLOCKED: pub fn BMessage_operator=(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn BMessage_operator new(self_: *mut c_void, size: usize) -> *mut c_void;
    // BLOCKED: pub fn BMessage_operator new1(self_: *mut c_void, None: usize, pointer: *mut c_void) -> *mut c_void;
    // BLOCKED: pub fn BMessage_operator new2(self_: *mut c_void, None: usize, no_throw: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn BMessage_operator delete(self_: *mut c_void, pointer: *mut c_void, size: usize);
    // NOT_SUPPORTED: pub fn BMessage_GetInfo(self_: *const c_void, type_requested: type_code, index: i32, name_found: *mut c_void, type_found: *mut c_void, count_found: *mut c_void) -> status_t;
    pub fn BMessage_GetInfo1(
        self_: *const c_void,
        name: *const c_char,
        type_found: *mut c_void,
        count_found: *mut c_void,
    ) -> status_t;
    pub fn BMessage_GetInfo2(
        self_: *const c_void,
        name: *const c_char,
        type_found: *mut c_void,
        fixed_size: *mut c_void,
    ) -> status_t;
    pub fn BMessage_GetInfo3(
        self_: *const c_void,
        name: *const c_char,
        type_found: *mut c_void,
        count_found: *mut c_void,
        fixed_size: *mut c_void,
    ) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_CountNames(self_: *const c_void, type_: type_code) -> i32;
    pub fn BMessage_IsEmpty(self_: *const c_void) -> bool;
    pub fn BMessage_IsSystem(self_: *const c_void) -> bool;
    pub fn BMessage_IsReply(self_: *const c_void) -> bool;
    pub fn BMessage_PrintToStream(self_: *const c_void);
    pub fn BMessage_Rename(
        self_: *mut c_void,
        old_entry: *const c_char,
        new_entry: *const c_char,
    ) -> status_t;
    pub fn BMessage_WasDelivered(self_: *const c_void) -> bool;
    pub fn BMessage_IsSourceWaiting(self_: *const c_void) -> bool;
    pub fn BMessage_IsSourceRemote(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn BMessage_ReturnAddress(self_: *const c_void) -> BMessenger;
    pub fn BMessage_Previous(self_: *const c_void) -> *mut c_void;
    pub fn BMessage_WasDropped(self_: *const c_void) -> bool;
    pub fn BMessage_DropPoint(self_: *const c_void, offset: *mut c_void) -> *mut c_void;
    pub fn BMessage_SendReply(self_: *mut c_void, command: u32, reply_to: *mut c_void) -> status_t;
    pub fn BMessage_SendReply1(
        self_: *mut c_void,
        reply: *mut c_void,
        reply_to: *mut c_void,
        timeout: bigtime_t,
    ) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_SendReply2(self_: *mut c_void, reply: *mut c_void, reply_to: BMessenger, timeout: bigtime_t) -> status_t;
    pub fn BMessage_SendReply3(
        self_: *mut c_void,
        command: u32,
        reply_to_reply: *mut c_void,
    ) -> status_t;
    pub fn BMessage_SendReply4(
        self_: *mut c_void,
        reply: *mut c_void,
        reply_to_reply: *mut c_void,
        send_timeout: bigtime_t,
        reply_timeout: bigtime_t,
    ) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_FlattenedSize(self_: *const c_void) -> ssize_t;
    // NOT_SUPPORTED: pub fn BMessage_Flatten(self_: *const c_void, buffer: *mut c_void, size: ssize_t) -> status_t;
    pub fn BMessage_Flatten1(
        self_: *const c_void,
        stream: *mut c_void,
        size: *mut c_void,
    ) -> status_t;
    pub fn BMessage_Unflatten(self_: *mut c_void, flat_buffer: *const c_char) -> status_t;
    pub fn BMessage_Unflatten1(self_: *mut c_void, stream: *mut c_void) -> status_t;
    pub fn BMessage_AddSpecifier(self_: *mut c_void, property: *const c_char) -> status_t;
    pub fn BMessage_AddSpecifier1(
        self_: *mut c_void,
        property: *const c_char,
        index: i32,
    ) -> status_t;
    pub fn BMessage_AddSpecifier2(
        self_: *mut c_void,
        property: *const c_char,
        index: i32,
        range: i32,
    ) -> status_t;
    pub fn BMessage_AddSpecifier3(
        self_: *mut c_void,
        property: *const c_char,
        name: *const c_char,
    ) -> status_t;
    pub fn BMessage_AddSpecifier4(self_: *mut c_void, specifier: *const c_void) -> status_t;
    pub fn BMessage_SetCurrentSpecifier(self_: *mut c_void, index: i32) -> status_t;
    pub fn BMessage_GetCurrentSpecifier(
        self_: *const c_void,
        index: *mut c_void,
        specifier: *mut c_void,
        what: *mut c_void,
        property: *const c_char,
    ) -> status_t;
    pub fn BMessage_HasSpecifiers(self_: *const c_void) -> bool;
    pub fn BMessage_PopSpecifier(self_: *mut c_void) -> status_t;
    pub fn BMessage_AddAlignment(
        self_: *mut c_void,
        name: *const c_char,
        alignment: *const c_void,
    ) -> status_t;
    pub fn BMessage_AddRect(self_: *mut c_void, name: *const c_char, rect: *mut c_void)
        -> status_t;
    pub fn BMessage_AddPoint(
        self_: *mut c_void,
        name: *const c_char,
        point: *mut c_void,
    ) -> status_t;
    pub fn BMessage_AddSize(self_: *mut c_void, name: *const c_char, size: *mut c_void)
        -> status_t;
    pub fn BMessage_AddString(
        self_: *mut c_void,
        name: *const c_char,
        string: *const c_char,
    ) -> status_t;
    pub fn BMessage_AddString1(
        self_: *mut c_void,
        name: *const c_char,
        string: *const c_void,
    ) -> status_t;
    pub fn BMessage_AddStrings(
        self_: *mut c_void,
        name: *const c_char,
        list: *const c_void,
    ) -> status_t;
    pub fn BMessage_AddInt8(self_: *mut c_void, name: *const c_char, value: i8) -> status_t;
    pub fn BMessage_AddUInt8(self_: *mut c_void, name: *const c_char, value: u8) -> status_t;
    pub fn BMessage_AddInt16(self_: *mut c_void, name: *const c_char, value: i16) -> status_t;
    pub fn BMessage_AddUInt16(self_: *mut c_void, name: *const c_char, value: u16) -> status_t;
    pub fn BMessage_AddInt32(self_: *mut c_void, name: *const c_char, value: i32) -> status_t;
    pub fn BMessage_AddUInt32(self_: *mut c_void, name: *const c_char, value: u32) -> status_t;
    pub fn BMessage_AddInt64(self_: *mut c_void, name: *const c_char, value: i64) -> status_t;
    pub fn BMessage_AddUInt64(self_: *mut c_void, name: *const c_char, value: u64) -> status_t;
    pub fn BMessage_AddBool(self_: *mut c_void, name: *const c_char, value: bool) -> status_t;
    pub fn BMessage_AddFloat(self_: *mut c_void, name: *const c_char, value: c_float) -> status_t;
    pub fn BMessage_AddDouble(self_: *mut c_void, name: *const c_char, value: c_double)
        -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_AddColor(self_: *mut c_void, name: *const c_char, value: rgb_color) -> status_t;
    pub fn BMessage_AddPointer(
        self_: *mut c_void,
        name: *const c_char,
        pointer: *const c_void,
    ) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_AddMessenger(self_: *mut c_void, name: *const c_char, messenger: BMessenger) -> status_t;
    pub fn BMessage_AddRef(
        self_: *mut c_void,
        name: *const c_char,
        ref_: *const c_void,
    ) -> status_t;
    pub fn BMessage_AddNodeRef(
        self_: *mut c_void,
        name: *const c_char,
        ref_: *const c_void,
    ) -> status_t;
    pub fn BMessage_AddMessage(
        self_: *mut c_void,
        name: *const c_char,
        message: *const c_void,
    ) -> status_t;
    // BLOCKED: pub fn BMessage_AddFlat(self_: *mut c_void, name: *const c_char, object: *mut c_void, count: i32) -> status_t;
    pub fn BMessage_AddFlat1(
        self_: *mut c_void,
        name: *const c_char,
        object: *const c_void,
        count: i32,
    ) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_AddData(self_: *mut c_void, name: *const c_char, type_: type_code, data: *const c_void, num_bytes: ssize_t, is_fixed_size: bool, count: i32) -> status_t;
    pub fn BMessage_Append(self_: *mut c_void, message: *const c_void) -> status_t;
    pub fn BMessage_RemoveData(self_: *mut c_void, name: *const c_char, index: i32) -> status_t;
    pub fn BMessage_RemoveName(self_: *mut c_void, name: *const c_char) -> status_t;
    pub fn BMessage_MakeEmpty(self_: *mut c_void) -> status_t;
    pub fn BMessage_FindAlignment(
        self_: *const c_void,
        name: *const c_char,
        alignment: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindAlignment1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        alignment: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindRect(
        self_: *const c_void,
        name: *const c_char,
        rect: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindRect1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        rect: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindPoint(
        self_: *const c_void,
        name: *const c_char,
        point: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindPoint1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        point: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindSize(
        self_: *const c_void,
        name: *const c_char,
        size: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindSize1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        size: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindString(
        self_: *const c_void,
        name: *const c_char,
        string: *const c_char,
    ) -> status_t;
    pub fn BMessage_FindString1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        string: *const c_char,
    ) -> status_t;
    pub fn BMessage_FindString2(
        self_: *const c_void,
        name: *const c_char,
        string: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindString3(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        string: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindStrings(
        self_: *const c_void,
        name: *const c_char,
        list: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindInt8(
        self_: *const c_void,
        name: *const c_char,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindInt81(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindUInt8(
        self_: *const c_void,
        name: *const c_char,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindUInt81(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindInt16(
        self_: *const c_void,
        name: *const c_char,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindInt161(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindUInt16(
        self_: *const c_void,
        name: *const c_char,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindUInt161(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    // BLOCKED: pub fn BMessage_FindInt32(self_: *const c_void, name: *const c_char, value: *mut c_void) -> status_t;
    pub fn BMessage_FindInt321(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindUInt32(
        self_: *const c_void,
        name: *const c_char,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindUInt321(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindInt64(
        self_: *const c_void,
        name: *const c_char,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindInt641(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindUInt64(
        self_: *const c_void,
        name: *const c_char,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindUInt641(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindBool(
        self_: *const c_void,
        name: *const c_char,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindBool1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindFloat(
        self_: *const c_void,
        name: *const c_char,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindFloat1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindDouble(
        self_: *const c_void,
        name: *const c_char,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindDouble1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindColor(
        self_: *const c_void,
        name: *const c_char,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindColor1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        value: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindPointer(
        self_: *const c_void,
        name: *const c_char,
        pointer: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindPointer1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        pointer: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindMessenger(
        self_: *const c_void,
        name: *const c_char,
        messenger: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindMessenger1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        messenger: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindRef(
        self_: *const c_void,
        name: *const c_char,
        ref_: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindRef1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        ref_: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindNodeRef(
        self_: *const c_void,
        name: *const c_char,
        ref_: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindNodeRef1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        ref_: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindMessage(
        self_: *const c_void,
        name: *const c_char,
        message: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindMessage1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        message: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindFlat(
        self_: *const c_void,
        name: *const c_char,
        object: *mut c_void,
    ) -> status_t;
    pub fn BMessage_FindFlat1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        object: *mut c_void,
    ) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_FindData(self_: *const c_void, name: *const c_char, type_: type_code, data: *const c_void, num_bytes: *mut c_void) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_FindData1(self_: *const c_void, name: *const c_char, type_: type_code, index: i32, data: *const c_void, num_bytes: *mut c_void) -> status_t;
    pub fn BMessage_ReplaceAlignment(
        self_: *mut c_void,
        name: *const c_char,
        alignment: *const c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceAlignment1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        alignment: *const c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceRect(
        self_: *mut c_void,
        name: *const c_char,
        rect: *mut c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceRect1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        rect: *mut c_void,
    ) -> status_t;
    pub fn BMessage_ReplacePoint(
        self_: *mut c_void,
        name: *const c_char,
        a_point: *mut c_void,
    ) -> status_t;
    pub fn BMessage_ReplacePoint1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        a_point: *mut c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceSize(
        self_: *mut c_void,
        name: *const c_char,
        a_size: *mut c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceSize1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        a_size: *mut c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceString(
        self_: *mut c_void,
        name: *const c_char,
        string: *const c_char,
    ) -> status_t;
    pub fn BMessage_ReplaceString1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        string: *const c_char,
    ) -> status_t;
    pub fn BMessage_ReplaceString2(
        self_: *mut c_void,
        name: *const c_char,
        string: *const c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceString3(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        string: *const c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceInt8(self_: *mut c_void, name: *const c_char, value: i8) -> status_t;
    pub fn BMessage_ReplaceInt81(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        value: i8,
    ) -> status_t;
    pub fn BMessage_ReplaceUInt8(self_: *mut c_void, name: *const c_char, value: u8) -> status_t;
    pub fn BMessage_ReplaceUInt81(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        value: u8,
    ) -> status_t;
    pub fn BMessage_ReplaceInt16(self_: *mut c_void, name: *const c_char, value: i16) -> status_t;
    pub fn BMessage_ReplaceInt161(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        value: i16,
    ) -> status_t;
    pub fn BMessage_ReplaceUInt16(self_: *mut c_void, name: *const c_char, value: u16) -> status_t;
    pub fn BMessage_ReplaceUInt161(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        value: u16,
    ) -> status_t;
    pub fn BMessage_ReplaceInt32(self_: *mut c_void, name: *const c_char, value: i32) -> status_t;
    pub fn BMessage_ReplaceInt321(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        value: i32,
    ) -> status_t;
    pub fn BMessage_ReplaceUInt32(self_: *mut c_void, name: *const c_char, value: u32) -> status_t;
    pub fn BMessage_ReplaceUInt321(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        value: u32,
    ) -> status_t;
    pub fn BMessage_ReplaceInt64(self_: *mut c_void, name: *const c_char, value: i64) -> status_t;
    pub fn BMessage_ReplaceInt641(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        value: i64,
    ) -> status_t;
    pub fn BMessage_ReplaceUInt64(self_: *mut c_void, name: *const c_char, value: u64) -> status_t;
    pub fn BMessage_ReplaceUInt641(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        value: u64,
    ) -> status_t;
    pub fn BMessage_ReplaceBool(
        self_: *mut c_void,
        name: *const c_char,
        a_boolean: bool,
    ) -> status_t;
    pub fn BMessage_ReplaceBool1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        value: bool,
    ) -> status_t;
    pub fn BMessage_ReplaceFloat(
        self_: *mut c_void,
        name: *const c_char,
        value: c_float,
    ) -> status_t;
    pub fn BMessage_ReplaceFloat1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        value: c_float,
    ) -> status_t;
    pub fn BMessage_ReplaceDouble(
        self_: *mut c_void,
        name: *const c_char,
        value: c_double,
    ) -> status_t;
    pub fn BMessage_ReplaceDouble1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        value: c_double,
    ) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_ReplaceColor(self_: *mut c_void, name: *const c_char, value: rgb_color) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_ReplaceColor1(self_: *mut c_void, name: *const c_char, index: i32, value: rgb_color) -> status_t;
    pub fn BMessage_ReplacePointer(
        self_: *mut c_void,
        name: *const c_char,
        pointer: *const c_void,
    ) -> status_t;
    pub fn BMessage_ReplacePointer1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        pointer: *const c_void,
    ) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_ReplaceMessenger(self_: *mut c_void, name: *const c_char, messenger: BMessenger) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_ReplaceMessenger1(self_: *mut c_void, name: *const c_char, index: i32, messenger: BMessenger) -> status_t;
    pub fn BMessage_ReplaceRef(
        self_: *mut c_void,
        name: *const c_char,
        ref_: *const c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceRef1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        ref_: *const c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceNodeRef(
        self_: *mut c_void,
        name: *const c_char,
        ref_: *const c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceNodeRef1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        ref_: *const c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceMessage(
        self_: *mut c_void,
        name: *const c_char,
        message: *const c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceMessage1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        message: *const c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceFlat(
        self_: *mut c_void,
        name: *const c_char,
        object: *mut c_void,
    ) -> status_t;
    pub fn BMessage_ReplaceFlat1(
        self_: *mut c_void,
        name: *const c_char,
        index: i32,
        object: *mut c_void,
    ) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_ReplaceData(self_: *mut c_void, name: *const c_char, type_: type_code, data: *const c_void, num_bytes: ssize_t) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_ReplaceData1(self_: *mut c_void, name: *const c_char, type_: type_code, index: i32, data: *const c_void, num_bytes: ssize_t) -> status_t;
    pub fn BMessage_HasSameData(
        self_: *const c_void,
        other: *const c_void,
        ignore_field_order: bool,
        deep: bool,
    ) -> bool;
    pub fn BMessage_HasAlignment(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasRect(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasPoint(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasSize(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasString(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasInt8(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasUInt8(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasInt16(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasUInt16(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasInt32(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasUInt32(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasInt64(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasUInt64(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasBool(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasFloat(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasDouble(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasColor(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasPointer(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasMessenger(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasRef(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasNodeRef(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasMessage(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_HasFlat(
        self_: *const c_void,
        name: *const c_char,
        object: *const c_void,
    ) -> bool;
    pub fn BMessage_HasFlat1(
        self_: *const c_void,
        name: *const c_char,
        n: i32,
        object: *const c_void,
    ) -> bool;
    // NOT_SUPPORTED: pub fn BMessage_HasData(self_: *const c_void, name: *const c_char, None: type_code, n: i32) -> bool;
    pub fn BMessage_FindRect2(self_: *const c_void, name: *const c_char, n: i32) -> *mut c_void;
    pub fn BMessage_FindPoint2(self_: *const c_void, name: *const c_char, n: i32) -> *mut c_void;
    pub fn BMessage_FindString4(self_: *const c_void, name: *const c_char, n: i32)
        -> *const c_char;
    pub fn BMessage_FindInt82(self_: *const c_void, name: *const c_char, n: i32) -> i8;
    pub fn BMessage_FindInt162(self_: *const c_void, name: *const c_char, n: i32) -> i16;
    // BLOCKED: pub fn BMessage_FindInt322(self_: *const c_void, name: *const c_char, n: i32) -> i32;
    pub fn BMessage_FindInt642(self_: *const c_void, name: *const c_char, n: i32) -> i64;
    pub fn BMessage_FindBool2(self_: *const c_void, name: *const c_char, n: i32) -> bool;
    pub fn BMessage_FindFloat2(self_: *const c_void, name: *const c_char, n: i32) -> c_float;
    pub fn BMessage_FindDouble2(self_: *const c_void, name: *const c_char, n: i32) -> c_double;
    pub fn BMessage_GetBool(self_: *const c_void, name: *const c_char, default_value: bool)
        -> bool;
    pub fn BMessage_GetBool1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: bool,
    ) -> bool;
    pub fn BMessage_GetInt8(self_: *const c_void, name: *const c_char, default_value: i8) -> i8;
    pub fn BMessage_GetInt81(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: i8,
    ) -> i8;
    pub fn BMessage_GetUInt8(self_: *const c_void, name: *const c_char, default_value: u8) -> u8;
    pub fn BMessage_GetUInt81(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: u8,
    ) -> u8;
    pub fn BMessage_GetInt16(self_: *const c_void, name: *const c_char, default_value: i16) -> i16;
    pub fn BMessage_GetInt161(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: i16,
    ) -> i16;
    pub fn BMessage_GetUInt16(self_: *const c_void, name: *const c_char, default_value: u16)
        -> u16;
    pub fn BMessage_GetUInt161(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: u16,
    ) -> u16;
    pub fn BMessage_GetInt32(self_: *const c_void, name: *const c_char, default_value: i32) -> i32;
    pub fn BMessage_GetInt321(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: i32,
    ) -> i32;
    pub fn BMessage_GetUInt32(self_: *const c_void, name: *const c_char, default_value: u32)
        -> u32;
    pub fn BMessage_GetUInt321(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: u32,
    ) -> u32;
    pub fn BMessage_GetInt64(self_: *const c_void, name: *const c_char, default_value: i64) -> i64;
    pub fn BMessage_GetInt641(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: i64,
    ) -> i64;
    pub fn BMessage_GetUInt64(self_: *const c_void, name: *const c_char, default_value: u64)
        -> u64;
    pub fn BMessage_GetUInt641(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: u64,
    ) -> u64;
    pub fn BMessage_GetFloat(
        self_: *const c_void,
        name: *const c_char,
        default_value: c_float,
    ) -> c_float;
    pub fn BMessage_GetFloat1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: c_float,
    ) -> c_float;
    pub fn BMessage_GetDouble(
        self_: *const c_void,
        name: *const c_char,
        default_value: c_double,
    ) -> c_double;
    pub fn BMessage_GetDouble1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: c_double,
    ) -> c_double;
    // NOT_SUPPORTED: pub fn BMessage_GetColor(self_: *const c_void, name: *const c_char, default_value: rgb_color) -> rgb_color;
    // NOT_SUPPORTED: pub fn BMessage_GetColor1(self_: *const c_void, name: *const c_char, index: i32, default_value: rgb_color) -> rgb_color;
    pub fn BMessage_GetPointer(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: *const c_void,
    ) -> *const c_void;
    pub fn BMessage_GetPointer1(
        self_: *const c_void,
        name: *const c_char,
        default_value: *const c_void,
    ) -> *const c_void;
    pub fn BMessage_GetString(
        self_: *const c_void,
        name: *const c_char,
        default_value: *const c_char,
    ) -> *const c_char;
    pub fn BMessage_GetString1(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: *const c_char,
    ) -> *const c_char;
    // NOT_SUPPORTED: pub fn BMessage_GetAlignment(self_: *const c_void, name: *const c_char, index: i32, default_value: *const c_void) -> BAlignment;
    // NOT_SUPPORTED: pub fn BMessage_GetAlignment1(self_: *const c_void, name: *const c_char, default_value: *const c_void) -> BAlignment;
    pub fn BMessage_GetRect(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: *const c_void,
    ) -> *mut c_void;
    pub fn BMessage_GetRect1(
        self_: *const c_void,
        name: *const c_char,
        default_value: *const c_void,
    ) -> *mut c_void;
    pub fn BMessage_GetPoint(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: *const c_void,
    ) -> *mut c_void;
    pub fn BMessage_GetPoint1(
        self_: *const c_void,
        name: *const c_char,
        default_value: *const c_void,
    ) -> *mut c_void;
    pub fn BMessage_GetSize(
        self_: *const c_void,
        name: *const c_char,
        index: i32,
        default_value: *const c_void,
    ) -> *mut c_void;
    pub fn BMessage_GetSize1(
        self_: *const c_void,
        name: *const c_char,
        default_value: *const c_void,
    ) -> *mut c_void;
    pub fn BMessage_SetBool(self_: *mut c_void, name: *const c_char, value: bool) -> status_t;
    pub fn BMessage_SetInt8(self_: *mut c_void, name: *const c_char, value: i8) -> status_t;
    pub fn BMessage_SetUInt8(self_: *mut c_void, name: *const c_char, value: u8) -> status_t;
    pub fn BMessage_SetInt16(self_: *mut c_void, name: *const c_char, value: i16) -> status_t;
    pub fn BMessage_SetUInt16(self_: *mut c_void, name: *const c_char, value: u16) -> status_t;
    pub fn BMessage_SetInt32(self_: *mut c_void, name: *const c_char, value: i32) -> status_t;
    pub fn BMessage_SetUInt32(self_: *mut c_void, name: *const c_char, value: u32) -> status_t;
    pub fn BMessage_SetInt64(self_: *mut c_void, name: *const c_char, value: i64) -> status_t;
    pub fn BMessage_SetUInt64(self_: *mut c_void, name: *const c_char, value: u64) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_SetColor(self_: *mut c_void, name: *const c_char, value: rgb_color) -> status_t;
    pub fn BMessage_SetPointer(
        self_: *mut c_void,
        name: *const c_char,
        value: *const c_void,
    ) -> status_t;
    pub fn BMessage_SetString(
        self_: *mut c_void,
        name: *const c_char,
        string: *const c_char,
    ) -> status_t;
    pub fn BMessage_SetString1(
        self_: *mut c_void,
        name: *const c_char,
        string: *const c_void,
    ) -> status_t;
    pub fn BMessage_SetFloat(self_: *mut c_void, name: *const c_char, value: c_float) -> status_t;
    pub fn BMessage_SetDouble(self_: *mut c_void, name: *const c_char, value: c_double)
        -> status_t;
    pub fn BMessage_SetAlignment(
        self_: *mut c_void,
        name: *const c_char,
        value: *const c_void,
    ) -> status_t;
    pub fn BMessage_SetPoint(
        self_: *mut c_void,
        name: *const c_char,
        value: *const c_void,
    ) -> status_t;
    pub fn BMessage_SetRect(
        self_: *mut c_void,
        name: *const c_char,
        value: *const c_void,
    ) -> status_t;
    pub fn BMessage_SetSize(
        self_: *mut c_void,
        name: *const c_char,
        value: *const c_void,
    ) -> status_t;
    // NOT_SUPPORTED: pub fn BMessage_SetData(self_: *mut c_void, name: *const c_char, type_: type_code, data: *const c_void, num_bytes: ssize_t, fixed_size: bool, count: c_int) -> status_t;
    pub fn BMessage_new() -> *mut c_void;
    pub fn BMessage_new1(other: *const c_void) -> *mut c_void;
    pub fn BMessage_new2(what: u32) -> *mut c_void;
    // DTOR: pub fn BMessage_~BMessage(self_: *mut c_void);

}

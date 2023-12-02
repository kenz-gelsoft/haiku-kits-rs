use super::*;

extern "C" {

    // BControl
    pub fn BControl_new(data: *mut c_void) -> *mut c_void;
    pub fn BControl_Instantiate(data: *mut c_void) -> *mut c_void;
    pub fn BControl_new1(
        frame: *mut c_void,
        name: *const c_char,
        label: *const c_char,
        message: *mut c_void,
        resizing_mode: u32,
        flags: u32,
    ) -> *mut c_void;
    pub fn BControl_new2(
        name: *const c_char,
        label: *const c_char,
        message: *mut c_void,
        flags: u32,
    ) -> *mut c_void;
    // DTOR: pub fn BControl_~BControl(self_: *mut c_void);
    pub fn BControl_IconBitmap(self_: *const c_void, which: u32) -> *const c_void;
    pub fn BControl_IsEnabled(self_: *const c_void) -> bool;
    pub fn BControl_Label(self_: *const c_void) -> *const c_char;
    pub fn BControl_SetEnabled(self_: *mut c_void, enabled: bool);
    pub fn BControl_SetIcon(self_: *mut c_void, bitmap: *const c_void, flags: u32) -> status_t;
    pub fn BControl_SetIconBitmap(
        self_: *mut c_void,
        bitmap: *const c_void,
        which: u32,
        flags: u32,
    ) -> status_t;
    pub fn BControl_SetLabel(self_: *mut c_void, string: *const c_char);
    pub fn BControl_SetValue(self_: *mut c_void, value: i32);
    pub fn BControl_Value(self_: *const c_void) -> i32;

}

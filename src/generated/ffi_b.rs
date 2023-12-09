use super::*;

extern "C" {

    // BButton
    pub fn BButton_dynamic_cast(ptr: *mut c_void) -> *mut c_void;
    pub fn BButton_Instantiate(data: *mut c_void) -> *mut c_void;
    pub fn BButton_new(data: *mut c_void) -> *mut c_void;
    pub fn BButton_new1(
        frame: *mut c_void,
        name: *const c_char,
        label: *const c_char,
        message: *mut c_void,
        resizing_mode: u32,
        flags: u32,
    ) -> *mut c_void;
    pub fn BButton_new2(label: *const c_char, message: *mut c_void) -> *mut c_void;
    pub fn BButton_new3(
        name: *const c_char,
        label: *const c_char,
        message: *mut c_void,
        flags: u32,
    ) -> *mut c_void;
    // DTOR: pub fn BButton_~BButton(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn BButton_Behavior(self_: *const c_void) -> BBehavior;
    pub fn BButton_IsDefault(self_: *const c_void) -> bool;
    pub fn BButton_IsFlat(self_: *const c_void) -> bool;
    pub fn BButton_MakeDefault(self_: *mut c_void, flag: bool);
    pub fn BButton_PopUpMessage(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn BButton_SetBehavior(self_: *mut c_void, behavior: BBehavior);
    pub fn BButton_SetFlat(self_: *mut c_void, flat: bool);
    pub fn BButton_SetPopUpMessage(self_: *mut c_void, message: *mut c_void);
    // Mix-in(s) to BButton
    pub fn BButton_AsInvoker(obj: *mut c_void) -> *mut c_void;

}

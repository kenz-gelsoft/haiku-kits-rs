use super::*;

extern "C" {

    // BSize
    pub fn BSize_delete(self_: *mut c_void);
    pub fn BSize_height(self_: *mut c_void) -> c_float;
    pub fn BSize_width(self_: *mut c_void) -> c_float;
    // BLOCKED: pub fn BSize_operator==(self_: *const c_void, other: *const c_void) -> bool;
    // BLOCKED: pub fn BSize_operator!=(self_: *const c_void, other: *const c_void) -> bool;
    // BLOCKED: pub fn BSize_operator=(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    pub fn BSize_new() -> *mut c_void;
    pub fn BSize_new1(other: *const c_void) -> *mut c_void;
    pub fn BSize_new2(width: c_float, height: c_float) -> *mut c_void;
    // BLOCKED: pub fn BSize_Height(self_: *const c_void) -> c_float;
    pub fn BSize_IntegerHeight(self_: *const c_void) -> i32;
    pub fn BSize_IntegerWidth(self_: *const c_void) -> i32;
    pub fn BSize_IsHeightSet(self_: *const c_void) -> bool;
    pub fn BSize_IsWidthSet(self_: *const c_void) -> bool;
    pub fn BSize_Set(self_: *mut c_void, width: c_float, height: c_float);
    pub fn BSize_SetHeight(self_: *mut c_void, height: c_float);
    pub fn BSize_SetWidth(self_: *mut c_void, width: c_float);
    // BLOCKED: pub fn BSize_Width(self_: *const c_void) -> c_float;

}

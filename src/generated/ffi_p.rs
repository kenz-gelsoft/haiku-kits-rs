use super::*;

extern "C" {

    // BPoint
    pub fn BPoint_delete(self_: *mut c_void);
    pub fn BPoint_x(self_: *mut c_void) -> c_float;
    pub fn BPoint_y(self_: *mut c_void) -> c_float;
    // BLOCKED: pub fn BPoint_operator=(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn BPoint_operator-(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn BPoint_operator+(self_: *const c_void, other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn BPoint_operator-1(self_: *const c_void, other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn BPoint_operator+=(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn BPoint_operator-=(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn BPoint_operator!=(self_: *const c_void, other: *const c_void) -> bool;
    // BLOCKED: pub fn BPoint_operator==(self_: *const c_void, other: *const c_void) -> bool;
    pub fn BPoint_new() -> *mut c_void;
    pub fn BPoint_new1(p: *const c_void) -> *mut c_void;
    pub fn BPoint_new2(x: c_float, y: c_float) -> *mut c_void;
    pub fn BPoint_ConstrainTo(self_: *mut c_void, rect: *mut c_void);
    pub fn BPoint_PrintToStream(self_: *const c_void);
    pub fn BPoint_Set(self_: *mut c_void, x: c_float, y: c_float);

}

use super::*;

extern "C" {

    // BRect
    pub fn BRect_delete(self_: *mut c_void);
    pub fn BRect_bottom(self_: *mut c_void) -> c_float;
    pub fn BRect_set_bottom(self_: *mut c_void, bottom: c_float);
    pub fn BRect_left(self_: *mut c_void) -> c_float;
    pub fn BRect_set_left(self_: *mut c_void, left: c_float);
    pub fn BRect_right(self_: *mut c_void) -> c_float;
    pub fn BRect_set_right(self_: *mut c_void, right: c_float);
    pub fn BRect_top(self_: *mut c_void) -> c_float;
    pub fn BRect_set_top(self_: *mut c_void, top: c_float);
    // BLOCKED: pub fn BRect_operator=(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn BRect_operator==(self_: *const c_void, other: *mut c_void) -> bool;
    // BLOCKED: pub fn BRect_operator!=(self_: *const c_void, other: *mut c_void) -> bool;
    // BLOCKED: pub fn BRect_operator&(self_: *const c_void, other: *mut c_void) -> *mut c_void;
    // BLOCKED: pub fn BRect_operator|(self_: *const c_void, other: *mut c_void) -> *mut c_void;
    pub fn BRect_InsetBy(self_: *mut c_void, inset: *mut c_void);
    pub fn BRect_InsetBy1(self_: *mut c_void, dx: c_float, dy: c_float);
    pub fn BRect_InsetBySelf(self_: *mut c_void, inset: *mut c_void) -> *mut c_void;
    pub fn BRect_InsetBySelf1(self_: *mut c_void, dx: c_float, dy: c_float) -> *mut c_void;
    pub fn BRect_InsetByCopy(self_: *const c_void, inset: *mut c_void) -> *mut c_void;
    pub fn BRect_InsetByCopy1(self_: *const c_void, dx: c_float, dy: c_float) -> *mut c_void;
    pub fn BRect_OffsetBy(self_: *mut c_void, delta: *mut c_void);
    pub fn BRect_OffsetBy1(self_: *mut c_void, dx: c_float, dy: c_float);
    pub fn BRect_OffsetTo(self_: *mut c_void, offset: *mut c_void);
    pub fn BRect_OffsetTo1(self_: *mut c_void, x: c_float, y: c_float);
    pub fn BRect_OffsetBySelf(self_: *mut c_void, offset: *mut c_void) -> *mut c_void;
    pub fn BRect_OffsetBySelf1(self_: *mut c_void, dx: c_float, dy: c_float) -> *mut c_void;
    pub fn BRect_OffsetByCopy(self_: *const c_void, offset: *mut c_void) -> *mut c_void;
    pub fn BRect_OffsetByCopy1(self_: *const c_void, dx: c_float, dy: c_float) -> *mut c_void;
    pub fn BRect_OffsetToSelf(self_: *mut c_void, offset: *mut c_void) -> *mut c_void;
    pub fn BRect_OffsetToSelf1(self_: *mut c_void, x: c_float, y: c_float) -> *mut c_void;
    pub fn BRect_OffsetToCopy(self_: *const c_void, offset: *mut c_void) -> *mut c_void;
    pub fn BRect_OffsetToCopy1(self_: *const c_void, x: c_float, y: c_float) -> *mut c_void;
    pub fn BRect_new() -> *mut c_void;
    pub fn BRect_new1(left_top: *mut c_void, right_bottom: *mut c_void) -> *mut c_void;
    pub fn BRect_new2(left_top: *mut c_void, size: *mut c_void) -> *mut c_void;
    pub fn BRect_new3(other: *const c_void) -> *mut c_void;
    pub fn BRect_new4(left: c_float, top: c_float, right: c_float, bottom: c_float) -> *mut c_void;
    pub fn BRect_new5(side: c_float) -> *mut c_void;
    pub fn BRect_Contains(self_: *const c_void, point: *mut c_void) -> bool;
    pub fn BRect_Contains1(self_: *const c_void, rect: *mut c_void) -> bool;
    pub fn BRect_Height(self_: *const c_void) -> c_float;
    pub fn BRect_IntegerHeight(self_: *const c_void) -> i32;
    pub fn BRect_IntegerWidth(self_: *const c_void) -> i32;
    pub fn BRect_Intersects(self_: *const c_void, rect: *mut c_void) -> bool;
    pub fn BRect_IsValid(self_: *const c_void) -> bool;
    pub fn BRect_LeftBottom(self_: *const c_void) -> *mut c_void;
    pub fn BRect_LeftTop(self_: *const c_void) -> *mut c_void;
    pub fn BRect_PrintToStream(self_: *const c_void);
    pub fn BRect_RightBottom(self_: *const c_void) -> *mut c_void;
    pub fn BRect_RightTop(self_: *const c_void) -> *mut c_void;
    pub fn BRect_Set(
        self_: *mut c_void,
        left: c_float,
        top: c_float,
        right: c_float,
        bottom: c_float,
    );
    pub fn BRect_SetLeftBottom(self_: *mut c_void, point: *const c_void);
    pub fn BRect_SetLeftTop(self_: *mut c_void, point: *const c_void);
    pub fn BRect_SetRightBottom(self_: *mut c_void, point: *const c_void);
    pub fn BRect_SetRightTop(self_: *mut c_void, point: *const c_void);
    pub fn BRect_Size(self_: *const c_void) -> *mut c_void;
    pub fn BRect_Width(self_: *const c_void) -> c_float;

}

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::os::raw::c_int;

pub const fn _rule_(t: c_int, l: c_int, b: c_int, r: c_int) -> c_int {
    (t << 12) | (l << 8) | (b << 4) | r
}

pub const fn B_TO_POSIX_ERROR(v: c_int) -> c_int {
    v
}
pub const fn B_FROM_POSIX_ERROR(v: c_int) -> c_int {
    v
}

pub const fn B_MOUSE_BUTTON(v: c_int) -> c_int {
    1 << (v - 1)
}

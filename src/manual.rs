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

pub const _VIEW_TOP_: c_int = 1;
pub const _VIEW_LEFT_: c_int = 2;
pub const _VIEW_BOTTOM_: c_int = 3;
pub const _VIEW_RIGHT_: c_int = 4;
pub const _VIEW_CENTER_: c_int = 5;
pub const B_BEOS_VERSION: c_int = 0x0500;
pub const INT_MIN: c_int = -2147483647;
pub const MAXPATHLEN: c_int = 1024;
pub const NAME_MAX: c_int = 256;
pub const O_APPEND: c_int = 0x00000800;
pub const O_RDONLY: c_int = 00;
pub const O_WRONLY: c_int = 01;
pub const O_RDWR: c_int = 02;
pub const O_EXCL: c_int = 0x0100;
pub const O_CREAT: c_int = 0x0200;
pub const O_TRUNC: c_int = 0x0400;
pub const SYMLOOP_MAX: c_int = 16;

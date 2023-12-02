#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use std::ffi::{CStr, CString, c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_void};

use super::*;
use methods::*;

mod ffi;
mod ffi_a;
mod ffi_c;
mod ffi_h;
mod ffi_i;
mod ffi_l;
mod ffi_m;
mod ffi_p;
mod ffi_r;
mod ffi_s;
mod ffi_v;
mod ffi_w;

pub mod methods;
mod methods_a;
mod methods_c;
mod methods_h;
mod methods_i;
mod methods_l;
mod methods_m;
mod methods_p;
mod methods_r;
mod methods_s;
mod methods_v;
mod methods_w;

pub mod class;
mod class_a;
mod class_c;
mod class_h;
mod class_i;
mod class_l;
mod class_m;
mod class_p;
mod class_r;
mod class_s;
mod class_v;
mod class_w;

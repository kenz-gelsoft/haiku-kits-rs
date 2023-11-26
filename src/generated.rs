#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use std::ffi::{CStr, CString, c_char, c_double, c_int, c_long, c_uchar, c_uint, c_void};

use super::*;
use methods::*;

mod ffi;
mod ffi_a;
mod ffi_h;
mod ffi_l;
mod ffi_w;

pub mod methods;
mod methods_a;
mod methods_h;
mod methods_l;
mod methods_w;

pub mod class;
mod class_a;
mod class_h;
mod class_l;
mod class_w;

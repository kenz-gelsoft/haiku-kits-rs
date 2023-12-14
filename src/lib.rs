#![doc = include_str!("../README.md")]

use std::ffi::OsString;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::str;

mod macros;

mod constants;
#[doc(hidden)]
pub use constants::*;
mod manual;
#[doc(hidden)]
pub use manual::*;

mod generated;
pub use generated::class::*;
//pub use generated::RustEvent;

use methods::*;

pub use typedefs::*;
#[allow(non_camel_case_types)]
mod typedefs {
    use std::os::raw::c_int;

    pub type bigtime_t = i64;
    pub type perform_code = u32;
    pub type screen_id = i32;
    pub type sem_id = i32;
    pub type status_t = i32;
    pub type team_id = i32;
    pub type thread_id = i32;

    // enum types
    // BView
    pub type cap_mode = c_int;
    pub type color_which = c_int;
    pub type drawing_mode = c_int;
    pub type join_mode = c_int;
    // BWindow
    pub type window_alignment = c_int;
    pub type window_feel = c_int;
    pub type window_look = c_int;
    pub type window_type = c_int;
}

mod ffi {
    use std::os::raw::{c_char, c_int, c_void};

    extern "C" {
        pub fn BArchivable_delete(self_: *mut c_void);

        pub fn RustHandler_new(
            aFn: *mut c_void,
            aParam: *mut c_void,
            name: *const c_char,
        ) -> *mut c_void;

//        pub fn wxEvtHandler_Bind(
//            self_: *mut c_void,
//            eventType: c_int,
//            aFn: *mut c_void,
//            aParam: *mut c_void,
//        );
//
//        pub fn wxEvtHandler_CallAfter(self_: *mut c_void, aFn: *mut c_void, aParam: *mut c_void);

        // ArrayInt
        pub fn wxArrayInt_new() -> *mut c_void;
        pub fn wxArrayInt_delete(self_: *mut c_void);
        pub fn wxArrayInt_Add(self_: *mut c_void, i: c_int);
        pub fn wxArrayInt_Item(self_: *mut c_void, index: usize) -> c_int;
    }
}

pub mod methods {
    pub use super::generated::methods::*;
    use super::*;

//    pub trait Bindable {
//        fn bind<E: EventMethods, F: Fn(&E) + 'static>(&self, event_type: RustEvent, closure: F);
//        fn call_after<F: Fn(*mut c_void) + 'static>(&self, closure: F);
//    }

    pub trait ArrayIntMethods: RustBindingMethods {
        fn add(&self, i: c_int) {
            unsafe { ffi::wxArrayInt_Add(self.as_ptr(), i) }
        }
        fn item(&self, index: usize) -> c_int {
            unsafe { ffi::wxArrayInt_Item(self.as_ptr(), index) }
        }
    }

    pub trait DynamicCast: ArchivableMethods {
        fn dynamic_cast<T: DynamicCast>(from: &T) -> Option<Self::CppManaged>;
    }

    pub trait OptionFrom<T> {
        unsafe fn option_from(ptr: *const i8) -> Option<T>;
    }
}

impl OptionFrom<&CStr> for CStr {
    unsafe fn option_from(ptr: *const i8) -> Option<&'static CStr> {
        if ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ptr))
        }
    }
}

// Rust closure to wx calablle function+param pair.
unsafe fn to_wx_callable<F: Fn(*mut c_void) + 'static>(closure: F) -> (*mut c_void, *mut c_void) {
    unsafe fn trampoline<F: Fn(*mut c_void) + 'static>(closure: *mut c_void, arg: *mut c_void) {
        let closure = &*(closure as *const F);
        closure(arg);
    }
    // pass the pointer in the heap to avoid move.
    let closure = Box::new(closure);
    (trampoline::<F> as _, Box::into_raw(closure) as _)
}

//impl<T: EvtHandlerMethods> Bindable for T {
//    fn bind<E: EventMethods, F: Fn(&E) + 'static>(&self, event_type: RustEvent, closure: F) {
//        unsafe {
//            let (f, param) = to_wx_callable(move |arg: *mut c_void| {
//                E::with_ptr(arg, |event| {
//                    closure(event);
//                });
//            });
//            ffi::wxEvtHandler_Bind(self.as_ptr(), event_type as c_int, f, param);
//        }
//    }
//    fn call_after<F: Fn(*mut c_void) + 'static>(&self, closure: F) {
//        unsafe {
//            let (f, param) = to_wx_callable(closure);
//            ffi::wxEvtHandler_CallAfter(self.as_ptr(), f, param);
//        }
//    }
//}

binding! {
    class RustHandler
        = RustHandlerFromCpp<false>(RustHandler) impl
        HandlerMethods,
        ArchivableMethods
}
impl<const FROM_CPP: bool> RustHandlerFromCpp<FROM_CPP> {
    pub fn new<F: Fn(*mut c_void) + 'static>(&self, closure: F, name: &str) -> Self {
        unsafe {
            let name = CString::new(name).unwrap();
            let (f, param) = to_wx_callable(closure);
            RustHandlerFromCpp(ffi::RustHandler_new(f, param, name.as_ptr()))
        }
    }
}

binding! {
    class ArrayInt
        = ArrayIntFromCpp<false>(wxArrayInt) impl
        ArrayIntMethods
}
impl<const FROM_CPP: bool> ArrayIntFromCpp<FROM_CPP> {
    pub fn new() -> Self {
        unsafe { ArrayIntFromCpp(ffi::wxArrayInt_new()) }
    }
}
impl<const FROM_CPP: bool> Drop for ArrayIntFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxArrayInt_delete(self.0) }
        }
    }
}

//impl<const FROM_CPP: bool> DateTimeMethodsManual for DateTimeFromCpp<FROM_CPP> {}

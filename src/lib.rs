#![doc = include_str!("../README.md")]

use std::ffi::OsString;
use std::marker::PhantomData;
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

#[cfg(windows)]
type ArgChar = u16;
#[cfg(not(windows))]
type ArgChar = u8;

pub use typedefs::*;
#[allow(non_camel_case_types)]
mod typedefs {
    pub type bigtime_t = i64;
    pub type perform_code = u32;
    pub type sem_id = i32;
    pub type status_t = i32;
    pub type team_id = i32;
    pub type thread_id = i32;
}

mod ffi {
    use std::os::raw::{c_int, c_void};

    extern "C" {
        pub fn BArchivable_delete(self_: *mut c_void);

        pub fn AppSetOnInit(aFn: *mut c_void, aParam: *mut c_void);
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

        pub fn wxRustEntry(argc: *mut c_int, argv: *mut *const super::ArgChar) -> c_int;

        // WeakRef
        pub fn OpaqueWeakRef_new(obj: *mut c_void) -> *mut c_void;
        pub fn OpaqueWeakRef_copy(obj: *mut c_void) -> *mut c_void;
        pub fn OpaqueWeakRef_delete(self_: *mut c_void);
        pub fn OpaqueWeakRef_Get(self_: *mut c_void) -> *mut c_void;
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

//    pub trait DynamicCast: ObjectMethods {
//        fn class_info() -> ClassInfoFromCpp<true>;
//        fn dynamic_cast<T: DynamicCast>(&self) -> Option<T::CppManaged> {
//            if self.is_kind_of(Some(&T::class_info())) {
//                unsafe { Some(T::from_cpp_managed_ptr(self.as_ptr())) }
//            } else {
//                None
//            }
//        }
//    }
//
//    pub trait Trackable<T>: EvtHandlerMethods {
//        fn to_weak_ref(&self) -> WeakRef<T>;
//    }
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
//
// Effectively all wxEvtHandlers are wxTrackable.
//impl<T: EvtHandlerMethods> Trackable<T> for T {
//    fn to_weak_ref(&self) -> WeakRef<T> {
//        unsafe { WeakRef::from(self.as_ptr()) }
//    }
//}

// wxApp
pub enum App {}
impl App {
    pub fn on_init<F: Fn(*mut c_void) + 'static>(closure: F) {
        unsafe {
            let (f, param) = to_wx_callable(closure);
            ffi::AppSetOnInit(f, param);
        }
    }
    pub fn run<F: Fn(*mut c_void) + 'static>(closure: F) {
        Self::on_init(closure);
        entry();
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

// wxEntry
pub fn entry() {
    #[cfg(windows)]
    fn to_wx_arg(arg: OsString) -> Vec<ArgChar> {
        use std::os::windows::prelude::OsStrExt;

        let mut wide: Vec<ArgChar> = arg.encode_wide().collect();
        wide.push(0);
        wide
    }
    #[cfg(not(windows))]
    fn to_wx_arg(arg: OsString) -> Vec<ArgChar> {
        use std::os::unix::prelude::OsStringExt;

        let mut vec: Vec<ArgChar> = arg.into_vec();
        vec.push(0);
        vec
    }

    let args: Vec<Vec<ArgChar>> = std::env::args_os().map(to_wx_arg).collect();
    let mut argc: c_int = args.len().try_into().unwrap();
    let mut argv: Vec<*const ArgChar> = args
        .iter()
        .map(|arg| arg.as_ptr() as *const ArgChar)
        .collect();
    unsafe {
        ffi::wxRustEntry(&mut argc, argv.as_mut_ptr());
    }
}

// wxWeakRef
pub struct WeakRef<T>(*mut c_void, PhantomData<T>);
impl<T: RustBindingMethods> WeakRef<T> {
    pub unsafe fn from(ptr: *mut c_void) -> Self {
        let ptr = if ptr.is_null() {
            ptr
        } else {
            ffi::OpaqueWeakRef_new(ptr)
        };
        WeakRef(ptr, PhantomData)
    }
    pub fn get(&self) -> Option<T::CppManaged> {
        unsafe {
            let ptr = self.0;
            let ptr = if ptr.is_null() {
                ptr
            } else {
                ffi::OpaqueWeakRef_Get(ptr)
            };
            if ptr.is_null() {
                None
            } else {
                Some(T::from_cpp_managed_ptr(ptr))
            }
        }
    }
}
impl<T: RustBindingMethods> Clone for WeakRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = ffi::OpaqueWeakRef_copy(self.0);
            WeakRef(ptr, PhantomData)
        }
    }
}
impl<T> Drop for WeakRef<T> {
    fn drop(&mut self) {
        unsafe { ffi::OpaqueWeakRef_delete(self.0) }
    }
}

//impl<const FROM_CPP: bool> DateTimeMethodsManual for DateTimeFromCpp<FROM_CPP> {}

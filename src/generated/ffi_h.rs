use super::*;

extern "C" {

    // BHandler
    pub fn BHandler_CLASSINFO() -> *mut c_void;
    pub fn BHandler_new(data: *mut c_void) -> *mut c_void;
    pub fn BHandler_MessageReceived(self_: *mut c_void, message: *mut c_void);
    pub fn BHandler_Looper(self_: *const c_void) -> *mut c_void;
    pub fn BHandler_SetName(self_: *mut c_void, name: *const c_void);
    pub fn BHandler_Name(self_: *const c_void) -> *const c_void;
    pub fn BHandler_SetNextHandler(self_: *mut c_void, handler: *mut c_void);
    pub fn BHandler_NextHandler(self_: *const c_void) -> *mut c_void;
    pub fn BHandler_AddFilter(self_: *mut c_void, filter: *mut c_void);
    pub fn BHandler_RemoveFilter(self_: *mut c_void, filter: *mut c_void) -> bool;
    pub fn BHandler_SetFilterList(self_: *mut c_void, filters: *mut c_void);
    pub fn BHandler_FilterList(self_: *mut c_void) -> *mut c_void;
    pub fn BHandler_LockLooper(self_: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn BHandler_LockLooperWithTimeout(self_: *mut c_void, timeout: i64) -> status_t;
    pub fn BHandler_UnlockLooper(self_: *mut c_void);
    pub fn BHandler_ResolveSpecifier(self_: *mut c_void, message: *mut c_void, index: i32, specifier: *mut c_void, what: i32, property: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn BHandler_GetSupportedSuites(self_: *mut c_void, data: *mut c_void) -> status_t;
    // NOT_SUPPORTED: pub fn BHandler_StartWatching(self_: *mut c_void, target: BMessenger, what: uint32) -> status_t;
    // NOT_SUPPORTED: pub fn BHandler_StartWatchingAll(self_: *mut c_void, target: BMessenger) -> status_t;
    // NOT_SUPPORTED: pub fn BHandler_StopWatching(self_: *mut c_void, target: BMessenger, what: uint32) -> status_t;
    // NOT_SUPPORTED: pub fn BHandler_StopWatchingAll(self_: *mut c_void, target: BMessenger) -> status_t;
    // NOT_SUPPORTED: pub fn BHandler_StartWatching1(self_: *mut c_void, observer: *mut c_void, what: uint32) -> status_t;
    // NOT_SUPPORTED: pub fn BHandler_StartWatchingAll1(self_: *mut c_void, observer: *mut c_void) -> status_t;
    // NOT_SUPPORTED: pub fn BHandler_StopWatching1(self_: *mut c_void, observer: *mut c_void, what: uint32) -> status_t;
    // NOT_SUPPORTED: pub fn BHandler_StopWatchingAll1(self_: *mut c_void, observer: *mut c_void) -> status_t;
    // NOT_SUPPORTED: pub fn BHandler_SendNotices(self_: *mut c_void, what: uint32, notice: *const c_void);
    pub fn BHandler_IsWatched(self_: *const c_void) -> bool;
    pub fn BHandler_new1(name: *const c_void) -> *mut c_void;
    // DTOR: pub fn BHandler_~BHandler(self_: *mut c_void);
    pub fn BHandler_Instantiate(data: *mut c_void) -> *mut c_void;

}

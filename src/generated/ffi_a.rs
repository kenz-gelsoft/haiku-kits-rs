use super::*;

extern "C" {

    // BApplication
    pub fn BApplication_new(data: *mut c_void) -> *mut c_void;
    pub fn BApplication_Instantiate(data: *mut c_void) -> *mut c_void;
    pub fn BApplication_ReadyToRun(self_: *mut c_void);
    pub fn BApplication_ArgvReceived(self_: *mut c_void, argc: i32, argv: *mut c_void);
    pub fn BApplication_AppActivated(self_: *mut c_void, active: bool);
    pub fn BApplication_RefsReceived(self_: *mut c_void, message: *mut c_void);
    pub fn BApplication_AboutRequested(self_: *mut c_void);
    pub fn BApplication_Pulse(self_: *mut c_void);
    pub fn BApplication_SetPulseRate(self_: *mut c_void, rate: bigtime_t);
    pub fn BApplication_ShowCursor(self_: *mut c_void);
    pub fn BApplication_HideCursor(self_: *mut c_void);
    pub fn BApplication_ObscureCursor(self_: *mut c_void);
    pub fn BApplication_IsCursorHidden(self_: *const c_void) -> bool;
    pub fn BApplication_SetCursor(self_: *mut c_void, cursor: *const c_void);
    pub fn BApplication_SetCursor1(self_: *mut c_void, cursor: *const c_void, sync: bool);
    pub fn BApplication_CountWindows(self_: *const c_void) -> i32;
    pub fn BApplication_WindowAt(self_: *const c_void, index: i32) -> *mut c_void;
    pub fn BApplication_CountLoopers(self_: *const c_void) -> i32;
    pub fn BApplication_LooperAt(self_: *const c_void, index: i32) -> *mut c_void;
    pub fn BApplication_IsLaunching(self_: *const c_void) -> bool;
    pub fn BApplication_Signature(self_: *const c_void) -> *const c_void;
    pub fn BApplication_GetAppInfo(self_: *const c_void, info: *mut c_void) -> status_t;
    pub fn BApplication_AppResources() -> *mut c_void;
    pub fn BApplication_RegisterLooper(self_: *mut c_void, looper: *mut c_void) -> status_t;
    pub fn BApplication_UnregisterLooper(self_: *mut c_void, looper: *mut c_void) -> status_t;
    pub fn BApplication_new1(signature: *const c_void) -> *mut c_void;
    pub fn BApplication_new2(signature: *const c_void, error: *mut c_void) -> *mut c_void;
    // DTOR: pub fn BApplication_~BApplication(self_: *mut c_void);
    pub fn BApplication_InitCheck(self_: *const c_void) -> status_t;

    // BArchivable
    pub fn BArchivable_new() -> *mut c_void;
    pub fn BArchivable_new1(from: *mut c_void) -> *mut c_void;
    // DTOR: pub fn BArchivable_~BArchivable(self_: *mut c_void);
    pub fn BArchivable_AllArchived(self_: *const c_void, archive: *mut c_void) -> status_t;
    pub fn BArchivable_AllUnarchived(self_: *mut c_void, archive: *const c_void) -> status_t;
    pub fn BArchivable_Archive(self_: *const c_void, into: *mut c_void, deep: bool) -> status_t;
    pub fn BArchivable_Perform(self_: *mut c_void, d: u32, arg: *mut c_void) -> status_t;
    pub fn BArchivable_Instantiate(archive: *mut c_void) -> *mut c_void;

}

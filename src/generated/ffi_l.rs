use super::*;

extern "C" {

    // BLooper
    pub fn BLooper_CLASSINFO() -> *mut c_void;
    pub fn BLooper_new(data: *mut c_void) -> *mut c_void;
    pub fn BLooper_Instantiate(data: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn BLooper_PostMessage(self_: *mut c_void, command: uint32) -> status_t;
    // NOT_SUPPORTED: pub fn BLooper_PostMessage1(self_: *mut c_void, message: *mut c_void) -> status_t;
    // NOT_SUPPORTED: pub fn BLooper_PostMessage2(self_: *mut c_void, command: uint32, handler: *mut c_void, reply_to: *mut c_void) -> status_t;
    // NOT_SUPPORTED: pub fn BLooper_PostMessage3(self_: *mut c_void, message: *mut c_void, handler: *mut c_void, reply_to: *mut c_void) -> status_t;
    pub fn BLooper_DispatchMessage(self_: *mut c_void, message: *mut c_void, handler: *mut c_void);
    pub fn BLooper_CurrentMessage(self_: *const c_void) -> *mut c_void;
    pub fn BLooper_DetachCurrentMessage(self_: *mut c_void) -> *mut c_void;
    pub fn BLooper_DispatchExternalMessage(self_: *mut c_void, message: *mut c_void, handler: *mut c_void, _detached: *mut c_void);
    pub fn BLooper_MessageQueue(self_: *const c_void) -> *mut c_void;
    pub fn BLooper_IsMessageWaiting(self_: *const c_void) -> bool;
    pub fn BLooper_AddHandler(self_: *mut c_void, handler: *mut c_void);
    pub fn BLooper_RemoveHandler(self_: *mut c_void, handler: *mut c_void) -> bool;
    pub fn BLooper_CountHandlers(self_: *const c_void) -> i32;
    pub fn BLooper_HandlerAt(self_: *const c_void, index: i32) -> *mut c_void;
    pub fn BLooper_IndexOf(self_: *const c_void, handler: *mut c_void) -> i32;
    pub fn BLooper_PreferredHandler(self_: *const c_void) -> *mut c_void;
    pub fn BLooper_SetPreferredHandler(self_: *mut c_void, handler: *mut c_void);
    // NOT_SUPPORTED: pub fn BLooper_Run(self_: *mut c_void) -> thread_id;
    pub fn BLooper_Loop(self_: *mut c_void);
    pub fn BLooper_Quit(self_: *mut c_void);
    pub fn BLooper_QuitRequested(self_: *mut c_void) -> bool;
    pub fn BLooper_Lock(self_: *mut c_void) -> bool;
    pub fn BLooper_Unlock(self_: *mut c_void);
    pub fn BLooper_IsLocked(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn BLooper_LockWithTimeout(self_: *mut c_void, timeout: bigtime_t) -> status_t;
    // NOT_SUPPORTED: pub fn BLooper_Thread(self_: *const c_void) -> thread_id;
    // NOT_SUPPORTED: pub fn BLooper_Team(self_: *const c_void) -> team_id;
    // NOT_SUPPORTED: pub fn BLooper_LooperForThread(thread: thread_id) -> *mut c_void;
    // NOT_SUPPORTED: pub fn BLooper_LockingThread(self_: *const c_void) -> thread_id;
    pub fn BLooper_CountLocks(self_: *const c_void) -> i32;
    pub fn BLooper_CountLockRequests(self_: *const c_void) -> i32;
    // NOT_SUPPORTED: pub fn BLooper_Sem(self_: *const c_void) -> sem_id;
    pub fn BLooper_AddCommonFilter(self_: *mut c_void, filter: *mut c_void);
    pub fn BLooper_RemoveCommonFilter(self_: *mut c_void, filter: *mut c_void) -> bool;
    pub fn BLooper_SetCommonFilterList(self_: *mut c_void, filters: *mut c_void);
    pub fn BLooper_CommonFilterList(self_: *const c_void) -> *mut c_void;
    pub fn BLooper_new1(name: *const c_void, priority: i32, port_capacity: i32) -> *mut c_void;
    // DTOR: pub fn BLooper_~BLooper(self_: *mut c_void);

}

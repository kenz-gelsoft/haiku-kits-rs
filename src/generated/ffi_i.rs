use super::*;

extern "C" {

    // BInvoker
    pub fn BInvoker_delete(self_: *mut c_void);
    pub fn BInvoker_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn BInvoker_new1(message: *mut c_void, target: BMessenger) -> *mut c_void;
    pub fn BInvoker_new2(
        message: *mut c_void,
        handler: *const c_void,
        looper: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn BInvoker_~BInvoker(self_: *mut c_void);
    pub fn BInvoker_Command(self_: *const c_void) -> u32;
    pub fn BInvoker_HandlerForReply(self_: *const c_void) -> *mut c_void;
    pub fn BInvoker_Invoke(self_: *mut c_void, message: *mut c_void) -> status_t;
    pub fn BInvoker_InvokeNotify(self_: *mut c_void, message: *mut c_void, kind: u32) -> status_t;
    pub fn BInvoker_IsTargetLocal(self_: *const c_void) -> bool;
    pub fn BInvoker_Message(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn BInvoker_Messenger(self_: *const c_void) -> BMessenger;
    pub fn BInvoker_SetHandlerForReply(self_: *mut c_void, handler: *mut c_void) -> status_t;
    pub fn BInvoker_SetMessage(self_: *mut c_void, message: *mut c_void) -> status_t;
    // NOT_SUPPORTED: pub fn BInvoker_SetTarget(self_: *mut c_void, messenger: BMessenger) -> status_t;
    pub fn BInvoker_SetTarget1(
        self_: *mut c_void,
        handler: *const c_void,
        looper: *const c_void,
    ) -> status_t;
    pub fn BInvoker_SetTimeout(self_: *mut c_void, timeout: bigtime_t) -> status_t;
    pub fn BInvoker_Target(self_: *const c_void, _looper: *mut c_void) -> *mut c_void;
    pub fn BInvoker_Timeout(self_: *const c_void) -> bigtime_t;

}

#include "generated.h"

extern "C" {

// CLASS: BLooper
void BLooper_delete(BLooper *self) {
    delete self;
}
BLooper *BLooper_new(BMessage * data) {
    return new BLooper(data);
}
BArchivable * BLooper_Instantiate(BMessage * data) {
    return BLooper::Instantiate(data);
}
status_t BLooper_PostMessage(BLooper * self, uint32 command) {
    return self->PostMessage(command);
}
status_t BLooper_PostMessage1(BLooper * self, BMessage * message) {
    return self->PostMessage(message);
}
status_t BLooper_PostMessage2(BLooper * self, uint32 command, BHandler * handler, BHandler * reply_to) {
    return self->PostMessage(command, handler, reply_to);
}
status_t BLooper_PostMessage3(BLooper * self, BMessage * message, BHandler * handler, BHandler * reply_to) {
    return self->PostMessage(message, handler, reply_to);
}
void BLooper_DispatchMessage(BLooper * self, BMessage * message, BHandler * handler) {
    return self->DispatchMessage(message, handler);
}
BMessage * BLooper_CurrentMessage(const BLooper * self) {
    return self->CurrentMessage();
}
BMessage * BLooper_DetachCurrentMessage(BLooper * self) {
    return self->DetachCurrentMessage();
}
void BLooper_DispatchExternalMessage(BLooper * self, BMessage * message, BHandler * handler, bool * _detached) {
    return self->DispatchExternalMessage(message, handler, *_detached);
}
BMessageQueue * BLooper_MessageQueue(const BLooper * self) {
    return self->MessageQueue();
}
bool BLooper_IsMessageWaiting(const BLooper * self) {
    return self->IsMessageWaiting();
}
void BLooper_AddHandler(BLooper * self, BHandler * handler) {
    return self->AddHandler(handler);
}
bool BLooper_RemoveHandler(BLooper * self, BHandler * handler) {
    return self->RemoveHandler(handler);
}
int32 BLooper_CountHandlers(const BLooper * self) {
    return self->CountHandlers();
}
BHandler * BLooper_HandlerAt(const BLooper * self, int32 index) {
    return self->HandlerAt(index);
}
int32 BLooper_IndexOf(const BLooper * self, BHandler * handler) {
    return self->IndexOf(handler);
}
BHandler * BLooper_PreferredHandler(const BLooper * self) {
    return self->PreferredHandler();
}
void BLooper_SetPreferredHandler(BLooper * self, BHandler * handler) {
    return self->SetPreferredHandler(handler);
}
void BLooper_Loop(BLooper * self) {
    return self->Loop();
}
void BLooper_Quit(BLooper * self) {
    return self->Quit();
}
bool BLooper_QuitRequested(BLooper * self) {
    return self->QuitRequested();
}
bool BLooper_Lock(BLooper * self) {
    return self->Lock();
}
void BLooper_Unlock(BLooper * self) {
    return self->Unlock();
}
bool BLooper_IsLocked(const BLooper * self) {
    return self->IsLocked();
}
status_t BLooper_LockWithTimeout(BLooper * self, bigtime_t timeout) {
    return self->LockWithTimeout(timeout);
}
int32 BLooper_CountLocks(const BLooper * self) {
    return self->CountLocks();
}
int32 BLooper_CountLockRequests(const BLooper * self) {
    return self->CountLockRequests();
}
void BLooper_AddCommonFilter(BLooper * self, BMessageFilter * filter) {
    return self->AddCommonFilter(filter);
}
bool BLooper_RemoveCommonFilter(BLooper * self, BMessageFilter * filter) {
    return self->RemoveCommonFilter(filter);
}
void BLooper_SetCommonFilterList(BLooper * self, BList * filters) {
    return self->SetCommonFilterList(filters);
}
BList * BLooper_CommonFilterList(const BLooper * self) {
    return self->CommonFilterList();
}
BLooper *BLooper_new1(const char * name, int32 priority, int32 port_capacity) {
    return new BLooper(name, priority, port_capacity);
}

} // extern "C"


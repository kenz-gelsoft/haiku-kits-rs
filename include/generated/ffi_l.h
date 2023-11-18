#pragma once

#include <Looper.h>

extern "C" {

// CLASS: BLooper
void BLooper_delete(BLooper *self);
BLooper *BLooper_new(BMessage * data);
BArchivable * BLooper_Instantiate(BMessage * data);
status_t BLooper_PostMessage(BLooper * self, uint32 command);
status_t BLooper_PostMessage1(BLooper * self, BMessage * message);
status_t BLooper_PostMessage2(BLooper * self, uint32 command, BHandler * handler, BHandler * reply_to);
status_t BLooper_PostMessage3(BLooper * self, BMessage * message, BHandler * handler, BHandler * reply_to);
void BLooper_DispatchMessage(BLooper * self, BMessage * message, BHandler * handler);
BMessage * BLooper_CurrentMessage(const BLooper * self);
BMessage * BLooper_DetachCurrentMessage(BLooper * self);
void BLooper_DispatchExternalMessage(BLooper * self, BMessage * message, BHandler * handler, bool * _detached);
BMessageQueue * BLooper_MessageQueue(const BLooper * self);
bool BLooper_IsMessageWaiting(const BLooper * self);
void BLooper_AddHandler(BLooper * self, BHandler * handler);
bool BLooper_RemoveHandler(BLooper * self, BHandler * handler);
int32 BLooper_CountHandlers(const BLooper * self);
BHandler * BLooper_HandlerAt(const BLooper * self, int32 index);
int32 BLooper_IndexOf(const BLooper * self, BHandler * handler);
BHandler * BLooper_PreferredHandler(const BLooper * self);
void BLooper_SetPreferredHandler(BLooper * self, BHandler * handler);
thread_id BLooper_Run(BLooper * self);
void BLooper_Loop(BLooper * self);
void BLooper_Quit(BLooper * self);
bool BLooper_QuitRequested(BLooper * self);
bool BLooper_Lock(BLooper * self);
void BLooper_Unlock(BLooper * self);
bool BLooper_IsLocked(const BLooper * self);
status_t BLooper_LockWithTimeout(BLooper * self, bigtime_t timeout);
thread_id BLooper_Thread(const BLooper * self);
team_id BLooper_Team(const BLooper * self);
BLooper * BLooper_LooperForThread(thread_id thread);
thread_id BLooper_LockingThread(const BLooper * self);
int32 BLooper_CountLocks(const BLooper * self);
int32 BLooper_CountLockRequests(const BLooper * self);
sem_id BLooper_Sem(const BLooper * self);
void BLooper_AddCommonFilter(BLooper * self, BMessageFilter * filter);
bool BLooper_RemoveCommonFilter(BLooper * self, BMessageFilter * filter);
void BLooper_SetCommonFilterList(BLooper * self, BList * filters);
BList * BLooper_CommonFilterList(const BLooper * self);
BLooper *BLooper_new1(const char * name, int32 priority, int32 port_capacity);

} // extern "C"


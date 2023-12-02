#pragma once

#include <Invoker.h>

extern "C" {

// CLASS: BInvoker
void BInvoker_delete(BInvoker *self);
BInvoker *BInvoker_new();
BInvoker *BInvoker_new2(BMessage * message, const BHandler * handler, const BLooper * looper);
uint32 BInvoker_Command(const BInvoker * self);
BHandler * BInvoker_HandlerForReply(const BInvoker * self);
status_t BInvoker_Invoke(BInvoker * self, BMessage * message);
status_t BInvoker_InvokeNotify(BInvoker * self, BMessage * message, uint32 kind);
bool BInvoker_IsTargetLocal(const BInvoker * self);
BMessage * BInvoker_Message(const BInvoker * self);
status_t BInvoker_SetHandlerForReply(BInvoker * self, BHandler * handler);
status_t BInvoker_SetMessage(BInvoker * self, BMessage * message);
status_t BInvoker_SetTarget1(BInvoker * self, const BHandler * handler, const BLooper * looper);
status_t BInvoker_SetTimeout(BInvoker * self, bigtime_t timeout);
BHandler * BInvoker_Target(const BInvoker * self, BLooper ** _looper);
bigtime_t BInvoker_Timeout(const BInvoker * self);

} // extern "C"


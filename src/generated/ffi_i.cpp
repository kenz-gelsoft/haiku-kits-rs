#include "generated.h"

extern "C" {

// CLASS: BInvoker
void BInvoker_delete(BInvoker *self) {
    delete self;
}
BInvoker *BInvoker_new() {
    return new BInvoker();
}
BInvoker *BInvoker_new2(BMessage * message, const BHandler * handler, const BLooper * looper) {
    return new BInvoker(message, handler, looper);
}
uint32 BInvoker_Command(const BInvoker * self) {
    return self->Command();
}
BHandler * BInvoker_HandlerForReply(const BInvoker * self) {
    return self->HandlerForReply();
}
status_t BInvoker_Invoke(BInvoker * self, BMessage * message) {
    return self->Invoke(message);
}
status_t BInvoker_InvokeNotify(BInvoker * self, BMessage * message, uint32 kind) {
    return self->InvokeNotify(message, kind);
}
bool BInvoker_IsTargetLocal(const BInvoker * self) {
    return self->IsTargetLocal();
}
BMessage * BInvoker_Message(const BInvoker * self) {
    return self->Message();
}
status_t BInvoker_SetHandlerForReply(BInvoker * self, BHandler * handler) {
    return self->SetHandlerForReply(handler);
}
status_t BInvoker_SetMessage(BInvoker * self, BMessage * message) {
    return self->SetMessage(message);
}
status_t BInvoker_SetTarget1(BInvoker * self, const BHandler * handler, const BLooper * looper) {
    return self->SetTarget(handler, looper);
}
status_t BInvoker_SetTimeout(BInvoker * self, bigtime_t timeout) {
    return self->SetTimeout(timeout);
}
BHandler * BInvoker_Target(const BInvoker * self, BLooper ** _looper) {
    return self->Target(_looper);
}
bigtime_t BInvoker_Timeout(const BInvoker * self) {
    return self->Timeout();
}

} // extern "C"


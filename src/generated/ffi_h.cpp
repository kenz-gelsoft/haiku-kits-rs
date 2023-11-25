#include "generated.h"

extern "C" {

// CLASS: BHandler
BHandler *BHandler_new(BMessage * data) {
    return new BHandler(data);
}
void BHandler_MessageReceived(BHandler * self, BMessage * message) {
    return self->MessageReceived(message);
}
BLooper * BHandler_Looper(const BHandler * self) {
    return self->Looper();
}
void BHandler_SetName(BHandler * self, const char * name) {
    return self->SetName(name);
}
const char *BHandler_Name(const BHandler * self) {
    return self->Name();
}
void BHandler_SetNextHandler(BHandler * self, BHandler * handler) {
    return self->SetNextHandler(handler);
}
BHandler * BHandler_NextHandler(const BHandler * self) {
    return self->NextHandler();
}
void BHandler_AddFilter(BHandler * self, BMessageFilter * filter) {
    return self->AddFilter(filter);
}
bool BHandler_RemoveFilter(BHandler * self, BMessageFilter * filter) {
    return self->RemoveFilter(filter);
}
void BHandler_SetFilterList(BHandler * self, BList * filters) {
    return self->SetFilterList(filters);
}
BList * BHandler_FilterList(BHandler * self) {
    return self->FilterList();
}
bool BHandler_LockLooper(BHandler * self) {
    return self->LockLooper();
}
status_t BHandler_LockLooperWithTimeout(BHandler * self, bigtime_t timeout) {
    return self->LockLooperWithTimeout(timeout);
}
void BHandler_UnlockLooper(BHandler * self) {
    return self->UnlockLooper();
}
BHandler * BHandler_ResolveSpecifier(BHandler * self, BMessage * message, int32 index, BMessage * specifier, int32 what, const char * property) {
    return self->ResolveSpecifier(message, index, specifier, what, property);
}
status_t BHandler_GetSupportedSuites(BHandler * self, BMessage * data) {
    return self->GetSupportedSuites(data);
}
status_t BHandler_StartWatching1(BHandler * self, BHandler * observer, uint32 what) {
    return self->StartWatching(observer, what);
}
status_t BHandler_StartWatchingAll1(BHandler * self, BHandler * observer) {
    return self->StartWatchingAll(observer);
}
status_t BHandler_StopWatching1(BHandler * self, BHandler * observer, uint32 what) {
    return self->StopWatching(observer, what);
}
status_t BHandler_StopWatchingAll1(BHandler * self, BHandler * observer) {
    return self->StopWatchingAll(observer);
}
void BHandler_SendNotices(BHandler * self, uint32 what, const BMessage * notice) {
    return self->SendNotices(what, notice);
}
bool BHandler_IsWatched(const BHandler * self) {
    return self->IsWatched();
}
BHandler *BHandler_new1(const char * name) {
    return new BHandler(name);
}
BArchivable * BHandler_Instantiate(BMessage * data) {
    return BHandler::Instantiate(data);
}

} // extern "C"


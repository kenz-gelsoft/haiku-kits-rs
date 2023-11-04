#include "generated.h"

extern "C" {

// CLASS: BHandler
void BHandler_delete(BHandler *self) {
    delete self;
}
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
const char * BHandler_Name(const BHandler * self) {
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
void BHandler_UnlockLooper(BHandler * self) {
    return self->UnlockLooper();
}
BHandler * BHandler_ResolveSpecifier(BHandler * self, BMessage * message, int32 index, BMessage * specifier, int32 what, const char * property) {
    return self->ResolveSpecifier(message, index, specifier, what, property);
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


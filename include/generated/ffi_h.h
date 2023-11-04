#pragma once

#include <None>

extern "C" {

// CLASS: BHandler
void BHandler_delete(BHandler *self);
BHandler *BHandler_new(BMessage * data);
void BHandler_MessageReceived(BHandler * self, BMessage * message);
BLooper * BHandler_Looper(const BHandler * self);
void BHandler_SetName(BHandler * self, const char * name);
const char * BHandler_Name(const BHandler * self);
void BHandler_SetNextHandler(BHandler * self, BHandler * handler);
BHandler * BHandler_NextHandler(const BHandler * self);
void BHandler_AddFilter(BHandler * self, BMessageFilter * filter);
bool BHandler_RemoveFilter(BHandler * self, BMessageFilter * filter);
void BHandler_SetFilterList(BHandler * self, BList * filters);
BList * BHandler_FilterList(BHandler * self);
bool BHandler_LockLooper(BHandler * self);
void BHandler_UnlockLooper(BHandler * self);
BHandler * BHandler_ResolveSpecifier(BHandler * self, BMessage * message, int32 index, BMessage * specifier, int32 what, const char * property);
bool BHandler_IsWatched(const BHandler * self);
BHandler *BHandler_new1(const char * name);
BArchivable * BHandler_Instantiate(BMessage * data);

} // extern "C"


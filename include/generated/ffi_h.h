#pragma once

#include <Handler.h>

extern "C" {

// CLASS: BHandler
BHandler *BHandler_new(BMessage * data);
void BHandler_MessageReceived(BHandler * self, BMessage * message);
BLooper * BHandler_Looper(const BHandler * self);
void BHandler_SetName(BHandler * self, const char * name);
const char *BHandler_Name(const BHandler * self);
void BHandler_SetNextHandler(BHandler * self, BHandler * handler);
BHandler * BHandler_NextHandler(const BHandler * self);
void BHandler_AddFilter(BHandler * self, BMessageFilter * filter);
bool BHandler_RemoveFilter(BHandler * self, BMessageFilter * filter);
void BHandler_SetFilterList(BHandler * self, BList * filters);
BList * BHandler_FilterList(BHandler * self);
bool BHandler_LockLooper(BHandler * self);
status_t BHandler_LockLooperWithTimeout(BHandler * self, bigtime_t timeout);
void BHandler_UnlockLooper(BHandler * self);
BHandler * BHandler_ResolveSpecifier(BHandler * self, BMessage * message, int32 index, BMessage * specifier, int32 what, const char * property);
status_t BHandler_GetSupportedSuites(BHandler * self, BMessage * data);
status_t BHandler_StartWatching1(BHandler * self, BHandler * observer, uint32 what);
status_t BHandler_StartWatchingAll1(BHandler * self, BHandler * observer);
status_t BHandler_StopWatching1(BHandler * self, BHandler * observer, uint32 what);
status_t BHandler_StopWatchingAll1(BHandler * self, BHandler * observer);
void BHandler_SendNotices(BHandler * self, uint32 what, const BMessage * notice);
bool BHandler_IsWatched(const BHandler * self);
BHandler *BHandler_new1(const char * name);
BArchivable * BHandler_Instantiate(BMessage * data);

} // extern "C"


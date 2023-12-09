#pragma once

#include <Application.h>
#include <Archivable.h>

extern "C" {

// CLASS: BApplication
BApplication *BApplication_dynamic_cast(BArchivable *ptr);
BApplication *BApplication_new(BMessage * data);
BArchivable * BApplication_Instantiate(BMessage * data);
void BApplication_ReadyToRun(BApplication * self);
void BApplication_ArgvReceived(BApplication * self, int32 argc, char ** argv);
void BApplication_AppActivated(BApplication * self, bool active);
void BApplication_RefsReceived(BApplication * self, BMessage * message);
void BApplication_AboutRequested(BApplication * self);
void BApplication_Pulse(BApplication * self);
void BApplication_SetPulseRate(BApplication * self, bigtime_t rate);
void BApplication_ShowCursor(BApplication * self);
void BApplication_HideCursor(BApplication * self);
void BApplication_ObscureCursor(BApplication * self);
bool BApplication_IsCursorHidden(const BApplication * self);
void BApplication_SetCursor(BApplication * self, const void * cursor);
void BApplication_SetCursor1(BApplication * self, const BCursor * cursor, bool sync);
int32 BApplication_CountWindows(const BApplication * self);
BWindow * BApplication_WindowAt(const BApplication * self, int32 index);
int32 BApplication_CountLoopers(const BApplication * self);
BLooper * BApplication_LooperAt(const BApplication * self, int32 index);
bool BApplication_IsLaunching(const BApplication * self);
const char *BApplication_Signature(const BApplication * self);
status_t BApplication_GetAppInfo(const BApplication * self, app_info * info);
BResources * BApplication_AppResources();
status_t BApplication_RegisterLooper(BApplication * self, BLooper * looper);
status_t BApplication_UnregisterLooper(BApplication * self, BLooper * looper);
BApplication *BApplication_new1(const char * signature);
BApplication *BApplication_new2(const char * signature, status_t * error);
status_t BApplication_InitCheck(const BApplication * self);

// CLASS: BArchivable
BArchivable *BArchivable_dynamic_cast(BArchivable *ptr);
BArchivable *BArchivable_new();
BArchivable *BArchivable_new1(BMessage * from);
status_t BArchivable_AllArchived(const BArchivable * self, BMessage * archive);
status_t BArchivable_AllUnarchived(BArchivable * self, const BMessage * archive);
status_t BArchivable_Archive(const BArchivable * self, BMessage * into, bool deep);
status_t BArchivable_Perform(BArchivable * self, perform_code d, void * arg);
BArchivable * BArchivable_Instantiate(BMessage * archive);

} // extern "C"


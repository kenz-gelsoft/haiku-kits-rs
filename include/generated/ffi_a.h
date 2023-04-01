#pragma once

#include <None>

extern "C" {

// CLASS: BApplication
void BApplication_delete(BApplication *self);
BApplication *BApplication_new(BMessage * data);
BArchivable * BApplication_Instantiate(BMessage * data);
void BApplication_ReadyToRun(BApplication * self);
void BApplication_AppActivated(BApplication * self, bool active);
void BApplication_RefsReceived(BApplication * self, BMessage * message);
void BApplication_AboutRequested(BApplication * self);
void BApplication_Pulse(BApplication * self);
void BApplication_ShowCursor(BApplication * self);
void BApplication_HideCursor(BApplication * self);
void BApplication_ObscureCursor(BApplication * self);
bool BApplication_IsCursorHidden(const BApplication * self);
void BApplication_SetCursor(BApplication * self, const void * cursor);
void BApplication_SetCursor1(BApplication * self, const BCursor * cursor, bool sync);
bool BApplication_IsLaunching(const BApplication * self);
const char * BApplication_Signature(const BApplication * self);
BResources * BApplication_AppResources();
BApplication *BApplication_new1(const char * signature);
BApplication *BApplication_new2(const char * signature, status_t * error);

// CLASS: BArchivable
void BArchivable_delete(BArchivable *self);
BArchivable *BArchivable_new();
BArchivable *BArchivable_new1(BMessage * from);
BArchivable * BArchivable_Instantiate(BMessage * archive);

} // extern "C"

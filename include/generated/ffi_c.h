#pragma once

#include <Control.h>

extern "C" {

// CLASS: BControl
BControl *BControl_new(BMessage * data);
BArchivable * BControl_Instantiate(BMessage * data);
BControl *BControl_new1(BRect* frame, const char * name, const char * label, BMessage * message, uint32 resizing_mode, uint32 flags);
BControl *BControl_new2(const char * name, const char * label, BMessage * message, uint32 flags);
const BBitmap * BControl_IconBitmap(const BControl * self, uint32 which);
bool BControl_IsEnabled(const BControl * self);
const char *BControl_Label(const BControl * self);
void BControl_SetEnabled(BControl * self, bool enabled);
status_t BControl_SetIcon(BControl * self, const BBitmap * bitmap, uint32 flags);
status_t BControl_SetIconBitmap(BControl * self, const BBitmap * bitmap, uint32 which, uint32 flags);
void BControl_SetLabel(BControl * self, const char * string);
void BControl_SetValue(BControl * self, int32 value);
int32 BControl_Value(const BControl * self);
// Mix-in(s) to BControl
BInvoker *BControl_AsInvoker(BControl* obj);

} // extern "C"


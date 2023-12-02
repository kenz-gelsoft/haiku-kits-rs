#pragma once

#include <Button.h>

extern "C" {

// CLASS: BButton
BArchivable * BButton_Instantiate(BMessage * data);
BButton *BButton_new(BMessage * data);
BButton *BButton_new1(BRect* frame, const char * name, const char * label, BMessage * message, uint32 resizing_mode, uint32 flags);
BButton *BButton_new2(const char * label, BMessage * message);
BButton *BButton_new3(const char * name, const char * label, BMessage * message, uint32 flags);
bool BButton_IsDefault(const BButton * self);
bool BButton_IsFlat(const BButton * self);
void BButton_MakeDefault(BButton * self, bool flag);
BMessage * BButton_PopUpMessage(const BButton * self);
void BButton_SetFlat(BButton * self, bool flat);
void BButton_SetPopUpMessage(BButton * self, BMessage * message);
// Mix-in(s) to BButton
BInvoker *BButton_AsInvoker(BButton* obj);

} // extern "C"


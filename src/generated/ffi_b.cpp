#include "generated.h"

extern "C" {

// CLASS: BButton
BArchivable * BButton_Instantiate(BMessage * data) {
    return BButton::Instantiate(data);
}
BButton *BButton_new(BMessage * data) {
    return new BButton(data);
}
BButton *BButton_new1(BRect* frame, const char * name, const char * label, BMessage * message, uint32 resizing_mode, uint32 flags) {
    return new BButton(*frame, name, label, message, resizing_mode, flags);
}
BButton *BButton_new2(const char * label, BMessage * message) {
    return new BButton(label, message);
}
BButton *BButton_new3(const char * name, const char * label, BMessage * message, uint32 flags) {
    return new BButton(name, label, message, flags);
}
bool BButton_IsDefault(const BButton * self) {
    return self->IsDefault();
}
bool BButton_IsFlat(const BButton * self) {
    return self->IsFlat();
}
void BButton_MakeDefault(BButton * self, bool flag) {
    return self->MakeDefault(flag);
}
BMessage * BButton_PopUpMessage(const BButton * self) {
    return self->PopUpMessage();
}
void BButton_SetFlat(BButton * self, bool flat) {
    return self->SetFlat(flat);
}
void BButton_SetPopUpMessage(BButton * self, BMessage * message) {
    return self->SetPopUpMessage(message);
}
// Mix-in(s) to BButton
BInvoker *BButton_AsInvoker(BButton* obj) {
    return static_cast<BInvoker*>(obj);
}

} // extern "C"


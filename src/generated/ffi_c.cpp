#include "generated.h"

extern "C" {

// CLASS: BControl
BControl *BControl_dynamic_cast(BArchivable *ptr) {
    return dynamic_cast<BControl *>(ptr);
}
BControl *BControl_new(BMessage * data) {
    return new BControl(data);
}
BArchivable * BControl_Instantiate(BMessage * data) {
    return BControl::Instantiate(data);
}
BControl *BControl_new1(BRect* frame, const char * name, const char * label, BMessage * message, uint32 resizing_mode, uint32 flags) {
    return new BControl(*frame, name, label, message, resizing_mode, flags);
}
BControl *BControl_new2(const char * name, const char * label, BMessage * message, uint32 flags) {
    return new BControl(name, label, message, flags);
}
const BBitmap * BControl_IconBitmap(const BControl * self, uint32 which) {
    return self->IconBitmap(which);
}
bool BControl_IsEnabled(const BControl * self) {
    return self->IsEnabled();
}
const char *BControl_Label(const BControl * self) {
    return self->Label();
}
void BControl_SetEnabled(BControl * self, bool enabled) {
    return self->SetEnabled(enabled);
}
status_t BControl_SetIcon(BControl * self, const BBitmap * bitmap, uint32 flags) {
    return self->SetIcon(bitmap, flags);
}
status_t BControl_SetIconBitmap(BControl * self, const BBitmap * bitmap, uint32 which, uint32 flags) {
    return self->SetIconBitmap(bitmap, which, flags);
}
void BControl_SetLabel(BControl * self, const char * string) {
    return self->SetLabel(string);
}
void BControl_SetValue(BControl * self, int32 value) {
    return self->SetValue(value);
}
int32 BControl_Value(const BControl * self) {
    return self->Value();
}
// Mix-in(s) to BControl
BInvoker *BControl_AsInvoker(BControl* obj) {
    return static_cast<BInvoker*>(obj);
}

} // extern "C"


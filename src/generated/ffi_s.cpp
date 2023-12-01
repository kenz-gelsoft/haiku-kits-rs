#include "generated.h"

extern "C" {

// CLASS: BSize
void BSize_delete(BSize *self) {
    delete self;
}
float BSize_height(BSize * self) {
    return self->height;
}
float BSize_width(BSize * self) {
    return self->width;
}
BSize *BSize_new() {
    return new BSize();
}
BSize *BSize_new1(const BSize * other) {
    return new BSize(*other);
}
BSize *BSize_new2(float width, float height) {
    return new BSize(width, height);
}
int32 BSize_IntegerHeight(const BSize * self) {
    return self->IntegerHeight();
}
int32 BSize_IntegerWidth(const BSize * self) {
    return self->IntegerWidth();
}
bool BSize_IsHeightSet(const BSize * self) {
    return self->IsHeightSet();
}
bool BSize_IsWidthSet(const BSize * self) {
    return self->IsWidthSet();
}
void BSize_Set(BSize * self, float width, float height) {
    return self->Set(width, height);
}
void BSize_SetHeight(BSize * self, float height) {
    return self->SetHeight(height);
}
void BSize_SetWidth(BSize * self, float width) {
    return self->SetWidth(width);
}

} // extern "C"


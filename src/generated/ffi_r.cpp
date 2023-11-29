#include "generated.h"

extern "C" {

// CLASS: BRect
void BRect_delete(BRect *self) {
    delete self;
}
void BRect_InsetBy(BRect * self, BPoint* inset) {
    return self->InsetBy(*inset);
}
void BRect_InsetBy1(BRect * self, float dx, float dy) {
    return self->InsetBy(dx, dy);
}
BRect * BRect_InsetBySelf(BRect * self, BPoint* inset) {
    return &(self->InsetBySelf(*inset));
}
BRect * BRect_InsetBySelf1(BRect * self, float dx, float dy) {
    return &(self->InsetBySelf(dx, dy));
}
BRect *BRect_InsetByCopy(const BRect * self, BPoint* inset) {
    return new BRect(self->InsetByCopy(*inset));
}
BRect *BRect_InsetByCopy1(const BRect * self, float dx, float dy) {
    return new BRect(self->InsetByCopy(dx, dy));
}
void BRect_OffsetBy(BRect * self, BPoint* delta) {
    return self->OffsetBy(*delta);
}
void BRect_OffsetBy1(BRect * self, float dx, float dy) {
    return self->OffsetBy(dx, dy);
}
void BRect_OffsetTo(BRect * self, BPoint* offset) {
    return self->OffsetTo(*offset);
}
void BRect_OffsetTo1(BRect * self, float x, float y) {
    return self->OffsetTo(x, y);
}
BRect * BRect_OffsetBySelf(BRect * self, BPoint* offset) {
    return &(self->OffsetBySelf(*offset));
}
BRect * BRect_OffsetBySelf1(BRect * self, float dx, float dy) {
    return &(self->OffsetBySelf(dx, dy));
}
BRect *BRect_OffsetByCopy(const BRect * self, BPoint* offset) {
    return new BRect(self->OffsetByCopy(*offset));
}
BRect *BRect_OffsetByCopy1(const BRect * self, float dx, float dy) {
    return new BRect(self->OffsetByCopy(dx, dy));
}
BRect * BRect_OffsetToSelf(BRect * self, BPoint* offset) {
    return &(self->OffsetToSelf(*offset));
}
BRect * BRect_OffsetToSelf1(BRect * self, float x, float y) {
    return &(self->OffsetToSelf(x, y));
}
BRect *BRect_OffsetToCopy(const BRect * self, BPoint* offset) {
    return new BRect(self->OffsetToCopy(*offset));
}
BRect *BRect_OffsetToCopy1(const BRect * self, float x, float y) {
    return new BRect(self->OffsetToCopy(x, y));
}
BRect *BRect_new() {
    return new BRect();
}
BRect *BRect_new1(BPoint* left_top, BPoint* right_bottom) {
    return new BRect(*left_top, *right_bottom);
}
BRect *BRect_new2(BPoint* left_top, BSize* size) {
    return new BRect(*left_top, *size);
}
BRect *BRect_new3(const BRect * other) {
    return new BRect(*other);
}
BRect *BRect_new4(float left, float top, float right, float bottom) {
    return new BRect(left, top, right, bottom);
}
BRect *BRect_new5(float side) {
    return new BRect(side);
}
bool BRect_Contains(const BRect * self, BPoint* point) {
    return self->Contains(*point);
}
bool BRect_Contains1(const BRect * self, BRect* rect) {
    return self->Contains(*rect);
}
float BRect_Height(const BRect * self) {
    return self->Height();
}
int32 BRect_IntegerHeight(const BRect * self) {
    return self->IntegerHeight();
}
int32 BRect_IntegerWidth(const BRect * self) {
    return self->IntegerWidth();
}
bool BRect_Intersects(const BRect * self, BRect* rect) {
    return self->Intersects(*rect);
}
bool BRect_IsValid(const BRect * self) {
    return self->IsValid();
}
BPoint *BRect_LeftBottom(const BRect * self) {
    return new BPoint(self->LeftBottom());
}
BPoint *BRect_LeftTop(const BRect * self) {
    return new BPoint(self->LeftTop());
}
void BRect_PrintToStream(const BRect * self) {
    return self->PrintToStream();
}
BPoint *BRect_RightBottom(const BRect * self) {
    return new BPoint(self->RightBottom());
}
BPoint *BRect_RightTop(const BRect * self) {
    return new BPoint(self->RightTop());
}
void BRect_Set(BRect * self, float left, float top, float right, float bottom) {
    return self->Set(left, top, right, bottom);
}
void BRect_SetLeftBottom(BRect * self, const BPoint* point) {
    return self->SetLeftBottom(*point);
}
void BRect_SetLeftTop(BRect * self, const BPoint* point) {
    return self->SetLeftTop(*point);
}
void BRect_SetRightBottom(BRect * self, const BPoint* point) {
    return self->SetRightBottom(*point);
}
void BRect_SetRightTop(BRect * self, const BPoint* point) {
    return self->SetRightTop(*point);
}
BSize *BRect_Size(const BRect * self) {
    return new BSize(self->Size());
}
float BRect_Width(const BRect * self) {
    return self->Width();
}

} // extern "C"


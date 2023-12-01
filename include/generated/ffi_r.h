#pragma once

#include <Rect.h>

extern "C" {

// CLASS: BRect
void BRect_delete(BRect *self);
float BRect_bottom(BRect * self);
void BRect_set_bottom(BRect * self, float bottom);
float BRect_left(BRect * self);
void BRect_set_left(BRect * self, float left);
float BRect_right(BRect * self);
void BRect_set_right(BRect * self, float right);
float BRect_top(BRect * self);
void BRect_set_top(BRect * self, float top);
void BRect_InsetBy(BRect * self, BPoint* inset);
void BRect_InsetBy1(BRect * self, float dx, float dy);
BRect * BRect_InsetBySelf(BRect * self, BPoint* inset);
BRect * BRect_InsetBySelf1(BRect * self, float dx, float dy);
BRect *BRect_InsetByCopy(const BRect * self, BPoint* inset);
BRect *BRect_InsetByCopy1(const BRect * self, float dx, float dy);
void BRect_OffsetBy(BRect * self, BPoint* delta);
void BRect_OffsetBy1(BRect * self, float dx, float dy);
void BRect_OffsetTo(BRect * self, BPoint* offset);
void BRect_OffsetTo1(BRect * self, float x, float y);
BRect * BRect_OffsetBySelf(BRect * self, BPoint* offset);
BRect * BRect_OffsetBySelf1(BRect * self, float dx, float dy);
BRect *BRect_OffsetByCopy(const BRect * self, BPoint* offset);
BRect *BRect_OffsetByCopy1(const BRect * self, float dx, float dy);
BRect * BRect_OffsetToSelf(BRect * self, BPoint* offset);
BRect * BRect_OffsetToSelf1(BRect * self, float x, float y);
BRect *BRect_OffsetToCopy(const BRect * self, BPoint* offset);
BRect *BRect_OffsetToCopy1(const BRect * self, float x, float y);
BRect *BRect_new();
BRect *BRect_new1(BPoint* left_top, BPoint* right_bottom);
BRect *BRect_new2(BPoint* left_top, BSize* size);
BRect *BRect_new3(const BRect * other);
BRect *BRect_new4(float left, float top, float right, float bottom);
BRect *BRect_new5(float side);
bool BRect_Contains(const BRect * self, BPoint* point);
bool BRect_Contains1(const BRect * self, BRect* rect);
float BRect_Height(const BRect * self);
int32 BRect_IntegerHeight(const BRect * self);
int32 BRect_IntegerWidth(const BRect * self);
bool BRect_Intersects(const BRect * self, BRect* rect);
bool BRect_IsValid(const BRect * self);
BPoint *BRect_LeftBottom(const BRect * self);
BPoint *BRect_LeftTop(const BRect * self);
void BRect_PrintToStream(const BRect * self);
BPoint *BRect_RightBottom(const BRect * self);
BPoint *BRect_RightTop(const BRect * self);
void BRect_Set(BRect * self, float left, float top, float right, float bottom);
void BRect_SetLeftBottom(BRect * self, const BPoint* point);
void BRect_SetLeftTop(BRect * self, const BPoint* point);
void BRect_SetRightBottom(BRect * self, const BPoint* point);
void BRect_SetRightTop(BRect * self, const BPoint* point);
BSize *BRect_Size(const BRect * self);
float BRect_Width(const BRect * self);

} // extern "C"


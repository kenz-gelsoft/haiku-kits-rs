#pragma once

#include <Size.h>

extern "C" {

// CLASS: BSize
void BSize_delete(BSize *self);
BSize *BSize_new();
BSize *BSize_new1(const BSize * other);
BSize *BSize_new2(float width, float height);
float BSize_Height(const BSize * self);
int32 BSize_IntegerHeight(const BSize * self);
int32 BSize_IntegerWidth(const BSize * self);
bool BSize_IsHeightSet(const BSize * self);
bool BSize_IsWidthSet(const BSize * self);
void BSize_Set(BSize * self, float width, float height);
void BSize_SetHeight(BSize * self, float height);
void BSize_SetWidth(BSize * self, float width);
float BSize_Width(const BSize * self);

} // extern "C"


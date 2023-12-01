#pragma once

#include <Size.h>

extern "C" {

// CLASS: BSize
void BSize_delete(BSize *self);
float BSize_height(BSize * self);
void BSize_set_height(BSize * self, float height);
float BSize_width(BSize * self);
void BSize_set_width(BSize * self, float width);
BSize *BSize_new();
BSize *BSize_new1(const BSize * other);
BSize *BSize_new2(float width, float height);
int32 BSize_IntegerHeight(const BSize * self);
int32 BSize_IntegerWidth(const BSize * self);
bool BSize_IsHeightSet(const BSize * self);
bool BSize_IsWidthSet(const BSize * self);
void BSize_Set(BSize * self, float width, float height);

} // extern "C"


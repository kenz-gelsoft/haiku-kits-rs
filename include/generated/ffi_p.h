#pragma once

#include <Point.h>

extern "C" {

// CLASS: BPoint
void BPoint_delete(BPoint *self);
float BPoint_x(BPoint * self);
float BPoint_y(BPoint * self);
BPoint *BPoint_new();
BPoint *BPoint_new1(const BPoint * p);
BPoint *BPoint_new2(float x, float y);
void BPoint_ConstrainTo(BPoint * self, BRect* rect);
void BPoint_PrintToStream(const BPoint * self);
void BPoint_Set(BPoint * self, float x, float y);

} // extern "C"


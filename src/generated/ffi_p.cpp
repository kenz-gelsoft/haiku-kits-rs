#include "generated.h"

extern "C" {

// CLASS: BPoint
void BPoint_delete(BPoint *self) {
    delete self;
}
float BPoint_x(BPoint * self) {
    return self->x;
}
void BPoint_set_x(BPoint * self, float x) {
    self->x = x;
}
float BPoint_y(BPoint * self) {
    return self->y;
}
void BPoint_set_y(BPoint * self, float y) {
    self->y = y;
}
BPoint *BPoint_new() {
    return new BPoint();
}
BPoint *BPoint_new1(const BPoint * p) {
    return new BPoint(*p);
}
BPoint *BPoint_new2(float x, float y) {
    return new BPoint(x, y);
}
void BPoint_ConstrainTo(BPoint * self, BRect* rect) {
    return self->ConstrainTo(*rect);
}
void BPoint_PrintToStream(const BPoint * self) {
    return self->PrintToStream();
}
void BPoint_Set(BPoint * self, float x, float y) {
    return self->Set(x, y);
}

} // extern "C"


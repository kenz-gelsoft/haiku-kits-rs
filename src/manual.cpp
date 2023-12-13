#include "manual.h"

void BArchivable_delete(BArchivable *self) {
    delete self;
}

RustHandler *RustHandler_new(void *f, void *param, const char *name/*= NULL*/) {
    return new RustHandler(f, param, name);
}

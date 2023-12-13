#include "manual.h"

void BArchivable_delete(BArchivable *self) {
    delete self;
}

#if 0
int wxRustEntry(int *argc, ArgChar **argv) {
    return wxEntry(*argc, argv);
}
#endif

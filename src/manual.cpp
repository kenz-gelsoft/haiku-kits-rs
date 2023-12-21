#include "manual.h"

void BArchivable_delete(BArchivable *self) {
    delete self;
}

RustWindow *RustWindow_new(void *f, void *param, BRect frame, const char *title, window_type type, uint32 flags, uint32 workspace) {
    return new RustWindow(f, param, frame, title, type, flags, workspace);
}

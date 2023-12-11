#include "manual.h"

#if 0
// wxApp
wxIMPLEMENT_APP_NO_MAIN(App);

static CxxClosure<int> globalOnInit;
void AppSetOnInit(void *f, void *params) {
    globalOnInit = CxxClosure<int>(f, params);
}

bool App::OnInit() {
    globalOnInit(/*unused*/0);
    return true;
}
#endif

void BArchivable_delete(BArchivable *self) {
    delete self;
}

#if 0
int wxRustEntry(int *argc, ArgChar **argv) {
    return wxEntry(*argc, argv);
}
#endif

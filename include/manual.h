#pragma once
// TODO include haiku-api headers

#include <Archivable.h>
#include <Window.h>

// BWindow
template <typename T>
class CxxClosure {
    typedef void (*TrampolineFunc)(void *, T);
    void *mFn;
    void *mParam;

public:
    CxxClosure() : mFn(), mParam()
    {}
    CxxClosure(void *f, void *param) : mFn(f), mParam(param)
    {}

    void operator ()(T arg) const {
        if (mParam) { // if set
            ((TrampolineFunc)mFn)(mParam, arg);
        } else {
            // TODO: provide debug info
        }
    }
};

class CxxClosureVoid {
    CxxClosure<void*> mTyped;

public:
    CxxClosureVoid(void *f, void *param) : mTyped(f, param)
    {}

    void operator ()() const {
        mTyped(/*unused*/0);
    }
};

class RustWindow: public BWindow
{
    CxxClosure<void*> mOnMessageReceived;
public:
    RustWindow(void *f, void *param, BRect frame, const char *title, window_type type, uint32 flags, uint32 workspace = B_CURRENT_WORKSPACE) :
        BWindow(frame, title, type, flags, workspace),
        mOnMessageReceived(f, param)
    {}
    void MessageReceived(BMessage *message) {
        mOnMessageReceived(message);
    }
};

extern "C" {

void BArchivable_delete(BArchivable *self);

RustWindow *RustWindow_new(void *f, void *param, BRect frame, const char *title, window_type type, uint32 flags, uint32 workspace);

} // extern "C"

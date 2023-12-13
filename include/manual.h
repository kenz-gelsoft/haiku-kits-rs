#pragma once
// TODO include haiku-api headers

#include <Archivable.h>
#include <Handler.h>

// BHandler
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

class RustHandler: public BHandler
{
    CxxClosure<void*> mOnMessageReceived;
public:
    RustHandler(void *f, void *param, const char *name = NULL) : BHandler(name),
        mOnMessageReceived(f, param)
    {}
    void MessageReceived(BMessage *message) {
        mOnMessageReceived(message);
    }
};

extern "C" {

void BArchivable_delete(BArchivable *self);

RustHandler *RustHandler_new(void *f, void *param, const char *name = NULL);

} // extern "C"

#pragma once
// TODO include haiku-api headers

// wxEvtHandler
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

extern "C" {

// wxApp
void AppSetOnInit(void *aFn, void *aParam);
class App : public wxApp {
    virtual bool OnInit();
};

void wxObject_delete(wxObject *self);

// String
struct UTF8Data {
    const char *data;
    size_t length;
};
wxString *wxString_new(const unsigned char *psz, const size_t nLength);
void wxString_delete(wxString *self);
UTF8Data wxString_UTF8Data(wxString *self);

// WeakRef
void *OpaqueWeakRef_new(void *obj);
void *OpaqueWeakRef_copy(void *obj);
void OpaqueWeakRef_delete(void *self);
void *OpaqueWeakRef_Get(void *self);

#ifdef __WXMSW__
typedef wxChar ArgChar;
#else
typedef char ArgChar;
#endif // __WXMSW__
int wxRustEntry(int *argc, ArgChar **argv);

} // extern "C"

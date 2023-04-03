#include "manual.h"

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

void wxObject_delete(wxObject *self) {
    delete self;
}

// String
wxString *wxString_new(const unsigned char *psz, const size_t nLength) {
    return new wxString(psz, wxConvUTF8, nLength);
}
void wxString_delete(wxString *self) {
    delete self;
}
UTF8Data wxString_UTF8Data(wxString *self) {
    auto utf8 = self->ToUTF8();
    return {
        utf8.data(),
        utf8.length()
    };
}

class OpaqueWeakRef : public wxTrackerNode
{
public:
    OpaqueWeakRef(void *ptr) :
        mPtr(ptr)
    {
        AsTrackable()->AddNode(this);
    }
    virtual ~OpaqueWeakRef()
    {
        AsTrackable()->RemoveNode(this);
        mPtr = nullptr;
    }
    void *Get() const
    {
        return mPtr;
    }
    virtual void OnObjectDestroy()
    {
        mPtr = nullptr;
    }
private:
    wxTrackable *AsTrackable() const
    {
        // Casting to any(not true) class ptr to use dynamic_cast,
        wxObject *obj = reinterpret_cast<wxObject *>(mPtr);
        // we need to dynamic_cast to get correct wxTrackable's vtable.
        wxTrackable *trackable = dynamic_cast<wxTrackable *>(obj);
        wxASSERT(trackable);
        return trackable;
    }
    void *mPtr;
}; 

void *OpaqueWeakRef_new(void *obj) {
    return new OpaqueWeakRef(obj);
}
void *OpaqueWeakRef_copy(void *obj) {
    return new OpaqueWeakRef(OpaqueWeakRef_Get(obj));
}
void OpaqueWeakRef_delete(void *self) {
    OpaqueWeakRef *weakRef = reinterpret_cast<OpaqueWeakRef *>(self);
    delete weakRef;
}
void *OpaqueWeakRef_Get(void *self) {
    OpaqueWeakRef *weakRef = reinterpret_cast<OpaqueWeakRef *>(self);
    return weakRef->Get();
}

int wxRustEntry(int *argc, ArgChar **argv) {
    return wxEntry(*argc, argv);
}

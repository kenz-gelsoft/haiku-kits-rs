#include <wx/bookctrl.h>

#include "manual.h"


enum WxRustEvent {
};

#define MAP_RUST_EVT(name) case RUST_EVT_##name: return wxEVT_##name;
#define DEFINE_TYPE_TAG_OF_EVT(name, clazz) \
    template<> wxEventTypeTag<clazz> TypeTagOf(int eventType) { \
        switch (eventType) { \
        MAP_RUST_EVT(name) \
        } \
        return wxEVT_NULL; \
    }

template<typename T> wxEventTypeTag<T> TypeTagOf(int eventType) {
    return wxEVT_NULL;
}

template<typename T> void BindIfEventIs(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    wxEventTypeTag<T> typeTag = TypeTagOf<T>(eventType);
    if (typeTag != wxEVT_NULL) {
        CxxClosure<T &> functor(aFn, aParam);
        self->Bind(typeTag, functor);
    }
}
void wxEvtHandler_Bind(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
}

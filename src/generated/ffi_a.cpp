#include "generated.h"

extern "C" {

// CLASS: BApplication
void BApplication_delete(BApplication *self) {
    delete self;
}
BApplication *BApplication_new(BMessage * data) {
    return new BApplication(data);
}
BArchivable * BApplication_Instantiate(BMessage * data) {
    return BApplication::Instantiate(data);
}
void BApplication_ReadyToRun(BApplication * self) {
    return self->ReadyToRun();
}
void BApplication_ArgvReceived(BApplication * self, int32 argc, char ** argv) {
    return self->ArgvReceived(argc, argv);
}
void BApplication_AppActivated(BApplication * self, bool active) {
    return self->AppActivated(active);
}
void BApplication_RefsReceived(BApplication * self, BMessage * message) {
    return self->RefsReceived(message);
}
void BApplication_AboutRequested(BApplication * self) {
    return self->AboutRequested();
}
void BApplication_Pulse(BApplication * self) {
    return self->Pulse();
}
void BApplication_ShowCursor(BApplication * self) {
    return self->ShowCursor();
}
void BApplication_HideCursor(BApplication * self) {
    return self->HideCursor();
}
void BApplication_ObscureCursor(BApplication * self) {
    return self->ObscureCursor();
}
bool BApplication_IsCursorHidden(const BApplication * self) {
    return self->IsCursorHidden();
}
void BApplication_SetCursor(BApplication * self, const void * cursor) {
    return self->SetCursor(cursor);
}
void BApplication_SetCursor1(BApplication * self, const BCursor * cursor, bool sync) {
    return self->SetCursor(cursor, sync);
}
int32 BApplication_CountWindows(const BApplication * self) {
    return self->CountWindows();
}
BWindow * BApplication_WindowAt(const BApplication * self, int32 index) {
    return self->WindowAt(index);
}
int32 BApplication_CountLoopers(const BApplication * self) {
    return self->CountLoopers();
}
BLooper * BApplication_LooperAt(const BApplication * self, int32 index) {
    return self->LooperAt(index);
}
bool BApplication_IsLaunching(const BApplication * self) {
    return self->IsLaunching();
}
const char * BApplication_Signature(const BApplication * self) {
    return self->Signature();
}
BResources * BApplication_AppResources() {
    return BApplication::AppResources();
}
BApplication *BApplication_new1(const char * signature) {
    return new BApplication(signature);
}
BApplication *BApplication_new2(const char * signature, status_t * error) {
    return new BApplication(signature, error);
}

// CLASS: BArchivable
void BArchivable_delete(BArchivable *self) {
    delete self;
}
BArchivable *BArchivable_new() {
    return new BArchivable();
}
BArchivable *BArchivable_new1(BMessage * from) {
    return new BArchivable(from);
}
BArchivable * BArchivable_Instantiate(BMessage * archive) {
    return BArchivable::Instantiate(archive);
}

} // extern "C"


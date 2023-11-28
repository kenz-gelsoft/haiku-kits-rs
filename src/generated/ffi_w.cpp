#include "generated.h"

extern "C" {

// CLASS: BWindow
void BWindow_AddShortcut(BWindow * self, uint32 key, uint32 modifiers, BMessage * message) {
    return self->AddShortcut(key, modifiers, message);
}
void BWindow_AddShortcut1(BWindow * self, uint32 key, uint32 modifiers, BMessage * message, BHandler * target) {
    return self->AddShortcut(key, modifiers, message, target);
}
bool BWindow_HasShortcut(BWindow * self, uint32 key, uint32 modifiers) {
    return self->HasShortcut(key, modifiers);
}
void BWindow_RemoveShortcut(BWindow * self, uint32 key, uint32 modifiers) {
    return self->RemoveShortcut(key, modifiers);
}
BWindow *BWindow_new(BMessage * archive) {
    return new BWindow(archive);
}
void BWindow_Activate(BWindow * self, bool active) {
    return self->Activate(active);
}
void BWindow_AddChild(BWindow * self, BLayoutItem * child) {
    return self->AddChild(child);
}
void BWindow_AddChild1(BWindow * self, BView * child, BView * before) {
    return self->AddChild(child, before);
}
status_t BWindow_AddToSubset(BWindow * self, BWindow * window) {
    return self->AddToSubset(window);
}
void BWindow_BeginViewTransaction(BWindow * self) {
    return self->BeginViewTransaction();
}
void BWindow_CenterIn(BWindow * self, const BRect * rect) {
    return self->CenterIn(*rect);
}
void BWindow_CenterOnScreen(BWindow * self) {
    return self->CenterOnScreen();
}
BView * BWindow_ChildAt(const BWindow * self, int32 index) {
    return self->ChildAt(index);
}
void BWindow_Close(BWindow * self) {
    return self->Close();
}
void BWindow_ConvertFromScreen(const BWindow * self, BPoint * point) {
    return self->ConvertFromScreen(point);
}
void BWindow_ConvertFromScreen2(const BWindow * self, BRect * rect) {
    return self->ConvertFromScreen(rect);
}
void BWindow_ConvertToScreen(const BWindow * self, BPoint * point) {
    return self->ConvertToScreen(point);
}
void BWindow_ConvertToScreen2(const BWindow * self, BRect * rect) {
    return self->ConvertToScreen(rect);
}
int32 BWindow_CountChildren(const BWindow * self) {
    return self->CountChildren();
}
BView * BWindow_CurrentFocus(const BWindow * self) {
    return self->CurrentFocus();
}
BButton * BWindow_DefaultButton(const BWindow * self) {
    return self->DefaultButton();
}
void BWindow_DisableUpdates(BWindow * self) {
    return self->DisableUpdates();
}
void BWindow_EnableUpdates(BWindow * self) {
    return self->EnableUpdates();
}
void BWindow_EndViewTransaction(BWindow * self) {
    return self->EndViewTransaction();
}
BView * BWindow_FindView1(const BWindow * self, const char * view_name) {
    return self->FindView(view_name);
}
uint32 BWindow_Flags(const BWindow * self) {
    return self->Flags();
}
void BWindow_Flush(const BWindow * self) {
    return self->Flush();
}
void BWindow_FrameResized(BWindow * self, float new_width, float new_height) {
    return self->FrameResized(new_width, new_height);
}
status_t BWindow_GetDecoratorSettings(const BWindow * self, BMessage * settings) {
    return self->GetDecoratorSettings(settings);
}
BLayout * BWindow_GetLayout(const BWindow * self) {
    return self->GetLayout();
}
void BWindow_GetSizeLimits(BWindow * self, float * min_width, float * max_width, float * min_height, float * max_height) {
    return self->GetSizeLimits(min_width, max_width, min_height, max_height);
}
status_t BWindow_GetWindowAlignment(const BWindow * self, window_alignment * mode, int32 * h, int32 * h_offset, int32 * width, int32 * width_offset, int32 * v, int32 * v_offset, int32 * height, int32 * height_offset) {
    return self->GetWindowAlignment(mode, h, h_offset, width, width_offset, v, v_offset, height, height_offset);
}
void BWindow_Hide(BWindow * self) {
    return self->Hide();
}
void BWindow_InvalidateLayout(BWindow * self, bool descendants) {
    return self->InvalidateLayout(descendants);
}
bool BWindow_InViewTransaction(const BWindow * self) {
    return self->InViewTransaction();
}
bool BWindow_IsActive(const BWindow * self) {
    return self->IsActive();
}
bool BWindow_IsFloating(const BWindow * self) {
    return self->IsFloating();
}
bool BWindow_IsFront(const BWindow * self) {
    return self->IsFront();
}
bool BWindow_IsHidden(const BWindow * self) {
    return self->IsHidden();
}
bool BWindow_IsMinimized(const BWindow * self) {
    return self->IsMinimized();
}
bool BWindow_IsModal(const BWindow * self) {
    return self->IsModal();
}
bool BWindow_IsOffscreenWindow(const BWindow * self) {
    return self->IsOffscreenWindow();
}
BMenuBar * BWindow_KeyMenuBar(const BWindow * self) {
    return self->KeyMenuBar();
}
BView * BWindow_LastMouseMovedView(const BWindow * self) {
    return self->LastMouseMovedView();
}
void BWindow_Layout(BWindow * self, bool force) {
    return self->Layout(force);
}
void BWindow_MenusBeginning(BWindow * self) {
    return self->MenusBeginning();
}
void BWindow_MenusEnded(BWindow * self) {
    return self->MenusEnded();
}
void BWindow_Minimize(BWindow * self, bool minimize) {
    return self->Minimize(minimize);
}
void BWindow_MoveBy(BWindow * self, float dx, float dy) {
    return self->MoveBy(dx, dy);
}
void BWindow_MoveOnScreen(BWindow * self, uint32 flags) {
    return self->MoveOnScreen(flags);
}
void BWindow_MoveTo1(BWindow * self, float x, float y) {
    return self->MoveTo(x, y);
}
bool BWindow_NeedsUpdate(const BWindow * self) {
    return self->NeedsUpdate();
}
bigtime_t BWindow_PulseRate(const BWindow * self) {
    return self->PulseRate();
}
bool BWindow_RemoveChild(BWindow * self, BView * child) {
    return self->RemoveChild(child);
}
status_t BWindow_RemoveFromSubset(BWindow * self, BWindow * window) {
    return self->RemoveFromSubset(window);
}
void BWindow_ResizeBy(BWindow * self, float dx, float dy) {
    return self->ResizeBy(dx, dy);
}
void BWindow_ResizeTo(BWindow * self, float width, float height) {
    return self->ResizeTo(width, height);
}
void BWindow_ResizeToPreferred(BWindow * self) {
    return self->ResizeToPreferred();
}
status_t BWindow_SendBehind(BWindow * self, const BWindow * window) {
    return self->SendBehind(window);
}
status_t BWindow_SetDecoratorSettings(BWindow * self, const BMessage * settings) {
    return self->SetDecoratorSettings(*settings);
}
void BWindow_SetDefaultButton(BWindow * self, BButton * button) {
    return self->SetDefaultButton(button);
}
status_t BWindow_SetFlags(BWindow * self, uint32 flags) {
    return self->SetFlags(flags);
}
void BWindow_SetKeyMenuBar(BWindow * self, BMenuBar * bar) {
    return self->SetKeyMenuBar(bar);
}
void BWindow_SetLayout(BWindow * self, BLayout * layout) {
    return self->SetLayout(layout);
}
void BWindow_SetPulseRate(BWindow * self, bigtime_t rate) {
    return self->SetPulseRate(rate);
}
void BWindow_SetSizeLimits(BWindow * self, float min_width, float max_width, float min_height, float max_height) {
    return self->SetSizeLimits(min_width, max_width, min_height, max_height);
}
void BWindow_SetTitle(BWindow * self, const char * title) {
    return self->SetTitle(title);
}
void BWindow_SetWorkspaces(BWindow * self, uint32 workspaces) {
    return self->SetWorkspaces(workspaces);
}
void BWindow_SetZoomLimits(BWindow * self, float max_width, float max_height) {
    return self->SetZoomLimits(max_width, max_height);
}
void BWindow_Show(BWindow * self) {
    return self->Show();
}
void BWindow_Sync(const BWindow * self) {
    return self->Sync();
}
const char *BWindow_Title(const BWindow * self) {
    return self->Title();
}
void BWindow_UpdateIfNeeded(BWindow * self) {
    return self->UpdateIfNeeded();
}
void BWindow_UpdateSizeLimits(BWindow * self) {
    return self->UpdateSizeLimits();
}
void BWindow_WindowActivated(BWindow * self, bool focus) {
    return self->WindowActivated(focus);
}
void BWindow_WorkspaceActivated(BWindow * self, int32 workspace, bool state) {
    return self->WorkspaceActivated(workspace, state);
}
uint32 BWindow_Workspaces(const BWindow * self) {
    return self->Workspaces();
}
void BWindow_WorkspacesChanged(BWindow * self, uint32 old_workspaces, uint32 new_workspaces) {
    return self->WorkspacesChanged(old_workspaces, new_workspaces);
}
void BWindow_Zoom(BWindow * self) {
    return self->Zoom();
}
BArchivable * BWindow_Instantiate(BMessage * archive) {
    return BWindow::Instantiate(archive);
}

} // extern "C"


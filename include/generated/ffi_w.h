#pragma once

#include <Window.h>

extern "C" {

// CLASS: BWindow
void BWindow_AddShortcut(BWindow * self, uint32 key, uint32 modifiers, BMessage * message);
void BWindow_AddShortcut1(BWindow * self, uint32 key, uint32 modifiers, BMessage * message, BHandler * target);
bool BWindow_HasShortcut(BWindow * self, uint32 key, uint32 modifiers);
void BWindow_RemoveShortcut(BWindow * self, uint32 key, uint32 modifiers);
BWindow *BWindow_new(BMessage * archive);
BWindow *BWindow_new1(BRect* frame, const char * title, window_look look, window_feel feel, uint32 flags, uint32 workspace);
BWindow *BWindow_new2(BRect* frame, const char * title, window_type type_, uint32 flags, uint32 workspace);
void BWindow_Activate(BWindow * self, bool active);
void BWindow_AddChild(BWindow * self, BLayoutItem * child);
void BWindow_AddChild1(BWindow * self, BView * child, BView * before);
status_t BWindow_AddToSubset(BWindow * self, BWindow * window);
void BWindow_BeginViewTransaction(BWindow * self);
BRect *BWindow_Bounds(const BWindow * self);
void BWindow_CenterIn(BWindow * self, const BRect * rect);
void BWindow_CenterOnScreen(BWindow * self);
void BWindow_CenterOnScreen1(BWindow * self, screen_id id);
BView * BWindow_ChildAt(const BWindow * self, int32 index);
void BWindow_Close(BWindow * self);
void BWindow_ConvertFromScreen(const BWindow * self, BPoint * point);
void BWindow_ConvertFromScreen2(const BWindow * self, BRect * rect);
void BWindow_ConvertToScreen(const BWindow * self, BPoint * point);
void BWindow_ConvertToScreen2(const BWindow * self, BRect * rect);
int32 BWindow_CountChildren(const BWindow * self);
BView * BWindow_CurrentFocus(const BWindow * self);
BRect *BWindow_DecoratorFrame(const BWindow * self);
BButton * BWindow_DefaultButton(const BWindow * self);
void BWindow_DisableUpdates(BWindow * self);
void BWindow_EnableUpdates(BWindow * self);
void BWindow_EndViewTransaction(BWindow * self);
window_feel BWindow_Feel(const BWindow * self);
BView * BWindow_FindView(const BWindow * self, BPoint* point);
BView * BWindow_FindView1(const BWindow * self, const char * view_name);
uint32 BWindow_Flags(const BWindow * self);
void BWindow_Flush(const BWindow * self);
BRect *BWindow_Frame(const BWindow * self);
void BWindow_FrameMoved(BWindow * self, BPoint* new_position);
void BWindow_FrameResized(BWindow * self, float new_width, float new_height);
status_t BWindow_GetDecoratorSettings(const BWindow * self, BMessage * settings);
BLayout * BWindow_GetLayout(const BWindow * self);
void BWindow_GetSizeLimits(BWindow * self, float * min_width, float * max_width, float * min_height, float * max_height);
status_t BWindow_GetWindowAlignment(const BWindow * self, window_alignment * mode, int32 * h, int32 * h_offset, int32 * width, int32 * width_offset, int32 * v, int32 * v_offset, int32 * height, int32 * height_offset);
void BWindow_Hide(BWindow * self);
void BWindow_InvalidateLayout(BWindow * self, bool descendants);
bool BWindow_InViewTransaction(const BWindow * self);
bool BWindow_IsActive(const BWindow * self);
bool BWindow_IsFloating(const BWindow * self);
bool BWindow_IsFront(const BWindow * self);
bool BWindow_IsHidden(const BWindow * self);
bool BWindow_IsMinimized(const BWindow * self);
bool BWindow_IsModal(const BWindow * self);
bool BWindow_IsOffscreenWindow(const BWindow * self);
BMenuBar * BWindow_KeyMenuBar(const BWindow * self);
BView * BWindow_LastMouseMovedView(const BWindow * self);
void BWindow_Layout(BWindow * self, bool force);
window_look BWindow_Look(const BWindow * self);
void BWindow_MenusBeginning(BWindow * self);
void BWindow_MenusEnded(BWindow * self);
void BWindow_Minimize(BWindow * self, bool minimize);
void BWindow_MoveBy(BWindow * self, float dx, float dy);
void BWindow_MoveOnScreen(BWindow * self, uint32 flags);
void BWindow_MoveTo(BWindow * self, BPoint* point);
void BWindow_MoveTo1(BWindow * self, float x, float y);
bool BWindow_NeedsUpdate(const BWindow * self);
bigtime_t BWindow_PulseRate(const BWindow * self);
bool BWindow_RemoveChild(BWindow * self, BView * child);
status_t BWindow_RemoveFromSubset(BWindow * self, BWindow * window);
void BWindow_ResizeBy(BWindow * self, float dx, float dy);
void BWindow_ResizeTo(BWindow * self, float width, float height);
void BWindow_ResizeToPreferred(BWindow * self);
status_t BWindow_SendBehind(BWindow * self, const BWindow * window);
status_t BWindow_SetDecoratorSettings(BWindow * self, const BMessage * settings);
void BWindow_SetDefaultButton(BWindow * self, BButton * button);
status_t BWindow_SetFeel(BWindow * self, window_feel feel);
status_t BWindow_SetFlags(BWindow * self, uint32 flags);
void BWindow_SetKeyMenuBar(BWindow * self, BMenuBar * bar);
void BWindow_SetLayout(BWindow * self, BLayout * layout);
status_t BWindow_SetLook(BWindow * self, window_look look);
void BWindow_SetPulseRate(BWindow * self, bigtime_t rate);
void BWindow_SetSizeLimits(BWindow * self, float min_width, float max_width, float min_height, float max_height);
void BWindow_SetTitle(BWindow * self, const char * title);
status_t BWindow_SetType(BWindow * self, window_type type_);
status_t BWindow_SetWindowAlignment(BWindow * self, window_alignment mode, int32 h, int32 h_offset, int32 width, int32 width_offset, int32 v, int32 v_offset, int32 height, int32 height_offset);
void BWindow_SetWorkspaces(BWindow * self, uint32 workspaces);
void BWindow_SetZoomLimits(BWindow * self, float max_width, float max_height);
void BWindow_Show(BWindow * self);
BSize *BWindow_Size(const BWindow * self);
void BWindow_Sync(const BWindow * self);
const char *BWindow_Title(const BWindow * self);
window_type BWindow_Type(const BWindow * self);
void BWindow_UpdateIfNeeded(BWindow * self);
void BWindow_UpdateSizeLimits(BWindow * self);
void BWindow_WindowActivated(BWindow * self, bool focus);
void BWindow_WorkspaceActivated(BWindow * self, int32 workspace, bool state);
uint32 BWindow_Workspaces(const BWindow * self);
void BWindow_WorkspacesChanged(BWindow * self, uint32 old_workspaces, uint32 new_workspaces);
void BWindow_Zoom(BWindow * self);
void BWindow_Zoom1(BWindow * self, BPoint* origin, float width, float height);
BArchivable * BWindow_Instantiate(BMessage * archive);

} // extern "C"


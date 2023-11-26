#pragma once

#include <Window.h>

extern "C" {

// CLASS: BWindow
void BWindow_AddShortcut(BWindow * self, uint32 key, uint32 modifiers, BMessage * message);
void BWindow_AddShortcut1(BWindow * self, uint32 key, uint32 modifiers, BMessage * message, BHandler * target);
bool BWindow_HasShortcut(BWindow * self, uint32 key, uint32 modifiers);
void BWindow_RemoveShortcut(BWindow * self, uint32 key, uint32 modifiers);
BWindow *BWindow_new(BMessage * archive);
void BWindow_Activate(BWindow * self, bool active);
void BWindow_AddChild(BWindow * self, BLayoutItem * child);
void BWindow_AddChild1(BWindow * self, BView * child, BView * before);
status_t BWindow_AddToSubset(BWindow * self, BWindow * window);
void BWindow_BeginViewTransaction(BWindow * self);
void BWindow_CenterIn(BWindow * self, const BRect * rect);
void BWindow_CenterOnScreen(BWindow * self);
BView * BWindow_ChildAt(const BWindow * self, int32 index);
void BWindow_Close(BWindow * self);
void BWindow_ConvertFromScreen(const BWindow * self, BPoint * point);
void BWindow_ConvertFromScreen2(const BWindow * self, BRect * rect);
void BWindow_ConvertToScreen(const BWindow * self, BPoint * point);
void BWindow_ConvertToScreen2(const BWindow * self, BRect * rect);
int32 BWindow_CountChildren(const BWindow * self);
BView * BWindow_CurrentFocus(const BWindow * self);
BButton * BWindow_DefaultButton(const BWindow * self);
void BWindow_DisableUpdates(BWindow * self);
void BWindow_EnableUpdates(BWindow * self);
void BWindow_EndViewTransaction(BWindow * self);
BView * BWindow_FindView1(const BWindow * self, const char * view_name);
uint32 BWindow_Flags(const BWindow * self);
void BWindow_Flush(const BWindow * self);
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
void BWindow_MenusBeginning(BWindow * self);
void BWindow_MenusEnded(BWindow * self);
void BWindow_Minimize(BWindow * self, bool minimize);
void BWindow_MoveOnScreen(BWindow * self, uint32 flags);
bool BWindow_NeedsUpdate(const BWindow * self);
bigtime_t BWindow_PulseRate(const BWindow * self);
bool BWindow_RemoveChild(BWindow * self, BView * child);
status_t BWindow_RemoveFromSubset(BWindow * self, BWindow * window);
void BWindow_ResizeToPreferred(BWindow * self);
status_t BWindow_SendBehind(BWindow * self, const BWindow * window);
status_t BWindow_SetDecoratorSettings(BWindow * self, const BMessage * settings);
void BWindow_SetDefaultButton(BWindow * self, BButton * button);
status_t BWindow_SetFlags(BWindow * self, uint32 flags);
void BWindow_SetKeyMenuBar(BWindow * self, BMenuBar * bar);
void BWindow_SetLayout(BWindow * self, BLayout * layout);
void BWindow_SetPulseRate(BWindow * self, bigtime_t rate);
void BWindow_SetTitle(BWindow * self, const char * title);
void BWindow_SetWorkspaces(BWindow * self, uint32 workspaces);
void BWindow_Show(BWindow * self);
void BWindow_Sync(const BWindow * self);
const char *BWindow_Title(const BWindow * self);
void BWindow_UpdateIfNeeded(BWindow * self);
void BWindow_UpdateSizeLimits(BWindow * self);
void BWindow_WindowActivated(BWindow * self, bool focus);
void BWindow_WorkspaceActivated(BWindow * self, int32 workspace, bool state);
uint32 BWindow_Workspaces(const BWindow * self);
void BWindow_WorkspacesChanged(BWindow * self, uint32 old_workspaces, uint32 new_workspaces);
void BWindow_Zoom(BWindow * self);
BArchivable * BWindow_Instantiate(BMessage * archive);

} // extern "C"


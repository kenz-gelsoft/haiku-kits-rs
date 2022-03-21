#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::os::raw::{c_char, c_void};
use std::pin::Pin;
use std::ptr;

use crate::WxString;
use crate::macros::wx_class;

// any pointer type used on ffi boundary.
// we chose this type as it's handy in cxx.
type UnsafeAnyPtr = *const c_char;

mod ffi {
    use std::os::raw::c_void;
    extern "C" {
        // CLASS: wxObject
        pub fn wxObject_new() -> *mut c_void;
        pub fn wxObject_new1(other: *const c_void) -> *mut c_void;
        pub fn wxObject_GetClassInfo(self_: *const c_void) -> *mut c_void;
        pub fn wxObject_GetRefData(self_: *const c_void) -> *mut c_void;
        pub fn wxObject_IsKindOf(self_: *const c_void, info: *const c_void) -> bool;
        pub fn wxObject_IsSameAs(self_: *const c_void, obj: *const c_void) -> bool;
        pub fn wxObject_Ref(self_: *mut c_void, clone: *const c_void);
        pub fn wxObject_SetRefData(self_: *mut c_void, data: *mut c_void);
        pub fn wxObject_UnRef(self_: *mut c_void);
        pub fn wxObject_UnShare(self_: *mut c_void);
        // CLASS: wxEvtHandler
        pub fn wxEvtHandler_QueueEvent(self_: *mut c_void, event: *mut c_void);
        pub fn wxEvtHandler_AddPendingEvent(self_: *mut c_void, event: *const c_void);
        pub fn wxEvtHandler_ProcessEvent(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_ProcessEventLocally(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_SafelyProcessEvent(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_ProcessPendingEvents(self_: *mut c_void);
        pub fn wxEvtHandler_DeletePendingEvents(self_: *mut c_void);
        pub fn wxEvtHandler_GetClientObject(self_: *const c_void) -> *mut c_void;
        pub fn wxEvtHandler_SetClientObject(self_: *mut c_void, data: *mut c_void);
        pub fn wxEvtHandler_GetEvtHandlerEnabled(self_: *const c_void) -> bool;
        pub fn wxEvtHandler_GetNextHandler(self_: *const c_void) -> *mut c_void;
        pub fn wxEvtHandler_GetPreviousHandler(self_: *const c_void) -> *mut c_void;
        pub fn wxEvtHandler_SetEvtHandlerEnabled(self_: *mut c_void, enabled: bool);
        pub fn wxEvtHandler_SetNextHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxEvtHandler_SetPreviousHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxEvtHandler_Unlink(self_: *mut c_void);
        pub fn wxEvtHandler_IsUnlinked(self_: *const c_void) -> bool;
        pub fn wxEvtHandler_AddFilter(filter: *mut c_void);
        pub fn wxEvtHandler_RemoveFilter(filter: *mut c_void);
        pub fn wxEvtHandler_new() -> *mut c_void;
        // CLASS: wxWindow
        pub fn wxWindow_AcceptsFocus(self_: *const c_void) -> bool;
        pub fn wxWindow_AcceptsFocusFromKeyboard(self_: *const c_void) -> bool;
        pub fn wxWindow_AcceptsFocusRecursively(self_: *const c_void) -> bool;
        pub fn wxWindow_DisableFocusFromKeyboard(self_: *mut c_void);
        pub fn wxWindow_IsFocusable(self_: *const c_void) -> bool;
        pub fn wxWindow_CanAcceptFocus(self_: *const c_void) -> bool;
        pub fn wxWindow_CanAcceptFocusFromKeyboard(self_: *const c_void) -> bool;
        pub fn wxWindow_HasFocus(self_: *const c_void) -> bool;
        pub fn wxWindow_SetCanFocus(self_: *mut c_void, can_focus: bool);
        pub fn wxWindow_EnableVisibleFocus(self_: *mut c_void, enable: bool);
        pub fn wxWindow_SetFocus(self_: *mut c_void);
        pub fn wxWindow_SetFocusFromKbd(self_: *mut c_void);
        pub fn wxWindow_AddChild(self_: *mut c_void, child: *mut c_void);
        pub fn wxWindow_DestroyChildren(self_: *mut c_void) -> bool;
        pub fn wxWindow_FindWindow(self_: *const c_void, id: i32) -> *mut c_void;
        pub fn wxWindow_FindWindow1(self_: *const c_void, name: *const c_void) -> *mut c_void;
        pub fn wxWindow_RemoveChild(self_: *mut c_void, child: *mut c_void);
        pub fn wxWindow_GetGrandParent(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetNextSibling(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetParent(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetPrevSibling(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_IsDescendant(self_: *const c_void, win: *mut c_void) -> bool;
        pub fn wxWindow_Reparent(self_: *mut c_void, new_parent: *mut c_void) -> bool;
        pub fn wxWindow_AlwaysShowScrollbars(self_: *mut c_void, hflag: bool, vflag: bool);
        pub fn wxWindow_GetScrollPos(self_: *const c_void, orientation: i32) -> i32;
        pub fn wxWindow_GetScrollRange(self_: *const c_void, orientation: i32) -> i32;
        pub fn wxWindow_GetScrollThumb(self_: *const c_void, orientation: i32) -> i32;
        pub fn wxWindow_CanScroll(self_: *const c_void, orient: i32) -> bool;
        pub fn wxWindow_HasScrollbar(self_: *const c_void, orient: i32) -> bool;
        pub fn wxWindow_IsScrollbarAlwaysShown(self_: *const c_void, orient: i32) -> bool;
        pub fn wxWindow_ScrollLines(self_: *mut c_void, lines: i32) -> bool;
        pub fn wxWindow_ScrollPages(self_: *mut c_void, pages: i32) -> bool;
        pub fn wxWindow_ScrollWindow(self_: *mut c_void, dx: i32, dy: i32, rect: *const c_void);
        pub fn wxWindow_LineUp(self_: *mut c_void) -> bool;
        pub fn wxWindow_LineDown(self_: *mut c_void) -> bool;
        pub fn wxWindow_PageUp(self_: *mut c_void) -> bool;
        pub fn wxWindow_PageDown(self_: *mut c_void) -> bool;
        pub fn wxWindow_SetScrollPos(self_: *mut c_void, orientation: i32, pos: i32, refresh: bool);
        pub fn wxWindow_SetScrollbar(self_: *mut c_void, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool);
        pub fn wxWindow_BeginRepositioningChildren(self_: *mut c_void) -> bool;
        pub fn wxWindow_EndRepositioningChildren(self_: *mut c_void);
        pub fn wxWindow_CacheBestSize(self_: *const c_void, size: *const c_void);
        pub fn wxWindow_ClientToWindowSize(self_: *const c_void, size: *const c_void) -> *mut c_void;
        pub fn wxWindow_WindowToClientSize(self_: *const c_void, size: *const c_void) -> *mut c_void;
        pub fn wxWindow_Fit(self_: *mut c_void);
        pub fn wxWindow_FitInside(self_: *mut c_void);
        pub fn wxWindow_FromDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP2(self_: *const c_void, d: i32) -> i32;
        pub fn wxWindow_ToDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP2(self_: *const c_void, d: i32) -> i32;
        pub fn wxWindow_GetBestSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetBestHeight(self_: *const c_void, width: i32) -> i32;
        pub fn wxWindow_GetBestWidth(self_: *const c_void, height: i32) -> i32;
        pub fn wxWindow_GetClientSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
        pub fn wxWindow_GetClientSize1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetEffectiveMinSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMaxClientSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMaxSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMinClientSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMinSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMinWidth(self_: *const c_void) -> i32;
        pub fn wxWindow_GetMinHeight(self_: *const c_void) -> i32;
        pub fn wxWindow_GetMaxWidth(self_: *const c_void) -> i32;
        pub fn wxWindow_GetMaxHeight(self_: *const c_void) -> i32;
        pub fn wxWindow_GetSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
        pub fn wxWindow_GetSize1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetVirtualSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetVirtualSize1(self_: *const c_void, width: *mut c_void, height: *mut c_void);
        pub fn wxWindow_GetBestVirtualSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetContentScaleFactor(self_: *const c_void) -> f64;
        pub fn wxWindow_GetDPIScaleFactor(self_: *const c_void) -> f64;
        pub fn wxWindow_GetWindowBorderSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_InformFirstDirection(self_: *mut c_void, direction: i32, size: i32, available_other_dir: i32) -> bool;
        pub fn wxWindow_InvalidateBestSize(self_: *mut c_void);
        pub fn wxWindow_PostSizeEvent(self_: *mut c_void);
        pub fn wxWindow_PostSizeEventToParent(self_: *mut c_void);
        pub fn wxWindow_SendSizeEvent(self_: *mut c_void, flags: i32);
        pub fn wxWindow_SendSizeEventToParent(self_: *mut c_void, flags: i32);
        pub fn wxWindow_SetClientSize(self_: *mut c_void, width: i32, height: i32);
        pub fn wxWindow_SetClientSize1(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetClientSize2(self_: *mut c_void, rect: *const c_void);
        pub fn wxWindow_SetContainingSizer(self_: *mut c_void, sizer: *mut c_void);
        pub fn wxWindow_SetInitialSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMaxClientSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMaxSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMinClientSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMinSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetSize(self_: *mut c_void, x: i32, y: i32, width: i32, height: i32, size_flags: i32);
        pub fn wxWindow_SetSize1(self_: *mut c_void, rect: *const c_void);
        pub fn wxWindow_SetSize2(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetSize3(self_: *mut c_void, width: i32, height: i32);
        pub fn wxWindow_SetSizeHints(self_: *mut c_void, min_size: *const c_void, max_size: *const c_void, inc_size: *const c_void);
        pub fn wxWindow_SetSizeHints1(self_: *mut c_void, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        pub fn wxWindow_SetVirtualSize(self_: *mut c_void, width: i32, height: i32);
        pub fn wxWindow_SetVirtualSize1(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_FromDIP3(sz: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP4(pt: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP5(d: i32, w: *const c_void) -> i32;
        pub fn wxWindow_ToDIP3(sz: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP4(pt: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP5(d: i32, w: *const c_void) -> i32;
        pub fn wxWindow_Center(self_: *mut c_void, dir: i32);
        pub fn wxWindow_CenterOnParent(self_: *mut c_void, dir: i32);
        pub fn wxWindow_Centre(self_: *mut c_void, direction: i32);
        pub fn wxWindow_CentreOnParent(self_: *mut c_void, direction: i32);
        pub fn wxWindow_GetPosition(self_: *const c_void, x: *mut c_void, y: *mut c_void);
        pub fn wxWindow_GetPosition1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetScreenPosition(self_: *const c_void, x: *mut c_void, y: *mut c_void);
        pub fn wxWindow_GetScreenPosition1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetScreenRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetClientAreaOrigin(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetClientRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_Move(self_: *mut c_void, x: i32, y: i32, flags: i32);
        pub fn wxWindow_Move1(self_: *mut c_void, pt: *const c_void, flags: i32);
        pub fn wxWindow_SetPosition(self_: *mut c_void, pt: *const c_void);
        pub fn wxWindow_ClientToScreen(self_: *const c_void, x: *mut c_void, y: *mut c_void);
        pub fn wxWindow_ClientToScreen1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_ConvertDialogToPixels(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_ConvertDialogToPixels1(self_: *const c_void, sz: *const c_void) -> *mut c_void;
        pub fn wxWindow_ConvertPixelsToDialog(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_ConvertPixelsToDialog1(self_: *const c_void, sz: *const c_void) -> *mut c_void;
        pub fn wxWindow_ScreenToClient(self_: *const c_void, x: *mut c_void, y: *mut c_void);
        pub fn wxWindow_ScreenToClient1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_ClearBackground(self_: *mut c_void);
        pub fn wxWindow_Freeze(self_: *mut c_void);
        pub fn wxWindow_Thaw(self_: *mut c_void);
        pub fn wxWindow_IsFrozen(self_: *const c_void) -> bool;
        pub fn wxWindow_GetCharHeight(self_: *const c_void) -> i32;
        pub fn wxWindow_GetCharWidth(self_: *const c_void) -> i32;
        pub fn wxWindow_GetDPI(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetTextExtent(self_: *const c_void, string: *const c_void, w: *mut c_void, h: *mut c_void, descent: *mut c_void, external_leading: *mut c_void, font: *const c_void);
        pub fn wxWindow_GetTextExtent1(self_: *const c_void, string: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetUpdateClientRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_HasTransparentBackground(self_: *mut c_void) -> bool;
        pub fn wxWindow_Refresh(self_: *mut c_void, erase_background: bool, rect: *const c_void);
        pub fn wxWindow_RefreshRect(self_: *mut c_void, rect: *const c_void, erase_background: bool);
        pub fn wxWindow_Update(self_: *mut c_void);
        pub fn wxWindow_SetBackgroundColour(self_: *mut c_void, colour: *const c_void) -> bool;
        pub fn wxWindow_IsTransparentBackgroundSupported(self_: *const c_void, reason: *mut c_void) -> bool;
        pub fn wxWindow_SetFont(self_: *mut c_void, font: *const c_void) -> bool;
        pub fn wxWindow_SetForegroundColour(self_: *mut c_void, colour: *const c_void) -> bool;
        pub fn wxWindow_SetOwnBackgroundColour(self_: *mut c_void, colour: *const c_void);
        pub fn wxWindow_InheritsBackgroundColour(self_: *const c_void) -> bool;
        pub fn wxWindow_UseBgCol(self_: *const c_void) -> bool;
        pub fn wxWindow_UseBackgroundColour(self_: *const c_void) -> bool;
        pub fn wxWindow_SetOwnFont(self_: *mut c_void, font: *const c_void);
        pub fn wxWindow_SetOwnForegroundColour(self_: *mut c_void, colour: *const c_void);
        pub fn wxWindow_UseForegroundColour(self_: *const c_void) -> bool;
        pub fn wxWindow_InheritsForegroundColour(self_: *const c_void) -> bool;
        pub fn wxWindow_SetPalette(self_: *mut c_void, pal: *const c_void);
        pub fn wxWindow_ShouldInheritColours(self_: *const c_void) -> bool;
        pub fn wxWindow_SetThemeEnabled(self_: *mut c_void, enable: bool);
        pub fn wxWindow_GetThemeEnabled(self_: *const c_void) -> bool;
        pub fn wxWindow_CanSetTransparent(self_: *mut c_void) -> bool;
        pub fn wxWindow_SetTransparent(self_: *mut c_void, alpha: u8) -> bool;
        pub fn wxWindow_GetEventHandler(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_HandleAsNavigationKey(self_: *mut c_void, event: *const c_void) -> bool;
        pub fn wxWindow_HandleWindowEvent(self_: *const c_void, event: *mut c_void) -> bool;
        pub fn wxWindow_ProcessWindowEvent(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxWindow_ProcessWindowEventLocally(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxWindow_PopEventHandler(self_: *mut c_void, delete_handler: bool) -> *mut c_void;
        pub fn wxWindow_PushEventHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxWindow_RemoveEventHandler(self_: *mut c_void, handler: *mut c_void) -> bool;
        pub fn wxWindow_SetEventHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxWindow_SetNextHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxWindow_SetPreviousHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxWindow_GetExtraStyle(self_: *const c_void) -> i32;
        pub fn wxWindow_GetWindowStyleFlag(self_: *const c_void) -> i32;
        pub fn wxWindow_GetWindowStyle(self_: *const c_void) -> i32;
        pub fn wxWindow_HasExtraStyle(self_: *const c_void, ex_flag: i32) -> bool;
        pub fn wxWindow_HasFlag(self_: *const c_void, flag: i32) -> bool;
        pub fn wxWindow_SetExtraStyle(self_: *mut c_void, ex_style: i32);
        pub fn wxWindow_SetWindowStyleFlag(self_: *mut c_void, style: i32);
        pub fn wxWindow_SetWindowStyle(self_: *mut c_void, style: i32);
        pub fn wxWindow_ToggleWindowStyle(self_: *mut c_void, flag: i32) -> bool;
        pub fn wxWindow_MoveAfterInTabOrder(self_: *mut c_void, win: *mut c_void);
        pub fn wxWindow_MoveBeforeInTabOrder(self_: *mut c_void, win: *mut c_void);
        pub fn wxWindow_Navigate(self_: *mut c_void, flags: i32) -> bool;
        pub fn wxWindow_NavigateIn(self_: *mut c_void, flags: i32) -> bool;
        pub fn wxWindow_Lower(self_: *mut c_void);
        pub fn wxWindow_Raise(self_: *mut c_void);
        pub fn wxWindow_Hide(self_: *mut c_void) -> bool;
        pub fn wxWindow_IsEnabled(self_: *const c_void) -> bool;
        pub fn wxWindow_IsExposed(self_: *const c_void, x: i32, y: i32) -> bool;
        pub fn wxWindow_IsExposed1(self_: *const c_void, pt: *mut c_void) -> bool;
        pub fn wxWindow_IsExposed2(self_: *const c_void, x: i32, y: i32, w: i32, h: i32) -> bool;
        pub fn wxWindow_IsExposed3(self_: *const c_void, rect: *mut c_void) -> bool;
        pub fn wxWindow_IsShown(self_: *const c_void) -> bool;
        pub fn wxWindow_IsShownOnScreen(self_: *const c_void) -> bool;
        pub fn wxWindow_Disable(self_: *mut c_void) -> bool;
        pub fn wxWindow_Enable(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxWindow_Show(self_: *mut c_void, show: bool) -> bool;
        pub fn wxWindow_GetHelpText(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetHelpText(self_: *mut c_void, help_text: *const c_void);
        pub fn wxWindow_GetToolTip(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetToolTipText(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetToolTip(self_: *mut c_void, tip_string: *const c_void);
        pub fn wxWindow_SetToolTip1(self_: *mut c_void, tip: *mut c_void);
        pub fn wxWindow_UnsetToolTip(self_: *mut c_void);
        pub fn wxWindow_GetPopupMenuSelectionFromUser(self_: *mut c_void, menu: *mut c_void, pos: *const c_void) -> i32;
        pub fn wxWindow_GetPopupMenuSelectionFromUser1(self_: *mut c_void, menu: *mut c_void, x: i32, y: i32) -> i32;
        pub fn wxWindow_PopupMenu(self_: *mut c_void, menu: *mut c_void, pos: *const c_void) -> bool;
        pub fn wxWindow_PopupMenu1(self_: *mut c_void, menu: *mut c_void, x: i32, y: i32) -> bool;
        pub fn wxWindow_GetValidator(self_: *mut c_void) -> *mut c_void;
        pub fn wxWindow_SetValidator(self_: *mut c_void, validator: *const c_void);
        pub fn wxWindow_TransferDataFromWindow(self_: *mut c_void) -> bool;
        pub fn wxWindow_TransferDataToWindow(self_: *mut c_void) -> bool;
        pub fn wxWindow_Validate(self_: *mut c_void) -> bool;
        pub fn wxWindow_GetId(self_: *const c_void) -> i32;
        pub fn wxWindow_GetLabel(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_AdjustForLayoutDirection(self_: *const c_void, x: i32, width: i32, width_total: i32) -> i32;
        pub fn wxWindow_GetName(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetId(self_: *mut c_void, winid: i32);
        pub fn wxWindow_SetLabel(self_: *mut c_void, label: *const c_void);
        pub fn wxWindow_SetName(self_: *mut c_void, name: *const c_void);
        pub fn wxWindow_GetAcceleratorTable(self_: *mut c_void) -> *mut c_void;
        pub fn wxWindow_SetAcceleratorTable(self_: *mut c_void, accel: *const c_void);
        pub fn wxWindow_Close(self_: *mut c_void, force: bool) -> bool;
        pub fn wxWindow_Destroy(self_: *mut c_void) -> bool;
        pub fn wxWindow_IsBeingDeleted(self_: *const c_void) -> bool;
        pub fn wxWindow_GetDropTarget(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetDropTarget(self_: *mut c_void, target: *mut c_void);
        pub fn wxWindow_DragAcceptFiles(self_: *mut c_void, accept: bool);
        pub fn wxWindow_GetContainingSizer(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetSizer(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetSizer(self_: *mut c_void, sizer: *mut c_void, delete_old: bool);
        pub fn wxWindow_SetSizerAndFit(self_: *mut c_void, sizer: *mut c_void, delete_old: bool);
        pub fn wxWindow_GetConstraints(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetConstraints(self_: *mut c_void, constraints: *mut c_void);
        pub fn wxWindow_Layout(self_: *mut c_void) -> bool;
        pub fn wxWindow_SetAutoLayout(self_: *mut c_void, auto_layout: bool);
        pub fn wxWindow_GetAutoLayout(self_: *const c_void) -> bool;
        pub fn wxWindow_CaptureMouse(self_: *mut c_void);
        pub fn wxWindow_GetCaret(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_HasCapture(self_: *const c_void) -> bool;
        pub fn wxWindow_ReleaseMouse(self_: *mut c_void);
        pub fn wxWindow_SetCaret(self_: *mut c_void, caret: *mut c_void);
        pub fn wxWindow_SetCursor(self_: *mut c_void, cursor: *const c_void) -> bool;
        pub fn wxWindow_WarpPointer(self_: *mut c_void, x: i32, y: i32);
        pub fn wxWindow_EnableTouchEvents(self_: *mut c_void, events_mask: i32) -> bool;
        pub fn wxWindow_DoUpdateWindowUI(self_: *mut c_void, event: *mut c_void);
        pub fn wxWindow_HasMultiplePages(self_: *const c_void) -> bool;
        pub fn wxWindow_InheritAttributes(self_: *mut c_void);
        pub fn wxWindow_InitDialog(self_: *mut c_void);
        pub fn wxWindow_IsDoubleBuffered(self_: *const c_void) -> bool;
        pub fn wxWindow_SetDoubleBuffered(self_: *mut c_void, on: bool);
        pub fn wxWindow_IsRetained(self_: *const c_void) -> bool;
        pub fn wxWindow_IsThisEnabled(self_: *const c_void) -> bool;
        pub fn wxWindow_IsTopLevel(self_: *const c_void) -> bool;
        pub fn wxWindow_OnInternalIdle(self_: *mut c_void);
        pub fn wxWindow_SendIdleEvents(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxWindow_RegisterHotKey(self_: *mut c_void, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool;
        pub fn wxWindow_UnregisterHotKey(self_: *mut c_void, hotkey_id: i32) -> bool;
        pub fn wxWindow_UpdateWindowUI(self_: *mut c_void, flags: i32);
        pub fn wxWindow_FindFocus() -> *mut c_void;
        pub fn wxWindow_FindWindowById(id: i32, parent: *const c_void) -> *mut c_void;
        pub fn wxWindow_FindWindowByLabel(label: *const c_void, parent: *const c_void) -> *mut c_void;
        pub fn wxWindow_FindWindowByName(name: *const c_void, parent: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetCapture() -> *mut c_void;
        pub fn wxWindow_NewControlId(count: i32) -> i32;
        pub fn wxWindow_UnreserveControlId(id: i32, count: i32);
        pub fn wxWindow_new() -> *mut c_void;
        pub fn wxWindow_new1(parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> *mut c_void;
        pub fn wxWindow_Create(self_: *mut c_void, parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> bool;
        // CLASS: wxControl
        pub fn wxControl_new(parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> *mut c_void;
        pub fn wxControl_new1() -> *mut c_void;
        pub fn wxControl_Create(self_: *mut c_void, parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> bool;
        pub fn wxControl_Command(self_: *mut c_void, event: *mut c_void);
        pub fn wxControl_GetLabel(self_: *const c_void) -> *mut c_void;
        pub fn wxControl_GetLabelText(self_: *const c_void) -> *mut c_void;
        pub fn wxControl_GetSizeFromTextSize(self_: *const c_void, xlen: i32, ylen: i32) -> *mut c_void;
        pub fn wxControl_GetSizeFromTextSize1(self_: *const c_void, tsize: *const c_void) -> *mut c_void;
        pub fn wxControl_GetSizeFromText(self_: *const c_void, text: *const c_void) -> *mut c_void;
        pub fn wxControl_SetLabel(self_: *mut c_void, label: *const c_void);
        pub fn wxControl_SetLabelText(self_: *mut c_void, text: *const c_void);
        pub fn wxControl_SetLabelMarkup(self_: *mut c_void, markup: *const c_void) -> bool;
        pub fn wxControl_GetLabelText1(label: *const c_void) -> *mut c_void;
        pub fn wxControl_RemoveMnemonics(str: *const c_void) -> *mut c_void;
        pub fn wxControl_EscapeMnemonics(text: *const c_void) -> *mut c_void;
        // CLASS: wxAnyButton
        pub fn wxAnyButton_new() -> *mut c_void;
        pub fn wxAnyButton_SetBitmapCurrent(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapDisabled(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapFocus(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapLabel(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapPressed(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_GetBitmapMargins(self_: *mut c_void) -> *mut c_void;
        pub fn wxAnyButton_SetBitmapMargins(self_: *mut c_void, x: i32, y: i32);
        pub fn wxAnyButton_SetBitmapMargins1(self_: *mut c_void, sz: *const c_void);
        // CLASS: wxButton
        pub fn wxButton_new() -> *mut c_void;
        pub fn wxButton_new1(parent: *mut c_void, id: i32, label: *const c_void, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> *mut c_void;
        pub fn wxButton_Create(self_: *mut c_void, parent: *mut c_void, id: i32, label: *const c_void, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> bool;
        pub fn wxButton_GetAuthNeeded(self_: *const c_void) -> bool;
        pub fn wxButton_GetLabel(self_: *const c_void) -> *mut c_void;
        pub fn wxButton_SetAuthNeeded(self_: *mut c_void, needed: bool);
        pub fn wxButton_SetDefault(self_: *mut c_void) -> *mut c_void;
        pub fn wxButton_SetLabel(self_: *mut c_void, label: *const c_void);
        pub fn wxButton_GetDefaultSize(win: *mut c_void) -> *mut c_void;
        // CLASS: wxNonOwnedWindow
        pub fn wxNonOwnedWindow_SetShape(self_: *mut c_void, region: *const c_void) -> bool;
        pub fn wxNonOwnedWindow_SetShape1(self_: *mut c_void, path: *const c_void) -> bool;
        // CLASS: wxTopLevelWindow
        pub fn wxTopLevelWindow_new() -> *mut c_void;
        pub fn wxTopLevelWindow_new1(parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_Create(self_: *mut c_void, parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> bool;
        pub fn wxTopLevelWindow_CanSetTransparent(self_: *mut c_void) -> bool;
        pub fn wxTopLevelWindow_CenterOnScreen(self_: *mut c_void, direction: i32);
        pub fn wxTopLevelWindow_CentreOnScreen(self_: *mut c_void, direction: i32);
        pub fn wxTopLevelWindow_EnableCloseButton(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_EnableMaximizeButton(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_EnableMinimizeButton(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_GetDefaultItem(self_: *const c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_GetTitle(self_: *const c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_Iconize(self_: *mut c_void, iconize: bool);
        pub fn wxTopLevelWindow_IsActive(self_: *mut c_void) -> bool;
        pub fn wxTopLevelWindow_IsAlwaysMaximized(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_IsFullScreen(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_IsIconized(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_IsMaximized(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_Layout(self_: *mut c_void) -> bool;
        pub fn wxTopLevelWindow_Maximize(self_: *mut c_void, maximize: bool);
        pub fn wxTopLevelWindow_RequestUserAttention(self_: *mut c_void, flags: i32);
        pub fn wxTopLevelWindow_Restore(self_: *mut c_void);
        pub fn wxTopLevelWindow_SetDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_SetTmpDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_GetTmpDefaultItem(self_: *const c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_SetIcon(self_: *mut c_void, icon: *const c_void);
        pub fn wxTopLevelWindow_SetIcons(self_: *mut c_void, icons: *const c_void);
        pub fn wxTopLevelWindow_SetMaxSize(self_: *mut c_void, size: *const c_void);
        pub fn wxTopLevelWindow_SetMinSize(self_: *mut c_void, size: *const c_void);
        pub fn wxTopLevelWindow_SetSizeHints(self_: *mut c_void, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        pub fn wxTopLevelWindow_SetSizeHints1(self_: *mut c_void, min_size: *const c_void, max_size: *const c_void, inc_size: *const c_void);
        pub fn wxTopLevelWindow_SetTitle(self_: *mut c_void, title: *const c_void);
        pub fn wxTopLevelWindow_SetTransparent(self_: *mut c_void, alpha: u8) -> bool;
        pub fn wxTopLevelWindow_ShouldPreventAppExit(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_OSXSetModified(self_: *mut c_void, modified: bool);
        pub fn wxTopLevelWindow_OSXIsModified(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_SetRepresentedFilename(self_: *mut c_void, filename: *const c_void);
        pub fn wxTopLevelWindow_ShowWithoutActivating(self_: *mut c_void);
        pub fn wxTopLevelWindow_EnableFullScreenView(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_ShowFullScreen(self_: *mut c_void, show: bool, style: i32) -> bool;
        pub fn wxTopLevelWindow_GetDefaultSize() -> *mut c_void;
        // CLASS: wxFrame
        pub fn wxFrame_new() -> *mut c_void;
        pub fn wxFrame_new1(parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_Centre(self_: *mut c_void, direction: i32);
        pub fn wxFrame_Create(self_: *mut c_void, parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> bool;
        pub fn wxFrame_CreateStatusBar(self_: *mut c_void, number: i32, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_CreateToolBar(self_: *mut c_void, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_DoGiveHelp(self_: *mut c_void, text: *const c_void, show: bool);
        pub fn wxFrame_GetClientAreaOrigin(self_: *const c_void) -> *mut c_void;
        pub fn wxFrame_GetMenuBar(self_: *const c_void) -> *mut c_void;
        pub fn wxFrame_GetStatusBar(self_: *const c_void) -> *mut c_void;
        pub fn wxFrame_GetStatusBarPane(self_: *const c_void) -> i32;
        pub fn wxFrame_GetToolBar(self_: *const c_void) -> *mut c_void;
        pub fn wxFrame_OnCreateStatusBar(self_: *mut c_void, number: i32, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_OnCreateToolBar(self_: *mut c_void, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_ProcessCommand(self_: *mut c_void, id: i32) -> bool;
        pub fn wxFrame_SetMenuBar(self_: *mut c_void, menu_bar: *mut c_void);
        pub fn wxFrame_SetStatusBar(self_: *mut c_void, status_bar: *mut c_void);
        pub fn wxFrame_SetStatusBarPane(self_: *mut c_void, n: i32);
        pub fn wxFrame_SetStatusText(self_: *mut c_void, text: *const c_void, number: i32);
        pub fn wxFrame_SetStatusWidths(self_: *mut c_void, n: i32, widths_field: *const c_void);
        pub fn wxFrame_SetToolBar(self_: *mut c_void, tool_bar: *mut c_void);
        pub fn wxFrame_PushStatusText(self_: *mut c_void, text: *const c_void, number: i32);
        pub fn wxFrame_PopStatusText(self_: *mut c_void, number: i32);
        // CLASS: wxPoint
        pub fn wxPoint_IsFullySpecified(self_: *const c_void) -> bool;
        pub fn wxPoint_SetDefaults(self_: *mut c_void, pt: *const c_void);
        pub fn wxPoint_new() -> *mut c_void;
        pub fn wxPoint_new1(x: i32, y: i32) -> *mut c_void;
        pub fn wxPoint_new2(pt: *const c_void) -> *mut c_void;
        // CLASS: wxRect
        pub fn wxRect_new() -> *mut c_void;
        pub fn wxRect_new1(x: i32, y: i32, width: i32, height: i32) -> *mut c_void;
        pub fn wxRect_new2(top_left: *const c_void, bottom_right: *const c_void) -> *mut c_void;
        pub fn wxRect_new3(pos: *const c_void, size: *const c_void) -> *mut c_void;
        pub fn wxRect_new4(size: *const c_void) -> *mut c_void;
        pub fn wxRect_CentreIn(self_: *const c_void, r: *const c_void, dir: i32) -> *mut c_void;
        pub fn wxRect_CenterIn(self_: *const c_void, r: *const c_void, dir: i32) -> *mut c_void;
        pub fn wxRect_Contains(self_: *const c_void, x: i32, y: i32) -> bool;
        pub fn wxRect_Contains1(self_: *const c_void, pt: *const c_void) -> bool;
        pub fn wxRect_Contains2(self_: *const c_void, rect: *const c_void) -> bool;
        pub fn wxRect_Deflate3(self_: *const c_void, dx: i32, dy: i32) -> *mut c_void;
        pub fn wxRect_GetBottom(self_: *const c_void) -> i32;
        pub fn wxRect_GetBottomLeft(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetBottomRight(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetHeight(self_: *const c_void) -> i32;
        pub fn wxRect_GetLeft(self_: *const c_void) -> i32;
        pub fn wxRect_GetPosition(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetRight(self_: *const c_void) -> i32;
        pub fn wxRect_GetSize(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetTop(self_: *const c_void) -> i32;
        pub fn wxRect_GetTopLeft(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetTopRight(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetWidth(self_: *const c_void) -> i32;
        pub fn wxRect_GetX(self_: *const c_void) -> i32;
        pub fn wxRect_GetY(self_: *const c_void) -> i32;
        pub fn wxRect_Inflate3(self_: *const c_void, dx: i32, dy: i32) -> *mut c_void;
        pub fn wxRect_Intersect1(self_: *const c_void, rect: *const c_void) -> *mut c_void;
        pub fn wxRect_Intersects(self_: *const c_void, rect: *const c_void) -> bool;
        pub fn wxRect_IsEmpty(self_: *const c_void) -> bool;
        pub fn wxRect_Offset(self_: *mut c_void, dx: i32, dy: i32);
        pub fn wxRect_Offset1(self_: *mut c_void, pt: *const c_void);
        pub fn wxRect_SetHeight(self_: *mut c_void, height: i32);
        pub fn wxRect_SetPosition(self_: *mut c_void, pos: *const c_void);
        pub fn wxRect_SetSize(self_: *mut c_void, s: *const c_void);
        pub fn wxRect_SetWidth(self_: *mut c_void, width: i32);
        pub fn wxRect_SetX(self_: *mut c_void, x: i32);
        pub fn wxRect_SetY(self_: *mut c_void, y: i32);
        pub fn wxRect_SetLeft(self_: *mut c_void, left: i32);
        pub fn wxRect_SetRight(self_: *mut c_void, right: i32);
        pub fn wxRect_SetTop(self_: *mut c_void, top: i32);
        pub fn wxRect_SetBottom(self_: *mut c_void, bottom: i32);
        pub fn wxRect_SetTopLeft(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_SetBottomRight(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_SetTopRight(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_SetBottomLeft(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_Union(self_: *const c_void, rect: *const c_void) -> *mut c_void;
        // CLASS: wxSize
        pub fn wxSize_new() -> *mut c_void;
        pub fn wxSize_new1(width: i32, height: i32) -> *mut c_void;
        pub fn wxSize_DecBy(self_: *mut c_void, pt: *const c_void);
        pub fn wxSize_DecBy1(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_DecBy2(self_: *mut c_void, dx: i32, dy: i32);
        pub fn wxSize_DecBy3(self_: *mut c_void, d: i32);
        pub fn wxSize_DecTo(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_DecToIfSpecified(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_GetHeight(self_: *const c_void) -> i32;
        pub fn wxSize_GetWidth(self_: *const c_void) -> i32;
        pub fn wxSize_IncBy(self_: *mut c_void, pt: *const c_void);
        pub fn wxSize_IncBy1(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_IncBy2(self_: *mut c_void, dx: i32, dy: i32);
        pub fn wxSize_IncBy3(self_: *mut c_void, d: i32);
        pub fn wxSize_IncTo(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_IsFullySpecified(self_: *const c_void) -> bool;
        pub fn wxSize_Set(self_: *mut c_void, width: i32, height: i32);
        pub fn wxSize_SetDefaults(self_: *mut c_void, size_default: *const c_void);
        pub fn wxSize_SetHeight(self_: *mut c_void, height: i32);
        pub fn wxSize_SetWidth(self_: *mut c_void, width: i32);
        // CLASS: wxValidator
        pub fn wxValidator_new() -> *mut c_void;
        pub fn wxValidator_Clone(self_: *const c_void) -> *mut c_void;
        pub fn wxValidator_GetWindow(self_: *const c_void) -> *mut c_void;
        pub fn wxValidator_SetWindow(self_: *mut c_void, window: *mut c_void);
        pub fn wxValidator_TransferFromWindow(self_: *mut c_void) -> bool;
        pub fn wxValidator_TransferToWindow(self_: *mut c_void) -> bool;
        pub fn wxValidator_Validate(self_: *mut c_void, parent: *mut c_void) -> bool;
        pub fn wxValidator_SuppressBellOnError(suppress: bool);
        pub fn wxValidator_IsSilent() -> bool;
    }
}

pub trait WxRustMethods {
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr;
    fn pinned<T>(&self) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(&mut *(self.as_ptr() as *mut _)) }
    }
}

// wxObject
wx_class! { Object(wxObject) impl
    ObjectMethods
}
impl Object {
    pub fn new() -> Object {
        unsafe { Object(ffi::wxObject_new()) }
    }
    pub fn new1(other: &Object) -> Object {
        unsafe {
            let other = other.as_ptr() as *mut c_void;
            Object(ffi::wxObject_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    fn get_class_info(&self) -> *mut c_void {
        unsafe { ffi::wxObject_GetClassInfo(self.as_ptr() as *mut c_void) }
    }
    fn get_ref_data(&self) -> *mut c_void {
        unsafe { ffi::wxObject_GetRefData(self.as_ptr() as *mut c_void) }
    }
    fn is_kind_of(&self, info: *const c_void) -> bool {
        unsafe { ffi::wxObject_IsKindOf(self.as_ptr() as *mut c_void, info) }
    }
    fn is_same_as(&self, obj: &Object) -> bool {
        unsafe {
            let obj = obj.as_ptr() as *mut c_void;
            ffi::wxObject_IsSameAs(self.as_ptr() as *mut c_void, obj)
        }
    }
    fn ref_(&self, clone: &Object) {
        unsafe {
            let clone = clone.as_ptr() as *mut c_void;
            ffi::wxObject_Ref(self.as_ptr() as *mut c_void, clone)
        }
    }
    fn set_ref_data(&self, data: *mut c_void) {
        unsafe { ffi::wxObject_SetRefData(self.as_ptr() as *mut c_void, data) }
    }
    fn un_ref(&self) {
        unsafe { ffi::wxObject_UnRef(self.as_ptr() as *mut c_void) }
    }
    fn un_share(&self) {
        unsafe { ffi::wxObject_UnShare(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn operator delete()
    // CXX_UNSUPPORTED: fn operator new()
}

// wxEvtHandler
wx_class! { EvtHandler(wxEvtHandler) impl
    EvtHandlerMethods,
    ObjectMethods
}
impl EvtHandler {
    pub fn new() -> EvtHandler {
        unsafe { EvtHandler(ffi::wxEvtHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait EvtHandlerMethods: ObjectMethods {
    fn queue_event(&self, event: *mut c_void) {
        unsafe { ffi::wxEvtHandler_QueueEvent(self.as_ptr() as *mut c_void, event) }
    }
    fn add_pending_event(&self, event: *const c_void) {
        unsafe { ffi::wxEvtHandler_AddPendingEvent(self.as_ptr() as *mut c_void, event) }
    }
    // CXX_UNSUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter1()
    fn process_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEvent(self.as_ptr() as *mut c_void, event) }
    }
    fn process_event_locally(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEventLocally(self.as_ptr() as *mut c_void, event) }
    }
    fn safely_process_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_SafelyProcessEvent(self.as_ptr() as *mut c_void, event) }
    }
    fn process_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_ProcessPendingEvents(self.as_ptr() as *mut c_void) }
    }
    fn delete_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_DeletePendingEvents(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn Connect()
    // CXX_UNSUPPORTED: fn Connect1()
    // CXX_UNSUPPORTED: fn Connect2()
    // CXX_UNSUPPORTED: fn Disconnect()
    // CXX_UNSUPPORTED: fn Disconnect1()
    // CXX_UNSUPPORTED: fn Disconnect2()
    // CXX_UNSUPPORTED: fn Bind()
    // BLOCKED: fn Bind1()
    // CXX_UNSUPPORTED: fn Unbind()
    // BLOCKED: fn Unbind1()
    // BLOCKED: fn GetClientData()
    fn get_client_object(&self) -> *mut c_void {
        unsafe { ffi::wxEvtHandler_GetClientObject(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn SetClientData()
    fn set_client_object(&self, data: *mut c_void) {
        unsafe { ffi::wxEvtHandler_SetClientObject(self.as_ptr() as *mut c_void, data) }
    }
    fn get_evt_handler_enabled(&self) -> bool {
        unsafe { ffi::wxEvtHandler_GetEvtHandlerEnabled(self.as_ptr() as *mut c_void) }
    }
    fn get_next_handler(&self) -> *mut c_void {
        unsafe { ffi::wxEvtHandler_GetNextHandler(self.as_ptr() as *mut c_void) }
    }
    fn get_previous_handler(&self) -> *mut c_void {
        unsafe { ffi::wxEvtHandler_GetPreviousHandler(self.as_ptr() as *mut c_void) }
    }
    fn set_evt_handler_enabled(&self, enabled: bool) {
        unsafe { ffi::wxEvtHandler_SetEvtHandlerEnabled(self.as_ptr() as *mut c_void, enabled) }
    }
    fn set_next_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetNextHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn set_previous_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetPreviousHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn unlink(&self) {
        unsafe { ffi::wxEvtHandler_Unlink(self.as_ptr() as *mut c_void) }
    }
    fn is_unlinked(&self) -> bool {
        unsafe { ffi::wxEvtHandler_IsUnlinked(self.as_ptr() as *mut c_void) }
    }
    fn add_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_AddFilter(filter) }
    }
    fn remove_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_RemoveFilter(filter) }
    }
    // DTOR: fn ~wxEvtHandler()
}

// wxWindow
wx_class! { Window(wxWindow) impl
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Window {
    pub fn new() -> Window {
        unsafe { Window(ffi::wxWindow_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, name: &str) -> Window {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            Window(ffi::wxWindow_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait WindowMethods: EvtHandlerMethods {
    fn accepts_focus(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocus(self.as_ptr() as *mut c_void) }
    }
    fn accepts_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusFromKeyboard(self.as_ptr() as *mut c_void) }
    }
    fn accepts_focus_recursively(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusRecursively(self.as_ptr() as *mut c_void) }
    }
    fn disable_focus_from_keyboard(&self) {
        unsafe { ffi::wxWindow_DisableFocusFromKeyboard(self.as_ptr() as *mut c_void) }
    }
    fn is_focusable(&self) -> bool {
        unsafe { ffi::wxWindow_IsFocusable(self.as_ptr() as *mut c_void) }
    }
    fn can_accept_focus(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocus(self.as_ptr() as *mut c_void) }
    }
    fn can_accept_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocusFromKeyboard(self.as_ptr() as *mut c_void) }
    }
    fn has_focus(&self) -> bool {
        unsafe { ffi::wxWindow_HasFocus(self.as_ptr() as *mut c_void) }
    }
    fn set_can_focus(&self, can_focus: bool) {
        unsafe { ffi::wxWindow_SetCanFocus(self.as_ptr() as *mut c_void, can_focus) }
    }
    fn enable_visible_focus(&self, enable: bool) {
        unsafe { ffi::wxWindow_EnableVisibleFocus(self.as_ptr() as *mut c_void, enable) }
    }
    fn set_focus(&self) {
        unsafe { ffi::wxWindow_SetFocus(self.as_ptr() as *mut c_void) }
    }
    fn set_focus_from_kbd(&self) {
        unsafe { ffi::wxWindow_SetFocusFromKbd(self.as_ptr() as *mut c_void) }
    }
    fn add_child<T: WindowMethods>(&self, child: Option<&T>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_AddChild(self.as_ptr() as *mut c_void, child)
        }
    }
    fn destroy_children(&self) -> bool {
        unsafe { ffi::wxWindow_DestroyChildren(self.as_ptr() as *mut c_void) }
    }
    fn find_window(&self, id: i32) -> *mut c_void {
        unsafe { ffi::wxWindow_FindWindow(self.as_ptr() as *mut c_void, id) }
    }
    fn find_window1(&self, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxWindow_FindWindow1(self.as_ptr() as *mut c_void, name)
        }
    }
    // BLOCKED: fn GetChildren()
    // BLOCKED: fn GetChildren1()
    fn remove_child<T: WindowMethods>(&self, child: Option<&T>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveChild(self.as_ptr() as *mut c_void, child)
        }
    }
    fn get_grand_parent(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetGrandParent(self.as_ptr() as *mut c_void) }
    }
    fn get_next_sibling(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetNextSibling(self.as_ptr() as *mut c_void) }
    }
    fn get_parent(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetParent(self.as_ptr() as *mut c_void) }
    }
    fn get_prev_sibling(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetPrevSibling(self.as_ptr() as *mut c_void) }
    }
    fn is_descendant<T: WindowMethods>(&self, win: Option<&T>) -> bool {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_IsDescendant(self.as_ptr() as *mut c_void, win)
        }
    }
    fn reparent<T: WindowMethods>(&self, new_parent: Option<&T>) -> bool {
        unsafe {
            let new_parent = match new_parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_Reparent(self.as_ptr() as *mut c_void, new_parent)
        }
    }
    fn always_show_scrollbars(&self, hflag: bool, vflag: bool) {
        unsafe { ffi::wxWindow_AlwaysShowScrollbars(self.as_ptr() as *mut c_void, hflag, vflag) }
    }
    fn get_scroll_pos(&self, orientation: i32) -> i32 {
        unsafe { ffi::wxWindow_GetScrollPos(self.as_ptr() as *mut c_void, orientation) }
    }
    fn get_scroll_range(&self, orientation: i32) -> i32 {
        unsafe { ffi::wxWindow_GetScrollRange(self.as_ptr() as *mut c_void, orientation) }
    }
    fn get_scroll_thumb(&self, orientation: i32) -> i32 {
        unsafe { ffi::wxWindow_GetScrollThumb(self.as_ptr() as *mut c_void, orientation) }
    }
    fn can_scroll(&self, orient: i32) -> bool {
        unsafe { ffi::wxWindow_CanScroll(self.as_ptr() as *mut c_void, orient) }
    }
    fn has_scrollbar(&self, orient: i32) -> bool {
        unsafe { ffi::wxWindow_HasScrollbar(self.as_ptr() as *mut c_void, orient) }
    }
    fn is_scrollbar_always_shown(&self, orient: i32) -> bool {
        unsafe { ffi::wxWindow_IsScrollbarAlwaysShown(self.as_ptr() as *mut c_void, orient) }
    }
    fn scroll_lines(&self, lines: i32) -> bool {
        unsafe { ffi::wxWindow_ScrollLines(self.as_ptr() as *mut c_void, lines) }
    }
    fn scroll_pages(&self, pages: i32) -> bool {
        unsafe { ffi::wxWindow_ScrollPages(self.as_ptr() as *mut c_void, pages) }
    }
    fn scroll_window(&self, dx: i32, dy: i32, rect: *const c_void) {
        unsafe { ffi::wxWindow_ScrollWindow(self.as_ptr() as *mut c_void, dx, dy, rect) }
    }
    fn line_up(&self) -> bool {
        unsafe { ffi::wxWindow_LineUp(self.as_ptr() as *mut c_void) }
    }
    fn line_down(&self) -> bool {
        unsafe { ffi::wxWindow_LineDown(self.as_ptr() as *mut c_void) }
    }
    fn page_up(&self) -> bool {
        unsafe { ffi::wxWindow_PageUp(self.as_ptr() as *mut c_void) }
    }
    fn page_down(&self) -> bool {
        unsafe { ffi::wxWindow_PageDown(self.as_ptr() as *mut c_void) }
    }
    fn set_scroll_pos(&self, orientation: i32, pos: i32, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollPos(self.as_ptr() as *mut c_void, orientation, pos, refresh) }
    }
    fn set_scrollbar(&self, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollbar(self.as_ptr() as *mut c_void, orientation, position, thumb_size, range, refresh) }
    }
    fn begin_repositioning_children(&self) -> bool {
        unsafe { ffi::wxWindow_BeginRepositioningChildren(self.as_ptr() as *mut c_void) }
    }
    fn end_repositioning_children(&self) {
        unsafe { ffi::wxWindow_EndRepositioningChildren(self.as_ptr() as *mut c_void) }
    }
    fn cache_best_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxWindow_CacheBestSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn client_to_window_size(&self, size: &Size) -> Size {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            Size(ffi::wxWindow_ClientToWindowSize(self.as_ptr() as *mut c_void, size))
        }
    }
    fn window_to_client_size(&self, size: &Size) -> Size {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            Size(ffi::wxWindow_WindowToClientSize(self.as_ptr() as *mut c_void, size))
        }
    }
    fn fit(&self) {
        unsafe { ffi::wxWindow_Fit(self.as_ptr() as *mut c_void) }
    }
    fn fit_inside(&self) {
        unsafe { ffi::wxWindow_FitInside(self.as_ptr() as *mut c_void) }
    }
    fn from_dip(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            Size(ffi::wxWindow_FromDIP(self.as_ptr() as *mut c_void, sz))
        }
    }
    fn from_dip1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi::wxWindow_FromDIP1(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn from_dip2(&self, d: i32) -> i32 {
        unsafe { ffi::wxWindow_FromDIP2(self.as_ptr() as *mut c_void, d) }
    }
    fn to_dip(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            Size(ffi::wxWindow_ToDIP(self.as_ptr() as *mut c_void, sz))
        }
    }
    fn to_dip1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi::wxWindow_ToDIP1(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn to_dip2(&self, d: i32) -> i32 {
        unsafe { ffi::wxWindow_ToDIP2(self.as_ptr() as *mut c_void, d) }
    }
    fn get_best_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetBestSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_best_height(&self, width: i32) -> i32 {
        unsafe { ffi::wxWindow_GetBestHeight(self.as_ptr() as *mut c_void, width) }
    }
    fn get_best_width(&self, height: i32) -> i32 {
        unsafe { ffi::wxWindow_GetBestWidth(self.as_ptr() as *mut c_void, height) }
    }
    fn get_client_size(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetClientSize(self.as_ptr() as *mut c_void, width, height) }
    }
    fn get_client_size1(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetClientSize1(self.as_ptr() as *mut c_void)) }
    }
    fn get_effective_min_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetEffectiveMinSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_max_client_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMaxClientSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_max_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMaxSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_min_client_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMinClientSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_min_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMinSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_min_width(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMinWidth(self.as_ptr() as *mut c_void) }
    }
    fn get_min_height(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMinHeight(self.as_ptr() as *mut c_void) }
    }
    fn get_max_width(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMaxWidth(self.as_ptr() as *mut c_void) }
    }
    fn get_max_height(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMaxHeight(self.as_ptr() as *mut c_void) }
    }
    fn get_size(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetSize(self.as_ptr() as *mut c_void, width, height) }
    }
    fn get_size1(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetSize1(self.as_ptr() as *mut c_void)) }
    }
    fn get_virtual_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetVirtualSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_virtual_size1(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetVirtualSize1(self.as_ptr() as *mut c_void, width, height) }
    }
    fn get_best_virtual_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetBestVirtualSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_content_scale_factor(&self) -> f64 {
        unsafe { ffi::wxWindow_GetContentScaleFactor(self.as_ptr() as *mut c_void) }
    }
    fn get_dpi_scale_factor(&self) -> f64 {
        unsafe { ffi::wxWindow_GetDPIScaleFactor(self.as_ptr() as *mut c_void) }
    }
    fn get_window_border_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetWindowBorderSize(self.as_ptr() as *mut c_void)) }
    }
    fn inform_first_direction(&self, direction: i32, size: i32, available_other_dir: i32) -> bool {
        unsafe { ffi::wxWindow_InformFirstDirection(self.as_ptr() as *mut c_void, direction, size, available_other_dir) }
    }
    fn invalidate_best_size(&self) {
        unsafe { ffi::wxWindow_InvalidateBestSize(self.as_ptr() as *mut c_void) }
    }
    fn post_size_event(&self) {
        unsafe { ffi::wxWindow_PostSizeEvent(self.as_ptr() as *mut c_void) }
    }
    fn post_size_event_to_parent(&self) {
        unsafe { ffi::wxWindow_PostSizeEventToParent(self.as_ptr() as *mut c_void) }
    }
    fn send_size_event(&self, flags: i32) {
        unsafe { ffi::wxWindow_SendSizeEvent(self.as_ptr() as *mut c_void, flags) }
    }
    fn send_size_event_to_parent(&self, flags: i32) {
        unsafe { ffi::wxWindow_SendSizeEventToParent(self.as_ptr() as *mut c_void, flags) }
    }
    fn set_client_size(&self, width: i32, height: i32) {
        unsafe { ffi::wxWindow_SetClientSize(self.as_ptr() as *mut c_void, width, height) }
    }
    fn set_client_size1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxWindow_SetClientSize1(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_client_size2(&self, rect: &Rect) {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            ffi::wxWindow_SetClientSize2(self.as_ptr() as *mut c_void, rect)
        }
    }
    fn set_containing_sizer(&self, sizer: *mut c_void) {
        unsafe { ffi::wxWindow_SetContainingSizer(self.as_ptr() as *mut c_void, sizer) }
    }
    fn set_initial_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxWindow_SetInitialSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_max_client_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxWindow_SetMaxClientSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_max_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxWindow_SetMaxSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_min_client_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxWindow_SetMinClientSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_min_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxWindow_SetMinSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_size(&self, x: i32, y: i32, width: i32, height: i32, size_flags: i32) {
        unsafe { ffi::wxWindow_SetSize(self.as_ptr() as *mut c_void, x, y, width, height, size_flags) }
    }
    fn set_size1(&self, rect: &Rect) {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            ffi::wxWindow_SetSize1(self.as_ptr() as *mut c_void, rect)
        }
    }
    fn set_size2(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxWindow_SetSize2(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_size3(&self, width: i32, height: i32) {
        unsafe { ffi::wxWindow_SetSize3(self.as_ptr() as *mut c_void, width, height) }
    }
    fn set_size_hints(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        unsafe {
            let min_size = min_size.as_ptr() as *mut c_void;
            let max_size = max_size.as_ptr() as *mut c_void;
            let inc_size = inc_size.as_ptr() as *mut c_void;
            ffi::wxWindow_SetSizeHints(self.as_ptr() as *mut c_void, min_size, max_size, inc_size)
        }
    }
    fn set_size_hints1(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        unsafe { ffi::wxWindow_SetSizeHints1(self.as_ptr() as *mut c_void, min_w, min_h, max_w, max_h, inc_w, inc_h) }
    }
    fn set_virtual_size(&self, width: i32, height: i32) {
        unsafe { ffi::wxWindow_SetVirtualSize(self.as_ptr() as *mut c_void, width, height) }
    }
    fn set_virtual_size1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxWindow_SetVirtualSize1(self.as_ptr() as *mut c_void, size)
        }
    }
    fn from_dip3<T: WindowMethods>(sz: &Size, w: Option<&T>) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            Size(ffi::wxWindow_FromDIP3(sz, w))
        }
    }
    fn from_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            Point(ffi::wxWindow_FromDIP4(pt, w))
        }
    }
    fn from_dip5<T: WindowMethods>(d: i32, w: Option<&T>) -> i32 {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FromDIP5(d, w)
        }
    }
    fn to_dip3<T: WindowMethods>(sz: &Size, w: Option<&T>) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            Size(ffi::wxWindow_ToDIP3(sz, w))
        }
    }
    fn to_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            Point(ffi::wxWindow_ToDIP4(pt, w))
        }
    }
    fn to_dip5<T: WindowMethods>(d: i32, w: Option<&T>) -> i32 {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ToDIP5(d, w)
        }
    }
    fn center(&self, dir: i32) {
        unsafe { ffi::wxWindow_Center(self.as_ptr() as *mut c_void, dir) }
    }
    fn center_on_parent(&self, dir: i32) {
        unsafe { ffi::wxWindow_CenterOnParent(self.as_ptr() as *mut c_void, dir) }
    }
    fn centre(&self, direction: i32) {
        unsafe { ffi::wxWindow_Centre(self.as_ptr() as *mut c_void, direction) }
    }
    fn centre_on_parent(&self, direction: i32) {
        unsafe { ffi::wxWindow_CentreOnParent(self.as_ptr() as *mut c_void, direction) }
    }
    fn get_position(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetPosition(self.as_ptr() as *mut c_void, x, y) }
    }
    fn get_position1(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetPosition1(self.as_ptr() as *mut c_void)) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetRect(self.as_ptr() as *mut c_void)) }
    }
    fn get_screen_position(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetScreenPosition(self.as_ptr() as *mut c_void, x, y) }
    }
    fn get_screen_position1(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetScreenPosition1(self.as_ptr() as *mut c_void)) }
    }
    fn get_screen_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetScreenRect(self.as_ptr() as *mut c_void)) }
    }
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetClientAreaOrigin(self.as_ptr() as *mut c_void)) }
    }
    fn get_client_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetClientRect(self.as_ptr() as *mut c_void)) }
    }
    fn move_(&self, x: i32, y: i32, flags: i32) {
        unsafe { ffi::wxWindow_Move(self.as_ptr() as *mut c_void, x, y, flags) }
    }
    fn move1(&self, pt: &Point, flags: i32) {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi::wxWindow_Move1(self.as_ptr() as *mut c_void, pt, flags)
        }
    }
    fn set_position(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi::wxWindow_SetPosition(self.as_ptr() as *mut c_void, pt)
        }
    }
    fn client_to_screen(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ClientToScreen(self.as_ptr() as *mut c_void, x, y) }
    }
    fn client_to_screen1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi::wxWindow_ClientToScreen1(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn convert_dialog_to_pixels(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi::wxWindow_ConvertDialogToPixels(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn convert_dialog_to_pixels1(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            Size(ffi::wxWindow_ConvertDialogToPixels1(self.as_ptr() as *mut c_void, sz))
        }
    }
    fn convert_pixels_to_dialog(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi::wxWindow_ConvertPixelsToDialog(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn convert_pixels_to_dialog1(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            Size(ffi::wxWindow_ConvertPixelsToDialog1(self.as_ptr() as *mut c_void, sz))
        }
    }
    fn screen_to_client(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ScreenToClient(self.as_ptr() as *mut c_void, x, y) }
    }
    fn screen_to_client1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi::wxWindow_ScreenToClient1(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn clear_background(&self) {
        unsafe { ffi::wxWindow_ClearBackground(self.as_ptr() as *mut c_void) }
    }
    fn freeze(&self) {
        unsafe { ffi::wxWindow_Freeze(self.as_ptr() as *mut c_void) }
    }
    fn thaw(&self) {
        unsafe { ffi::wxWindow_Thaw(self.as_ptr() as *mut c_void) }
    }
    fn is_frozen(&self) -> bool {
        unsafe { ffi::wxWindow_IsFrozen(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn GetBackgroundColour()
    // CXX_UNSUPPORTED: fn GetBackgroundStyle()
    fn get_char_height(&self) -> i32 {
        unsafe { ffi::wxWindow_GetCharHeight(self.as_ptr() as *mut c_void) }
    }
    fn get_char_width(&self) -> i32 {
        unsafe { ffi::wxWindow_GetCharWidth(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn GetDefaultAttributes()
    fn get_dpi(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetDPI(self.as_ptr() as *mut c_void)) }
    }
    // CXX_UNSUPPORTED: fn GetFont()
    // CXX_UNSUPPORTED: fn GetForegroundColour()
    fn get_text_extent(&self, string: &str, w: *mut c_void, h: *mut c_void, descent: *mut c_void, external_leading: *mut c_void, font: *const c_void) {
        unsafe {
            let string = crate::wx_string_from(string);
            ffi::wxWindow_GetTextExtent(self.as_ptr() as *mut c_void, string, w, h, descent, external_leading, font)
        }
    }
    fn get_text_extent1(&self, string: &str) -> Size {
        unsafe {
            let string = crate::wx_string_from(string);
            Size(ffi::wxWindow_GetTextExtent1(self.as_ptr() as *mut c_void, string))
        }
    }
    // BLOCKED: fn GetUpdateRegion()
    fn get_update_client_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetUpdateClientRect(self.as_ptr() as *mut c_void)) }
    }
    fn has_transparent_background(&self) -> bool {
        unsafe { ffi::wxWindow_HasTransparentBackground(self.as_ptr() as *mut c_void) }
    }
    fn refresh(&self, erase_background: bool, rect: *const c_void) {
        unsafe { ffi::wxWindow_Refresh(self.as_ptr() as *mut c_void, erase_background, rect) }
    }
    fn refresh_rect(&self, rect: &Rect, erase_background: bool) {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            ffi::wxWindow_RefreshRect(self.as_ptr() as *mut c_void, rect, erase_background)
        }
    }
    fn update(&self) {
        unsafe { ffi::wxWindow_Update(self.as_ptr() as *mut c_void) }
    }
    fn set_background_colour(&self, colour: *const c_void) -> bool {
        unsafe { ffi::wxWindow_SetBackgroundColour(self.as_ptr() as *mut c_void, colour) }
    }
    // CXX_UNSUPPORTED: fn SetBackgroundStyle()
    fn is_transparent_background_supported(&self, reason: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsTransparentBackgroundSupported(self.as_ptr() as *mut c_void, reason) }
    }
    fn set_font(&self, font: *const c_void) -> bool {
        unsafe { ffi::wxWindow_SetFont(self.as_ptr() as *mut c_void, font) }
    }
    fn set_foreground_colour(&self, colour: *const c_void) -> bool {
        unsafe { ffi::wxWindow_SetForegroundColour(self.as_ptr() as *mut c_void, colour) }
    }
    fn set_own_background_colour(&self, colour: *const c_void) {
        unsafe { ffi::wxWindow_SetOwnBackgroundColour(self.as_ptr() as *mut c_void, colour) }
    }
    fn inherits_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsBackgroundColour(self.as_ptr() as *mut c_void) }
    }
    fn use_bg_col(&self) -> bool {
        unsafe { ffi::wxWindow_UseBgCol(self.as_ptr() as *mut c_void) }
    }
    fn use_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseBackgroundColour(self.as_ptr() as *mut c_void) }
    }
    fn set_own_font(&self, font: *const c_void) {
        unsafe { ffi::wxWindow_SetOwnFont(self.as_ptr() as *mut c_void, font) }
    }
    fn set_own_foreground_colour(&self, colour: *const c_void) {
        unsafe { ffi::wxWindow_SetOwnForegroundColour(self.as_ptr() as *mut c_void, colour) }
    }
    fn use_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseForegroundColour(self.as_ptr() as *mut c_void) }
    }
    fn inherits_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsForegroundColour(self.as_ptr() as *mut c_void) }
    }
    fn set_palette(&self, pal: *const c_void) {
        unsafe { ffi::wxWindow_SetPalette(self.as_ptr() as *mut c_void, pal) }
    }
    fn should_inherit_colours(&self) -> bool {
        unsafe { ffi::wxWindow_ShouldInheritColours(self.as_ptr() as *mut c_void) }
    }
    fn set_theme_enabled(&self, enable: bool) {
        unsafe { ffi::wxWindow_SetThemeEnabled(self.as_ptr() as *mut c_void, enable) }
    }
    fn get_theme_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_GetThemeEnabled(self.as_ptr() as *mut c_void) }
    }
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi::wxWindow_CanSetTransparent(self.as_ptr() as *mut c_void) }
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        unsafe { ffi::wxWindow_SetTransparent(self.as_ptr() as *mut c_void, alpha) }
    }
    fn get_event_handler(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetEventHandler(self.as_ptr() as *mut c_void) }
    }
    fn handle_as_navigation_key(&self, event: *const c_void) -> bool {
        unsafe { ffi::wxWindow_HandleAsNavigationKey(self.as_ptr() as *mut c_void, event) }
    }
    fn handle_window_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_HandleWindowEvent(self.as_ptr() as *mut c_void, event) }
    }
    fn process_window_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_ProcessWindowEvent(self.as_ptr() as *mut c_void, event) }
    }
    fn process_window_event_locally(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_ProcessWindowEventLocally(self.as_ptr() as *mut c_void, event) }
    }
    fn pop_event_handler(&self, delete_handler: bool) -> *mut c_void {
        unsafe { ffi::wxWindow_PopEventHandler(self.as_ptr() as *mut c_void, delete_handler) }
    }
    fn push_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_PushEventHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn remove_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) -> bool {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveEventHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn set_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetEventHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn set_next_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetNextHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn set_previous_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetPreviousHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn get_extra_style(&self) -> i32 {
        unsafe { ffi::wxWindow_GetExtraStyle(self.as_ptr() as *mut c_void) }
    }
    fn get_window_style_flag(&self) -> i32 {
        unsafe { ffi::wxWindow_GetWindowStyleFlag(self.as_ptr() as *mut c_void) }
    }
    fn get_window_style(&self) -> i32 {
        unsafe { ffi::wxWindow_GetWindowStyle(self.as_ptr() as *mut c_void) }
    }
    fn has_extra_style(&self, ex_flag: i32) -> bool {
        unsafe { ffi::wxWindow_HasExtraStyle(self.as_ptr() as *mut c_void, ex_flag) }
    }
    fn has_flag(&self, flag: i32) -> bool {
        unsafe { ffi::wxWindow_HasFlag(self.as_ptr() as *mut c_void, flag) }
    }
    fn set_extra_style(&self, ex_style: i32) {
        unsafe { ffi::wxWindow_SetExtraStyle(self.as_ptr() as *mut c_void, ex_style) }
    }
    fn set_window_style_flag(&self, style: i32) {
        unsafe { ffi::wxWindow_SetWindowStyleFlag(self.as_ptr() as *mut c_void, style) }
    }
    fn set_window_style(&self, style: i32) {
        unsafe { ffi::wxWindow_SetWindowStyle(self.as_ptr() as *mut c_void, style) }
    }
    fn toggle_window_style(&self, flag: i32) -> bool {
        unsafe { ffi::wxWindow_ToggleWindowStyle(self.as_ptr() as *mut c_void, flag) }
    }
    fn move_after_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveAfterInTabOrder(self.as_ptr() as *mut c_void, win)
        }
    }
    fn move_before_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveBeforeInTabOrder(self.as_ptr() as *mut c_void, win)
        }
    }
    fn navigate(&self, flags: i32) -> bool {
        unsafe { ffi::wxWindow_Navigate(self.as_ptr() as *mut c_void, flags) }
    }
    fn navigate_in(&self, flags: i32) -> bool {
        unsafe { ffi::wxWindow_NavigateIn(self.as_ptr() as *mut c_void, flags) }
    }
    fn lower(&self) {
        unsafe { ffi::wxWindow_Lower(self.as_ptr() as *mut c_void) }
    }
    fn raise(&self) {
        unsafe { ffi::wxWindow_Raise(self.as_ptr() as *mut c_void) }
    }
    fn hide(&self) -> bool {
        unsafe { ffi::wxWindow_Hide(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn HideWithEffect()
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsEnabled(self.as_ptr() as *mut c_void) }
    }
    fn is_exposed(&self, x: i32, y: i32) -> bool {
        unsafe { ffi::wxWindow_IsExposed(self.as_ptr() as *mut c_void, x, y) }
    }
    fn is_exposed1(&self, pt: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsExposed1(self.as_ptr() as *mut c_void, pt) }
    }
    fn is_exposed2(&self, x: i32, y: i32, w: i32, h: i32) -> bool {
        unsafe { ffi::wxWindow_IsExposed2(self.as_ptr() as *mut c_void, x, y, w, h) }
    }
    fn is_exposed3(&self, rect: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsExposed3(self.as_ptr() as *mut c_void, rect) }
    }
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxWindow_IsShown(self.as_ptr() as *mut c_void) }
    }
    fn is_shown_on_screen(&self) -> bool {
        unsafe { ffi::wxWindow_IsShownOnScreen(self.as_ptr() as *mut c_void) }
    }
    fn disable(&self) -> bool {
        unsafe { ffi::wxWindow_Disable(self.as_ptr() as *mut c_void) }
    }
    fn enable(&self, enable: bool) -> bool {
        unsafe { ffi::wxWindow_Enable(self.as_ptr() as *mut c_void, enable) }
    }
    fn show(&self, show: bool) -> bool {
        unsafe { ffi::wxWindow_Show(self.as_ptr() as *mut c_void, show) }
    }
    // CXX_UNSUPPORTED: fn ShowWithEffect()
    fn get_help_text(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetHelpText(self.as_ptr() as *mut c_void)) }
    }
    fn set_help_text(&self, help_text: &str) {
        unsafe {
            let help_text = crate::wx_string_from(help_text);
            ffi::wxWindow_SetHelpText(self.as_ptr() as *mut c_void, help_text)
        }
    }
    // CXX_UNSUPPORTED: fn GetHelpTextAtPoint()
    fn get_tool_tip(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetToolTip(self.as_ptr() as *mut c_void) }
    }
    fn get_tool_tip_text(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetToolTipText(self.as_ptr() as *mut c_void)) }
    }
    fn set_tool_tip(&self, tip_string: &str) {
        unsafe {
            let tip_string = crate::wx_string_from(tip_string);
            ffi::wxWindow_SetToolTip(self.as_ptr() as *mut c_void, tip_string)
        }
    }
    fn set_tool_tip1(&self, tip: *mut c_void) {
        unsafe { ffi::wxWindow_SetToolTip1(self.as_ptr() as *mut c_void, tip) }
    }
    fn unset_tool_tip(&self) {
        unsafe { ffi::wxWindow_UnsetToolTip(self.as_ptr() as *mut c_void) }
    }
    fn get_popup_menu_selection_from_user(&self, menu: *mut c_void, pos: &Point) -> i32 {
        unsafe {
            let pos = pos.as_ptr() as *mut c_void;
            ffi::wxWindow_GetPopupMenuSelectionFromUser(self.as_ptr() as *mut c_void, menu, pos)
        }
    }
    fn get_popup_menu_selection_from_user1(&self, menu: *mut c_void, x: i32, y: i32) -> i32 {
        unsafe { ffi::wxWindow_GetPopupMenuSelectionFromUser1(self.as_ptr() as *mut c_void, menu, x, y) }
    }
    fn popup_menu(&self, menu: *mut c_void, pos: &Point) -> bool {
        unsafe {
            let pos = pos.as_ptr() as *mut c_void;
            ffi::wxWindow_PopupMenu(self.as_ptr() as *mut c_void, menu, pos)
        }
    }
    fn popup_menu1(&self, menu: *mut c_void, x: i32, y: i32) -> bool {
        unsafe { ffi::wxWindow_PopupMenu1(self.as_ptr() as *mut c_void, menu, x, y) }
    }
    fn get_validator(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetValidator(self.as_ptr() as *mut c_void) }
    }
    fn set_validator(&self, validator: &Validator) {
        unsafe {
            let validator = validator.as_ptr() as *mut c_void;
            ffi::wxWindow_SetValidator(self.as_ptr() as *mut c_void, validator)
        }
    }
    fn transfer_data_from_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataFromWindow(self.as_ptr() as *mut c_void) }
    }
    fn transfer_data_to_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataToWindow(self.as_ptr() as *mut c_void) }
    }
    fn validate(&self) -> bool {
        unsafe { ffi::wxWindow_Validate(self.as_ptr() as *mut c_void) }
    }
    fn get_id(&self) -> i32 {
        unsafe { ffi::wxWindow_GetId(self.as_ptr() as *mut c_void) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetLabel(self.as_ptr() as *mut c_void)) }
    }
    // CXX_UNSUPPORTED: fn GetLayoutDirection()
    fn adjust_for_layout_direction(&self, x: i32, width: i32, width_total: i32) -> i32 {
        unsafe { ffi::wxWindow_AdjustForLayoutDirection(self.as_ptr() as *mut c_void, x, width, width_total) }
    }
    fn get_name(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetName(self.as_ptr() as *mut c_void)) }
    }
    // CXX_UNSUPPORTED: fn GetWindowVariant()
    fn set_id(&self, winid: i32) {
        unsafe { ffi::wxWindow_SetId(self.as_ptr() as *mut c_void, winid) }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = crate::wx_string_from(label);
            ffi::wxWindow_SetLabel(self.as_ptr() as *mut c_void, label)
        }
    }
    // CXX_UNSUPPORTED: fn SetLayoutDirection()
    fn set_name(&self, name: &str) {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxWindow_SetName(self.as_ptr() as *mut c_void, name)
        }
    }
    // CXX_UNSUPPORTED: fn SetWindowVariant()
    fn get_accelerator_table(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetAcceleratorTable(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn GetAccessible()
    fn set_accelerator_table(&self, accel: *const c_void) {
        unsafe { ffi::wxWindow_SetAcceleratorTable(self.as_ptr() as *mut c_void, accel) }
    }
    // CXX_UNSUPPORTED: fn SetAccessible()
    fn close(&self, force: bool) -> bool {
        unsafe { ffi::wxWindow_Close(self.as_ptr() as *mut c_void, force) }
    }
    fn destroy(&self) -> bool {
        unsafe { ffi::wxWindow_Destroy(self.as_ptr() as *mut c_void) }
    }
    fn is_being_deleted(&self) -> bool {
        unsafe { ffi::wxWindow_IsBeingDeleted(self.as_ptr() as *mut c_void) }
    }
    fn get_drop_target(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetDropTarget(self.as_ptr() as *mut c_void) }
    }
    fn set_drop_target(&self, target: *mut c_void) {
        unsafe { ffi::wxWindow_SetDropTarget(self.as_ptr() as *mut c_void, target) }
    }
    fn drag_accept_files(&self, accept: bool) {
        unsafe { ffi::wxWindow_DragAcceptFiles(self.as_ptr() as *mut c_void, accept) }
    }
    fn get_containing_sizer(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetContainingSizer(self.as_ptr() as *mut c_void) }
    }
    fn get_sizer(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetSizer(self.as_ptr() as *mut c_void) }
    }
    fn set_sizer(&self, sizer: *mut c_void, delete_old: bool) {
        unsafe { ffi::wxWindow_SetSizer(self.as_ptr() as *mut c_void, sizer, delete_old) }
    }
    fn set_sizer_and_fit(&self, sizer: *mut c_void, delete_old: bool) {
        unsafe { ffi::wxWindow_SetSizerAndFit(self.as_ptr() as *mut c_void, sizer, delete_old) }
    }
    fn get_constraints(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetConstraints(self.as_ptr() as *mut c_void) }
    }
    fn set_constraints(&self, constraints: *mut c_void) {
        unsafe { ffi::wxWindow_SetConstraints(self.as_ptr() as *mut c_void, constraints) }
    }
    fn layout(&self) -> bool {
        unsafe { ffi::wxWindow_Layout(self.as_ptr() as *mut c_void) }
    }
    fn set_auto_layout(&self, auto_layout: bool) {
        unsafe { ffi::wxWindow_SetAutoLayout(self.as_ptr() as *mut c_void, auto_layout) }
    }
    fn get_auto_layout(&self) -> bool {
        unsafe { ffi::wxWindow_GetAutoLayout(self.as_ptr() as *mut c_void) }
    }
    fn capture_mouse(&self) {
        unsafe { ffi::wxWindow_CaptureMouse(self.as_ptr() as *mut c_void) }
    }
    fn get_caret(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetCaret(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn GetCursor()
    fn has_capture(&self) -> bool {
        unsafe { ffi::wxWindow_HasCapture(self.as_ptr() as *mut c_void) }
    }
    fn release_mouse(&self) {
        unsafe { ffi::wxWindow_ReleaseMouse(self.as_ptr() as *mut c_void) }
    }
    fn set_caret(&self, caret: *mut c_void) {
        unsafe { ffi::wxWindow_SetCaret(self.as_ptr() as *mut c_void, caret) }
    }
    fn set_cursor(&self, cursor: *const c_void) -> bool {
        unsafe { ffi::wxWindow_SetCursor(self.as_ptr() as *mut c_void, cursor) }
    }
    fn warp_pointer(&self, x: i32, y: i32) {
        unsafe { ffi::wxWindow_WarpPointer(self.as_ptr() as *mut c_void, x, y) }
    }
    fn enable_touch_events(&self, events_mask: i32) -> bool {
        unsafe { ffi::wxWindow_EnableTouchEvents(self.as_ptr() as *mut c_void, events_mask) }
    }
    // CXX_UNSUPPORTED: fn HitTest()
    // CXX_UNSUPPORTED: fn HitTest1()
    // CXX_UNSUPPORTED: fn GetBorder()
    // CXX_UNSUPPORTED: fn GetBorder1()
    fn do_update_window_ui(&self, event: *mut c_void) {
        unsafe { ffi::wxWindow_DoUpdateWindowUI(self.as_ptr() as *mut c_void, event) }
    }
    // CXX_UNSUPPORTED: fn GetHandle()
    fn has_multiple_pages(&self) -> bool {
        unsafe { ffi::wxWindow_HasMultiplePages(self.as_ptr() as *mut c_void) }
    }
    fn inherit_attributes(&self) {
        unsafe { ffi::wxWindow_InheritAttributes(self.as_ptr() as *mut c_void) }
    }
    fn init_dialog(&self) {
        unsafe { ffi::wxWindow_InitDialog(self.as_ptr() as *mut c_void) }
    }
    fn is_double_buffered(&self) -> bool {
        unsafe { ffi::wxWindow_IsDoubleBuffered(self.as_ptr() as *mut c_void) }
    }
    fn set_double_buffered(&self, on: bool) {
        unsafe { ffi::wxWindow_SetDoubleBuffered(self.as_ptr() as *mut c_void, on) }
    }
    fn is_retained(&self) -> bool {
        unsafe { ffi::wxWindow_IsRetained(self.as_ptr() as *mut c_void) }
    }
    fn is_this_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsThisEnabled(self.as_ptr() as *mut c_void) }
    }
    fn is_top_level(&self) -> bool {
        unsafe { ffi::wxWindow_IsTopLevel(self.as_ptr() as *mut c_void) }
    }
    fn on_internal_idle(&self) {
        unsafe { ffi::wxWindow_OnInternalIdle(self.as_ptr() as *mut c_void) }
    }
    fn send_idle_events(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_SendIdleEvents(self.as_ptr() as *mut c_void, event) }
    }
    fn register_hot_key(&self, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool {
        unsafe { ffi::wxWindow_RegisterHotKey(self.as_ptr() as *mut c_void, hotkey_id, modifiers, virtual_key_code) }
    }
    fn unregister_hot_key(&self, hotkey_id: i32) -> bool {
        unsafe { ffi::wxWindow_UnregisterHotKey(self.as_ptr() as *mut c_void, hotkey_id) }
    }
    fn update_window_ui(&self, flags: i32) {
        unsafe { ffi::wxWindow_UpdateWindowUI(self.as_ptr() as *mut c_void, flags) }
    }
    // CXX_UNSUPPORTED: fn GetClassDefaultAttributes()
    fn find_focus() -> *mut c_void {
        unsafe { ffi::wxWindow_FindFocus() }
    }
    fn find_window_by_id<T: WindowMethods>(id: i32, parent: Option<&T>) -> *mut c_void {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FindWindowById(id, parent)
        }
    }
    fn find_window_by_label<T: WindowMethods>(label: &str, parent: Option<&T>) -> *mut c_void {
        unsafe {
            let label = crate::wx_string_from(label);
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FindWindowByLabel(label, parent)
        }
    }
    fn find_window_by_name<T: WindowMethods>(name: &str, parent: Option<&T>) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FindWindowByName(name, parent)
        }
    }
    fn get_capture() -> *mut c_void {
        unsafe { ffi::wxWindow_GetCapture() }
    }
    fn new_control_id(count: i32) -> i32 {
        unsafe { ffi::wxWindow_NewControlId(count) }
    }
    fn unreserve_control_id(id: i32, count: i32) {
        unsafe { ffi::wxWindow_UnreserveControlId(id, count) }
    }
    // DTOR: fn ~wxWindow()
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            ffi::wxWindow_Create(self.as_ptr() as *mut c_void, parent, id, pos, size, style, name)
        }
    }
}

// wxControl
wx_class! { Control(wxControl) impl
    ControlMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Control {
    pub fn new<T: WindowMethods>(parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, validator: &Validator, name: &str) -> Control {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let validator = validator.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            Control(ffi::wxControl_new(parent, id, pos, size, style, validator, name))
        }
    }
    pub fn new1() -> Control {
        unsafe { Control(ffi::wxControl_new1()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ControlMethods: WindowMethods {
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, validator: &Validator, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let validator = validator.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            ffi::wxControl_Create(self.as_ptr() as *mut c_void, parent, id, pos, size, style, validator, name)
        }
    }
    fn command(&self, event: *mut c_void) {
        unsafe { ffi::wxControl_Command(self.as_ptr() as *mut c_void, event) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi::wxControl_GetLabel(self.as_ptr() as *mut c_void)) }
    }
    fn get_label_text(&self) -> WxString {
        unsafe { WxString(ffi::wxControl_GetLabelText(self.as_ptr() as *mut c_void)) }
    }
    fn get_size_from_text_size(&self, xlen: i32, ylen: i32) -> Size {
        unsafe { Size(ffi::wxControl_GetSizeFromTextSize(self.as_ptr() as *mut c_void, xlen, ylen)) }
    }
    fn get_size_from_text_size1(&self, tsize: &Size) -> Size {
        unsafe {
            let tsize = tsize.as_ptr() as *mut c_void;
            Size(ffi::wxControl_GetSizeFromTextSize1(self.as_ptr() as *mut c_void, tsize))
        }
    }
    fn get_size_from_text(&self, text: &str) -> Size {
        unsafe {
            let text = crate::wx_string_from(text);
            Size(ffi::wxControl_GetSizeFromText(self.as_ptr() as *mut c_void, text))
        }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = crate::wx_string_from(label);
            ffi::wxControl_SetLabel(self.as_ptr() as *mut c_void, label)
        }
    }
    fn set_label_text(&self, text: &str) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxControl_SetLabelText(self.as_ptr() as *mut c_void, text)
        }
    }
    fn set_label_markup(&self, markup: &str) -> bool {
        unsafe {
            let markup = crate::wx_string_from(markup);
            ffi::wxControl_SetLabelMarkup(self.as_ptr() as *mut c_void, markup)
        }
    }
    fn get_label_text1(label: &str) -> WxString {
        unsafe {
            let label = crate::wx_string_from(label);
            WxString(ffi::wxControl_GetLabelText1(label))
        }
    }
    fn remove_mnemonics(str: &str) -> WxString {
        unsafe {
            let str = crate::wx_string_from(str);
            WxString(ffi::wxControl_RemoveMnemonics(str))
        }
    }
    fn escape_mnemonics(text: &str) -> WxString {
        unsafe {
            let text = crate::wx_string_from(text);
            WxString(ffi::wxControl_EscapeMnemonics(text))
        }
    }
    // BLOCKED: fn Ellipsize()
}

// wxAnyButton
wx_class! { AnyButton(wxAnyButton) impl
    AnyButtonMethods,
    ControlMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl AnyButton {
    pub fn new() -> AnyButton {
        unsafe { AnyButton(ffi::wxAnyButton_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait AnyButtonMethods: ControlMethods {
    // DTOR: fn ~wxAnyButton()
    // CXX_UNSUPPORTED: fn GetBitmap()
    // CXX_UNSUPPORTED: fn GetBitmapCurrent()
    // CXX_UNSUPPORTED: fn GetBitmapDisabled()
    // CXX_UNSUPPORTED: fn GetBitmapFocus()
    // CXX_UNSUPPORTED: fn GetBitmapLabel()
    // CXX_UNSUPPORTED: fn GetBitmapPressed()
    // CXX_UNSUPPORTED: fn SetBitmap()
    fn set_bitmap_current(&self, bitmap: *const c_void) {
        unsafe { ffi::wxAnyButton_SetBitmapCurrent(self.as_ptr() as *mut c_void, bitmap) }
    }
    fn set_bitmap_disabled(&self, bitmap: *const c_void) {
        unsafe { ffi::wxAnyButton_SetBitmapDisabled(self.as_ptr() as *mut c_void, bitmap) }
    }
    fn set_bitmap_focus(&self, bitmap: *const c_void) {
        unsafe { ffi::wxAnyButton_SetBitmapFocus(self.as_ptr() as *mut c_void, bitmap) }
    }
    fn set_bitmap_label(&self, bitmap: *const c_void) {
        unsafe { ffi::wxAnyButton_SetBitmapLabel(self.as_ptr() as *mut c_void, bitmap) }
    }
    fn set_bitmap_pressed(&self, bitmap: *const c_void) {
        unsafe { ffi::wxAnyButton_SetBitmapPressed(self.as_ptr() as *mut c_void, bitmap) }
    }
    fn get_bitmap_margins(&self) -> Size {
        unsafe { Size(ffi::wxAnyButton_GetBitmapMargins(self.as_ptr() as *mut c_void)) }
    }
    fn set_bitmap_margins(&self, x: i32, y: i32) {
        unsafe { ffi::wxAnyButton_SetBitmapMargins(self.as_ptr() as *mut c_void, x, y) }
    }
    fn set_bitmap_margins1(&self, sz: &Size) {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            ffi::wxAnyButton_SetBitmapMargins1(self.as_ptr() as *mut c_void, sz)
        }
    }
    // CXX_UNSUPPORTED: fn SetBitmapPosition()
}

// wxButton
wx_class! { Button(wxButton) impl
    ButtonMethods,
    AnyButtonMethods,
    ControlMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Button {
    pub fn new() -> Button {
        unsafe { Button(ffi::wxButton_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, label: &str, pos: &Point, size: &Size, style: i32, validator: &Validator, name: &str) -> Button {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let label = crate::wx_string_from(label);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let validator = validator.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            Button(ffi::wxButton_new1(parent, id, label, pos, size, style, validator, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ButtonMethods: AnyButtonMethods {
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, label: &str, pos: &Point, size: &Size, style: i32, validator: &Validator, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let label = crate::wx_string_from(label);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let validator = validator.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            ffi::wxButton_Create(self.as_ptr() as *mut c_void, parent, id, label, pos, size, style, validator, name)
        }
    }
    fn get_auth_needed(&self) -> bool {
        unsafe { ffi::wxButton_GetAuthNeeded(self.as_ptr() as *mut c_void) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi::wxButton_GetLabel(self.as_ptr() as *mut c_void)) }
    }
    fn set_auth_needed(&self, needed: bool) {
        unsafe { ffi::wxButton_SetAuthNeeded(self.as_ptr() as *mut c_void, needed) }
    }
    fn set_default(&self) -> *mut c_void {
        unsafe { ffi::wxButton_SetDefault(self.as_ptr() as *mut c_void) }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = crate::wx_string_from(label);
            ffi::wxButton_SetLabel(self.as_ptr() as *mut c_void, label)
        }
    }
    fn get_default_size<T: WindowMethods>(win: Option<&T>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            Size(ffi::wxButton_GetDefaultSize(win))
        }
    }
}

// wxNonOwnedWindow
wx_class! { NonOwnedWindow(wxNonOwnedWindow) impl
    NonOwnedWindowMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl NonOwnedWindow {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait NonOwnedWindowMethods: WindowMethods {
    fn set_shape(&self, region: *const c_void) -> bool {
        unsafe { ffi::wxNonOwnedWindow_SetShape(self.as_ptr() as *mut c_void, region) }
    }
    fn set_shape1(&self, path: *const c_void) -> bool {
        unsafe { ffi::wxNonOwnedWindow_SetShape1(self.as_ptr() as *mut c_void, path) }
    }
}

// wxTopLevelWindow
wx_class! { TopLevelWindow(wxTopLevelWindow) impl
    TopLevelWindowMethods,
    NonOwnedWindowMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl TopLevelWindow {
    pub fn new() -> TopLevelWindow {
        unsafe { TopLevelWindow(ffi::wxTopLevelWindow_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> TopLevelWindow {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            TopLevelWindow(ffi::wxTopLevelWindow_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait TopLevelWindowMethods: NonOwnedWindowMethods {
    // DTOR: fn ~wxTopLevelWindow()
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            ffi::wxTopLevelWindow_Create(self.as_ptr() as *mut c_void, parent, id, title, pos, size, style, name)
        }
    }
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_CanSetTransparent(self.as_ptr() as *mut c_void) }
    }
    fn center_on_screen(&self, direction: i32) {
        unsafe { ffi::wxTopLevelWindow_CenterOnScreen(self.as_ptr() as *mut c_void, direction) }
    }
    fn centre_on_screen(&self, direction: i32) {
        unsafe { ffi::wxTopLevelWindow_CentreOnScreen(self.as_ptr() as *mut c_void, direction) }
    }
    fn enable_close_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableCloseButton(self.as_ptr() as *mut c_void, enable) }
    }
    fn enable_maximize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMaximizeButton(self.as_ptr() as *mut c_void, enable) }
    }
    fn enable_minimize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMinimizeButton(self.as_ptr() as *mut c_void, enable) }
    }
    fn get_default_item(&self) -> *mut c_void {
        unsafe { ffi::wxTopLevelWindow_GetDefaultItem(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn GetIcon()
    // BLOCKED: fn GetIcons()
    fn get_title(&self) -> WxString {
        unsafe { WxString(ffi::wxTopLevelWindow_GetTitle(self.as_ptr() as *mut c_void)) }
    }
    fn iconize(&self, iconize: bool) {
        unsafe { ffi::wxTopLevelWindow_Iconize(self.as_ptr() as *mut c_void, iconize) }
    }
    fn is_active(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsActive(self.as_ptr() as *mut c_void) }
    }
    fn is_always_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsAlwaysMaximized(self.as_ptr() as *mut c_void) }
    }
    fn is_full_screen(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsFullScreen(self.as_ptr() as *mut c_void) }
    }
    fn is_iconized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsIconized(self.as_ptr() as *mut c_void) }
    }
    fn is_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsMaximized(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn IsUsingNativeDecorations()
    fn layout(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_Layout(self.as_ptr() as *mut c_void) }
    }
    fn maximize(&self, maximize: bool) {
        unsafe { ffi::wxTopLevelWindow_Maximize(self.as_ptr() as *mut c_void, maximize) }
    }
    // BLOCKED: fn MSWGetSystemMenu()
    fn request_user_attention(&self, flags: i32) {
        unsafe { ffi::wxTopLevelWindow_RequestUserAttention(self.as_ptr() as *mut c_void, flags) }
    }
    fn restore(&self) {
        unsafe { ffi::wxTopLevelWindow_Restore(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn RestoreToGeometry()
    // BLOCKED: fn SaveGeometry()
    fn set_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut c_void {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxTopLevelWindow_SetDefaultItem(self.as_ptr() as *mut c_void, win)
        }
    }
    fn set_tmp_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut c_void {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxTopLevelWindow_SetTmpDefaultItem(self.as_ptr() as *mut c_void, win)
        }
    }
    fn get_tmp_default_item(&self) -> *mut c_void {
        unsafe { ffi::wxTopLevelWindow_GetTmpDefaultItem(self.as_ptr() as *mut c_void) }
    }
    fn set_icon(&self, icon: *const c_void) {
        unsafe { ffi::wxTopLevelWindow_SetIcon(self.as_ptr() as *mut c_void, icon) }
    }
    fn set_icons(&self, icons: *const c_void) {
        unsafe { ffi::wxTopLevelWindow_SetIcons(self.as_ptr() as *mut c_void, icons) }
    }
    fn set_max_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxTopLevelWindow_SetMaxSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_min_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxTopLevelWindow_SetMinSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_size_hints(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        unsafe { ffi::wxTopLevelWindow_SetSizeHints(self.as_ptr() as *mut c_void, min_w, min_h, max_w, max_h, inc_w, inc_h) }
    }
    fn set_size_hints1(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        unsafe {
            let min_size = min_size.as_ptr() as *mut c_void;
            let max_size = max_size.as_ptr() as *mut c_void;
            let inc_size = inc_size.as_ptr() as *mut c_void;
            ffi::wxTopLevelWindow_SetSizeHints1(self.as_ptr() as *mut c_void, min_size, max_size, inc_size)
        }
    }
    fn set_title(&self, title: &str) {
        unsafe {
            let title = crate::wx_string_from(title);
            ffi::wxTopLevelWindow_SetTitle(self.as_ptr() as *mut c_void, title)
        }
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        unsafe { ffi::wxTopLevelWindow_SetTransparent(self.as_ptr() as *mut c_void, alpha) }
    }
    fn should_prevent_app_exit(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShouldPreventAppExit(self.as_ptr() as *mut c_void) }
    }
    fn osx_set_modified(&self, modified: bool) {
        unsafe { ffi::wxTopLevelWindow_OSXSetModified(self.as_ptr() as *mut c_void, modified) }
    }
    fn osx_is_modified(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_OSXIsModified(self.as_ptr() as *mut c_void) }
    }
    fn set_represented_filename(&self, filename: &str) {
        unsafe {
            let filename = crate::wx_string_from(filename);
            ffi::wxTopLevelWindow_SetRepresentedFilename(self.as_ptr() as *mut c_void, filename)
        }
    }
    fn show_without_activating(&self) {
        unsafe { ffi::wxTopLevelWindow_ShowWithoutActivating(self.as_ptr() as *mut c_void) }
    }
    fn enable_full_screen_view(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableFullScreenView(self.as_ptr() as *mut c_void, enable) }
    }
    fn show_full_screen(&self, show: bool, style: i32) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShowFullScreen(self.as_ptr() as *mut c_void, show, style) }
    }
    // BLOCKED: fn UseNativeDecorations()
    // BLOCKED: fn UseNativeDecorationsByDefault()
    fn get_default_size() -> Size {
        unsafe { Size(ffi::wxTopLevelWindow_GetDefaultSize()) }
    }
}

// wxFrame
wx_class! { Frame(wxFrame) impl
    FrameMethods,
    TopLevelWindowMethods,
    NonOwnedWindowMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Frame {
    pub fn new() -> Frame {
        unsafe { Frame(ffi::wxFrame_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> Frame {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            Frame(ffi::wxFrame_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait FrameMethods: TopLevelWindowMethods {
    // DTOR: fn ~wxFrame()
    fn centre(&self, direction: i32) {
        unsafe { ffi::wxFrame_Centre(self.as_ptr() as *mut c_void, direction) }
    }
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            ffi::wxFrame_Create(self.as_ptr() as *mut c_void, parent, id, title, pos, size, style, name)
        }
    }
    fn create_status_bar(&self, number: i32, style: i32, id: i32, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_CreateStatusBar(self.as_ptr() as *mut c_void, number, style, id, name)
        }
    }
    fn create_tool_bar(&self, style: i32, id: i32, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_CreateToolBar(self.as_ptr() as *mut c_void, style, id, name)
        }
    }
    fn do_give_help(&self, text: &str, show: bool) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxFrame_DoGiveHelp(self.as_ptr() as *mut c_void, text, show)
        }
    }
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point(ffi::wxFrame_GetClientAreaOrigin(self.as_ptr() as *mut c_void)) }
    }
    fn get_menu_bar(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_GetMenuBar(self.as_ptr() as *mut c_void) }
    }
    fn get_status_bar(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_GetStatusBar(self.as_ptr() as *mut c_void) }
    }
    fn get_status_bar_pane(&self) -> i32 {
        unsafe { ffi::wxFrame_GetStatusBarPane(self.as_ptr() as *mut c_void) }
    }
    fn get_tool_bar(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_GetToolBar(self.as_ptr() as *mut c_void) }
    }
    fn on_create_status_bar(&self, number: i32, style: i32, id: i32, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_OnCreateStatusBar(self.as_ptr() as *mut c_void, number, style, id, name)
        }
    }
    fn on_create_tool_bar(&self, style: i32, id: i32, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_OnCreateToolBar(self.as_ptr() as *mut c_void, style, id, name)
        }
    }
    fn process_command(&self, id: i32) -> bool {
        unsafe { ffi::wxFrame_ProcessCommand(self.as_ptr() as *mut c_void, id) }
    }
    fn set_menu_bar(&self, menu_bar: *mut c_void) {
        unsafe { ffi::wxFrame_SetMenuBar(self.as_ptr() as *mut c_void, menu_bar) }
    }
    fn set_status_bar(&self, status_bar: *mut c_void) {
        unsafe { ffi::wxFrame_SetStatusBar(self.as_ptr() as *mut c_void, status_bar) }
    }
    fn set_status_bar_pane(&self, n: i32) {
        unsafe { ffi::wxFrame_SetStatusBarPane(self.as_ptr() as *mut c_void, n) }
    }
    fn set_status_text(&self, text: &str, number: i32) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxFrame_SetStatusText(self.as_ptr() as *mut c_void, text, number)
        }
    }
    fn set_status_widths(&self, n: i32, widths_field: *const c_void) {
        unsafe { ffi::wxFrame_SetStatusWidths(self.as_ptr() as *mut c_void, n, widths_field) }
    }
    fn set_tool_bar(&self, tool_bar: *mut c_void) {
        unsafe { ffi::wxFrame_SetToolBar(self.as_ptr() as *mut c_void, tool_bar) }
    }
    // BLOCKED: fn MSWGetTaskBarButton()
    fn push_status_text(&self, text: &str, number: i32) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxFrame_PushStatusText(self.as_ptr() as *mut c_void, text, number)
        }
    }
    fn pop_status_text(&self, number: i32) {
        unsafe { ffi::wxFrame_PopStatusText(self.as_ptr() as *mut c_void, number) }
    }
}

// wxPoint
wx_class! { Point(wxPoint) impl
    PointMethods
}
impl Point {
    pub fn new() -> Point {
        unsafe { Point(ffi::wxPoint_new()) }
    }
    pub fn new1(x: i32, y: i32) -> Point {
        unsafe { Point(ffi::wxPoint_new1(x, y)) }
    }
    pub fn new2(pt: *const c_void) -> Point {
        unsafe { Point(ffi::wxPoint_new2(pt)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait PointMethods: WxRustMethods {
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxPoint_IsFullySpecified(self.as_ptr() as *mut c_void) }
    }
    fn set_defaults(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi::wxPoint_SetDefaults(self.as_ptr() as *mut c_void, pt)
        }
    }
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator+1()
    // BLOCKED: fn operator-1()
    // BLOCKED: fn operator+2()
    // BLOCKED: fn operator-2()
    // BLOCKED: fn operator+=1()
    // BLOCKED: fn operator-=1()
    // BLOCKED: fn operator/()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*1()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator*=()
}

// wxRect
wx_class! { Rect(wxRect) impl
    RectMethods
}
impl Rect {
    pub fn new() -> Rect {
        unsafe { Rect(ffi::wxRect_new()) }
    }
    pub fn new1(x: i32, y: i32, width: i32, height: i32) -> Rect {
        unsafe { Rect(ffi::wxRect_new1(x, y, width, height)) }
    }
    pub fn new2(top_left: &Point, bottom_right: &Point) -> Rect {
        unsafe {
            let top_left = top_left.as_ptr() as *mut c_void;
            let bottom_right = bottom_right.as_ptr() as *mut c_void;
            Rect(ffi::wxRect_new2(top_left, bottom_right))
        }
    }
    pub fn new3(pos: &Point, size: &Size) -> Rect {
        unsafe {
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            Rect(ffi::wxRect_new3(pos, size))
        }
    }
    pub fn new4(size: &Size) -> Rect {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            Rect(ffi::wxRect_new4(size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait RectMethods: WxRustMethods {
    fn centre_in(&self, r: &Rect, dir: i32) -> Rect {
        unsafe {
            let r = r.as_ptr() as *mut c_void;
            Rect(ffi::wxRect_CentreIn(self.as_ptr() as *mut c_void, r, dir))
        }
    }
    fn center_in(&self, r: &Rect, dir: i32) -> Rect {
        unsafe {
            let r = r.as_ptr() as *mut c_void;
            Rect(ffi::wxRect_CenterIn(self.as_ptr() as *mut c_void, r, dir))
        }
    }
    fn contains(&self, x: i32, y: i32) -> bool {
        unsafe { ffi::wxRect_Contains(self.as_ptr() as *mut c_void, x, y) }
    }
    fn contains1(&self, pt: &Point) -> bool {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi::wxRect_Contains1(self.as_ptr() as *mut c_void, pt)
        }
    }
    fn contains2(&self, rect: &Rect) -> bool {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            ffi::wxRect_Contains2(self.as_ptr() as *mut c_void, rect)
        }
    }
    // BLOCKED: fn Deflate()
    // BLOCKED: fn Deflate1()
    // BLOCKED: fn Deflate2()
    fn deflate3(&self, dx: i32, dy: i32) -> Rect {
        unsafe { Rect(ffi::wxRect_Deflate3(self.as_ptr() as *mut c_void, dx, dy)) }
    }
    fn get_bottom(&self) -> i32 {
        unsafe { ffi::wxRect_GetBottom(self.as_ptr() as *mut c_void) }
    }
    fn get_bottom_left(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetBottomLeft(self.as_ptr() as *mut c_void)) }
    }
    fn get_bottom_right(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetBottomRight(self.as_ptr() as *mut c_void)) }
    }
    fn get_height(&self) -> i32 {
        unsafe { ffi::wxRect_GetHeight(self.as_ptr() as *mut c_void) }
    }
    fn get_left(&self) -> i32 {
        unsafe { ffi::wxRect_GetLeft(self.as_ptr() as *mut c_void) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetPosition(self.as_ptr() as *mut c_void)) }
    }
    fn get_right(&self) -> i32 {
        unsafe { ffi::wxRect_GetRight(self.as_ptr() as *mut c_void) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size(ffi::wxRect_GetSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_top(&self) -> i32 {
        unsafe { ffi::wxRect_GetTop(self.as_ptr() as *mut c_void) }
    }
    fn get_top_left(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetTopLeft(self.as_ptr() as *mut c_void)) }
    }
    fn get_top_right(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetTopRight(self.as_ptr() as *mut c_void)) }
    }
    fn get_width(&self) -> i32 {
        unsafe { ffi::wxRect_GetWidth(self.as_ptr() as *mut c_void) }
    }
    fn get_x(&self) -> i32 {
        unsafe { ffi::wxRect_GetX(self.as_ptr() as *mut c_void) }
    }
    fn get_y(&self) -> i32 {
        unsafe { ffi::wxRect_GetY(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn Inflate()
    // BLOCKED: fn Inflate1()
    // BLOCKED: fn Inflate2()
    fn inflate3(&self, dx: i32, dy: i32) -> Rect {
        unsafe { Rect(ffi::wxRect_Inflate3(self.as_ptr() as *mut c_void, dx, dy)) }
    }
    // BLOCKED: fn Intersect()
    fn intersect1(&self, rect: &Rect) -> Rect {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            Rect(ffi::wxRect_Intersect1(self.as_ptr() as *mut c_void, rect))
        }
    }
    fn intersects(&self, rect: &Rect) -> bool {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            ffi::wxRect_Intersects(self.as_ptr() as *mut c_void, rect)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRect_IsEmpty(self.as_ptr() as *mut c_void) }
    }
    fn offset(&self, dx: i32, dy: i32) {
        unsafe { ffi::wxRect_Offset(self.as_ptr() as *mut c_void, dx, dy) }
    }
    fn offset1(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi::wxRect_Offset1(self.as_ptr() as *mut c_void, pt)
        }
    }
    fn set_height(&self, height: i32) {
        unsafe { ffi::wxRect_SetHeight(self.as_ptr() as *mut c_void, height) }
    }
    fn set_position(&self, pos: &Point) {
        unsafe {
            let pos = pos.as_ptr() as *mut c_void;
            ffi::wxRect_SetPosition(self.as_ptr() as *mut c_void, pos)
        }
    }
    fn set_size(&self, s: &Size) {
        unsafe {
            let s = s.as_ptr() as *mut c_void;
            ffi::wxRect_SetSize(self.as_ptr() as *mut c_void, s)
        }
    }
    fn set_width(&self, width: i32) {
        unsafe { ffi::wxRect_SetWidth(self.as_ptr() as *mut c_void, width) }
    }
    fn set_x(&self, x: i32) {
        unsafe { ffi::wxRect_SetX(self.as_ptr() as *mut c_void, x) }
    }
    fn set_y(&self, y: i32) {
        unsafe { ffi::wxRect_SetY(self.as_ptr() as *mut c_void, y) }
    }
    fn set_left(&self, left: i32) {
        unsafe { ffi::wxRect_SetLeft(self.as_ptr() as *mut c_void, left) }
    }
    fn set_right(&self, right: i32) {
        unsafe { ffi::wxRect_SetRight(self.as_ptr() as *mut c_void, right) }
    }
    fn set_top(&self, top: i32) {
        unsafe { ffi::wxRect_SetTop(self.as_ptr() as *mut c_void, top) }
    }
    fn set_bottom(&self, bottom: i32) {
        unsafe { ffi::wxRect_SetBottom(self.as_ptr() as *mut c_void, bottom) }
    }
    fn set_top_left(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut c_void;
            ffi::wxRect_SetTopLeft(self.as_ptr() as *mut c_void, p)
        }
    }
    fn set_bottom_right(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut c_void;
            ffi::wxRect_SetBottomRight(self.as_ptr() as *mut c_void, p)
        }
    }
    fn set_top_right(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut c_void;
            ffi::wxRect_SetTopRight(self.as_ptr() as *mut c_void, p)
        }
    }
    fn set_bottom_left(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut c_void;
            ffi::wxRect_SetBottomLeft(self.as_ptr() as *mut c_void, p)
        }
    }
    fn union(&self, rect: &Rect) -> Rect {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            Rect(ffi::wxRect_Union(self.as_ptr() as *mut c_void, rect))
        }
    }
    // BLOCKED: fn Union1()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
}

// wxSize
wx_class! { Size(wxSize) impl
    SizeMethods
}
impl Size {
    pub fn new() -> Size {
        unsafe { Size(ffi::wxSize_new()) }
    }
    pub fn new1(width: i32, height: i32) -> Size {
        unsafe { Size(ffi::wxSize_new1(width, height)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait SizeMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator/()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*1()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator*=()
    fn dec_by(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi::wxSize_DecBy(self.as_ptr() as *mut c_void, pt)
        }
    }
    fn dec_by1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxSize_DecBy1(self.as_ptr() as *mut c_void, size)
        }
    }
    fn dec_by2(&self, dx: i32, dy: i32) {
        unsafe { ffi::wxSize_DecBy2(self.as_ptr() as *mut c_void, dx, dy) }
    }
    fn dec_by3(&self, d: i32) {
        unsafe { ffi::wxSize_DecBy3(self.as_ptr() as *mut c_void, d) }
    }
    fn dec_to(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxSize_DecTo(self.as_ptr() as *mut c_void, size)
        }
    }
    fn dec_to_if_specified(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxSize_DecToIfSpecified(self.as_ptr() as *mut c_void, size)
        }
    }
    fn get_height(&self) -> i32 {
        unsafe { ffi::wxSize_GetHeight(self.as_ptr() as *mut c_void) }
    }
    fn get_width(&self) -> i32 {
        unsafe { ffi::wxSize_GetWidth(self.as_ptr() as *mut c_void) }
    }
    fn inc_by(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi::wxSize_IncBy(self.as_ptr() as *mut c_void, pt)
        }
    }
    fn inc_by1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxSize_IncBy1(self.as_ptr() as *mut c_void, size)
        }
    }
    fn inc_by2(&self, dx: i32, dy: i32) {
        unsafe { ffi::wxSize_IncBy2(self.as_ptr() as *mut c_void, dx, dy) }
    }
    fn inc_by3(&self, d: i32) {
        unsafe { ffi::wxSize_IncBy3(self.as_ptr() as *mut c_void, d) }
    }
    fn inc_to(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi::wxSize_IncTo(self.as_ptr() as *mut c_void, size)
        }
    }
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxSize_IsFullySpecified(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn Scale()
    fn set(&self, width: i32, height: i32) {
        unsafe { ffi::wxSize_Set(self.as_ptr() as *mut c_void, width, height) }
    }
    fn set_defaults(&self, size_default: &Size) {
        unsafe {
            let size_default = size_default.as_ptr() as *mut c_void;
            ffi::wxSize_SetDefaults(self.as_ptr() as *mut c_void, size_default)
        }
    }
    fn set_height(&self, height: i32) {
        unsafe { ffi::wxSize_SetHeight(self.as_ptr() as *mut c_void, height) }
    }
    fn set_width(&self, width: i32) {
        unsafe { ffi::wxSize_SetWidth(self.as_ptr() as *mut c_void, width) }
    }
}

// wxValidator
wx_class! { Validator(wxValidator) impl
    ValidatorMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Validator {
    pub fn new() -> Validator {
        unsafe { Validator(ffi::wxValidator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ValidatorMethods: EvtHandlerMethods {
    // DTOR: fn ~wxValidator()
    fn clone(&self) -> *mut c_void {
        unsafe { ffi::wxValidator_Clone(self.as_ptr() as *mut c_void) }
    }
    fn get_window(&self) -> *mut c_void {
        unsafe { ffi::wxValidator_GetWindow(self.as_ptr() as *mut c_void) }
    }
    fn set_window<T: WindowMethods>(&self, window: Option<&T>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxValidator_SetWindow(self.as_ptr() as *mut c_void, window)
        }
    }
    fn transfer_from_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferFromWindow(self.as_ptr() as *mut c_void) }
    }
    fn transfer_to_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferToWindow(self.as_ptr() as *mut c_void) }
    }
    fn validate<T: WindowMethods>(&self, parent: Option<&T>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi::wxValidator_Validate(self.as_ptr() as *mut c_void, parent)
        }
    }
    fn suppress_bell_on_error(suppress: bool) {
        unsafe { ffi::wxValidator_SuppressBellOnError(suppress) }
    }
    fn is_silent() -> bool {
        unsafe { ffi::wxValidator_IsSilent() }
    }
}


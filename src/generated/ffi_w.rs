use super::*;

extern "C" {

    // BWindow
    pub fn BWindow_AddShortcut(self_: *mut c_void, key: u32, modifiers: u32, message: *mut c_void);
    pub fn BWindow_AddShortcut1(
        self_: *mut c_void,
        key: u32,
        modifiers: u32,
        message: *mut c_void,
        target: *mut c_void,
    );
    pub fn BWindow_HasShortcut(self_: *mut c_void, key: u32, modifiers: u32) -> bool;
    pub fn BWindow_RemoveShortcut(self_: *mut c_void, key: u32, modifiers: u32);
    pub fn BWindow_new(archive: *mut c_void) -> *mut c_void;
    pub fn BWindow_new1(
        frame: *mut c_void,
        title: *const c_char,
        look: window_look,
        feel: window_feel,
        flags: u32,
        workspace: u32,
    ) -> *mut c_void;
    pub fn BWindow_new2(
        frame: *mut c_void,
        title: *const c_char,
        type_: window_type,
        flags: u32,
        workspace: u32,
    ) -> *mut c_void;
    // DTOR: pub fn BWindow_~BWindow(self_: *mut c_void);
    pub fn BWindow_Activate(self_: *mut c_void, active: bool);
    pub fn BWindow_AddChild(self_: *mut c_void, child: *mut c_void);
    pub fn BWindow_AddChild1(self_: *mut c_void, child: *mut c_void, before: *mut c_void);
    pub fn BWindow_AddToSubset(self_: *mut c_void, window: *mut c_void) -> status_t;
    pub fn BWindow_BeginViewTransaction(self_: *mut c_void);
    pub fn BWindow_Bounds(self_: *const c_void) -> *mut c_void;
    pub fn BWindow_CenterIn(self_: *mut c_void, rect: *const c_void);
    pub fn BWindow_CenterOnScreen(self_: *mut c_void);
    pub fn BWindow_CenterOnScreen1(self_: *mut c_void, id: screen_id);
    pub fn BWindow_ChildAt(self_: *const c_void, index: i32) -> *mut c_void;
    pub fn BWindow_Close(self_: *mut c_void);
    pub fn BWindow_ConvertFromScreen(self_: *const c_void, point: *mut c_void);
    // BLOCKED: pub fn BWindow_ConvertFromScreen1(self_: *const c_void, point: *mut c_void) -> *mut c_void;
    pub fn BWindow_ConvertFromScreen2(self_: *const c_void, rect: *mut c_void);
    // BLOCKED: pub fn BWindow_ConvertFromScreen3(self_: *const c_void, rect: *mut c_void) -> *mut c_void;
    pub fn BWindow_ConvertToScreen(self_: *const c_void, point: *mut c_void);
    // BLOCKED: pub fn BWindow_ConvertToScreen1(self_: *const c_void, point: *mut c_void) -> *mut c_void;
    pub fn BWindow_ConvertToScreen2(self_: *const c_void, rect: *mut c_void);
    // BLOCKED: pub fn BWindow_ConvertToScreen3(self_: *const c_void, rect: *mut c_void) -> *mut c_void;
    pub fn BWindow_CountChildren(self_: *const c_void) -> i32;
    pub fn BWindow_CurrentFocus(self_: *const c_void) -> *mut c_void;
    pub fn BWindow_DecoratorFrame(self_: *const c_void) -> *mut c_void;
    pub fn BWindow_DefaultButton(self_: *const c_void) -> *mut c_void;
    pub fn BWindow_DisableUpdates(self_: *mut c_void);
    pub fn BWindow_EnableUpdates(self_: *mut c_void);
    pub fn BWindow_EndViewTransaction(self_: *mut c_void);
    pub fn BWindow_Feel(self_: *const c_void) -> window_feel;
    pub fn BWindow_FindView(self_: *const c_void, point: *mut c_void) -> *mut c_void;
    pub fn BWindow_FindView1(self_: *const c_void, view_name: *const c_char) -> *mut c_void;
    pub fn BWindow_Flags(self_: *const c_void) -> u32;
    pub fn BWindow_Flush(self_: *const c_void);
    pub fn BWindow_Frame(self_: *const c_void) -> *mut c_void;
    pub fn BWindow_FrameMoved(self_: *mut c_void, new_position: *mut c_void);
    pub fn BWindow_FrameResized(self_: *mut c_void, new_width: c_float, new_height: c_float);
    pub fn BWindow_GetDecoratorSettings(self_: *const c_void, settings: *mut c_void) -> status_t;
    pub fn BWindow_GetLayout(self_: *const c_void) -> *mut c_void;
    pub fn BWindow_GetSizeLimits(
        self_: *mut c_void,
        min_width: *mut c_void,
        max_width: *mut c_void,
        min_height: *mut c_void,
        max_height: *mut c_void,
    );
    pub fn BWindow_GetWindowAlignment(
        self_: *const c_void,
        mode: *mut c_void,
        h: *mut c_void,
        h_offset: *mut c_void,
        width: *mut c_void,
        width_offset: *mut c_void,
        v: *mut c_void,
        v_offset: *mut c_void,
        height: *mut c_void,
        height_offset: *mut c_void,
    ) -> status_t;
    pub fn BWindow_Hide(self_: *mut c_void);
    pub fn BWindow_InvalidateLayout(self_: *mut c_void, descendants: bool);
    pub fn BWindow_InViewTransaction(self_: *const c_void) -> bool;
    pub fn BWindow_IsActive(self_: *const c_void) -> bool;
    pub fn BWindow_IsFloating(self_: *const c_void) -> bool;
    pub fn BWindow_IsFront(self_: *const c_void) -> bool;
    pub fn BWindow_IsHidden(self_: *const c_void) -> bool;
    pub fn BWindow_IsMinimized(self_: *const c_void) -> bool;
    pub fn BWindow_IsModal(self_: *const c_void) -> bool;
    pub fn BWindow_IsOffscreenWindow(self_: *const c_void) -> bool;
    pub fn BWindow_KeyMenuBar(self_: *const c_void) -> *mut c_void;
    pub fn BWindow_LastMouseMovedView(self_: *const c_void) -> *mut c_void;
    pub fn BWindow_Layout(self_: *mut c_void, force: bool);
    pub fn BWindow_Look(self_: *const c_void) -> window_look;
    pub fn BWindow_MenusBeginning(self_: *mut c_void);
    pub fn BWindow_MenusEnded(self_: *mut c_void);
    pub fn BWindow_Minimize(self_: *mut c_void, minimize: bool);
    pub fn BWindow_MoveBy(self_: *mut c_void, dx: c_float, dy: c_float);
    pub fn BWindow_MoveOnScreen(self_: *mut c_void, flags: u32);
    pub fn BWindow_MoveTo(self_: *mut c_void, point: *mut c_void);
    pub fn BWindow_MoveTo1(self_: *mut c_void, x: c_float, y: c_float);
    pub fn BWindow_NeedsUpdate(self_: *const c_void) -> bool;
    pub fn BWindow_PulseRate(self_: *const c_void) -> bigtime_t;
    pub fn BWindow_RemoveChild(self_: *mut c_void, child: *mut c_void) -> bool;
    pub fn BWindow_RemoveFromSubset(self_: *mut c_void, window: *mut c_void) -> status_t;
    pub fn BWindow_ResizeBy(self_: *mut c_void, dx: c_float, dy: c_float);
    pub fn BWindow_ResizeTo(self_: *mut c_void, width: c_float, height: c_float);
    pub fn BWindow_ResizeToPreferred(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn BWindow_ScreenChanged(self_: *mut c_void, screen_size: *mut c_void, depth: color_space);
    pub fn BWindow_SendBehind(self_: *mut c_void, window: *const c_void) -> status_t;
    pub fn BWindow_SetDecoratorSettings(self_: *mut c_void, settings: *const c_void) -> status_t;
    pub fn BWindow_SetDefaultButton(self_: *mut c_void, button: *mut c_void);
    pub fn BWindow_SetFeel(self_: *mut c_void, feel: window_feel) -> status_t;
    pub fn BWindow_SetFlags(self_: *mut c_void, flags: u32) -> status_t;
    pub fn BWindow_SetKeyMenuBar(self_: *mut c_void, bar: *mut c_void);
    pub fn BWindow_SetLayout(self_: *mut c_void, layout: *mut c_void);
    pub fn BWindow_SetLook(self_: *mut c_void, look: window_look) -> status_t;
    pub fn BWindow_SetPulseRate(self_: *mut c_void, rate: bigtime_t);
    pub fn BWindow_SetSizeLimits(
        self_: *mut c_void,
        min_width: c_float,
        max_width: c_float,
        min_height: c_float,
        max_height: c_float,
    );
    pub fn BWindow_SetTitle(self_: *mut c_void, title: *const c_char);
    pub fn BWindow_SetType(self_: *mut c_void, type_: window_type) -> status_t;
    pub fn BWindow_SetWindowAlignment(
        self_: *mut c_void,
        mode: window_alignment,
        h: i32,
        h_offset: i32,
        width: i32,
        width_offset: i32,
        v: i32,
        v_offset: i32,
        height: i32,
        height_offset: i32,
    ) -> status_t;
    pub fn BWindow_SetWorkspaces(self_: *mut c_void, workspaces: u32);
    pub fn BWindow_SetZoomLimits(self_: *mut c_void, max_width: c_float, max_height: c_float);
    pub fn BWindow_Show(self_: *mut c_void);
    pub fn BWindow_Size(self_: *const c_void) -> *mut c_void;
    pub fn BWindow_Sync(self_: *const c_void);
    pub fn BWindow_Title(self_: *const c_void) -> *const c_char;
    pub fn BWindow_Type(self_: *const c_void) -> window_type;
    pub fn BWindow_UpdateIfNeeded(self_: *mut c_void);
    pub fn BWindow_UpdateSizeLimits(self_: *mut c_void);
    pub fn BWindow_WindowActivated(self_: *mut c_void, focus: bool);
    pub fn BWindow_WorkspaceActivated(self_: *mut c_void, workspace: i32, state: bool);
    pub fn BWindow_Workspaces(self_: *const c_void) -> u32;
    pub fn BWindow_WorkspacesChanged(self_: *mut c_void, old_workspaces: u32, new_workspaces: u32);
    pub fn BWindow_Zoom(self_: *mut c_void);
    pub fn BWindow_Zoom1(self_: *mut c_void, origin: *mut c_void, width: c_float, height: c_float);
    pub fn BWindow_Instantiate(archive: *mut c_void) -> *mut c_void;

}

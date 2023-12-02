use super::*;

extern "C" {

    // BView
    pub fn BView_Instantiate(archive: *mut c_void) -> *mut c_void;
    pub fn BView_AttachedToWindow(self_: *mut c_void);
    pub fn BView_AllAttached(self_: *mut c_void);
    pub fn BView_DetachedFromWindow(self_: *mut c_void);
    pub fn BView_AllDetached(self_: *mut c_void);
    pub fn BView_Draw(self_: *mut c_void, update_rect: *mut c_void);
    pub fn BView_MouseDown(self_: *mut c_void, where_: *mut c_void);
    pub fn BView_MouseUp(self_: *mut c_void, where_: *mut c_void);
    pub fn BView_MouseMoved(
        self_: *mut c_void,
        where_: *mut c_void,
        code: u32,
        drag_message: *const c_void,
    );
    pub fn BView_WindowActivated(self_: *mut c_void, active: bool);
    pub fn BView_KeyDown(self_: *mut c_void, bytes: *const c_char, num_bytes: i32);
    pub fn BView_KeyUp(self_: *mut c_void, bytes: *const c_char, num_bytes: i32);
    pub fn BView_Pulse(self_: *mut c_void);
    pub fn BView_FrameMoved(self_: *mut c_void, new_position: *mut c_void);
    pub fn BView_FrameResized(self_: *mut c_void, new_width: c_float, new_height: c_float);
    pub fn BView_TargetedByScrollView(self_: *mut c_void, scroll_view: *mut c_void);
    pub fn BView_DrawAfterChildren(self_: *mut c_void, update_rect: *mut c_void);
    pub fn BView_AddChild(self_: *mut c_void, child: *mut c_void, before: *mut c_void);
    pub fn BView_AddChild1(self_: *mut c_void, child: *mut c_void) -> bool;
    pub fn BView_RemoveChild(self_: *mut c_void, child: *mut c_void) -> bool;
    pub fn BView_CountChildren(self_: *const c_void) -> i32;
    pub fn BView_ChildAt(self_: *const c_void, index: i32) -> *mut c_void;
    pub fn BView_NextSibling(self_: *const c_void) -> *mut c_void;
    pub fn BView_PreviousSibling(self_: *const c_void) -> *mut c_void;
    pub fn BView_RemoveSelf(self_: *mut c_void) -> bool;
    pub fn BView_FindView(self_: *const c_void, name: *const c_char) -> *mut c_void;
    pub fn BView_Parent(self_: *const c_void) -> *mut c_void;
    pub fn BView_BeginRectTracking(self_: *mut c_void, start_rect: *mut c_void, style: u32);
    pub fn BView_EndRectTracking(self_: *mut c_void);
    pub fn BView_GetMouse(
        self_: *mut c_void,
        location: *mut c_void,
        buttons: *mut c_void,
        check_message_queue: bool,
    );
    pub fn BView_DragMessage(
        self_: *mut c_void,
        message: *mut c_void,
        drag_rect: *mut c_void,
        reply_to: *mut c_void,
    );
    pub fn BView_DragMessage1(
        self_: *mut c_void,
        message: *mut c_void,
        bitmap: *mut c_void,
        offset: *mut c_void,
        reply_to: *mut c_void,
    );
    pub fn BView_DragMessage2(
        self_: *mut c_void,
        message: *mut c_void,
        bitmap: *mut c_void,
        drag_mode: drawing_mode,
        offset: *mut c_void,
        reply_to: *mut c_void,
    );
    pub fn BView_SetEventMask(self_: *mut c_void, mask: u32, options: u32) -> status_t;
    pub fn BView_EventMask(self_: *mut c_void) -> u32;
    pub fn BView_SetMouseEventMask(self_: *mut c_void, mask: u32, options: u32) -> status_t;
    pub fn BView_ScrollBy(self_: *mut c_void, dh: c_float, dv: c_float);
    pub fn BView_ScrollTo(self_: *mut c_void, x: c_float, y: c_float);
    pub fn BView_ScrollTo1(self_: *mut c_void, where_: *mut c_void);
    pub fn BView_MakeFocus(self_: *mut c_void, focus: bool);
    // NOT_SUPPORTED: pub fn BView_ScrollBar(self_: *const c_void, direction: orientation) -> *mut c_void;
    pub fn BView_ConvertToScreen(self_: *const c_void, point: *mut c_void);
    // BLOCKED: pub fn BView_ConvertToScreen1(self_: *const c_void, point: *mut c_void) -> *mut c_void;
    pub fn BView_ConvertFromScreen(self_: *const c_void, point: *mut c_void);
    // BLOCKED: pub fn BView_ConvertFromScreen1(self_: *const c_void, point: *mut c_void) -> *mut c_void;
    pub fn BView_ConvertToScreen2(self_: *const c_void, rect: *mut c_void);
    // BLOCKED: pub fn BView_ConvertToScreen3(self_: *const c_void, rect: *mut c_void) -> *mut c_void;
    pub fn BView_ConvertFromScreen2(self_: *const c_void, rect: *mut c_void);
    // BLOCKED: pub fn BView_ConvertFromScreen3(self_: *const c_void, rect: *mut c_void) -> *mut c_void;
    pub fn BView_ConvertToParent(self_: *const c_void, point: *mut c_void);
    // BLOCKED: pub fn BView_ConvertToParent1(self_: *const c_void, point: *mut c_void) -> *mut c_void;
    pub fn BView_ConvertFromParent(self_: *const c_void, point: *mut c_void);
    // BLOCKED: pub fn BView_ConvertFromParent1(self_: *const c_void, point: *mut c_void) -> *mut c_void;
    pub fn BView_ConvertToParent2(self_: *const c_void, rect: *mut c_void);
    // BLOCKED: pub fn BView_ConvertToParent3(self_: *const c_void, rect: *mut c_void) -> *mut c_void;
    pub fn BView_ConvertFromParent2(self_: *const c_void, rect: *mut c_void);
    // BLOCKED: pub fn BView_ConvertFromParent3(self_: *const c_void, rect: *mut c_void) -> *mut c_void;
    pub fn BView_SetFlags(self_: *mut c_void, flags: u32);
    pub fn BView_Flags(self_: *const c_void) -> u32;
    pub fn BView_GetClippingRegion(self_: *const c_void, region: *mut c_void);
    pub fn BView_ConstrainClippingRegion(self_: *mut c_void, region: *mut c_void);
    pub fn BView_ClipToPicture(
        self_: *mut c_void,
        picture: *mut c_void,
        where_: *mut c_void,
        sync: bool,
    );
    pub fn BView_ClipToInversePicture(
        self_: *mut c_void,
        picture: *mut c_void,
        where_: *mut c_void,
        sync: bool,
    );
    pub fn BView_ClipToRect(self_: *mut c_void, rect: *mut c_void);
    pub fn BView_ClipToInverseRect(self_: *mut c_void, rect: *mut c_void);
    pub fn BView_ClipToShape(self_: *mut c_void, shape: *mut c_void);
    pub fn BView_ClipToInverseShape(self_: *mut c_void, shape: *mut c_void);
    pub fn BView_SetDrawingMode(self_: *mut c_void, mode: drawing_mode);
    pub fn BView_DrawingMode(self_: *const c_void) -> drawing_mode;
    // NOT_SUPPORTED: pub fn BView_SetBlendingMode(self_: *mut c_void, src_alpha: source_alpha, alpha_func: alpha_function);
    pub fn BView_GetBlendingMode(
        self_: *const c_void,
        src_alpha: *mut c_void,
        alpha_func: *mut c_void,
    );
    pub fn BView_SetPenSize(self_: *mut c_void, size: c_float);
    pub fn BView_PenSize(self_: *const c_void) -> c_float;
    pub fn BView_HasDefaultColors(self_: *const c_void) -> bool;
    pub fn BView_HasSystemColors(self_: *const c_void) -> bool;
    pub fn BView_AdoptParentColors(self_: *mut c_void);
    pub fn BView_AdoptSystemColors(self_: *mut c_void);
    pub fn BView_AdoptViewColors(self_: *mut c_void, view: *mut c_void);
    // NOT_SUPPORTED: pub fn BView_SetViewColor(self_: *mut c_void, color: rgb_color);
    // NOT_SUPPORTED: pub fn BView_SetViewColor1(self_: *mut c_void, red: uchar, green: uchar, blue: uchar, alpha: uchar);
    // NOT_SUPPORTED: pub fn BView_ViewColor(self_: *const c_void) -> rgb_color;
    pub fn BView_SetViewUIColor(self_: *mut c_void, which: color_which, tint: c_float);
    pub fn BView_ViewUIColor(self_: *const c_void, tint: *mut c_void) -> color_which;
    // NOT_SUPPORTED: pub fn BView_SetHighColor(self_: *mut c_void, color: rgb_color);
    // NOT_SUPPORTED: pub fn BView_SetHighColor1(self_: *mut c_void, red: uchar, green: uchar, blue: uchar, alpha: uchar);
    // NOT_SUPPORTED: pub fn BView_HighColor(self_: *const c_void) -> rgb_color;
    pub fn BView_SetHighUIColor(self_: *mut c_void, which: color_which, tint: c_float);
    pub fn BView_HighUIColor(self_: *const c_void, tint: *mut c_void) -> color_which;
    // NOT_SUPPORTED: pub fn BView_SetLowColor(self_: *mut c_void, color: rgb_color);
    // NOT_SUPPORTED: pub fn BView_SetLowColor1(self_: *mut c_void, red: uchar, green: uchar, blue: uchar, alpha: uchar);
    // NOT_SUPPORTED: pub fn BView_LowColor(self_: *const c_void) -> rgb_color;
    pub fn BView_SetLowUIColor(self_: *mut c_void, which: color_which, tint: c_float);
    pub fn BView_LowUIColor(self_: *const c_void, tint: *mut c_void) -> color_which;
    // NOT_SUPPORTED: pub fn BView_SetLineMode(self_: *mut c_void, line_cap: cap_mode, line_join: join_mode, miter_limit: c_float);
    // NOT_SUPPORTED: pub fn BView_LineJoinMode(self_: *const c_void) -> join_mode;
    // NOT_SUPPORTED: pub fn BView_LineCapMode(self_: *const c_void) -> cap_mode;
    pub fn BView_LineMiterLimit(self_: *const c_void) -> c_float;
    pub fn BView_SetFillRule(self_: *mut c_void, rule: i32);
    pub fn BView_FillRule(self_: *const c_void) -> i32;
    pub fn BView_SetOrigin(self_: *mut c_void, where_: *mut c_void);
    pub fn BView_SetOrigin1(self_: *mut c_void, x: c_float, y: c_float);
    pub fn BView_Origin(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn BView_SetTransform(self_: *mut c_void, transform: BAffineTransform);
    // NOT_SUPPORTED: pub fn BView_Transform(self_: *const c_void) -> BAffineTransform;
    pub fn BView_TranslateBy(self_: *mut c_void, x: c_double, y: c_double);
    pub fn BView_ScaleBy(self_: *mut c_void, x: c_double, y: c_double);
    pub fn BView_RotateBy(self_: *mut c_void, angle_radians: c_double);
    // NOT_SUPPORTED: pub fn BView_TransformTo(self_: *const c_void, basis: coordinate_space) -> BAffineTransform;
    pub fn BView_PushState(self_: *mut c_void);
    pub fn BView_PopState(self_: *mut c_void);
    pub fn BView_MovePenTo(self_: *mut c_void, pt: *mut c_void);
    pub fn BView_MovePenTo1(self_: *mut c_void, x: c_float, y: c_float);
    pub fn BView_MovePenBy(self_: *mut c_void, x: c_float, y: c_float);
    pub fn BView_PenLocation(self_: *const c_void) -> *mut c_void;
    pub fn BView_SetFont(self_: *mut c_void, font: *const c_void, mask: u32);
    pub fn BView_GetFont(self_: *const c_void, font: *mut c_void);
    pub fn BView_TruncateString(
        self_: *const c_void,
        in_out: *mut c_void,
        mode: u32,
        width: c_float,
    );
    pub fn BView_StringWidth(self_: *const c_void, string: *const c_char) -> c_float;
    pub fn BView_StringWidth1(self_: *const c_void, string: *const c_char, length: i32) -> c_float;
    // NOT_SUPPORTED: pub fn BView_GetStringWidths(self_: *const c_void, string_array: *mut c_void, length_array: i32, num_strings: i32, width_array: c_float);
    pub fn BView_SetFontSize(self_: *mut c_void, size: c_float);
    pub fn BView_ForceFontAliasing(self_: *mut c_void, enable: bool);
    pub fn BView_GetFontHeight(self_: *const c_void, height: *mut c_void);
    pub fn BView_SetScale(self_: *const c_void, scale: c_float);
    pub fn BView_Scale(self_: *const c_void) -> c_float;
    pub fn BView_SetViewBitmap(
        self_: *mut c_void,
        bitmap: *const c_void,
        src_rect: *mut c_void,
        dst_rect: *mut c_void,
        follow_flags: u32,
        options: u32,
    );
    pub fn BView_SetViewBitmap1(
        self_: *mut c_void,
        bitmap: *const c_void,
        follow_flags: u32,
        options: u32,
    );
    pub fn BView_ClearViewBitmap(self_: *mut c_void);
    pub fn BView_SetViewOverlay(
        self_: *mut c_void,
        overlay: *const c_void,
        src_rect: *mut c_void,
        dst_rect: *mut c_void,
        color_key: *mut c_void,
        follow_flags: u32,
        options: u32,
    ) -> status_t;
    pub fn BView_SetViewOverlay1(
        self_: *mut c_void,
        overlay: *const c_void,
        color_key: *mut c_void,
        follow_flags: u32,
        options: u32,
    ) -> status_t;
    pub fn BView_ClearViewOverlay(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn BView_StrokeLine(self_: *mut c_void, to_point: *mut c_void, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_StrokeLine1(self_: *mut c_void, start: *mut c_void, end: *mut c_void, pattern: ::pattern);
    pub fn BView_BeginLineArray(self_: *mut c_void, count: i32);
    // NOT_SUPPORTED: pub fn BView_AddLine(self_: *mut c_void, start: *mut c_void, end: *mut c_void, color: rgb_color);
    pub fn BView_EndLineArray(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn BView_StrokePolygon(self_: *mut c_void, polygon: *const c_void, closed: bool, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_StrokePolygon1(self_: *mut c_void, point_array: *const c_void, num_points: i32, closed: bool, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_StrokePolygon2(self_: *mut c_void, point_array: *const c_void, num_points: i32, bounds: *mut c_void, closed: bool, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillPolygon(self_: *mut c_void, polygon: *const c_void, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillPolygon1(self_: *mut c_void, point_array: *const c_void, num_points: i32, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillPolygon2(self_: *mut c_void, point_array: *const c_void, num_points: i32, bounds: *mut c_void, pattern: ::pattern);
    pub fn BView_FillPolygon3(self_: *mut c_void, polygon: *const c_void, gradient: *const c_void);
    pub fn BView_FillPolygon4(
        self_: *mut c_void,
        point_array: *const c_void,
        num_points: i32,
        gradient: *const c_void,
    );
    pub fn BView_FillPolygon5(
        self_: *mut c_void,
        point_array: *const c_void,
        num_points: i32,
        bounds: *mut c_void,
        gradient: *const c_void,
    );
    // NOT_SUPPORTED: pub fn BView_StrokeTriangle(self_: *mut c_void, point1: *mut c_void, point2: *mut c_void, point3: *mut c_void, bounds: *mut c_void, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_StrokeTriangle1(self_: *mut c_void, point1: *mut c_void, point2: *mut c_void, point3: *mut c_void, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillTriangle(self_: *mut c_void, point1: *mut c_void, point2: *mut c_void, point3: *mut c_void, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillTriangle1(self_: *mut c_void, point1: *mut c_void, point2: *mut c_void, point3: *mut c_void, bounds: *mut c_void, pattern: ::pattern);
    pub fn BView_FillTriangle2(
        self_: *mut c_void,
        point1: *mut c_void,
        point2: *mut c_void,
        point3: *mut c_void,
        gradient: *const c_void,
    );
    pub fn BView_FillTriangle3(
        self_: *mut c_void,
        point1: *mut c_void,
        point2: *mut c_void,
        point3: *mut c_void,
        bounds: *mut c_void,
        gradient: *const c_void,
    );
    // NOT_SUPPORTED: pub fn BView_StrokeRect(self_: *mut c_void, rect: *mut c_void, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillRect(self_: *mut c_void, rect: *mut c_void, pattern: ::pattern);
    pub fn BView_FillRect1(self_: *mut c_void, rect: *mut c_void, gradient: *const c_void);
    // NOT_SUPPORTED: pub fn BView_FillRegion(self_: *mut c_void, rectegion: *mut c_void, pattern: ::pattern);
    pub fn BView_FillRegion1(self_: *mut c_void, rectegion: *mut c_void, gradient: *const c_void);
    pub fn BView_InvertRect(self_: *mut c_void, rect: *mut c_void);
    // NOT_SUPPORTED: pub fn BView_StrokeRoundRect(self_: *mut c_void, rect: *mut c_void, x_radius: c_float, y_radius: c_float, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillRoundRect(self_: *mut c_void, rect: *mut c_void, x_radius: c_float, y_radius: c_float, pattern: ::pattern);
    pub fn BView_FillRoundRect1(
        self_: *mut c_void,
        rect: *mut c_void,
        x_radius: c_float,
        y_radius: c_float,
        gradient: *const c_void,
    );
    // NOT_SUPPORTED: pub fn BView_StrokeEllipse(self_: *mut c_void, center: *mut c_void, x_radius: c_float, y_radius: c_float, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_StrokeEllipse1(self_: *mut c_void, rect: *mut c_void, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillEllipse(self_: *mut c_void, center: *mut c_void, x_radius: c_float, y_radius: c_float, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillEllipse1(self_: *mut c_void, rect: *mut c_void, pattern: ::pattern);
    pub fn BView_FillEllipse2(
        self_: *mut c_void,
        center: *mut c_void,
        x_radius: c_float,
        y_radius: c_float,
        gradient: *const c_void,
    );
    pub fn BView_FillEllipse3(self_: *mut c_void, rect: *mut c_void, gradient: *const c_void);
    // NOT_SUPPORTED: pub fn BView_StrokeArc(self_: *mut c_void, center: *mut c_void, x_radius: c_float, y_radius: c_float, start_angle: c_float, arc_angle: c_float, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_StrokeArc1(self_: *mut c_void, rect: *mut c_void, start_angle: c_float, arc_angle: c_float, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillArc(self_: *mut c_void, center: *mut c_void, x_radius: c_float, y_radius: c_float, start_angle: c_float, arc_angle: c_float, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillArc1(self_: *mut c_void, rect: *mut c_void, start_angle: c_float, arc_angle: c_float, pattern: ::pattern);
    pub fn BView_FillArc2(
        self_: *mut c_void,
        center: *mut c_void,
        x_radius: c_float,
        y_radius: c_float,
        start_angle: c_float,
        arc_angle: c_float,
        gradient: *const c_void,
    );
    pub fn BView_FillArc3(
        self_: *mut c_void,
        rect: *mut c_void,
        start_angle: c_float,
        arc_angle: c_float,
        gradient: *const c_void,
    );
    // NOT_SUPPORTED: pub fn BView_StrokeBezier(self_: *mut c_void, control_points: *mut c_void, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillBezier(self_: *mut c_void, control_points: *mut c_void, pattern: ::pattern);
    pub fn BView_FillBezier1(
        self_: *mut c_void,
        control_points: *mut c_void,
        gradient: *const c_void,
    );
    // NOT_SUPPORTED: pub fn BView_StrokeShape(self_: *mut c_void, shape: *mut c_void, pattern: ::pattern);
    // NOT_SUPPORTED: pub fn BView_FillShape(self_: *mut c_void, shape: *mut c_void, pattern: ::pattern);
    pub fn BView_FillShape1(self_: *mut c_void, shape: *mut c_void, gradient: *const c_void);
    pub fn BView_CopyBits(self_: *mut c_void, src: *mut c_void, dst: *mut c_void);
    pub fn BView_DrawBitmapAsync(
        self_: *mut c_void,
        a_bitmap: *const c_void,
        bitmap_rect: *mut c_void,
        view_rect: *mut c_void,
        options: u32,
    );
    pub fn BView_DrawBitmapAsync1(
        self_: *mut c_void,
        a_bitmap: *const c_void,
        bitmap_rect: *mut c_void,
        view_rect: *mut c_void,
    );
    pub fn BView_DrawBitmapAsync2(
        self_: *mut c_void,
        a_bitmap: *const c_void,
        view_rect: *mut c_void,
    );
    pub fn BView_DrawBitmapAsync3(self_: *mut c_void, a_bitmap: *const c_void, where_: *mut c_void);
    pub fn BView_DrawBitmapAsync4(self_: *mut c_void, a_bitmap: *const c_void);
    pub fn BView_DrawBitmap(
        self_: *mut c_void,
        a_bitmap: *const c_void,
        bitmap_rect: *mut c_void,
        view_rect: *mut c_void,
        options: u32,
    );
    pub fn BView_DrawBitmap1(
        self_: *mut c_void,
        a_bitmap: *const c_void,
        bitmap_rect: *mut c_void,
        view_rect: *mut c_void,
    );
    pub fn BView_DrawBitmap2(self_: *mut c_void, a_bitmap: *const c_void, view_rect: *mut c_void);
    pub fn BView_DrawBitmap3(self_: *mut c_void, a_bitmap: *const c_void, where_: *mut c_void);
    pub fn BView_DrawBitmap4(self_: *mut c_void, a_bitmap: *const c_void);
    pub fn BView_DrawTiledBitmapAsync(
        self_: *mut c_void,
        a_bitmap: *const c_void,
        view_rect: *mut c_void,
        phase: *mut c_void,
    );
    pub fn BView_DrawTiledBitmap(
        self_: *mut c_void,
        a_bitmap: *const c_void,
        view_rect: *mut c_void,
        phase: *mut c_void,
    );
    pub fn BView_DrawChar(self_: *mut c_void, a_char: c_char);
    pub fn BView_DrawChar1(self_: *mut c_void, a_char: c_char, location: *mut c_void);
    pub fn BView_DrawString(self_: *mut c_void, string: *const c_char, delta: *mut c_void);
    // BLOCKED: pub fn BView_DrawString1(self_: *mut c_void, string: *const c_char, location: *mut c_void, delta: *mut c_void);
    pub fn BView_DrawString2(
        self_: *mut c_void,
        string: *const c_char,
        length: i32,
        delta: *mut c_void,
    );
    // BLOCKED: pub fn BView_DrawString3(self_: *mut c_void, string: *const c_char, length: i32, location: *mut c_void, delta: *mut c_void);
    pub fn BView_DrawString4(
        self_: *mut c_void,
        string: *const c_char,
        locations: *const c_void,
        location_count: i32,
    );
    pub fn BView_DrawString5(
        self_: *mut c_void,
        string: *const c_char,
        length: i32,
        locations: *const c_void,
        location_count: i32,
    );
    pub fn BView_Invalidate(self_: *mut c_void, inval_rect: *mut c_void);
    pub fn BView_Invalidate1(self_: *mut c_void, inval_region: *const c_void);
    pub fn BView_Invalidate2(self_: *mut c_void);
    pub fn BView_DelayedInvalidate(self_: *mut c_void, delay: bigtime_t);
    pub fn BView_DelayedInvalidate1(self_: *mut c_void, delay: bigtime_t, inval_rect: *mut c_void);
    pub fn BView_SetDiskMode(self_: *mut c_void, filename: *mut c_void, offset: c_long);
    pub fn BView_BeginPicture(self_: *mut c_void, a_picture: *mut c_void);
    pub fn BView_AppendToPicture(self_: *mut c_void, a_picture: *mut c_void);
    pub fn BView_EndPicture(self_: *mut c_void) -> *mut c_void;
    pub fn BView_DrawPicture(self_: *mut c_void, a_picture: *const c_void);
    pub fn BView_DrawPicture1(self_: *mut c_void, a_picture: *const c_void, where_: *mut c_void);
    pub fn BView_DrawPicture2(
        self_: *mut c_void,
        filename: *const c_char,
        offset: c_long,
        where_: *mut c_void,
    );
    pub fn BView_DrawPictureAsync(self_: *mut c_void, a_picture: *const c_void);
    pub fn BView_DrawPictureAsync1(
        self_: *mut c_void,
        a_picture: *const c_void,
        where_: *mut c_void,
    );
    pub fn BView_DrawPictureAsync2(
        self_: *mut c_void,
        filename: *const c_char,
        offset: c_long,
        where_: *mut c_void,
    );
    pub fn BView_BeginLayer(self_: *mut c_void, opacity: u8);
    pub fn BView_EndLayer(self_: *mut c_void);
    pub fn BView_MoveBy(self_: *mut c_void, dh: c_float, dv: c_float);
    pub fn BView_MoveTo(self_: *mut c_void, where_: *mut c_void);
    pub fn BView_MoveTo1(self_: *mut c_void, x: c_float, y: c_float);
    pub fn BView_ResizeBy(self_: *mut c_void, dh: c_float, dv: c_float);
    pub fn BView_ResizeTo(self_: *mut c_void, width: c_float, height: c_float);
    pub fn BView_ResizeTo1(self_: *mut c_void, size: *mut c_void);
    pub fn BView_MinSize(self_: *mut c_void) -> *mut c_void;
    pub fn BView_MaxSize(self_: *mut c_void) -> *mut c_void;
    pub fn BView_PreferredSize(self_: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn BView_LayoutAlignment(self_: *mut c_void) -> BAlignment;
    pub fn BView_SetExplicitMinSize(self_: *mut c_void, size: *mut c_void);
    pub fn BView_SetExplicitMaxSize(self_: *mut c_void, size: *mut c_void);
    pub fn BView_SetExplicitPreferredSize(self_: *mut c_void, size: *mut c_void);
    pub fn BView_SetExplicitSize(self_: *mut c_void, size: *mut c_void);
    // NOT_SUPPORTED: pub fn BView_SetExplicitAlignment(self_: *mut c_void, alignment: BAlignment);
    pub fn BView_ExplicitMinSize(self_: *const c_void) -> *mut c_void;
    pub fn BView_ExplicitMaxSize(self_: *const c_void) -> *mut c_void;
    pub fn BView_ExplicitPreferredSize(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn BView_ExplicitAlignment(self_: *const c_void) -> BAlignment;
    pub fn BView_HasHeightForWidth(self_: *mut c_void) -> bool;
    pub fn BView_GetHeightForWidth(
        self_: *mut c_void,
        width: c_float,
        min: *mut c_void,
        max: *mut c_void,
        preferred: *mut c_void,
    );
    pub fn BView_InvalidateLayout(self_: *mut c_void, descendants: bool);
    pub fn BView_SetLayout(self_: *mut c_void, layout: *mut c_void);
    pub fn BView_GetLayout(self_: *const c_void) -> *mut c_void;
    pub fn BView_EnableLayoutInvalidation(self_: *mut c_void);
    pub fn BView_DisableLayoutInvalidation(self_: *mut c_void);
    pub fn BView_IsLayoutInvalidationDisabled(self_: *mut c_void) -> bool;
    pub fn BView_IsLayoutValid(self_: *const c_void) -> bool;
    pub fn BView_ResetLayoutInvalidation(self_: *mut c_void);
    pub fn BView_LayoutContext(self_: *const c_void) -> *mut c_void;
    pub fn BView_Layout(self_: *mut c_void, force: bool);
    pub fn BView_Relayout(self_: *mut c_void);
    pub fn BView_SetToolTip(self_: *mut c_void, text: *const c_char);
    pub fn BView_SetToolTip1(self_: *mut c_void, tip: *mut c_void);
    pub fn BView_ToolTip(self_: *const c_void) -> *mut c_void;
    pub fn BView_ShowToolTip(self_: *mut c_void, tip: *mut c_void);
    pub fn BView_HideToolTip(self_: *mut c_void);
    pub fn BView_new(archive: *mut c_void) -> *mut c_void;
    pub fn BView_new1(
        frame: *mut c_void,
        name: *const c_char,
        resizing_mode: u32,
        flags: u32,
    ) -> *mut c_void;
    pub fn BView_new2(name: *const c_char, flags: u32, layout: *mut c_void) -> *mut c_void;
    // DTOR: pub fn BView_~BView(self_: *mut c_void);
    pub fn BView_Bounds(self_: *const c_void) -> *mut c_void;
    pub fn BView_Flush(self_: *const c_void);
    pub fn BView_Frame(self_: *const c_void) -> *mut c_void;
    pub fn BView_GetPreferredSize(self_: *mut c_void, _width: *mut c_void, _height: *mut c_void);
    pub fn BView_Hide(self_: *mut c_void);
    pub fn BView_IsFocus(self_: *const c_void) -> bool;
    pub fn BView_IsHidden(self_: *const c_void) -> bool;
    pub fn BView_IsHidden1(self_: *const c_void, looking_from: *const c_void) -> bool;
    pub fn BView_IsPrinting(self_: *const c_void) -> bool;
    pub fn BView_LeftTop(self_: *const c_void) -> *mut c_void;
    pub fn BView_ResizeToPreferred(self_: *mut c_void);
    pub fn BView_ResizingMode(self_: *const c_void) -> u32;
    pub fn BView_SetResizingMode(self_: *mut c_void, mode: u32);
    pub fn BView_SetViewCursor(self_: *mut c_void, cursor: *const c_void, sync: bool);
    pub fn BView_Show(self_: *mut c_void);
    pub fn BView_Sync(self_: *const c_void);
    pub fn BView_Window(self_: *const c_void) -> *mut c_void;

}

use super::*;

// BView
/// This trait represents [C++ `BView` class](https://www.haiku-os.org/docs/api/classBView.html)'s methods and inheritance.
///
/// See [`ViewFromCpp`] documentation for the class usage.
pub trait ViewMethods: HandlerMethods {
    /// Hook method called when the object is attached to a window.
    ///
    /// See [C++ `BView::AttachedToWindow()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae8117cb4634f4a4413988eea07cf75a0).
    fn attached_to_window(&self) {
        unsafe { ffi::BView_AttachedToWindow(self.as_ptr()) }
    }
    /// Similar to AttachedToWindow() but this method is triggered after all child views have already been attached to a window.
    ///
    /// See [C++ `BView::AllAttached()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a6d6379e0993bd525f2cac5f848718870).
    fn all_attached(&self) {
        unsafe { ffi::BView_AllAttached(self.as_ptr()) }
    }
    /// Hook method called when the object is detached from a window.
    ///
    /// See [C++ `BView::DetachedFromWindow()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a8abc2ea500951a0a22cf018e958ba791).
    fn detached_from_window(&self) {
        unsafe { ffi::BView_DetachedFromWindow(self.as_ptr()) }
    }
    /// Similar to AttachedToWindow() but this method is triggered after all child views have already been detached from a window.
    ///
    /// See [C++ `BView::AllDetached()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a19dd2213f4b953b54524d7f523d9b406).
    fn all_detached(&self) {
        unsafe { ffi::BView_AllDetached(self.as_ptr()) }
    }
    /// Draws the area of the view that intersects updateRect.
    ///
    /// See [C++ `BView::Draw()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aa55014241fc2f5556415850692b52db2).
    fn draw<R: RectMethods>(&self, update_rect: &R) {
        unsafe {
            let update_rect = update_rect.as_ptr();
            ffi::BView_Draw(self.as_ptr(), update_rect)
        }
    }
    /// Hook method called when a mouse button is pressed.
    ///
    /// See [C++ `BView::MouseDown()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a5f5bd975d15df94f2ebbc80eb79cc3f6).
    fn mouse_down<P: PointMethods>(&self, where_: &P) {
        unsafe {
            let where_ = where_.as_ptr();
            ffi::BView_MouseDown(self.as_ptr(), where_)
        }
    }
    /// Hook method called when a mouse button is released.
    ///
    /// See [C++ `BView::MouseUp()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#acdb92b30069157b2e3c522a3ef9d6ae4).
    fn mouse_up<P: PointMethods>(&self, where_: &P) {
        unsafe {
            let where_ = where_.as_ptr();
            ffi::BView_MouseUp(self.as_ptr(), where_)
        }
    }
    /// Hook method called when the mouse is moved.
    ///
    /// See [C++ `BView::MouseMoved()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ac8b20516e42bab2f1eeb130e2432bde0).
    fn mouse_moved<P: PointMethods, M: MessageMethods>(
        &self,
        where_: &P,
        code: u32,
        drag_message: Option<&M>,
    ) {
        unsafe {
            let where_ = where_.as_ptr();
            let drag_message = match drag_message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_MouseMoved(self.as_ptr(), where_, code, drag_message)
        }
    }
    /// Hook method called when the attached window is activated or deactivated.
    ///
    /// See [C++ `BView::WindowActivated()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a5b7aeff0b2608ec63e2c01153a7717f6).
    fn window_activated(&self, active: bool) {
        unsafe { ffi::BView_WindowActivated(self.as_ptr(), active) }
    }
    /// Hook method called when a keyboard key is pressed.
    ///
    /// See [C++ `BView::KeyDown()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#abea50ea665ce5fcb76b32f1302a6852d).
    fn key_down(&self, bytes: &str, num_bytes: i32) {
        unsafe {
            let bytes = CString::from_vec_unchecked(bytes.into());
            let bytes = bytes.as_ptr();
            ffi::BView_KeyDown(self.as_ptr(), bytes, num_bytes)
        }
    }
    /// Hook method called when a keyboard key is released.
    ///
    /// See [C++ `BView::KeyUp()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#acbc528bb70be49993bd99aa136dbb089).
    fn key_up(&self, bytes: &str, num_bytes: i32) {
        unsafe {
            let bytes = CString::from_vec_unchecked(bytes.into());
            let bytes = bytes.as_ptr();
            ffi::BView_KeyUp(self.as_ptr(), bytes, num_bytes)
        }
    }
    /// Hook method called when the view receives a B_PULSE message.
    ///
    /// See [C++ `BView::Pulse()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a0ae8ec3e94ee3d41cb329e3df6966566).
    fn pulse(&self) {
        unsafe { ffi::BView_Pulse(self.as_ptr()) }
    }
    /// Hook method called when the view is moved.
    ///
    /// See [C++ `BView::FrameMoved()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a551c22b45301e13a3ba959eece3cec7f).
    fn frame_moved<P: PointMethods>(&self, new_position: &P) {
        unsafe {
            let new_position = new_position.as_ptr();
            ffi::BView_FrameMoved(self.as_ptr(), new_position)
        }
    }
    /// Hook method called when the view is resized.
    ///
    /// See [C++ `BView::FrameResized()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a85d88f9c2bb6800560874b2061378fa0).
    fn frame_resized(&self, new_width: c_float, new_height: c_float) {
        unsafe { ffi::BView_FrameResized(self.as_ptr(), new_width, new_height) }
    }
    /// Hook method called when the view becomes the target of scrollView.
    ///
    /// See [C++ `BView::TargetedByScrollView()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a668a4ed5419c86a7f2696ca7ece28df7).
    fn targeted_by_scroll_view(&self, scroll_view: *mut c_void) {
        unsafe { ffi::BView_TargetedByScrollView(self.as_ptr(), scroll_view) }
    }
    /// Perform any drawing that needs to be done after child view have already been drawn.
    ///
    /// See [C++ `BView::DrawAfterChildren()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#abb50623b4236926b068fc5a2264f6a48).
    fn draw_after_children<R: RectMethods>(&self, update_rect: &R) {
        unsafe {
            let update_rect = update_rect.as_ptr();
            ffi::BView_DrawAfterChildren(self.as_ptr(), update_rect)
        }
    }
    /// Adds child to the view hierarchy immediately before before.
    ///
    /// See [C++ `BView::AddChild()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ab6382c582e5e1be527d3199459d05e8f).
    fn add_child_view<V: ViewMethods, V2: ViewMethods>(
        &self,
        child: Option<&V>,
        before: Option<&V2>,
    ) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let before = match before {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_AddChild(self.as_ptr(), child, before)
        }
    }
    /// Add the child layout item to the view hierarchy.
    ///
    /// See [C++ `BView::AddChild()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a5d30ae4fa0119bf5286f52f83c2f351c).
    fn add_child_layoutitem(&self, child: *mut c_void) -> bool {
        unsafe { ffi::BView_AddChild1(self.as_ptr(), child) }
    }
    /// Removes child from the view hierarchy.
    ///
    /// See [C++ `BView::RemoveChild()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a5f93a23d06267da83c128bce222ceb17).
    fn remove_child<V: ViewMethods>(&self, child: Option<&V>) -> bool {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_RemoveChild(self.as_ptr(), child)
        }
    }
    /// Returns the number of child views that this view has.
    ///
    /// See [C++ `BView::CountChildren()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae30d094a1cb1f3357c4aa7caa4b0e29d).
    fn count_children(&self) -> i32 {
        unsafe { ffi::BView_CountChildren(self.as_ptr()) }
    }
    /// Returns a pointer to the child view found at index.
    ///
    /// See [C++ `BView::ChildAt()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aaf91592f50f2f44d16f23e7504f494cb).
    fn child_at(&self, index: i32) -> Option<ViewFromCpp<true>> {
        unsafe { View::option_from(ffi::BView_ChildAt(self.as_ptr(), index)) }
    }
    /// Returns a pointer to the next sibling view.
    ///
    /// See [C++ `BView::NextSibling()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a76e7d862fe78e4d2766573bf073b9d30).
    fn next_sibling(&self) -> Option<ViewFromCpp<true>> {
        unsafe { View::option_from(ffi::BView_NextSibling(self.as_ptr())) }
    }
    /// Returns a pointer to the previous sibling view.
    ///
    /// See [C++ `BView::PreviousSibling()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a2a46f78756eacca3f3d4c90d31c758b1).
    fn previous_sibling(&self) -> Option<ViewFromCpp<true>> {
        unsafe { View::option_from(ffi::BView_PreviousSibling(self.as_ptr())) }
    }
    /// Removes the view and all child views from the view hierarchy.
    ///
    /// See [C++ `BView::RemoveSelf()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a87e0848b1760a9f28aad4516467a03ed).
    fn remove_self(&self) -> bool {
        unsafe { ffi::BView_RemoveSelf(self.as_ptr()) }
    }
    /// Returns the view in the view hierarchy with the specified name.
    ///
    /// See [C++ `BView::FindView()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ab83ef89f1876913174c825ff7cd02c4a).
    fn find_view(&self, name: &str) -> Option<ViewFromCpp<true>> {
        unsafe {
            let name = CString::from_vec_unchecked(name.into());
            let name = name.as_ptr();
            View::option_from(ffi::BView_FindView(self.as_ptr(), name))
        }
    }
    /// Returns a pointer to the view's parent.
    ///
    /// See [C++ `BView::Parent()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a332f34d9862000dbc859fd4e308839dd).
    fn parent(&self) -> Option<ViewFromCpp<true>> {
        unsafe { View::option_from(ffi::BView_Parent(self.as_ptr())) }
    }
    /// Displays an outline rectangle on the view and initiates tracking.
    ///
    /// See [C++ `BView::BeginRectTracking()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ac1eb1eeee67ec97205a4acc6955185bd).
    fn begin_rect_tracking<R: RectMethods>(&self, start_rect: &R, style: u32) {
        unsafe {
            let start_rect = start_rect.as_ptr();
            ffi::BView_BeginRectTracking(self.as_ptr(), start_rect, style)
        }
    }
    /// Ends tracking removing the outline rectangle from the view.
    ///
    /// See [C++ `BView::EndRectTracking()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a7bcc3532c03eeeb929278de8fcc1fd25).
    fn end_rect_tracking(&self) {
        unsafe { ffi::BView_EndRectTracking(self.as_ptr()) }
    }
    /// Fills out the cursor location and the current state of the mouse buttons.
    ///
    /// See [C++ `BView::GetMouse()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a830ce9cf2ac2ffb5f73f526ee5d6e53d).
    fn get_mouse<P: PointMethods>(
        &self,
        location: Option<&P>,
        buttons: *mut c_void,
        check_message_queue: bool,
    ) {
        unsafe {
            let location = match location {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_GetMouse(self.as_ptr(), location, buttons, check_message_queue)
        }
    }
    /// Initiates a drag-and-drop session.
    ///
    /// See [C++ `BView::DragMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a1f4f0c3c5946df1d14e6031e5e12f40b).
    fn drag_message_rect<M: MessageMethods, R: RectMethods, H: HandlerMethods>(
        &self,
        message: Option<&M>,
        drag_rect: &R,
        reply_to: Option<&H>,
    ) {
        unsafe {
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let drag_rect = drag_rect.as_ptr();
            let reply_to = match reply_to {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_DragMessage(self.as_ptr(), message, drag_rect, reply_to)
        }
    }
    /// Initiates a drag-and-drop session of an image.
    ///
    /// See [C++ `BView::DragMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a466287a3d939759cc09e4a6d1c9982cd).
    fn drag_message_bitmap_point<M: MessageMethods, P: PointMethods, H: HandlerMethods>(
        &self,
        message: Option<&M>,
        bitmap: *mut c_void,
        offset: &P,
        reply_to: Option<&H>,
    ) {
        unsafe {
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let offset = offset.as_ptr();
            let reply_to = match reply_to {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_DragMessage1(self.as_ptr(), message, bitmap, offset, reply_to)
        }
    }
    /// Initiates a drag-and-drop session of an image with drawing_mode set by dragMode.
    ///
    /// See [C++ `BView::DragMessage()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae1caa1d0bef9452db00c2adc8e3280b4).
    fn drag_message_bitmap_drawing_mode<M: MessageMethods, P: PointMethods, H: HandlerMethods>(
        &self,
        message: Option<&M>,
        bitmap: *mut c_void,
        drag_mode: drawing_mode,
        offset: &P,
        reply_to: Option<&H>,
    ) {
        unsafe {
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let offset = offset.as_ptr();
            let reply_to = match reply_to {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_DragMessage2(self.as_ptr(), message, bitmap, drag_mode, offset, reply_to)
        }
    }
    /// Sets whether or not the view can accept mouse and keyboard events when not in focus.
    ///
    /// See [C++ `BView::SetEventMask()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a5d8d6b469a8a11a577d1d138cfc31162).
    fn set_event_mask(&self, mask: u32, options: u32) -> status_t {
        unsafe { ffi::BView_SetEventMask(self.as_ptr(), mask, options) }
    }
    /// Returns the current event mask.
    ///
    /// See [C++ `BView::EventMask()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a8851d500773c732ddbb1a4b31f0ad664).
    fn event_mask(&self) -> u32 {
        unsafe { ffi::BView_EventMask(self.as_ptr()) }
    }
    /// Sets whether or not the view can accept mouse and keyboard events when not in focus from within MouseDown() until the following MouseUp() event.
    ///
    /// See [C++ `BView::SetMouseEventMask()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae7705783d064a8613dcc6f36a2866ab2).
    fn set_mouse_event_mask(&self, mask: u32, options: u32) -> status_t {
        unsafe { ffi::BView_SetMouseEventMask(self.as_ptr(), mask, options) }
    }
    /// Scroll the view by deltaX horizontally and deltaY vertically.
    ///
    /// See [C++ `BView::ScrollBy()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae20e2a3c46fc30a2a292c832eead7387).
    fn scroll_by(&self, dh: c_float, dv: c_float) {
        unsafe { ffi::BView_ScrollBy(self.as_ptr(), dh, dv) }
    }
    /// Scroll the view to the point specified by coordinates.
    ///
    /// See [C++ `BView::ScrollTo()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a33ed2a9eeff474a756b548996a82b964).
    fn scroll_to_float(&self, x: c_float, y: c_float) {
        unsafe { ffi::BView_ScrollTo(self.as_ptr(), x, y) }
    }
    /// Scroll the view to the point specified by where.
    ///
    /// See [C++ `BView::ScrollTo()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a2b76ddc60289ce92e1fb706332edbafc).
    fn scroll_to_point<P: PointMethods>(&self, where_: &P) {
        unsafe {
            let where_ = where_.as_ptr();
            ffi::BView_ScrollTo1(self.as_ptr(), where_)
        }
    }
    /// Makes the view the current focus view of the window or gives up being the window's focus view.
    ///
    /// See [C++ `BView::MakeFocus()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a266a368d81c6e1668b2b9b22b5016853).
    fn make_focus(&self, focus: bool) {
        unsafe { ffi::BView_MakeFocus(self.as_ptr(), focus) }
    }
    // NOT_SUPPORTED: fn ScrollBar()
    /// Convert point to the screen's coordinate system in place.
    ///
    /// See [C++ `BView::ConvertToScreen()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aefefb1242613b869feb6b8e0cc119518).
    fn convert_to_screen_point<P: PointMethods>(&self, point: Option<&P>) {
        unsafe {
            let point = match point {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_ConvertToScreen(self.as_ptr(), point)
        }
    }
    // BLOCKED: fn ConvertToScreen1()
    /// Convert point from the screen's coordinate system to the view's coordinate system in place.
    ///
    /// See [C++ `BView::ConvertFromScreen()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a4b794ccd899dc6a0cafd886169122df5).
    fn convert_from_screen_point<P: PointMethods>(&self, point: Option<&P>) {
        unsafe {
            let point = match point {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_ConvertFromScreen(self.as_ptr(), point)
        }
    }
    // BLOCKED: fn ConvertFromScreen1()
    /// Convert rect to the screen's coordinate system in place.
    ///
    /// See [C++ `BView::ConvertToScreen()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ad99f5af006f28c485f02484752a2ac5d).
    fn convert_to_screen_rect<R: RectMethods>(&self, rect: Option<&R>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_ConvertToScreen2(self.as_ptr(), rect)
        }
    }
    // BLOCKED: fn ConvertToScreen3()
    /// Convert rect from the screen's coordinate system to the view's coordinate system in place.
    ///
    /// See [C++ `BView::ConvertFromScreen()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a3fbc496f437925cb02ccf9e817406e4e).
    fn convert_from_screen_rect<R: RectMethods>(&self, rect: Option<&R>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_ConvertFromScreen2(self.as_ptr(), rect)
        }
    }
    // BLOCKED: fn ConvertFromScreen3()
    /// Convert point to the parent's coordinate system in place.
    ///
    /// See [C++ `BView::ConvertToParent()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ab20700567eabd5f8135be179b4232e2c).
    fn convert_to_parent_point<P: PointMethods>(&self, point: Option<&P>) {
        unsafe {
            let point = match point {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_ConvertToParent(self.as_ptr(), point)
        }
    }
    // BLOCKED: fn ConvertToParent1()
    /// Convert point from the parent's coordinate system to the view's coordinate system in place.
    ///
    /// See [C++ `BView::ConvertFromParent()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a43c16140dd4491a6a6f7bad11bbea1f9).
    fn convert_from_parent_point<P: PointMethods>(&self, point: Option<&P>) {
        unsafe {
            let point = match point {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_ConvertFromParent(self.as_ptr(), point)
        }
    }
    // BLOCKED: fn ConvertFromParent1()
    /// Convert rect to the parent's coordinate system in place.
    ///
    /// See [C++ `BView::ConvertToParent()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ac7bd2e0e7aac40d1be9a47999b5701a9).
    fn convert_to_parent_rect<R: RectMethods>(&self, rect: Option<&R>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_ConvertToParent2(self.as_ptr(), rect)
        }
    }
    // BLOCKED: fn ConvertToParent3()
    /// Convert rect from the parent's coordinate system to the view's coordinate system in place.
    ///
    /// See [C++ `BView::ConvertFromParent()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ab2aeeb14409a18f950173e7cb22e147c).
    fn convert_from_parent_rect<R: RectMethods>(&self, rect: Option<&R>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_ConvertFromParent2(self.as_ptr(), rect)
        }
    }
    // BLOCKED: fn ConvertFromParent3()
    /// Sets the view flags to the flags mask.
    ///
    /// See [C++ `BView::SetFlags()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a4740f9a1e1163073b042102a421a2e33).
    fn set_flags(&self, flags: u32) {
        unsafe { ffi::BView_SetFlags(self.as_ptr(), flags) }
    }
    /// Return the view flags set in the constructor or by SetFlags().
    ///
    /// See [C++ `BView::Flags()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a561b10abf94dd65a26b02253af667508).
    fn flags(&self) -> u32 {
        unsafe { ffi::BView_Flags(self.as_ptr()) }
    }
    /// Fill out region with the view's clipping region.
    ///
    /// See [C++ `BView::GetClippingRegion()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#acf50a063c9621f35a84b1d0245b12d69).
    fn get_clipping_region(&self, region: *mut c_void) {
        unsafe { ffi::BView_GetClippingRegion(self.as_ptr(), region) }
    }
    /// Set the clipping region the region restricting the area that the view can draw in.
    ///
    /// See [C++ `BView::ConstrainClippingRegion()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a656b8d82a6a7c7c6ab33c3dd08aad6ec).
    fn constrain_clipping_region(&self, region: *mut c_void) {
        unsafe { ffi::BView_ConstrainClippingRegion(self.as_ptr(), region) }
    }
    /// Intersects the current clipping region of the view with the pixels of picture.
    ///
    /// See [C++ `BView::ClipToPicture()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a9c92c9e8e306d00dbfef81f2b0282339).
    fn clip_to_picture<P: PointMethods>(&self, picture: *mut c_void, where_: &P, sync: bool) {
        unsafe {
            let where_ = where_.as_ptr();
            ffi::BView_ClipToPicture(self.as_ptr(), picture, where_, sync)
        }
    }
    /// Intersects the current clipping region of the view with the pixels outside of picture.
    ///
    /// See [C++ `BView::ClipToInversePicture()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a18d61bcae2a7ca8a1a0977ac23d15171).
    fn clip_to_inverse_picture<P: PointMethods>(
        &self,
        picture: *mut c_void,
        where_: &P,
        sync: bool,
    ) {
        unsafe {
            let where_ = where_.as_ptr();
            ffi::BView_ClipToInversePicture(self.as_ptr(), picture, where_, sync)
        }
    }
    /// Intersects the current clipping region of the view with the pixels of rect.
    ///
    /// See [C++ `BView::ClipToRect()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a22ff147a2f0a0bd65de494c9a039acee).
    fn clip_to_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::BView_ClipToRect(self.as_ptr(), rect)
        }
    }
    /// Intersects the current clipping region of the view with the pixels outside of rect.
    ///
    /// See [C++ `BView::ClipToInverseRect()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ad1762d160500ec3ce90c7bd73944d8a7).
    fn clip_to_inverse_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::BView_ClipToInverseRect(self.as_ptr(), rect)
        }
    }
    /// Intersects the current clipping region of the view with the pixels of shape.
    ///
    /// See [C++ `BView::ClipToShape()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ac0884c62949361a7eb6a4834f46b902c).
    fn clip_to_shape(&self, shape: *mut c_void) {
        unsafe { ffi::BView_ClipToShape(self.as_ptr(), shape) }
    }
    /// Intersects the current clipping region of the view with the pixels outside of shape.
    ///
    /// See [C++ `BView::ClipToInverseShape()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a17dcb4f2e92994cde9a9ece9caa619c2).
    fn clip_to_inverse_shape(&self, shape: *mut c_void) {
        unsafe { ffi::BView_ClipToInverseShape(self.as_ptr(), shape) }
    }
    /// Sets the drawing mode of the view.
    ///
    /// See [C++ `BView::SetDrawingMode()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a0c392ed0e36213e1007322a9cbca71c0).
    fn set_drawing_mode(&self, mode: drawing_mode) {
        unsafe { ffi::BView_SetDrawingMode(self.as_ptr(), mode) }
    }
    /// Return the current drawing_mode.
    ///
    /// See [C++ `BView::DrawingMode()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae7953a2e451fb6e5a439df9e9dc8451a).
    fn drawing_mode(&self) -> drawing_mode {
        unsafe { ffi::BView_DrawingMode(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetBlendingMode()
    /// Fill out srcAlpha and alphaFunc with the alpha mode and alpha function of the view.
    ///
    /// See [C++ `BView::GetBlendingMode()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ad50d5ba2249fdf3dd8640b3573c2a2e8).
    fn get_blending_mode(&self, src_alpha: *mut c_void, alpha_func: *mut c_void) {
        unsafe { ffi::BView_GetBlendingMode(self.as_ptr(), src_alpha, alpha_func) }
    }
    /// Set the pen size to size.
    ///
    /// See [C++ `BView::SetPenSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a80ea4cb97d2e6e2c3088e610a9e157e6).
    fn set_pen_size(&self, size: c_float) {
        unsafe { ffi::BView_SetPenSize(self.as_ptr(), size) }
    }
    /// Return the current pen size.
    ///
    /// See [C++ `BView::PenSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a69d8657d26b49773fa2de914f263b204).
    fn pen_size(&self) -> c_float {
        unsafe { ffi::BView_PenSize(self.as_ptr()) }
    }
    /// Tests if the view has any colors set.
    ///
    /// See [C++ `BView::HasDefaultColors()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a606e0e7e03516dccb53bf073827460a8).
    fn has_default_colors(&self) -> bool {
        unsafe { ffi::BView_HasDefaultColors(self.as_ptr()) }
    }
    /// Tests if the view is using system "panel" colors. B_PANEL_BACKGROUND_COLOR for ViewUIColor() B_PANEL_BACKGROUND_COLOR for LowUIColor() B_PANEL_TEXT_COLOR for HighUIColor()
    ///
    /// See [C++ `BView::HasSystemColors()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aa9aacc95606b650e6a8d776b51cb902b).
    fn has_system_colors(&self) -> bool {
        unsafe { ffi::BView_HasSystemColors(self.as_ptr()) }
    }
    /// Attempts to use the colors of any parent view. Will adopt view, low, and high colors. Should be called in AttachedToWindow() or AllAttached().
    ///
    /// See [C++ `BView::AdoptParentColors()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ac780f029d415ee0a915792b4e756fa77).
    fn adopt_parent_colors(&self) {
        unsafe { ffi::BView_AdoptParentColors(self.as_ptr()) }
    }
    /// Instructs view to use standard system "panel" colors. B_PANEL_BACKGROUND_COLOR for ViewUIColor() B_PANEL_BACKGROUND_COLOR for LowUIColor() B_PANEL_TEXT_COLOR for HighUIColor()
    ///
    /// See [C++ `BView::AdoptSystemColors()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a0e429a82fe06e9ab390b06d993ee31be).
    fn adopt_system_colors(&self) {
        unsafe { ffi::BView_AdoptSystemColors(self.as_ptr()) }
    }
    /// Attempts to use the colors of a given view. Will adopt view, low, and high colors.
    ///
    /// See [C++ `BView::AdoptViewColors()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ab68ad7103708884f9e6fd84e8a579746).
    fn adopt_view_colors<V: ViewMethods>(&self, view: Option<&V>) {
        unsafe {
            let view = match view {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_AdoptViewColors(self.as_ptr(), view)
        }
    }
    // NOT_SUPPORTED: fn SetViewColor()
    // NOT_SUPPORTED: fn SetViewColor1()
    // NOT_SUPPORTED: fn ViewColor()
    /// Set the view color of the view to a system constant. The color will update live with user changes.
    ///
    /// See [C++ `BView::SetViewUIColor()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a34f020c4bb1008cef972cf1eec7dba0f).
    fn set_view_ui_color(&self, which: color_which, tint: c_float) {
        unsafe { ffi::BView_SetViewUIColor(self.as_ptr(), which, tint) }
    }
    /// Return the current view color constant being used.
    ///
    /// See [C++ `BView::ViewUIColor()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aec6c89446eb4f5ded151adced9126df8).
    fn view_ui_color(&self, tint: *mut c_void) -> color_which {
        unsafe { ffi::BView_ViewUIColor(self.as_ptr(), tint) }
    }
    // NOT_SUPPORTED: fn SetHighColor()
    // NOT_SUPPORTED: fn SetHighColor1()
    // NOT_SUPPORTED: fn HighColor()
    /// Set the high color of the view to a system constant. The color will update live with user changes.
    ///
    /// See [C++ `BView::SetHighUIColor()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a54eefc28bf7a4bc30612cf83aff2963b).
    fn set_high_ui_color(&self, which: color_which, tint: c_float) {
        unsafe { ffi::BView_SetHighUIColor(self.as_ptr(), which, tint) }
    }
    /// Return the current high color constant being used.
    ///
    /// See [C++ `BView::HighUIColor()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aa86f548fba2e82d022066198be6d5af6).
    fn high_ui_color(&self, tint: *mut c_void) -> color_which {
        unsafe { ffi::BView_HighUIColor(self.as_ptr(), tint) }
    }
    // NOT_SUPPORTED: fn SetLowColor()
    // NOT_SUPPORTED: fn SetLowColor1()
    // NOT_SUPPORTED: fn LowColor()
    /// Set the low color of the view to a system constant. The color will update live with user changes.
    ///
    /// See [C++ `BView::SetLowUIColor()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a879ab88d4e64345069f1041d3a7b652f).
    fn set_low_ui_color(&self, which: color_which, tint: c_float) {
        unsafe { ffi::BView_SetLowUIColor(self.as_ptr(), which, tint) }
    }
    /// Return the current low color constant being used.
    ///
    /// See [C++ `BView::LowUIColor()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae32ca6d8f106d44b5b51df82a7eb64c5).
    fn low_ui_color(&self, tint: *mut c_void) -> color_which {
        unsafe { ffi::BView_LowUIColor(self.as_ptr(), tint) }
    }
    // NOT_SUPPORTED: fn SetLineMode()
    // NOT_SUPPORTED: fn LineJoinMode()
    // NOT_SUPPORTED: fn LineCapMode()
    /// Returns the miter limit used for B_MITER_JOIN join mode.
    ///
    /// See [C++ `BView::LineMiterLimit()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ab1cd7cf77a4fa01755d186a05163cfbe).
    fn line_miter_limit(&self) -> c_float {
        unsafe { ffi::BView_LineMiterLimit(self.as_ptr()) }
    }
    /// Sets the fill rule for the view.
    ///
    /// See [C++ `BView::SetFillRule()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a2019def93337069c0b7d416e103522e9).
    fn set_fill_rule(&self, rule: i32) {
        unsafe { ffi::BView_SetFillRule(self.as_ptr(), rule) }
    }
    /// Return the current fill mode.
    ///
    /// See [C++ `BView::FillRule()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a3a2fd2d7313a9bb7dff359a4f4362a5f).
    fn fill_rule(&self) -> i32 {
        unsafe { ffi::BView_FillRule(self.as_ptr()) }
    }
    /// Sets the origin in the view's coordinate system.
    ///
    /// See [C++ `BView::SetOrigin()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a769e5fa6486adf12e7a3511ed372a06c).
    fn set_origin_point<P: PointMethods>(&self, where_: &P) {
        unsafe {
            let where_ = where_.as_ptr();
            ffi::BView_SetOrigin(self.as_ptr(), where_)
        }
    }
    /// Sets the origin in the view's coordinate system.
    ///
    /// See [C++ `BView::SetOrigin()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a7cf4e3ac5b0759b9c38d7b36517f2337).
    fn set_origin_float(&self, x: c_float, y: c_float) {
        unsafe { ffi::BView_SetOrigin1(self.as_ptr(), x, y) }
    }
    /// Returns the origin point in the view's coordinate system.
    ///
    /// See [C++ `BView::Origin()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aefbf71a7a403900e960ffa3943dc9901).
    fn origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::BView_Origin(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn SetTransform()
    // NOT_SUPPORTED: fn Transform()
    /// Translate the current view by coordinates.
    ///
    /// See [C++ `BView::TranslateBy()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a8ae6d4e1cdfeeee33b753d56bee6ae7f).
    fn translate_by(&self, x: c_double, y: c_double) {
        unsafe { ffi::BView_TranslateBy(self.as_ptr(), x, y) }
    }
    /// Scale the current view by factors x and y.
    ///
    /// See [C++ `BView::ScaleBy()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ac83f9fdfb882f849e3ac79bfb06a0fd6).
    fn scale_by(&self, x: c_double, y: c_double) {
        unsafe { ffi::BView_ScaleBy(self.as_ptr(), x, y) }
    }
    /// Rotate the current view by angleRadians.
    ///
    /// See [C++ `BView::RotateBy()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a02ab6b6196b140b20ae516a55680e30b).
    fn rotate_by(&self, angle_radians: c_double) {
        unsafe { ffi::BView_RotateBy(self.as_ptr(), angle_radians) }
    }
    // NOT_SUPPORTED: fn TransformTo()
    /// Saves the drawing state to the stack.
    ///
    /// See [C++ `BView::PushState()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a1dcb914cd0c6ef8d1291aa8492968d40).
    fn push_state(&self) {
        unsafe { ffi::BView_PushState(self.as_ptr()) }
    }
    /// Restores the drawing state from the stack.
    ///
    /// See [C++ `BView::PopState()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#abc0d202fe13366b80703afbdb3832951).
    fn pop_state(&self) {
        unsafe { ffi::BView_PopState(self.as_ptr()) }
    }
    /// Move the pen to point in the view's coordinate system.
    ///
    /// See [C++ `BView::MovePenTo()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#acb61254a2121f569f6223a95cd835abd).
    fn move_pen_to_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::BView_MovePenTo(self.as_ptr(), pt)
        }
    }
    /// Move the pen to the point specified by x and y in the view's coordinate system.
    ///
    /// See [C++ `BView::MovePenTo()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae32d7951a4ce9cff6b81899f5208a25f).
    fn move_pen_to_float(&self, x: c_float, y: c_float) {
        unsafe { ffi::BView_MovePenTo1(self.as_ptr(), x, y) }
    }
    /// Move the pen by x pixels horizontally and y pixels vertically.
    ///
    /// See [C++ `BView::MovePenBy()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a979f8f0090284919b6e7c6c04d91cbde).
    fn move_pen_by(&self, x: c_float, y: c_float) {
        unsafe { ffi::BView_MovePenBy(self.as_ptr(), x, y) }
    }
    /// Return the current pen location as a BPoint object.
    ///
    /// See [C++ `BView::PenLocation()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a2774e13c2b12c6a5fee333e95b76b07d).
    fn pen_location(&self) -> Point {
        unsafe { Point::from_ptr(ffi::BView_PenLocation(self.as_ptr())) }
    }
    /// Set the font of the view.
    ///
    /// See [C++ `BView::SetFont()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a9247127c151153542a3e6bc1f3904433).
    fn set_font(&self, font: *const c_void, mask: u32) {
        unsafe { ffi::BView_SetFont(self.as_ptr(), font, mask) }
    }
    /// Fill out font with the font set to the view.
    ///
    /// See [C++ `BView::GetFont()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#af244d01f85e6d9ae7f5fcca4637dc78c).
    fn get_font(&self, font: *mut c_void) {
        unsafe { ffi::BView_GetFont(self.as_ptr(), font) }
    }
    /// Truncate string with truncation mode mode so that it is no wider than width set in the view's font.
    ///
    /// See [C++ `BView::TruncateString()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ac2e293fbdd999e4857b44b6e4d4c3820).
    fn truncate_string(&self, in_out: *mut c_void, mode: u32, width: c_float) {
        unsafe { ffi::BView_TruncateString(self.as_ptr(), in_out, mode, width) }
    }
    /// Return the width of string set in the font of the view.
    ///
    /// See [C++ `BView::StringWidth()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#adef480de8a8bcd2fff3129660501616e).
    fn string_width(&self, string: &str) -> c_float {
        unsafe {
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            ffi::BView_StringWidth(self.as_ptr(), string)
        }
    }
    /// Return the width of string set in the font of the view up to length characters.
    ///
    /// See [C++ `BView::StringWidth()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aef3638cf597407e6a9f31bc75cd42097).
    fn string_width_int32(&self, string: &str, length: i32) -> c_float {
        unsafe {
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            ffi::BView_StringWidth1(self.as_ptr(), string, length)
        }
    }
    // NOT_SUPPORTED: fn GetStringWidths()
    /// Set the size of the view's font to size.
    ///
    /// See [C++ `BView::SetFontSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a443e4d9ff182f85a6b1c8478ef22ffe8).
    fn set_font_size(&self, size: c_float) {
        unsafe { ffi::BView_SetFontSize(self.as_ptr(), size) }
    }
    /// Turn anti-aliasing on and off when printing.
    ///
    /// See [C++ `BView::ForceFontAliasing()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a424161ccf6f52a354fdd67ed1376d196).
    fn force_font_aliasing(&self, enable: bool) {
        unsafe { ffi::BView_ForceFontAliasing(self.as_ptr(), enable) }
    }
    /// Fill out the font_height struct with the view font.
    ///
    /// See [C++ `BView::GetFontHeight()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a5bf95d66754634c647b2ddf7d39b4dfb).
    fn get_font_height(&self, height: *mut c_void) {
        unsafe { ffi::BView_GetFontHeight(self.as_ptr(), height) }
    }
    /// Sets the scale of the coordinate system the view uses for drawing.
    ///
    /// See [C++ `BView::SetScale()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a077136ea06f0a6b39aa0b8b24311cce8).
    fn set_scale(&self, scale: c_float) {
        unsafe { ffi::BView_SetScale(self.as_ptr(), scale) }
    }
    /// Return the current drawing scale.
    ///
    /// See [C++ `BView::Scale()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a50e12d6b8080b3bc6bc617d31d1ab200).
    fn scale(&self) -> c_float {
        unsafe { ffi::BView_Scale(self.as_ptr()) }
    }
    /// Sets the background bitmap of the view.
    ///
    /// See [C++ `BView::SetViewBitmap()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ac247ddc58f1cb719d60e354a77c51c1e).
    fn set_view_bitmap_rect<R: RectMethods, R2: RectMethods>(
        &self,
        bitmap: *const c_void,
        src_rect: &R,
        dst_rect: &R2,
        follow_flags: u32,
        options: u32,
    ) {
        unsafe {
            let src_rect = src_rect.as_ptr();
            let dst_rect = dst_rect.as_ptr();
            ffi::BView_SetViewBitmap(
                self.as_ptr(),
                bitmap,
                src_rect,
                dst_rect,
                follow_flags,
                options,
            )
        }
    }
    /// Sets the background bitmap of the view.
    ///
    /// See [C++ `BView::SetViewBitmap()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a046a9c1656823db5401e530cf8121734).
    fn set_view_bitmap_uint32(&self, bitmap: *const c_void, follow_flags: u32, options: u32) {
        unsafe { ffi::BView_SetViewBitmap1(self.as_ptr(), bitmap, follow_flags, options) }
    }
    /// Clears the background bitmap of the view if it has one.
    ///
    /// See [C++ `BView::ClearViewBitmap()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae7582b9a2b0b446a1de16c681f23baf9).
    fn clear_view_bitmap(&self) {
        unsafe { ffi::BView_ClearViewBitmap(self.as_ptr()) }
    }
    /// Sets the overlay bitmap of the view.
    ///
    /// See [C++ `BView::SetViewOverlay()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a576100502a8c6dfb46ff36b08cbe9405).
    fn set_view_overlay_rect<R: RectMethods, R2: RectMethods>(
        &self,
        overlay: *const c_void,
        src_rect: &R,
        dst_rect: &R2,
        color_key: *mut c_void,
        follow_flags: u32,
        options: u32,
    ) -> status_t {
        unsafe {
            let src_rect = src_rect.as_ptr();
            let dst_rect = dst_rect.as_ptr();
            ffi::BView_SetViewOverlay(
                self.as_ptr(),
                overlay,
                src_rect,
                dst_rect,
                color_key,
                follow_flags,
                options,
            )
        }
    }
    /// Sets the overlay bitmap of the view.
    ///
    /// See [C++ `BView::SetViewOverlay()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a9770c1bb97d7ffd797f7c40ca1339f98).
    fn set_view_overlay_rgb_color(
        &self,
        overlay: *const c_void,
        color_key: *mut c_void,
        follow_flags: u32,
        options: u32,
    ) -> status_t {
        unsafe {
            ffi::BView_SetViewOverlay1(self.as_ptr(), overlay, color_key, follow_flags, options)
        }
    }
    /// Clears the overlay bitmap of the view if it has one.
    ///
    /// See [C++ `BView::ClearViewOverlay()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a0ae48f37c3f56b422841638971488cc0).
    fn clear_view_overlay(&self) {
        unsafe { ffi::BView_ClearViewOverlay(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn StrokeLine()
    // NOT_SUPPORTED: fn StrokeLine1()
    /// Begin a line array of up to count lines.
    ///
    /// See [C++ `BView::BeginLineArray()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ab3f2e9f54d2b93952b27e182ac552983).
    fn begin_line_array(&self, count: i32) {
        unsafe { ffi::BView_BeginLineArray(self.as_ptr(), count) }
    }
    // NOT_SUPPORTED: fn AddLine()
    /// End the line array drawing the lines.
    ///
    /// See [C++ `BView::EndLineArray()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a24ba922f9698936bc27f48b3c51349a1).
    fn end_line_array(&self) {
        unsafe { ffi::BView_EndLineArray(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn StrokePolygon()
    // NOT_SUPPORTED: fn StrokePolygon1()
    // NOT_SUPPORTED: fn StrokePolygon2()
    // NOT_SUPPORTED: fn FillPolygon()
    // NOT_SUPPORTED: fn FillPolygon1()
    // NOT_SUPPORTED: fn FillPolygon2()
    /// Fill a polygon shape with the specified gradient pattern.
    ///
    /// See [C++ `BView::FillPolygon()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a96cdb60a8c87e4958380a3077a2197a5).
    fn fill_polygon_polygon(&self, polygon: *const c_void, gradient: *const c_void) {
        unsafe { ffi::BView_FillPolygon3(self.as_ptr(), polygon, gradient) }
    }
    /// Fill a polygon shape made up of points specified by pointArray with the specified gradient pattern.
    ///
    /// See [C++ `BView::FillPolygon()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a22a5ce1f024e622b6eade6a9176ff1f7).
    fn fill_polygon_point_gradient<P: PointMethods>(
        &self,
        point_array: Option<&P>,
        num_points: i32,
        gradient: *const c_void,
    ) {
        unsafe {
            let point_array = match point_array {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_FillPolygon4(self.as_ptr(), point_array, num_points, gradient)
        }
    }
    /// Fill a polygon shape made up of points specified by pointArray inscribed by bounds with the specified gradient pattern.
    ///
    /// See [C++ `BView::FillPolygon()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a57b0870cd6a0109c347d80c34ca2f34a).
    fn fill_polygon_point_rect<P: PointMethods, R: RectMethods>(
        &self,
        point_array: Option<&P>,
        num_points: i32,
        bounds: &R,
        gradient: *const c_void,
    ) {
        unsafe {
            let point_array = match point_array {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let bounds = bounds.as_ptr();
            ffi::BView_FillPolygon5(self.as_ptr(), point_array, num_points, bounds, gradient)
        }
    }
    // NOT_SUPPORTED: fn StrokeTriangle()
    // NOT_SUPPORTED: fn StrokeTriangle1()
    // NOT_SUPPORTED: fn FillTriangle()
    // NOT_SUPPORTED: fn FillTriangle1()
    /// Fill the triangle specified by points point1, point2, and point3 with the specified gradient pattern.
    ///
    /// See [C++ `BView::FillTriangle()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a5dd1c64839c11a79e80581e37656cc26).
    fn fill_triangle_gradient<P: PointMethods, P2: PointMethods, P3: PointMethods>(
        &self,
        point1: &P,
        point2: &P2,
        point3: &P3,
        gradient: *const c_void,
    ) {
        unsafe {
            let point1 = point1.as_ptr();
            let point2 = point2.as_ptr();
            let point3 = point3.as_ptr();
            ffi::BView_FillTriangle2(self.as_ptr(), point1, point2, point3, gradient)
        }
    }
    /// Fill the triangle specified by points point1, point2, and point3 and enclosed by bounds with the specified gradient pattern.
    ///
    /// See [C++ `BView::FillTriangle()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a0878dea14a61d58c951bd8e11c0a2ef6).
    fn fill_triangle_rect<P: PointMethods, P2: PointMethods, P3: PointMethods, R: RectMethods>(
        &self,
        point1: &P,
        point2: &P2,
        point3: &P3,
        bounds: &R,
        gradient: *const c_void,
    ) {
        unsafe {
            let point1 = point1.as_ptr();
            let point2 = point2.as_ptr();
            let point3 = point3.as_ptr();
            let bounds = bounds.as_ptr();
            ffi::BView_FillTriangle3(self.as_ptr(), point1, point2, point3, bounds, gradient)
        }
    }
    // NOT_SUPPORTED: fn StrokeRect()
    // NOT_SUPPORTED: fn FillRect()
    /// Fill the rectangle specified by rect with the specified gradient pattern.
    ///
    /// See [C++ `BView::FillRect()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aae6a3ff6683bb15ac8c647c946475e8f).
    fn fill_rect<R: RectMethods>(&self, rect: &R, gradient: *const c_void) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::BView_FillRect1(self.as_ptr(), rect, gradient)
        }
    }
    // NOT_SUPPORTED: fn FillRegion()
    /// Fill region with the specified gradient pattern.
    ///
    /// See [C++ `BView::FillRegion()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a56828d13947717462b661e1645305509).
    fn fill_region(&self, rectegion: *mut c_void, gradient: *const c_void) {
        unsafe { ffi::BView_FillRegion1(self.as_ptr(), rectegion, gradient) }
    }
    /// Inverts the colors within rect.
    ///
    /// See [C++ `BView::InvertRect()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#af7e5ebc41cd5a471dcdcfa1f3db801a5).
    fn invert_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::BView_InvertRect(self.as_ptr(), rect)
        }
    }
    // NOT_SUPPORTED: fn StrokeRoundRect()
    // NOT_SUPPORTED: fn FillRoundRect()
    /// Fill the rounded rectangle with horizontal radius xRadius and vertical radius yRadius with the specified gradient pattern.
    ///
    /// See [C++ `BView::FillRoundRect()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a73956d28b356b2eee83ebc19bd6e398b).
    fn fill_round_rect<R: RectMethods>(
        &self,
        rect: &R,
        x_radius: c_float,
        y_radius: c_float,
        gradient: *const c_void,
    ) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::BView_FillRoundRect1(self.as_ptr(), rect, x_radius, y_radius, gradient)
        }
    }
    // NOT_SUPPORTED: fn StrokeEllipse()
    // NOT_SUPPORTED: fn StrokeEllipse1()
    // NOT_SUPPORTED: fn FillEllipse()
    // NOT_SUPPORTED: fn FillEllipse1()
    /// Fill an ellipse with the specified gradient pattern starting at center with a horizontal radius of xRadius and a vertical radius of yRadius.
    ///
    /// See [C++ `BView::FillEllipse()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a2738cf2f3d39f560d0cff90543f45afa).
    fn fill_ellipse_point<P: PointMethods>(
        &self,
        center: &P,
        x_radius: c_float,
        y_radius: c_float,
        gradient: *const c_void,
    ) {
        unsafe {
            let center = center.as_ptr();
            ffi::BView_FillEllipse2(self.as_ptr(), center, x_radius, y_radius, gradient)
        }
    }
    /// Fill an ellipse with the specified gradient pattern inscribed within rect.
    ///
    /// See [C++ `BView::FillEllipse()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a86de91f260577187eaf22ba0fdf32fba).
    fn fill_ellipse_rect<R: RectMethods>(&self, rect: &R, gradient: *const c_void) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::BView_FillEllipse3(self.as_ptr(), rect, gradient)
        }
    }
    // NOT_SUPPORTED: fn StrokeArc()
    // NOT_SUPPORTED: fn StrokeArc1()
    // NOT_SUPPORTED: fn FillArc()
    // NOT_SUPPORTED: fn FillArc1()
    /// Fill an arc with the specified gradient pattern starting at center with a horizontal radius of xRadius and a vertical radius of yRadius starting at startAngle and drawing arcAngle degrees.
    ///
    /// See [C++ `BView::FillArc()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a49017edefe45ca5c74954e1227e673bb).
    fn fill_arc_point<P: PointMethods>(
        &self,
        center: &P,
        x_radius: c_float,
        y_radius: c_float,
        start_angle: c_float,
        arc_angle: c_float,
        gradient: *const c_void,
    ) {
        unsafe {
            let center = center.as_ptr();
            ffi::BView_FillArc2(
                self.as_ptr(),
                center,
                x_radius,
                y_radius,
                start_angle,
                arc_angle,
                gradient,
            )
        }
    }
    /// Fill an arc with the specified gradient pattern inscribed within rect starting at startAngle and drawing arcAngle degrees.
    ///
    /// See [C++ `BView::FillArc()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a4304fe45708f72d0ba20b579870c24ce).
    fn fill_arc_rect<R: RectMethods>(
        &self,
        rect: &R,
        start_angle: c_float,
        arc_angle: c_float,
        gradient: *const c_void,
    ) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::BView_FillArc3(self.as_ptr(), rect, start_angle, arc_angle, gradient)
        }
    }
    // NOT_SUPPORTED: fn StrokeBezier()
    // NOT_SUPPORTED: fn FillBezier()
    /// Fill a bezier curve.
    ///
    /// See [C++ `BView::FillBezier()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a7cf8a5eaa5451b2c7df0d6fa392012bd).
    fn fill_bezier<P: PointMethods>(&self, control_points: Option<&P>, gradient: *const c_void) {
        unsafe {
            let control_points = match control_points {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_FillBezier1(self.as_ptr(), control_points, gradient)
        }
    }
    // NOT_SUPPORTED: fn StrokeShape()
    // NOT_SUPPORTED: fn FillShape()
    /// Fill shape with the specified gradient pattern.
    ///
    /// See [C++ `BView::FillShape()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a9a82da733397e97b985875be941f77a6).
    fn fill_shape(&self, shape: *mut c_void, gradient: *const c_void) {
        unsafe { ffi::BView_FillShape1(self.as_ptr(), shape, gradient) }
    }
    /// Copy the bits from the src rectangle to the dst rectangle in the view's coordinate system.
    ///
    /// See [C++ `BView::CopyBits()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a8ace5c5cc811c4ad16acf281ffd1a48d).
    fn copy_bits<R: RectMethods, R2: RectMethods>(&self, src: &R, dst: &R2) {
        unsafe {
            let src = src.as_ptr();
            let dst = dst.as_ptr();
            ffi::BView_CopyBits(self.as_ptr(), src, dst)
        }
    }
    /// Draws bitmap on the view within viewRect asynchronously. bitmap portion is scaled to fit viewRect.
    ///
    /// See [C++ `BView::DrawBitmapAsync()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae66727344d35f05c56b6163ad70e360c).
    fn draw_bitmap_async_rect_rect_uint32<R: RectMethods, R2: RectMethods>(
        &self,
        a_bitmap: *const c_void,
        bitmap_rect: &R,
        view_rect: &R2,
        options: u32,
    ) {
        unsafe {
            let bitmap_rect = bitmap_rect.as_ptr();
            let view_rect = view_rect.as_ptr();
            ffi::BView_DrawBitmapAsync(self.as_ptr(), a_bitmap, bitmap_rect, view_rect, options)
        }
    }
    /// Draws bitmap on the view within viewRect asynchronously. bitmap portion is scaled to fit viewRect.
    ///
    /// See [C++ `BView::DrawBitmapAsync()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a0ef432afa41aefcb3fdd537e2d290001).
    fn draw_bitmap_async_rect_rect<R: RectMethods, R2: RectMethods>(
        &self,
        a_bitmap: *const c_void,
        bitmap_rect: &R,
        view_rect: &R2,
    ) {
        unsafe {
            let bitmap_rect = bitmap_rect.as_ptr();
            let view_rect = view_rect.as_ptr();
            ffi::BView_DrawBitmapAsync1(self.as_ptr(), a_bitmap, bitmap_rect, view_rect)
        }
    }
    /// Draws bitmap on the view within viewRect asynchronously. bitmap is scaled to fit viewRect.
    ///
    /// See [C++ `BView::DrawBitmapAsync()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aa3275b791787a66f3638611cd6a5ae09).
    fn draw_bitmap_async_rect<R: RectMethods>(&self, a_bitmap: *const c_void, view_rect: &R) {
        unsafe {
            let view_rect = view_rect.as_ptr();
            ffi::BView_DrawBitmapAsync2(self.as_ptr(), a_bitmap, view_rect)
        }
    }
    /// Draws bitmap on the view offset by where asynchronously.
    ///
    /// See [C++ `BView::DrawBitmapAsync()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a6931c5fe803eb5fa9e66438d347c4b6d).
    fn draw_bitmap_async_point<P: PointMethods>(&self, a_bitmap: *const c_void, where_: &P) {
        unsafe {
            let where_ = where_.as_ptr();
            ffi::BView_DrawBitmapAsync3(self.as_ptr(), a_bitmap, where_)
        }
    }
    /// Draws bitmap on the view asynchronously.
    ///
    /// See [C++ `BView::DrawBitmapAsync()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ab50fba0500357abebb9f3ef3832f2e68).
    fn draw_bitmap_async(&self, a_bitmap: *const c_void) {
        unsafe { ffi::BView_DrawBitmapAsync4(self.as_ptr(), a_bitmap) }
    }
    /// Draws bitmap on the view within viewRect. bitmap portion is scaled to fit viewRect.
    ///
    /// See [C++ `BView::DrawBitmap()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a6171150993fbf7c73966c7c302a95573).
    fn draw_bitmap_rect_rect_uint32<R: RectMethods, R2: RectMethods>(
        &self,
        a_bitmap: *const c_void,
        bitmap_rect: &R,
        view_rect: &R2,
        options: u32,
    ) {
        unsafe {
            let bitmap_rect = bitmap_rect.as_ptr();
            let view_rect = view_rect.as_ptr();
            ffi::BView_DrawBitmap(self.as_ptr(), a_bitmap, bitmap_rect, view_rect, options)
        }
    }
    /// Draws bitmap on the view within viewRect. bitmap portion is scaled to fit viewRect.
    ///
    /// See [C++ `BView::DrawBitmap()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a63db7245fd9b9fefd461d7596c15f9cc).
    fn draw_bitmap_rect_rect<R: RectMethods, R2: RectMethods>(
        &self,
        a_bitmap: *const c_void,
        bitmap_rect: &R,
        view_rect: &R2,
    ) {
        unsafe {
            let bitmap_rect = bitmap_rect.as_ptr();
            let view_rect = view_rect.as_ptr();
            ffi::BView_DrawBitmap1(self.as_ptr(), a_bitmap, bitmap_rect, view_rect)
        }
    }
    /// Draws bitmap on the view within viewRect. bitmap is scaled to fit viewRect.
    ///
    /// See [C++ `BView::DrawBitmap()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#abce81666128d183d59b5cf2ed75c7f07).
    fn draw_bitmap_rect<R: RectMethods>(&self, a_bitmap: *const c_void, view_rect: &R) {
        unsafe {
            let view_rect = view_rect.as_ptr();
            ffi::BView_DrawBitmap2(self.as_ptr(), a_bitmap, view_rect)
        }
    }
    /// Draws bitmap on the view offset by where.
    ///
    /// See [C++ `BView::DrawBitmap()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a1c8b6e3e9218388a68ae43dd408a187c).
    fn draw_bitmap_point<P: PointMethods>(&self, a_bitmap: *const c_void, where_: &P) {
        unsafe {
            let where_ = where_.as_ptr();
            ffi::BView_DrawBitmap3(self.as_ptr(), a_bitmap, where_)
        }
    }
    /// Draws bitmap on the view.
    ///
    /// See [C++ `BView::DrawBitmap()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a769c3e3dee7d86ed50308f7a3f46ef91).
    fn draw_bitmap(&self, a_bitmap: *const c_void) {
        unsafe { ffi::BView_DrawBitmap4(self.as_ptr(), a_bitmap) }
    }
    /// Draws bitmap on the view within viewRect asynchronously. If bitmap is smaller, it is cloned to fill remaining space in viewRect.
    ///
    /// See [C++ `BView::DrawTiledBitmapAsync()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ad281a4e40f212e3cb56c23fa6e574a84).
    fn draw_tiled_bitmap_async<R: RectMethods, P: PointMethods>(
        &self,
        a_bitmap: *const c_void,
        view_rect: &R,
        phase: &P,
    ) {
        unsafe {
            let view_rect = view_rect.as_ptr();
            let phase = phase.as_ptr();
            ffi::BView_DrawTiledBitmapAsync(self.as_ptr(), a_bitmap, view_rect, phase)
        }
    }
    /// Draws bitmap on the view within viewRect. If bitmap is smaller, it is cloned to fill remaining space in viewRect.
    ///
    /// See [C++ `BView::DrawTiledBitmap()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a8848d931fd0354860a0e71934e66cb03).
    fn draw_tiled_bitmap<R: RectMethods, P: PointMethods>(
        &self,
        a_bitmap: *const c_void,
        view_rect: &R,
        phase: &P,
    ) {
        unsafe {
            let view_rect = view_rect.as_ptr();
            let phase = phase.as_ptr();
            ffi::BView_DrawTiledBitmap(self.as_ptr(), a_bitmap, view_rect, phase)
        }
    }
    /// Draws character c onto to the view at the current pen position.
    ///
    /// See [C++ `BView::DrawChar()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a02ba5c68f21d4345ec3ee683496ce851).
    fn draw_char(&self, a_char: c_char) {
        unsafe { ffi::BView_DrawChar(self.as_ptr(), a_char) }
    }
    /// Draws character c at the specified location in the view.
    ///
    /// See [C++ `BView::DrawChar()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a2ed7c3628692426116fc0bb450c0f0c7).
    fn draw_char_point<P: PointMethods>(&self, a_char: c_char, location: &P) {
        unsafe {
            let location = location.as_ptr();
            ffi::BView_DrawChar1(self.as_ptr(), a_char, location)
        }
    }
    /// Draw string onto the view at the current pen position.
    ///
    /// See [C++ `BView::DrawString()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a5c261dbbb4e8157799e85d6ad1530920).
    fn draw_string_escapement_delta(&self, string: &str, delta: *mut c_void) {
        unsafe {
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            ffi::BView_DrawString(self.as_ptr(), string, delta)
        }
    }
    // BLOCKED: fn DrawString1()
    /// Draw string up to length characters onto the view at the current pen position.
    ///
    /// See [C++ `BView::DrawString()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ad3830d09ed0300a1144d47ac78cd2dde).
    fn draw_string_int32_escapement_delta(&self, string: &str, length: i32, delta: *mut c_void) {
        unsafe {
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            ffi::BView_DrawString2(self.as_ptr(), string, length, delta)
        }
    }
    // BLOCKED: fn DrawString3()
    /// Draw string locationCount times at the specified locations.
    ///
    /// See [C++ `BView::DrawString()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a5dacf5d193a0c492f296bc1e402e6370).
    fn draw_string_point<P: PointMethods>(
        &self,
        string: &str,
        locations: Option<&P>,
        location_count: i32,
    ) {
        unsafe {
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            let locations = match locations {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_DrawString4(self.as_ptr(), string, locations, location_count)
        }
    }
    /// Draw string up to length characters locationCount times at the specified locations.
    ///
    /// See [C++ `BView::DrawString()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a135868eb5cd1b35ee1c2d03517b9a33f).
    fn draw_string_int32_point<P: PointMethods>(
        &self,
        string: &str,
        length: i32,
        locations: Option<&P>,
        location_count: i32,
    ) {
        unsafe {
            let string = CString::from_vec_unchecked(string.into());
            let string = string.as_ptr();
            let locations = match locations {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_DrawString5(self.as_ptr(), string, length, locations, location_count)
        }
    }
    /// Sends a message to App Server to redraw the portion of the view specified by invalRect.
    ///
    /// See [C++ `BView::Invalidate()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a8c767d948dc9435c006cabdcab1cd08e).
    fn invalidate_rect<R: RectMethods>(&self, inval_rect: &R) {
        unsafe {
            let inval_rect = inval_rect.as_ptr();
            ffi::BView_Invalidate(self.as_ptr(), inval_rect)
        }
    }
    /// Sends a message to App Server to redraw the portion of the view specified by region.
    ///
    /// See [C++ `BView::Invalidate()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ac70b682819c0f9b008c428e02015b19b).
    fn invalidate_region(&self, inval_region: *const c_void) {
        unsafe { ffi::BView_Invalidate1(self.as_ptr(), inval_region) }
    }
    /// Sends a message to App Server to redraw the view.
    ///
    /// See [C++ `BView::Invalidate()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#adfced0dcb244e199f872fdbd48349a05).
    fn invalidate(&self) {
        unsafe { ffi::BView_Invalidate2(self.as_ptr()) }
    }
    /// Sends a message to App Server to redraw the entire view after a certain, minimum, delay. Repeated calls to this method may be merged, but the view is guaranteed to be redrawn after the delay given in the first call of this method.
    ///
    /// See [C++ `BView::DelayedInvalidate()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae2a9da1ca04489dd60ef8aefa86331ed).
    fn delayed_invalidate(&self, delay: bigtime_t) {
        unsafe { ffi::BView_DelayedInvalidate(self.as_ptr(), delay) }
    }
    /// Sends a message to App Server to redraw the portion of the view specified by invalRect after a certain, minimum, delay. Repeated calls to this method may be merged, but the invalidated rect is guaranteed to be redrawn after the minimum delay given by the first call of this method.
    ///
    /// See [C++ `BView::DelayedInvalidate()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a6389a46cf1842eb0d412b44958955e61).
    fn delayed_invalidate_rect<R: RectMethods>(&self, delay: bigtime_t, inval_rect: &R) {
        unsafe {
            let inval_rect = inval_rect.as_ptr();
            ffi::BView_DelayedInvalidate1(self.as_ptr(), delay, inval_rect)
        }
    }
    /// Unimplemented.
    ///
    /// See [C++ `BView::SetDiskMode()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a8ba0fe3a3dbeec1b7fd8c9655070ef64).
    fn set_disk_mode(&self, filename: *mut c_void, offset: c_long) {
        unsafe { ffi::BView_SetDiskMode(self.as_ptr(), filename, offset) }
    }
    /// Begins sending drawing instructions to picture.
    ///
    /// See [C++ `BView::BeginPicture()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a14155192773aad22ebe47a9975afac5f).
    fn begin_picture(&self, a_picture: *mut c_void) {
        unsafe { ffi::BView_BeginPicture(self.as_ptr(), a_picture) }
    }
    /// Appends drawing instructions to picture without clearing it first.
    ///
    /// See [C++ `BView::AppendToPicture()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a2d73d2f2b0b35390929cf54fbfa5ecff).
    fn append_to_picture(&self, a_picture: *mut c_void) {
        unsafe { ffi::BView_AppendToPicture(self.as_ptr(), a_picture) }
    }
    /// Ends the drawing instruction recording session and returns the BPicture object passed to BeginPicture() or AppendToPicture().
    ///
    /// See [C++ `BView::EndPicture()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae01506892a9b2d39f6df13726be9d0af).
    fn end_picture(&self) -> *mut c_void {
        unsafe { ffi::BView_EndPicture(self.as_ptr()) }
    }
    /// Draws the picture at the view's current pen position.
    ///
    /// See [C++ `BView::DrawPicture()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae4b252943befd9f0b62e30517b3ecec2).
    fn draw_picture_picture(&self, a_picture: *const c_void) {
        unsafe { ffi::BView_DrawPicture(self.as_ptr(), a_picture) }
    }
    /// Draws the picture at the location in the view specified by where.
    ///
    /// See [C++ `BView::DrawPicture()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a2fdf36421cccce9136cdd9966a971955).
    fn draw_picture_picture_point<P: PointMethods>(&self, a_picture: *const c_void, where_: &P) {
        unsafe {
            let where_ = where_.as_ptr();
            ffi::BView_DrawPicture1(self.as_ptr(), a_picture, where_)
        }
    }
    /// Draws the picture from the file specified by filename offset by offset bytes at the location in the view specified by where.
    ///
    /// See [C++ `BView::DrawPicture()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a9f5781a2b37113c55950c93b621984a7).
    fn draw_picture_str<P: PointMethods>(&self, filename: &str, offset: c_long, where_: &P) {
        unsafe {
            let filename = CString::from_vec_unchecked(filename.into());
            let filename = filename.as_ptr();
            let where_ = where_.as_ptr();
            ffi::BView_DrawPicture2(self.as_ptr(), filename, offset, where_)
        }
    }
    /// Draws the picture at the view's current pen position.
    ///
    /// See [C++ `BView::DrawPictureAsync()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a65498927c21d6681a0e77c4e3d29efba).
    fn draw_picture_async_picture(&self, a_picture: *const c_void) {
        unsafe { ffi::BView_DrawPictureAsync(self.as_ptr(), a_picture) }
    }
    /// Draws the picture at the location in the view specified by where.
    ///
    /// See [C++ `BView::DrawPictureAsync()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a05881f8b07989e1e9db0bd41f8181341).
    fn draw_picture_async_picture_point<P: PointMethods>(
        &self,
        a_picture: *const c_void,
        where_: &P,
    ) {
        unsafe {
            let where_ = where_.as_ptr();
            ffi::BView_DrawPictureAsync1(self.as_ptr(), a_picture, where_)
        }
    }
    /// Draws the picture from the file specified by filename offset by offset bytes at the location in the view specified by where.
    ///
    /// See [C++ `BView::DrawPictureAsync()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a7f2fde4e7719bded5af33805b8242b06).
    fn draw_picture_async_str<P: PointMethods>(&self, filename: &str, offset: c_long, where_: &P) {
        unsafe {
            let filename = CString::from_vec_unchecked(filename.into());
            let filename = filename.as_ptr();
            let where_ = where_.as_ptr();
            ffi::BView_DrawPictureAsync2(self.as_ptr(), filename, offset, where_)
        }
    }
    /// Begins a drawing layer.
    ///
    /// See [C++ `BView::BeginLayer()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#abe65757b7c83c8db390bf0c87457a343).
    fn begin_layer(&self, opacity: u8) {
        unsafe { ffi::BView_BeginLayer(self.as_ptr(), opacity) }
    }
    /// Finish a layer and blend it in with the view.
    ///
    /// See [C++ `BView::EndLayer()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a9f54d4478a12823f116168a45af30898).
    fn end_layer(&self) {
        unsafe { ffi::BView_EndLayer(self.as_ptr()) }
    }
    /// Moves the view deltaX pixels horizontally and deltaY pixels vertically in the parent view's coordinate system.
    ///
    /// See [C++ `BView::MoveBy()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a6e122baca4494f6ce739a507054fa81f).
    fn move_by(&self, dh: c_float, dv: c_float) {
        unsafe { ffi::BView_MoveBy(self.as_ptr(), dh, dv) }
    }
    /// Move the view to the location specified by where in the parent view's coordinate system.
    ///
    /// See [C++ `BView::MoveTo()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#acce9c93118a3db24078f16bdabc6fbec).
    fn move_to_point<P: PointMethods>(&self, where_: &P) {
        unsafe {
            let where_ = where_.as_ptr();
            ffi::BView_MoveTo(self.as_ptr(), where_)
        }
    }
    /// Move the view to the coordinates specified by x in the horizontal dimension and y in the vertical dimension in the parent view's coordinate system.
    ///
    /// See [C++ `BView::MoveTo()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a1c9b2df47ba1b9f3a1ce3286995fb8bd).
    fn move_to_float(&self, x: c_float, y: c_float) {
        unsafe { ffi::BView_MoveTo1(self.as_ptr(), x, y) }
    }
    /// Resize the view by deltaWidth horizontally and deltaHeight vertically without moving the top left corner of the view.
    ///
    /// See [C++ `BView::ResizeBy()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ace23acc948bbc9b9272da8dc7f01d3ce).
    fn resize_by(&self, dh: c_float, dv: c_float) {
        unsafe { ffi::BView_ResizeBy(self.as_ptr(), dh, dv) }
    }
    /// Resize the view to the specified width and height.
    ///
    /// See [C++ `BView::ResizeTo()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a742bccd6034ef9ade65b0e596fba55f2).
    fn resize_to_float(&self, width: c_float, height: c_float) {
        unsafe { ffi::BView_ResizeTo(self.as_ptr(), width, height) }
    }
    /// Resize the view to the dimension specified by size.
    ///
    /// See [C++ `BView::ResizeTo()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a6134097569f75679616257a361086d7a).
    fn resize_to_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::BView_ResizeTo1(self.as_ptr(), size)
        }
    }
    /// Return the minimum size of the view.
    ///
    /// See [C++ `BView::MinSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#acfb0cffde35b853dab6bc5d1bb03265f).
    fn min_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::BView_MinSize(self.as_ptr())) }
    }
    /// Return the maximum size of the view.
    ///
    /// See [C++ `BView::MaxSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ad1f1843a380e850948cf7e4ed2d5f1ba).
    fn max_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::BView_MaxSize(self.as_ptr())) }
    }
    /// Return the preferred size of the view.
    ///
    /// See [C++ `BView::PreferredSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a7f51d63e71b60a2721d04436ffbcfe36).
    fn preferred_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::BView_PreferredSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn LayoutAlignment()
    /// Set this view's min size, to be used by MinSize().
    ///
    /// See [C++ `BView::SetExplicitMinSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#abfcb5c761f9b06da3b4a6788aad5d0ee).
    fn set_explicit_min_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::BView_SetExplicitMinSize(self.as_ptr(), size)
        }
    }
    /// Set this view's max size, to be used by MaxSize().
    ///
    /// See [C++ `BView::SetExplicitMaxSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#acd759dd7505050f9cbd8213c71991b8b).
    fn set_explicit_max_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::BView_SetExplicitMaxSize(self.as_ptr(), size)
        }
    }
    /// Set this view's preferred size, to be used by PreferredSize().
    ///
    /// See [C++ `BView::SetExplicitPreferredSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a98b7d450210099ed59b7cd34f2575e20).
    fn set_explicit_preferred_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::BView_SetExplicitPreferredSize(self.as_ptr(), size)
        }
    }
    /// Set this view's size.
    ///
    /// See [C++ `BView::SetExplicitSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aeef8a27a5577db87251412ce44421494).
    fn set_explicit_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::BView_SetExplicitSize(self.as_ptr(), size)
        }
    }
    // NOT_SUPPORTED: fn SetExplicitAlignment()
    /// Returns the explicit minimum size.
    ///
    /// See [C++ `BView::ExplicitMinSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a1b24db754473008766cdd0fd9c4ee3b7).
    fn explicit_min_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::BView_ExplicitMinSize(self.as_ptr())) }
    }
    /// Returns the explicit maximum size.
    ///
    /// See [C++ `BView::ExplicitMaxSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a4e63cc8ca1f604ccb20f5891adade70f).
    fn explicit_max_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::BView_ExplicitMaxSize(self.as_ptr())) }
    }
    /// Returns the explicit preferred size.
    ///
    /// See [C++ `BView::ExplicitPreferredSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a218cd0d238104a6ff31c1d72ae2d9e6e).
    fn explicit_preferred_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::BView_ExplicitPreferredSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn ExplicitAlignment()
    /// Returns whether the layout of the view can calculate a height for a given width.
    ///
    /// See [C++ `BView::HasHeightForWidth()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#abd411461b7e23ad17e7294f8f5e1cbeb).
    fn has_height_for_width(&self) -> bool {
        unsafe { ffi::BView_HasHeightForWidth(self.as_ptr()) }
    }
    /// Returns the min, max and preferred height for a given width.
    ///
    /// See [C++ `BView::GetHeightForWidth()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ad34bee968b1ec98a0bf89343340b6b07).
    fn get_height_for_width(
        &self,
        width: c_float,
        min: *mut c_void,
        max: *mut c_void,
        preferred: *mut c_void,
    ) {
        unsafe { ffi::BView_GetHeightForWidth(self.as_ptr(), width, min, max, preferred) }
    }
    /// Invalidate layout.
    ///
    /// See [C++ `BView::InvalidateLayout()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a4a711316094e43076a3f5478d937accf).
    fn invalidate_layout(&self, descendants: bool) {
        unsafe { ffi::BView_InvalidateLayout(self.as_ptr(), descendants) }
    }
    /// Sets the layout of the view.
    ///
    /// See [C++ `BView::SetLayout()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#acac1ebbba8c76e543e1b06e84dffab02).
    fn set_layout(&self, layout: *mut c_void) {
        unsafe { ffi::BView_SetLayout(self.as_ptr(), layout) }
    }
    /// Get the layout of the view.
    ///
    /// See [C++ `BView::GetLayout()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aec2975b51966c397aa52b9e3eebd2c85).
    fn get_layout(&self) -> *mut c_void {
        unsafe { ffi::BView_GetLayout(self.as_ptr()) }
    }
    /// Enable layout invalidation.
    ///
    /// See [C++ `BView::EnableLayoutInvalidation()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ad43cc5c22f83d95632d3d81eff73f425).
    fn enable_layout_invalidation(&self) {
        unsafe { ffi::BView_EnableLayoutInvalidation(self.as_ptr()) }
    }
    /// Disable layout invalidation.
    ///
    /// See [C++ `BView::DisableLayoutInvalidation()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aa165cbaac7d6915feaae6048c4378c85).
    fn disable_layout_invalidation(&self) {
        unsafe { ffi::BView_DisableLayoutInvalidation(self.as_ptr()) }
    }
    /// Returns whether or not layout invalidation is disabled.
    ///
    /// See [C++ `BView::IsLayoutInvalidationDisabled()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a4e86b719129a12afbda7c10371eeca47).
    fn is_layout_invalidation_disabled(&self) -> bool {
        unsafe { ffi::BView_IsLayoutInvalidationDisabled(self.as_ptr()) }
    }
    /// Returns whether or not the layout is valid.
    ///
    /// See [C++ `BView::IsLayoutValid()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a00a5d5d6edee7d488d77df03945b3e94).
    fn is_layout_valid(&self) -> bool {
        unsafe { ffi::BView_IsLayoutValid(self.as_ptr()) }
    }
    /// Service call for BView derived classes re-enabling InvalidateLayout() notifications.
    ///
    /// See [C++ `BView::ResetLayoutInvalidation()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a127933a3820f0bfe0497b5e7ddfa191f).
    fn reset_layout_invalidation(&self) {
        unsafe { ffi::BView_ResetLayoutInvalidation(self.as_ptr()) }
    }
    /// Returns the BLayoutContext for this View.
    ///
    /// See [C++ `BView::LayoutContext()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#ae066513c0a788a9544744d409f28cada).
    fn layout_context(&self) -> *mut c_void {
        unsafe { ffi::BView_LayoutContext(self.as_ptr()) }
    }
    /// Layout the view.
    ///
    /// See [C++ `BView::Layout()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a854b6ca54eeca610db9eab8b58d68a8b).
    fn layout(&self, force: bool) {
        unsafe { ffi::BView_Layout(self.as_ptr(), force) }
    }
    /// Relayout the view.
    ///
    /// See [C++ `BView::Relayout()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a2cf4e698a4322ecfb946e440e0f7ac62).
    fn relayout(&self) {
        unsafe { ffi::BView_Relayout(self.as_ptr()) }
    }
    /// Set the tool tip of the view to text.
    ///
    /// See [C++ `BView::SetToolTip()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a75fb825a3ef69c1c6fbf3535ee2260ab).
    fn set_tool_tip_str(&self, text: &str) {
        unsafe {
            let text = CString::from_vec_unchecked(text.into());
            let text = text.as_ptr();
            ffi::BView_SetToolTip(self.as_ptr(), text)
        }
    }
    /// Set the tool tip of the view to the tip object.
    ///
    /// See [C++ `BView::SetToolTip()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#aa377bad64048a2d3cd1c2b272ae3079b).
    fn set_tool_tip_tooltip(&self, tip: *mut c_void) {
        unsafe { ffi::BView_SetToolTip1(self.as_ptr(), tip) }
    }
    /// Return the tool tip set to the view or NULL if not set.
    ///
    /// See [C++ `BView::ToolTip()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a127ceec2101b12c0fbe1e0e66a7b9dc1).
    fn tool_tip(&self) -> *mut c_void {
        unsafe { ffi::BView_ToolTip(self.as_ptr()) }
    }
    /// Show the tool tip at the current mouse position.
    ///
    /// See [C++ `BView::ShowToolTip()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a61d28f55d6fdc55b59c38c44b9788205).
    fn show_tool_tip(&self, tip: *mut c_void) {
        unsafe { ffi::BView_ShowToolTip(self.as_ptr(), tip) }
    }
    /// Hide the view's tool tip.
    ///
    /// See [C++ `BView::HideToolTip()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#af75bbc1131ad6a90e299c6cbcf623fa7).
    fn hide_tool_tip(&self) {
        unsafe { ffi::BView_HideToolTip(self.as_ptr()) }
    }
    // DTOR: fn ~BView()
    /// Returns the view's frame rectangle in the view's coordinate system.
    ///
    /// See [C++ `BView::Bounds()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#acbb8ea442346e91428689163db9d633e).
    fn bounds(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::BView_Bounds(self.as_ptr())) }
    }
    /// Flushes the attached window's connection to App Server.
    ///
    /// See [C++ `BView::Flush()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a15465930f84d2d01f7818bd79bc91c3e).
    fn flush(&self) {
        unsafe { ffi::BView_Flush(self.as_ptr()) }
    }
    /// Returns the view's frame rectangle in the parent's coordinate system.
    ///
    /// See [C++ `BView::Frame()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a487b04a3aaffdd1a1afb730dca36a7e2).
    fn frame(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::BView_Frame(self.as_ptr())) }
    }
    /// Fill out the preferred width and height of the view into the _width and _height parameters.
    ///
    /// See [C++ `BView::GetPreferredSize()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a089c3ff794c454388a9388c7b6568478).
    fn get_preferred_size(&self, _width: *mut c_void, _height: *mut c_void) {
        unsafe { ffi::BView_GetPreferredSize(self.as_ptr(), _width, _height) }
    }
    /// Hides the view without removing it from the view hierarchy.
    ///
    /// See [C++ `BView::Hide()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a7688dc04178a2955950154ee3670d029).
    fn hide(&self) {
        unsafe { ffi::BView_Hide(self.as_ptr()) }
    }
    /// Returns whether or not the view is the window's current focus view.
    ///
    /// See [C++ `BView::IsFocus()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a165719de682845f80735042a499692ff).
    fn is_focus(&self) -> bool {
        unsafe { ffi::BView_IsFocus(self.as_ptr()) }
    }
    /// Returns whether or not the view is hidden.
    ///
    /// See [C++ `BView::IsHidden()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a33a65050c2315b19b4c5af16c48e3cdb).
    fn is_hidden(&self) -> bool {
        unsafe { ffi::BView_IsHidden(self.as_ptr()) }
    }
    /// Returns whether or not the view is hidden from the perspective of lookingFrom.
    ///
    /// See [C++ `BView::IsHidden()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a95cfa8ee585230ea8fb224e88eddc503).
    fn is_hidden_view<V: ViewMethods>(&self, looking_from: Option<&V>) -> bool {
        unsafe {
            let looking_from = match looking_from {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BView_IsHidden1(self.as_ptr(), looking_from)
        }
    }
    /// Returns whether or not the view is drawing to a printer.
    ///
    /// See [C++ `BView::IsPrinting()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a8e3f1722e40fdf01c5131e49199ec3ff).
    fn is_printing(&self) -> bool {
        unsafe { ffi::BView_IsPrinting(self.as_ptr()) }
    }
    /// Returns the left top corner point.
    ///
    /// See [C++ `BView::LeftTop()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a5d252dfe28eee049be2192d1a01ed6e3).
    fn left_top(&self) -> Point {
        unsafe { Point::from_ptr(ffi::BView_LeftTop(self.as_ptr())) }
    }
    /// Resizes the view to its preferred size keeping the position of the left top corner constant.
    ///
    /// See [C++ `BView::ResizeToPreferred()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#abe8c63c85555e800e063388e7cf3e1a7).
    fn resize_to_preferred(&self) {
        unsafe { ffi::BView_ResizeToPreferred(self.as_ptr()) }
    }
    /// Returns the resizing mode flags mask set in the constructor or by SetResizingMode().
    ///
    /// See [C++ `BView::ResizingMode()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a1e93df523fc9edc807274ac6cfd2dc8d).
    fn resizing_mode(&self) -> u32 {
        unsafe { ffi::BView_ResizingMode(self.as_ptr()) }
    }
    /// Sets the resizing mode of the view according to the mode mask.
    ///
    /// See [C++ `BView::SetResizingMode()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a38d29ea581880c3978d7e50dcc851696).
    fn set_resizing_mode(&self, mode: u32) {
        unsafe { ffi::BView_SetResizingMode(self.as_ptr(), mode) }
    }
    /// Assigns cursor to the view.
    ///
    /// See [C++ `BView::SetViewCursor()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a39b4878773b4436e3c76aee51c4fbc09).
    fn set_view_cursor(&self, cursor: *const c_void, sync: bool) {
        unsafe { ffi::BView_SetViewCursor(self.as_ptr(), cursor, sync) }
    }
    /// Shows the view making it visible.
    ///
    /// See [C++ `BView::Show()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a06e0584057d6c7a7c7e048b6272b7016).
    fn show(&self) {
        unsafe { ffi::BView_Show(self.as_ptr()) }
    }
    /// Synchronizes the attached window's connection to App Server.
    ///
    /// See [C++ `BView::Sync()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a27e217abb3fefa45b6e4ceb7a54cb362).
    fn sync(&self) {
        unsafe { ffi::BView_Sync(self.as_ptr()) }
    }
    /// Returns the window the view is attached to.
    ///
    /// See [C++ `BView::Window()`'s documentation](https://www.haiku-os.org/docs/api/classBView.html#a32733d357b76bc3504cae326d174f4a2).
    fn window(&self) -> Option<WindowFromCpp<true>> {
        unsafe { Window::option_from(ffi::BView_Window(self.as_ptr())) }
    }
}

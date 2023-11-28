use super::*;


// BWindow
    /// This trait represents [C++ `BWindow` class](https://www.haiku-os.org/docs/api/classBWindow.html)'s methods and inheritance.
    ///
    /// See [`WindowFromCpp`] documentation for the class usage.
pub trait WindowMethods: LooperMethods {
    /// Creates a keyboard shortcut that sends a message to the window.
    ///
    /// See [C++ `BWindow::AddShortcut()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a5b05894e227eb22cba63ddaff289a95b).
    fn add_shortcut<M: MessageMethods>(&self, key: u32, modifiers: u32, message: Option<&M>) {
        unsafe {
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BWindow_AddShortcut(self.as_ptr(), key, modifiers, message)
        }
    }
    /// Creates a keyboard shortcut that sends a message to the specified target.
    ///
    /// See [C++ `BWindow::AddShortcut()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a169c891ff22f6a76f10c15bd16cde3c5).
    fn add_shortcut_handler<M: MessageMethods, H: HandlerMethods>(&self, key: u32, modifiers: u32, message: Option<&M>, target: Option<&H>) {
        unsafe {
            let message = match message {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let target = match target {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BWindow_AddShortcut1(self.as_ptr(), key, modifiers, message, target)
        }
    }
    /// Returns whether or not the specified shortcut is set on the window.
    ///
    /// See [C++ `BWindow::HasShortcut()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a08765e38e5cd5f37e19981d217b985d3).
    fn has_shortcut(&self, key: u32, modifiers: u32) -> bool {
        unsafe { ffi::BWindow_HasShortcut(self.as_ptr(), key, modifiers) }
    }
    /// Removes the specified shortcut from the window.
    ///
    /// See [C++ `BWindow::RemoveShortcut()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a81d5e6ed3a8a7b7f5013055bf69593ba).
    fn remove_shortcut(&self, key: u32, modifiers: u32) {
        unsafe { ffi::BWindow_RemoveShortcut(self.as_ptr(), key, modifiers) }
    }
    // DTOR: fn ~BWindow()
    /// Activates or deactivates the window based on active.
    ///
    /// See [C++ `BWindow::Activate()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a24ef781656049451c5c6ed06eeb98c06).
    fn activate(&self, active: bool) {
        unsafe { ffi::BWindow_Activate(self.as_ptr(), active) }
    }
    /// Add the child layout item to the view hierarchy.
    ///
    /// See [C++ `BWindow::AddChild()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a798386ef9d12eb4f0dafff375e3b85d6).
    fn add_child_layoutitem(&self, child: *mut c_void) {
        unsafe { ffi::BWindow_AddChild(self.as_ptr(), child) }
    }
    /// Adds child to the view hierarchy immediately before before.
    ///
    /// See [C++ `BWindow::AddChild()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ac3b09ede3b0256df9cbcc7b2a3d6eda4).
    fn add_child_view(&self, child: *mut c_void, before: *mut c_void) {
        unsafe { ffi::BWindow_AddChild1(self.as_ptr(), child, before) }
    }
    /// Adds window to be in the subset of the BWindow.
    ///
    /// See [C++ `BWindow::AddToSubset()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ac2abecfda66af23ee5e944530ce69c97).
    fn add_to_subset<W: WindowMethods>(&self, window: Option<&W>) -> status_t {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BWindow_AddToSubset(self.as_ptr(), window)
        }
    }
    /// Stall updates to App Server allowing you to batch drawing commands to limit flickering.
    ///
    /// See [C++ `BWindow::BeginViewTransaction()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a160e5f8757a65eaf3fec463fd6f2ba33).
    fn begin_view_transaction(&self) {
        unsafe { ffi::BWindow_BeginViewTransaction(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Bounds()
    /// Center the window in rect.
    ///
    /// See [C++ `BWindow::CenterIn()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a4ea700aa901a1ac76b23605d82c79de8).
    fn center_in(&self, rect: *const c_void) {
        unsafe { ffi::BWindow_CenterIn(self.as_ptr(), rect) }
    }
    /// Centers the window on the screen the window is currently on.
    ///
    /// See [C++ `BWindow::CenterOnScreen()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a1fbaa783dffbd7bb5f0b135cde5e0237).
    fn center_on_screen(&self) {
        unsafe { ffi::BWindow_CenterOnScreen(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn CenterOnScreen1()
    /// Returns a pointer to the child view found at index.
    ///
    /// See [C++ `BWindow::ChildAt()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#aeaeec9bcb12b7b03132e61dba501b19b).
    fn child_at(&self, index: i32) -> *mut c_void {
        unsafe { ffi::BWindow_ChildAt(self.as_ptr(), index) }
    }
    /// Deprecated alias for BWindow::Quit().
    ///
    /// See [C++ `BWindow::Close()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a3be508837c0b14b176b916e45c9c1f31).
    fn close(&self) {
        unsafe { ffi::BWindow_Close(self.as_ptr()) }
    }
    /// Convert point from the screen's coordinate system to the window's coordinate system in place.
    ///
    /// See [C++ `BWindow::ConvertFromScreen()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#aaa76e90a0578a4081eb4765467d594ea).
    fn convert_from_screen_point(&self, point: *mut c_void) {
        unsafe { ffi::BWindow_ConvertFromScreen(self.as_ptr(), point) }
    }
    // NOT_SUPPORTED: fn ConvertFromScreen1()
    /// Convert rect from the screen's coordinate system to the window's coordinate system in place.
    ///
    /// See [C++ `BWindow::ConvertFromScreen()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a7b3b4a579b4e14cd6acf1a95aee75687).
    fn convert_from_screen_rect(&self, rect: *mut c_void) {
        unsafe { ffi::BWindow_ConvertFromScreen2(self.as_ptr(), rect) }
    }
    // NOT_SUPPORTED: fn ConvertFromScreen3()
    /// Convert point to the screen's coordinate system in place.
    ///
    /// See [C++ `BWindow::ConvertToScreen()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a218bb4c66ca3f58fbc203a36996fd119).
    fn convert_to_screen_point(&self, point: *mut c_void) {
        unsafe { ffi::BWindow_ConvertToScreen(self.as_ptr(), point) }
    }
    // NOT_SUPPORTED: fn ConvertToScreen1()
    /// Convert rect to the screen's coordinate system in place.
    ///
    /// See [C++ `BWindow::ConvertToScreen()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a83c3d8f1fb020740f7b4c3658e5222b5).
    fn convert_to_screen_rect(&self, rect: *mut c_void) {
        unsafe { ffi::BWindow_ConvertToScreen2(self.as_ptr(), rect) }
    }
    // NOT_SUPPORTED: fn ConvertToScreen3()
    /// Returns the number of child views that the window has.
    ///
    /// See [C++ `BWindow::CountChildren()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a742200112899822a889c2e69bf86db5d).
    fn count_children(&self) -> i32 {
        unsafe { ffi::BWindow_CountChildren(self.as_ptr()) }
    }
    /// Returns a pointer to the current focus view of the window.
    ///
    /// See [C++ `BWindow::CurrentFocus()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a31d98b696b4e2a98ae83c841e342fe1b).
    fn current_focus(&self) -> *mut c_void {
        unsafe { ffi::BWindow_CurrentFocus(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn DecoratorFrame()
    /// Returns a pointer to the default button set on the window.
    ///
    /// See [C++ `BWindow::DefaultButton()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a6995f1dcc605c006e4118cc4b687e766).
    fn default_button(&self) -> *mut c_void {
        unsafe { ffi::BWindow_DefaultButton(self.as_ptr()) }
    }
    /// Suppresses drawing within the window.
    ///
    /// See [C++ `BWindow::DisableUpdates()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a83111af72c672b2b7029e712e9b702f3).
    fn disable_updates(&self) {
        unsafe { ffi::BWindow_DisableUpdates(self.as_ptr()) }
    }
    /// Re-enable drawing within the window.
    ///
    /// See [C++ `BWindow::EnableUpdates()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ab71c5c4ddd8f1cc677115157ec1c0c96).
    fn enable_updates(&self) {
        unsafe { ffi::BWindow_EnableUpdates(self.as_ptr()) }
    }
    /// Ends a view transaction allowing update to go to App Server again.
    ///
    /// See [C++ `BWindow::EndViewTransaction()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a54d387b099fb88e5aa565bdc40f338eb).
    fn end_view_transaction(&self) {
        unsafe { ffi::BWindow_EndViewTransaction(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Feel()
    // NOT_SUPPORTED: fn FindView()
    /// Returns the attached view with the specified viewName.
    ///
    /// See [C++ `BWindow::FindView()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ac295d2dff72e4c5254cf597d4b31f9c6).
    fn find_view(&self, view_name: &str) -> *mut c_void {
        unsafe {
            let view_name = CString::from_vec_unchecked(view_name.into());
            let view_name = view_name.as_ptr();
            ffi::BWindow_FindView1(self.as_ptr(), view_name)
        }
    }
    /// Returns the current window flags.
    ///
    /// See [C++ `BWindow::Flags()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ab99cdac9e2a0270c8afc07cfae7a02a8).
    fn flags(&self) -> u32 {
        unsafe { ffi::BWindow_Flags(self.as_ptr()) }
    }
    /// Flushes the window's connection to App Server causing any pending messages to be processed then returns immediately.
    ///
    /// See [C++ `BWindow::Flush()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a2e74fd82a8095399d73a28920cc7c9b9).
    fn flush(&self) {
        unsafe { ffi::BWindow_Flush(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Frame()
    // NOT_SUPPORTED: fn FrameMoved()
    /// Hook method that gets called when the window is resized.
    ///
    /// See [C++ `BWindow::FrameResized()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a280fab2b2900abc61dd4bcb7a2a4793f).
    fn frame_resized(&self, new_width: c_float, new_height: c_float) {
        unsafe { ffi::BWindow_FrameResized(self.as_ptr(), new_width, new_height) }
    }
    /// Fill out the window's decorator settings into settings.
    ///
    /// See [C++ `BWindow::GetDecoratorSettings()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a58fba1b0ad6c2035cf1dac6304d21912).
    fn get_decorator_settings<M: MessageMethods>(&self, settings: Option<&M>) -> status_t {
        unsafe {
            let settings = match settings {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BWindow_GetDecoratorSettings(self.as_ptr(), settings)
        }
    }
    /// Get the layout of the window.
    ///
    /// See [C++ `BWindow::GetLayout()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#adb07dc64e991f7a42dfad000b2a1b0c7).
    fn get_layout(&self) -> *mut c_void {
        unsafe { ffi::BWindow_GetLayout(self.as_ptr()) }
    }
    /// Fills out the size limits set on the window.
    ///
    /// See [C++ `BWindow::GetSizeLimits()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#aebce716c628bac67367875d1129db8c5).
    fn get_size_limits(&self, min_width: *mut c_void, max_width: *mut c_void, min_height: *mut c_void, max_height: *mut c_void) {
        unsafe { ffi::BWindow_GetSizeLimits(self.as_ptr(), min_width, max_width, min_height, max_height) }
    }
    /// Fills out the pointers with the alignment of the content of the window on the screen.
    ///
    /// See [C++ `BWindow::GetWindowAlignment()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a62a354bee61a0b5cd5ac499b5f728b7f).
    fn get_window_alignment(&self, mode: *mut c_void, h: *mut c_void, h_offset: *mut c_void, width: *mut c_void, width_offset: *mut c_void, v: *mut c_void, v_offset: *mut c_void, height: *mut c_void, height_offset: *mut c_void) -> status_t {
        unsafe { ffi::BWindow_GetWindowAlignment(self.as_ptr(), mode, h, h_offset, width, width_offset, v, v_offset, height, height_offset) }
    }
    /// Removes the window from the screen, removes it from Deskbar's window list, and passes active status to another window.
    ///
    /// See [C++ `BWindow::Hide()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a53b85e4d43d34e5259b3b16ff6c52b32).
    fn hide(&self) {
        unsafe { ffi::BWindow_Hide(self.as_ptr()) }
    }
    /// Invalidate layout.
    ///
    /// See [C++ `BWindow::InvalidateLayout()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a6abf2efb0ab304b75d72df3b6a2b9df3).
    fn invalidate_layout(&self, descendants: bool) {
        unsafe { ffi::BWindow_InvalidateLayout(self.as_ptr(), descendants) }
    }
    /// Returns whether or not the window is currently in a view transaction.
    ///
    /// See [C++ `BWindow::InViewTransaction()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ae5d1ae4e9598ed736c557031b92753ce).
    fn in_view_transaction(&self) -> bool {
        unsafe { ffi::BWindow_InViewTransaction(self.as_ptr()) }
    }
    /// Returns whether or not the window is active.
    ///
    /// See [C++ `BWindow::IsActive()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#abbaa2f20d86a5c4d12591fefef0c49fa).
    fn is_active(&self) -> bool {
        unsafe { ffi::BWindow_IsActive(self.as_ptr()) }
    }
    /// Returns whether or not the window is floating.
    ///
    /// See [C++ `BWindow::IsFloating()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a54158d02e55dbeb212ccf9af5b5a2f23).
    fn is_floating(&self) -> bool {
        unsafe { ffi::BWindow_IsFloating(self.as_ptr()) }
    }
    /// Returns whether or not the window is the frontmost on screen.
    ///
    /// See [C++ `BWindow::IsFront()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a61ab745cf98f47cac71ed3b750d4d21f).
    fn is_front(&self) -> bool {
        unsafe { ffi::BWindow_IsFront(self.as_ptr()) }
    }
    /// Returns whether or not the window is hidden.
    ///
    /// See [C++ `BWindow::IsHidden()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ae92c2b3b1340af7740b7c4e3e14dcd30).
    fn is_hidden(&self) -> bool {
        unsafe { ffi::BWindow_IsHidden(self.as_ptr()) }
    }
    /// Returns whether or not the window is minimized.
    ///
    /// See [C++ `BWindow::IsMinimized()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a6c209b1d5ab1b82f43993074b1373678).
    fn is_minimized(&self) -> bool {
        unsafe { ffi::BWindow_IsMinimized(self.as_ptr()) }
    }
    /// Returns whether or not the window is modal.
    ///
    /// See [C++ `BWindow::IsModal()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#aa10b6592ab7e291ce7152a72605c68fa).
    fn is_modal(&self) -> bool {
        unsafe { ffi::BWindow_IsModal(self.as_ptr()) }
    }
    /// Tests if window is used for drawing into a BBitmap. This is mostly used by the Interface Kit itself.
    ///
    /// See [C++ `BWindow::IsOffscreenWindow()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a61d9b0d1e0de31231b835cc5293f8ed2).
    fn is_offscreen_window(&self) -> bool {
        unsafe { ffi::BWindow_IsOffscreenWindow(self.as_ptr()) }
    }
    /// Returns a pointer to the key menu bar set to the window.
    ///
    /// See [C++ `BWindow::KeyMenuBar()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a3f4cf14e9f6453e63b58acfd7098941c).
    fn key_menu_bar(&self) -> *mut c_void {
        unsafe { ffi::BWindow_KeyMenuBar(self.as_ptr()) }
    }
    /// Returns a pointer to the attached view that most recently received a B_MOUSE_MOVED message.
    ///
    /// See [C++ `BWindow::LastMouseMovedView()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a59ef137725976cea5008e5efd683357e).
    fn last_mouse_moved_view(&self) -> *mut c_void {
        unsafe { ffi::BWindow_LastMouseMovedView(self.as_ptr()) }
    }
    /// Update the size limits and do the layout of the topmost view attached to the window.
    ///
    /// See [C++ `BWindow::Layout()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ae3f008c018e413d75ddea1a60b0c9df7).
    fn layout(&self, force: bool) {
        unsafe { ffi::BWindow_Layout(self.as_ptr(), force) }
    }
    // NOT_SUPPORTED: fn Look()
    /// Hook method that gets called just before a menu owned by the window is shown.
    ///
    /// See [C++ `BWindow::MenusBeginning()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a981d5c2993f4514fbcd1bdaf297a2a2d).
    fn menus_beginning(&self) {
        unsafe { ffi::BWindow_MenusBeginning(self.as_ptr()) }
    }
    /// Hook method that gets called just before a menu owned by the window is hidden.
    ///
    /// See [C++ `BWindow::MenusEnded()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a7bb882de172223e68ee6249596b80564).
    fn menus_ended(&self) {
        unsafe { ffi::BWindow_MenusEnded(self.as_ptr()) }
    }
    /// Minimizes or un-minimizes the window based on minimize.
    ///
    /// See [C++ `BWindow::Minimize()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#aabe75f91b52b94de6989b4cf49bd89b5).
    fn minimize(&self, minimize: bool) {
        unsafe { ffi::BWindow_Minimize(self.as_ptr(), minimize) }
    }
    /// Move the window by dx pixels horizontally and dy pixels vertically.
    ///
    /// See [C++ `BWindow::MoveBy()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a146584fa5815bd413f71671f33a9de89).
    fn move_by(&self, dx: c_float, dy: c_float) {
        unsafe { ffi::BWindow_MoveBy(self.as_ptr(), dx, dy) }
    }
    /// Update window size and position to make it visible on screen.
    ///
    /// See [C++ `BWindow::MoveOnScreen()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ae426291c30652864d0a464518f9cda12).
    fn move_on_screen(&self, flags: u32) {
        unsafe { ffi::BWindow_MoveOnScreen(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn MoveTo()
    /// Move the window to the specified x and y coordinates.
    ///
    /// See [C++ `BWindow::MoveTo()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#af419313a1b9b73a2c1cdb2365ff16189).
    fn move_to(&self, x: c_float, y: c_float) {
        unsafe { ffi::BWindow_MoveTo1(self.as_ptr(), x, y) }
    }
    /// Returns whether or not any of the attached views need to be updated.
    ///
    /// See [C++ `BWindow::NeedsUpdate()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a9c03d9a6a454edec5eb38657e5c66490).
    fn needs_update(&self) -> bool {
        unsafe { ffi::BWindow_NeedsUpdate(self.as_ptr()) }
    }
    /// Returns the pulse rate of the window.
    ///
    /// See [C++ `BWindow::PulseRate()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a8ea37743fc8c09aab2a511a7c14fa9c4).
    fn pulse_rate(&self) -> bigtime_t {
        unsafe { ffi::BWindow_PulseRate(self.as_ptr()) }
    }
    /// Removes child from the view hierarchy.
    ///
    /// See [C++ `BWindow::RemoveChild()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ac271fe9be15e5d6e0a5d59b7b2ed3e8d).
    fn remove_child(&self, child: *mut c_void) -> bool {
        unsafe { ffi::BWindow_RemoveChild(self.as_ptr(), child) }
    }
    /// Remove window from the subset of the BWindow.
    ///
    /// See [C++ `BWindow::RemoveFromSubset()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#af9ef1e4afee9bfde7c7d9d8e2796c2de).
    fn remove_from_subset<W: WindowMethods>(&self, window: Option<&W>) -> status_t {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BWindow_RemoveFromSubset(self.as_ptr(), window)
        }
    }
    /// Resize the window by dx pixels horizontally and dy pixels vertically.
    ///
    /// See [C++ `BWindow::ResizeBy()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a6aba4ddafc654f362799b6a96e6c76cd).
    fn resize_by(&self, dx: c_float, dy: c_float) {
        unsafe { ffi::BWindow_ResizeBy(self.as_ptr(), dx, dy) }
    }
    /// Resize the window to the specified width and height.
    ///
    /// See [C++ `BWindow::ResizeTo()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a31e2ea325258646128cfe8618cb79edc).
    fn resize_to(&self, width: c_float, height: c_float) {
        unsafe { ffi::BWindow_ResizeTo(self.as_ptr(), width, height) }
    }
    /// Resize the window to the preferred size of the window's layout.
    ///
    /// See [C++ `BWindow::ResizeToPreferred()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a68db6173b92973252fc1876c5376bcd4).
    fn resize_to_preferred(&self) {
        unsafe { ffi::BWindow_ResizeToPreferred(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn ScreenChanged()
    /// Moves the BWindow object behind window.
    ///
    /// See [C++ `BWindow::SendBehind()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a88c3aa6ee3ea29d7868bafec749f891e).
    fn send_behind<W: WindowMethods>(&self, window: Option<&W>) -> status_t {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::BWindow_SendBehind(self.as_ptr(), window)
        }
    }
    /// Set the window decorator settings according to settings.
    ///
    /// See [C++ `BWindow::SetDecoratorSettings()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#aecfdd5136ed18c193357a8e3130ba578).
    fn set_decorator_settings<M: MessageMethods>(&self, settings: &M) -> status_t {
        unsafe {
            let settings = settings.as_ptr();
            ffi::BWindow_SetDecoratorSettings(self.as_ptr(), settings)
        }
    }
    /// Set the default button of the window to button.
    ///
    /// See [C++ `BWindow::SetDefaultButton()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ab6c969b403bd24dc8b1e9d846a4ae414).
    fn set_default_button(&self, button: *mut c_void) {
        unsafe { ffi::BWindow_SetDefaultButton(self.as_ptr(), button) }
    }
    // NOT_SUPPORTED: fn SetFeel()
    /// Changes the window flags set in the constructor to flags.
    ///
    /// See [C++ `BWindow::SetFlags()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a8f0caf4693a1952ac9bbe9a52bfdd778).
    fn set_flags(&self, flags: u32) -> status_t {
        unsafe { ffi::BWindow_SetFlags(self.as_ptr(), flags) }
    }
    /// Set the specified menu bar as the key menu bar for the window.
    ///
    /// See [C++ `BWindow::SetKeyMenuBar()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#aa1e9322c76ffdad1886253171f8a5972).
    fn set_key_menu_bar(&self, bar: *mut c_void) {
        unsafe { ffi::BWindow_SetKeyMenuBar(self.as_ptr(), bar) }
    }
    /// Sets the layout of the window.
    ///
    /// See [C++ `BWindow::SetLayout()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ad0e630f2691a0f7919b809b7ea265095).
    fn set_layout(&self, layout: *mut c_void) {
        unsafe { ffi::BWindow_SetLayout(self.as_ptr(), layout) }
    }
    // NOT_SUPPORTED: fn SetLook()
    /// Sets how often B_PULSE messages are posted to the window.
    ///
    /// See [C++ `BWindow::SetPulseRate()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ad69001922aad8daf7b65f82eb4b439f0).
    fn set_pulse_rate(&self, rate: bigtime_t) {
        unsafe { ffi::BWindow_SetPulseRate(self.as_ptr(), rate) }
    }
    /// Set size limits on the window.
    ///
    /// See [C++ `BWindow::SetSizeLimits()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a8668ecf18ad145391f66704c3339eb3d).
    fn set_size_limits(&self, min_width: c_float, max_width: c_float, min_height: c_float, max_height: c_float) {
        unsafe { ffi::BWindow_SetSizeLimits(self.as_ptr(), min_width, max_width, min_height, max_height) }
    }
    /// Sets the window title to title.
    ///
    /// See [C++ `BWindow::SetTitle()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a7cd4bf7dc0079e9a52b0ac1e7e2f897b).
    fn set_title(&self, title: &str) {
        unsafe {
            let title = CString::from_vec_unchecked(title.into());
            let title = title.as_ptr();
            ffi::BWindow_SetTitle(self.as_ptr(), title)
        }
    }
    // NOT_SUPPORTED: fn SetType()
    // NOT_SUPPORTED: fn SetWindowAlignment()
    /// Sets the set of workspaces where the window can be displayed.
    ///
    /// See [C++ `BWindow::SetWorkspaces()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a3549fdb63a64637fa8e2054e8c21272d).
    fn set_workspaces(&self, workspaces: u32) {
        unsafe { ffi::BWindow_SetWorkspaces(self.as_ptr(), workspaces) }
    }
    /// Sets the maximum size that the window will zoom to when Zoom() is called.
    ///
    /// See [C++ `BWindow::SetZoomLimits()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ad3c2e560e41b377456faf9bd5dabcdc0).
    fn set_zoom_limits(&self, max_width: c_float, max_height: c_float) {
        unsafe { ffi::BWindow_SetZoomLimits(self.as_ptr(), max_width, max_height) }
    }
    /// Shows the window on screen, places it frontmost on the screen, adds the window to Deskbar's window list, and makes it the active window.
    ///
    /// See [C++ `BWindow::Show()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#af7009117df50add72cf0e799ff7d0fa5).
    fn show(&self) {
        unsafe { ffi::BWindow_Show(self.as_ptr()) }
    }
    /// Returns the size of the window.
    ///
    /// See [C++ `BWindow::Size()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#afc61a0c6129f233e56fa373107bd8790).
    fn size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::BWindow_Size(self.as_ptr())) }
    }
    /// Synchronizes the attached window's connection to App Server causing any pending messages to be processed and then waits for the App Server to respond.
    ///
    /// See [C++ `BWindow::Sync()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a332bea9ef16fedc16134bf587b52fa09).
    fn sync(&self) {
        unsafe { ffi::BWindow_Sync(self.as_ptr()) }
    }
    /// Returns the window title as set by the constructor or SetTitle().
    ///
    /// See [C++ `BWindow::Title()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a73c7a05ef33d579de61b83b5daaf3c6b).
    fn title(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::BWindow_Title(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn Type()
    /// Invokes Draw() immediately on each child view that needs updating.
    ///
    /// See [C++ `BWindow::UpdateIfNeeded()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a49b0c49eeef751b17bdc08d2e4d58cb9).
    fn update_if_needed(&self) {
        unsafe { ffi::BWindow_UpdateIfNeeded(self.as_ptr()) }
    }
    /// Updates the window's size limits from the minimum and maximum sizes of its top view.
    ///
    /// See [C++ `BWindow::UpdateSizeLimits()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a99df6b9964c69b1c69c47fab6262abee).
    fn update_size_limits(&self) {
        unsafe { ffi::BWindow_UpdateSizeLimits(self.as_ptr()) }
    }
    /// Hook method that gets called when the window becomes activated or deactivated.
    ///
    /// See [C++ `BWindow::WindowActivated()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a0a17d973a17bb8d3aef09d45d58a9aca).
    fn window_activated(&self, focus: bool) {
        unsafe { ffi::BWindow_WindowActivated(self.as_ptr(), focus) }
    }
    /// Hook method that gets called when the active workspace changes.
    ///
    /// See [C++ `BWindow::WorkspaceActivated()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#ac3c293ef3d60df9b3125fb15d0c67cf7).
    fn workspace_activated(&self, workspace: i32, state: bool) {
        unsafe { ffi::BWindow_WorkspaceActivated(self.as_ptr(), workspace, state) }
    }
    /// Returns the set of workspaces where the window can be displayed.
    ///
    /// See [C++ `BWindow::Workspaces()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#aba3b83e88043338b66c2b2f501fbbbee).
    fn workspaces(&self) -> u32 {
        unsafe { ffi::BWindow_Workspaces(self.as_ptr()) }
    }
    /// Hook method that gets called whenever the workspaces the window is in changes.
    ///
    /// See [C++ `BWindow::WorkspacesChanged()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#af8dc496d4239a2c343a364a4ad9aabc4).
    fn workspaces_changed(&self, old_workspaces: u32, new_workspaces: u32) {
        unsafe { ffi::BWindow_WorkspacesChanged(self.as_ptr(), old_workspaces, new_workspaces) }
    }
    /// Resize the window to the minimum of the screen size, the maximum values set by SetSizeLimits(), and the maximum values set by SetZoomLimits().
    ///
    /// See [C++ `BWindow::Zoom()`'s documentation](https://www.haiku-os.org/docs/api/classBWindow.html#a722772f2902e82bf65010ac2c13570bc).
    fn zoom(&self) {
        unsafe { ffi::BWindow_Zoom(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Zoom1()
}

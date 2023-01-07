use super::*;

// wxSVGFileDC
wxwidgets! {
    /// A wxSVGFileDC is a device context onto which graphics and text can be drawn, and the output produced as a vector file, in SVG format.
    /// - [`SVGFileDC`] represents a C++ `wxSVGFileDC` class instance which your code has ownership, [`SVGFileDCInRust`]`<false>` represents one which don't own.
    /// - Use [`SVGFileDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSVGFileDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html) for more details.
    #[doc(alias = "wxSVGFileDC")]
    #[doc(alias = "SVGFileDC")]
    class SVGFileDC
        = SVGFileDCInRust<true>(wxSVGFileDC) impl
        SVGFileDCMethods,
        // DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> SVGFileDCInRust<OWNED> {
    /// Initializes a wxSVGFileDC with the given filename, width and height at dpi resolution, and an optional title.
    ///
    /// See [C++ `wxSVGFileDC::wxSVGFileDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html#ab7b8446a6dff6f1533343f16ca4dec9e).
    pub fn new(
        filename: &str,
        width: c_int,
        height: c_int,
        dpi: c_double,
        title: &str,
    ) -> SVGFileDCInRust<OWNED> {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            SVGFileDCInRust(ffi::wxSVGFileDC_new(filename, width, height, dpi, title))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SVGFileDCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SVGFileDCInRust<OWNED>> for DCInRust<OWNED> {
    fn from(o: SVGFileDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SVGFileDCInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SVGFileDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SVGFileDCInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSVGFileDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SVGFileDCInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> DCMethods for SVGFileDCInRust<OWNED> {
    /// Draws a rectangle the size of the SVG using the wxDC::SetBackground() brush.
    ///
    /// See [C++ `wxSVGFileDC::Clear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html#a88eb12ff20b15f2e7d91483394a2ed16).
    fn clear(&self) {
        unsafe { ffi::wxSVGFileDC_Clear(self.as_ptr()) }
    }
    /// Destroys the current clipping region so that none of the DC is clipped.
    ///
    /// See [C++ `wxSVGFileDC::DestroyClippingRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html#a83be1b0f0b66a4949268e34348af3ee8).
    fn destroy_clipping_region(&self) {
        unsafe { ffi::wxSVGFileDC_DestroyClippingRegion(self.as_ptr()) }
    }
    /// Function not implemented in this DC class.
    ///
    /// See [C++ `wxSVGFileDC::CrossHair()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html#ae9c7ad3de5259a461eb4fd7c56b58d90).
    fn cross_hair_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxSVGFileDC_CrossHair(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn FloodFill()
    ///
    /// See [C++ `wxSVGFileDC::GetPixel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html#a0304a4081244e8e9944a357c855a713b).
    fn get_pixel<C: ColourMethods>(&self, x: c_int, y: c_int, colour: Option<&C>) -> bool {
        unsafe {
            let colour = match colour {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSVGFileDC_GetPixel(self.as_ptr(), x, y, colour)
        }
    }
    ///
    /// See [C++ `wxSVGFileDC::SetPalette()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html#a56f9674ee5fff78f9f884586c7106bfc).
    fn set_palette<P: PaletteMethods>(&self, palette: &P) {
        unsafe {
            let palette = palette.as_ptr();
            ffi::wxSVGFileDC_SetPalette(self.as_ptr(), palette)
        }
    }
    ///
    /// See [C++ `wxSVGFileDC::GetDepth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html#adb3e731d36ffa571ec823d19ca639771).
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxSVGFileDC_GetDepth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetLogicalFunction()
    // NOT_SUPPORTED: fn GetLogicalFunction()
    ///
    /// See [C++ `wxSVGFileDC::StartDoc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html#afc23fca3c1919a917ba4fa4ea1a47bd6).
    fn start_doc(&self, message: &str) -> bool {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxSVGFileDC_StartDoc(self.as_ptr(), message)
        }
    }
    ///
    /// See [C++ `wxSVGFileDC::EndDoc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html#a70fee3fe2c116cd5698e89c63992dfe1).
    fn end_doc(&self) {
        unsafe { ffi::wxSVGFileDC_EndDoc(self.as_ptr()) }
    }
    ///
    /// See [C++ `wxSVGFileDC::StartPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html#a02ccc9acf97f0aff154e2a1dbd6cf0f6).
    fn start_page(&self) {
        unsafe { ffi::wxSVGFileDC_StartPage(self.as_ptr()) }
    }
    ///
    /// See [C++ `wxSVGFileDC::EndPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html#aabbe60d5f7816974da2ab92195de961a).
    fn end_page(&self) {
        unsafe { ffi::wxSVGFileDC_EndPage(self.as_ptr()) }
    }
}

// wxSashEvent
wxwidgets! {
    /// A sash event is sent when the sash of a wxSashWindow has been dragged by the user.
    /// - [`SashEvent`] represents a C++ `wxSashEvent` class instance which your code has ownership, [`SashEventInRust`]`<false>` represents one which don't own.
    /// - Use [`SashEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSashEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_sash_event.html) for more details.
    #[doc(alias = "wxSashEvent")]
    #[doc(alias = "SashEvent")]
    class SashEvent
        = SashEventInRust<true>(wxSashEvent) impl
        SashEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SashEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxSashEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SashEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SashEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: SashEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: SashEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SashEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SashEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSashEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SashEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSashLayoutWindow
wxwidgets! {
    /// wxSashLayoutWindow responds to OnCalculateLayout events generated by wxLayoutAlgorithm.
    /// - [`SashLayoutWindow`] represents a C++ `wxSashLayoutWindow` class instance which your code has ownership, [`SashLayoutWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`SashLayoutWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSashLayoutWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_sash_layout_window.html) for more details.
    #[doc(alias = "wxSashLayoutWindow")]
    #[doc(alias = "SashLayoutWindow")]
    class SashLayoutWindow
        = SashLayoutWindowInRust<true>(wxSashLayoutWindow) impl
        SashLayoutWindowMethods,
        SashWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SashLayoutWindowInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxSashLayoutWindow::wxSashLayoutWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sash_layout_window.html#a842e094abe06cbd25f645c32d24b5a3e).
    pub fn new_2step() -> SashLayoutWindowInRust<OWNED> {
        unsafe { SashLayoutWindowInRust(ffi::wxSashLayoutWindow_new()) }
    }
    /// Constructs a sash layout window, which can be a child of a frame, dialog or any other non-control window.
    ///
    /// See [C++ `wxSashLayoutWindow::wxSashLayoutWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sash_layout_window.html#a1c8bff48c9191b36d0fc197eb9a1ca46).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SashLayoutWindowInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SashLayoutWindowInRust(ffi::wxSashLayoutWindow_new1(
                parent, id, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SashLayoutWindowInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SashLayoutWindowInRust<OWNED>> for SashWindowInRust<OWNED> {
    fn from(o: SashLayoutWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashLayoutWindowInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: SashLayoutWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashLayoutWindowInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: SashLayoutWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashLayoutWindowInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SashLayoutWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SashLayoutWindowInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSashLayoutWindow_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for SashLayoutWindowInRust<OWNED> {
    /// Initializes a sash layout window, which can be a child of a frame, dialog or any other non-control window.
    ///
    /// See [C++ `wxSashLayoutWindow::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sash_layout_window.html#a01ab3e617deb3a4ca348b2bfcd0ab26e).
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSashLayoutWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxSashWindow
wxwidgets! {
    /// wxSashWindow allows any of its edges to have a sash which can be dragged to resize the window.
    /// - [`SashWindow`] represents a C++ `wxSashWindow` class instance which your code has ownership, [`SashWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`SashWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSashWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_sash_window.html) for more details.
    #[doc(alias = "wxSashWindow")]
    #[doc(alias = "SashWindow")]
    class SashWindow
        = SashWindowInRust<true>(wxSashWindow) impl
        SashWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SashWindowInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxSashWindow::wxSashWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sash_window.html#a49c7b9a829ab48237bbe479e5011f297).
    pub fn new_2step() -> SashWindowInRust<OWNED> {
        unsafe { SashWindowInRust(ffi::wxSashWindow_new()) }
    }
    /// Constructs a sash window, which can be a child of a frame, dialog or any other non-control window.
    ///
    /// See [C++ `wxSashWindow::wxSashWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sash_window.html#aed974ee33685e7a209f061e39cf13451).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SashWindowInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SashWindowInRust(ffi::wxSashWindow_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SashWindowInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SashWindowInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: SashWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashWindowInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: SashWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashWindowInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SashWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SashWindowInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSashWindow_CLASSINFO()) }
    }
}

// wxScreenDC
wxwidgets! {
    /// A wxScreenDC can be used to paint on the screen.
    /// - [`ScreenDC`] represents a C++ `wxScreenDC` class instance which your code has ownership, [`ScreenDCInRust`]`<false>` represents one which don't own.
    /// - Use [`ScreenDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxScreenDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_screen_d_c.html) for more details.
    #[doc(alias = "wxScreenDC")]
    #[doc(alias = "ScreenDC")]
    class ScreenDC
        = ScreenDCInRust<true>(wxScreenDC) impl
        ScreenDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScreenDCInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxScreenDC::wxScreenDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_screen_d_c.html#a05147c9296ea7012f345f0803f52c020).
    pub fn new() -> ScreenDCInRust<OWNED> {
        unsafe { ScreenDCInRust(ffi::wxScreenDC_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ScreenDCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ScreenDCInRust<OWNED>> for DCInRust<OWNED> {
    fn from(o: ScreenDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScreenDCInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ScreenDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScreenDCInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxScreenDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ScreenDCInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxScrollBar
wxwidgets! {
    /// A wxScrollBar is a control that represents a horizontal or vertical scrollbar.
    /// - [`ScrollBar`] represents a C++ `wxScrollBar` class instance which your code has ownership, [`ScrollBarInRust`]`<false>` represents one which don't own.
    /// - Use [`ScrollBar`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxScrollBar` class's documentation](https://docs.wxwidgets.org/3.2/classwx_scroll_bar.html) for more details.
    #[doc(alias = "wxScrollBar")]
    #[doc(alias = "ScrollBar")]
    class ScrollBar
        = ScrollBarInRust<true>(wxScrollBar) impl
        ScrollBarMethods,
        // ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScrollBarInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxScrollBar::wxScrollBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_scroll_bar.html#a8c38e80a7c369efa77ed166f01d6d86c).
    pub fn new_2step() -> ScrollBarInRust<OWNED> {
        unsafe { ScrollBarInRust(ffi::wxScrollBar_new()) }
    }
    /// Constructor, creating and showing a scrollbar.
    ///
    /// See [C++ `wxScrollBar::wxScrollBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_scroll_bar.html#a5914fbd50ef3b1d841d72d51d73cf9f4).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ScrollBarInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ScrollBarInRust(ffi::wxScrollBar_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ScrollBarInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ScrollBarInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ScrollBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollBarInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ScrollBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollBarInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ScrollBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollBarInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ScrollBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScrollBarInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxScrollBar_CLASSINFO()) }
    }
}
impl<const OWNED: bool> ControlMethods for ScrollBarInRust<OWNED> {
    /// Scrollbar creation function called by the scrollbar constructor.
    ///
    /// See [C++ `wxScrollBar::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_scroll_bar.html#a7a677f2a9d40b7aaa5a25cf72123a56f).
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxScrollBar_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
}

// wxScrollEvent
wxwidgets! {
    /// A scroll event holds information about events sent from stand-alone scrollbars (see wxScrollBar) and sliders (see wxSlider).
    /// - [`ScrollEvent`] represents a C++ `wxScrollEvent` class instance which your code has ownership, [`ScrollEventInRust`]`<false>` represents one which don't own.
    /// - Use [`ScrollEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxScrollEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_scroll_event.html) for more details.
    #[doc(alias = "wxScrollEvent")]
    #[doc(alias = "ScrollEvent")]
    class ScrollEvent
        = ScrollEventInRust<true>(wxScrollEvent) impl
        ScrollEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScrollEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxScrollEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ScrollEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ScrollEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: ScrollEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: ScrollEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ScrollEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScrollEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxScrollEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ScrollEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxScrollWinEvent
wxwidgets! {
    /// A scroll event holds information about events sent from scrolling windows.
    /// - [`ScrollWinEvent`] represents a C++ `wxScrollWinEvent` class instance which your code has ownership, [`ScrollWinEventInRust`]`<false>` represents one which don't own.
    /// - Use [`ScrollWinEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxScrollWinEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_scroll_win_event.html) for more details.
    #[doc(alias = "wxScrollWinEvent")]
    #[doc(alias = "ScrollWinEvent")]
    class ScrollWinEvent
        = ScrollWinEventInRust<true>(wxScrollWinEvent) impl
        ScrollWinEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScrollWinEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxScrollWinEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ScrollWinEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ScrollWinEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: ScrollWinEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollWinEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ScrollWinEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScrollWinEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxScrollWinEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ScrollWinEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSearchCtrl
wxwidgets! {
    /// A search control is a composite control with a search button, a text control, and a cancel button.
    /// - [`SearchCtrl`] represents a C++ `wxSearchCtrl` class instance which your code has ownership, [`SearchCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`SearchCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSearchCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_search_ctrl.html) for more details.
    #[doc(alias = "wxSearchCtrl")]
    #[doc(alias = "SearchCtrl")]
    class SearchCtrl
        = SearchCtrlInRust<true>(wxSearchCtrl) impl
        SearchCtrlMethods,
        // TextCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SearchCtrlInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxSearchCtrl::wxSearchCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_search_ctrl.html#afce4a40295a3b98eee43cc191ff3a48f).
    pub fn new_2step() -> SearchCtrlInRust<OWNED> {
        unsafe { SearchCtrlInRust(ffi::wxSearchCtrl_new()) }
    }
    /// Constructor, creating and showing a text control.
    ///
    /// See [C++ `wxSearchCtrl::wxSearchCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_search_ctrl.html#a6663657075e790177b0af7b274396fcd).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> SearchCtrlInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SearchCtrlInRust(ffi::wxSearchCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SearchCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SearchCtrlInRust<OWNED>> for TextCtrlInRust<OWNED> {
    fn from(o: SearchCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: SearchCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: SearchCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: SearchCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SearchCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SearchCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSearchCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSearchCtrl
impl<const OWNED: bool> TextEntryMethods for SearchCtrlInRust<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxSearchCtrl_AsTextEntry(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TextCtrlMethods for SearchCtrlInRust<OWNED> {
    ///
    /// See [C++ `wxSearchCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_search_ctrl.html#a6a438d8cb2a837e62f4e60cf264c72ae).
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSearchCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
}

// wxSetCursorEvent
wxwidgets! {
    /// A wxSetCursorEvent is generated from wxWindow when the mouse cursor is about to be set as a result of mouse motion.
    /// - [`SetCursorEvent`] represents a C++ `wxSetCursorEvent` class instance which your code has ownership, [`SetCursorEventInRust`]`<false>` represents one which don't own.
    /// - Use [`SetCursorEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSetCursorEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_set_cursor_event.html) for more details.
    #[doc(alias = "wxSetCursorEvent")]
    #[doc(alias = "SetCursorEvent")]
    class SetCursorEvent
        = SetCursorEventInRust<true>(wxSetCursorEvent) impl
        SetCursorEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SetCursorEventInRust<OWNED> {
    /// Constructor, used by the library itself internally to initialize the event object.
    ///
    /// See [C++ `wxSetCursorEvent::wxSetCursorEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_set_cursor_event.html#a862a2635ac71d7a652100027ae85fa6a).
    pub fn new(x: c_int, y: c_int) -> SetCursorEventInRust<OWNED> {
        unsafe { SetCursorEventInRust(ffi::wxSetCursorEvent_new(x, y)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SetCursorEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SetCursorEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: SetCursorEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SetCursorEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SetCursorEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SetCursorEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSetCursorEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SetCursorEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSettableHeaderColumn
wxwidgets! {
    /// Adds methods to set the column attributes to wxHeaderColumn.
    /// - [`SettableHeaderColumn`] represents a C++ `wxSettableHeaderColumn` class instance which your code has ownership, [`SettableHeaderColumnInRust`]`<false>` represents one which don't own.
    /// - Use [`SettableHeaderColumn`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSettableHeaderColumn` class's documentation](https://docs.wxwidgets.org/3.2/classwx_settable_header_column.html) for more details.
    #[doc(alias = "wxSettableHeaderColumn")]
    #[doc(alias = "SettableHeaderColumn")]
    class SettableHeaderColumn
        = SettableHeaderColumnInRust<true>(wxSettableHeaderColumn) impl
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const OWNED: bool> SettableHeaderColumnInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SettableHeaderColumnInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SettableHeaderColumnInRust<OWNED>> for HeaderColumnInRust<OWNED> {
    fn from(o: SettableHeaderColumnInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for SettableHeaderColumnInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSettableHeaderColumn_delete(self.0) }
        }
    }
}

// wxShowEvent
wxwidgets! {
    /// An event being sent when the window is shown or hidden.
    /// - [`ShowEvent`] represents a C++ `wxShowEvent` class instance which your code has ownership, [`ShowEventInRust`]`<false>` represents one which don't own.
    /// - Use [`ShowEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxShowEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_show_event.html) for more details.
    #[doc(alias = "wxShowEvent")]
    #[doc(alias = "ShowEvent")]
    class ShowEvent
        = ShowEventInRust<true>(wxShowEvent) impl
        ShowEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ShowEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxShowEvent::wxShowEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_show_event.html#a67164260c2e02eb6809192fe50cc5d1c).
    pub fn new(winid: c_int, show: bool) -> ShowEventInRust<OWNED> {
        unsafe { ShowEventInRust(ffi::wxShowEvent_new(winid, show)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ShowEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ShowEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: ShowEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ShowEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ShowEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ShowEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxShowEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ShowEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSimplebook
wxwidgets! {
    /// wxSimplebook is a control showing exactly one of its several pages.
    /// - [`Simplebook`] represents a C++ `wxSimplebook` class instance which your code has ownership, [`SimplebookInRust`]`<false>` represents one which don't own.
    /// - Use [`Simplebook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSimplebook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_simplebook.html) for more details.
    #[doc(alias = "wxSimplebook")]
    #[doc(alias = "Simplebook")]
    class Simplebook
        = SimplebookInRust<true>(wxSimplebook) impl
        SimplebookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SimplebookInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxSimplebook::wxSimplebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_simplebook.html#a7912157673b19a8ee7b9f02e4523dab9).
    pub fn new_2step() -> SimplebookInRust<OWNED> {
        unsafe { SimplebookInRust(ffi::wxSimplebook_new()) }
    }
    /// Constructs a simple book control.
    ///
    /// See [C++ `wxSimplebook::wxSimplebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_simplebook.html#a819e3cd45f3ae703dc9b2d89b504fe50).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SimplebookInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SimplebookInRust(ffi::wxSimplebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SimplebookInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SimplebookInRust<OWNED>> for BookCtrlBaseInRust<OWNED> {
    fn from(o: SimplebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: SimplebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: SimplebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: SimplebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SimplebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SimplebookInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSimplebook_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for SimplebookInRust<OWNED> {
    /// Really create the window of an object created using default constructor.
    ///
    /// See [C++ `wxSimplebook::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_simplebook.html#a7ead9e1f4612887b5eb274f6ddfb93ff).
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSimplebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxSize
wxwidgets! {
    /// A wxSize is a useful data structure for graphics operations.
    /// - [`Size`] represents a C++ `wxSize` class instance which your code has ownership, [`SizeInRust`]`<false>` represents one which don't own.
    /// - Use [`Size`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSize` class's documentation](https://docs.wxwidgets.org/3.2/classwx_size.html) for more details.
    #[doc(alias = "wxSize")]
    #[doc(alias = "Size")]
    class Size
        = SizeInRust<true>(wxSize) impl
        SizeMethods
}
impl<const OWNED: bool> SizeInRust<OWNED> {
    /// Initializes this size object with zero width and height.
    ///
    /// See [C++ `wxSize::wxSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_size.html#a89bbb1a42ad12573ff42809221e243a7).
    pub fn new() -> SizeInRust<OWNED> {
        unsafe { SizeInRust(ffi::wxSize_new()) }
    }
    /// Initializes this size object with the given width and height.
    ///
    /// See [C++ `wxSize::wxSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_size.html#aaa5ee9cd2943878582267508255c5bc8).
    pub fn new_with_int(width: c_int, height: c_int) -> SizeInRust<OWNED> {
        unsafe { SizeInRust(ffi::wxSize_new1(width, height)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SizeInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for SizeInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSize_delete(self.0) }
        }
    }
}

// wxSizeEvent
wxwidgets! {
    /// A size event holds information about size change events of wxWindow.
    /// - [`SizeEvent`] represents a C++ `wxSizeEvent` class instance which your code has ownership, [`SizeEventInRust`]`<false>` represents one which don't own.
    /// - Use [`SizeEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSizeEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_size_event.html) for more details.
    #[doc(alias = "wxSizeEvent")]
    #[doc(alias = "SizeEvent")]
    class SizeEvent
        = SizeEventInRust<true>(wxSizeEvent) impl
        SizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SizeEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxSizeEvent::wxSizeEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_size_event.html#acce432b5d8aa28bd845022fa44a868cc).
    pub fn new<S: SizeMethods>(sz: &S, id: c_int) -> SizeEventInRust<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            SizeEventInRust(ffi::wxSizeEvent_new(sz, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SizeEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SizeEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: SizeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SizeEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SizeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SizeEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSizeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SizeEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSizer
wxwidgets! {
    /// wxSizer is the abstract base class used for laying out subwindows in a window.
    /// - [`Sizer`] represents a C++ `wxSizer` class instance which your code has ownership, [`SizerInRust`]`<false>` represents one which don't own.
    /// - Use [`Sizer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSizer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_sizer.html) for more details.
    #[doc(alias = "wxSizer")]
    #[doc(alias = "Sizer")]
    class Sizer
        = SizerInRust<true>(wxSizer) impl
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SizerInRust<OWNED> {
    // BLOCKED: fn wxSizer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SizerInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SizerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SizerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSizer_CLASSINFO()) }
    }
}

// wxSizerFlags
wxwidgets! {
    /// Container for sizer items flags providing readable names for them.
    /// - [`SizerFlags`] represents a C++ `wxSizerFlags` class instance which your code has ownership, [`SizerFlagsInRust`]`<false>` represents one which don't own.
    /// - Use [`SizerFlags`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSizerFlags` class's documentation](https://docs.wxwidgets.org/3.2/classwx_sizer_flags.html) for more details.
    #[doc(alias = "wxSizerFlags")]
    #[doc(alias = "SizerFlags")]
    class SizerFlags
        = SizerFlagsInRust<true>(wxSizerFlags) impl
        SizerFlagsMethods
}
impl<const OWNED: bool> SizerFlagsInRust<OWNED> {
    /// Creates the wxSizer with the proportion specified by proportion.
    ///
    /// See [C++ `wxSizerFlags::wxSizerFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sizer_flags.html#a2fe0499abe5461a2b8b4fe5fa2c054d4).
    pub fn new(proportion: c_int) -> SizerFlagsInRust<OWNED> {
        unsafe { SizerFlagsInRust(ffi::wxSizerFlags_new(proportion)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SizerFlagsInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for SizerFlagsInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSizerFlags_delete(self.0) }
        }
    }
}

// wxSizerItem
wxwidgets! {
    /// The wxSizerItem class is used to track the position, size and other attributes of each item managed by a wxSizer.
    /// - [`SizerItem`] represents a C++ `wxSizerItem` class instance which your code has ownership, [`SizerItemInRust`]`<false>` represents one which don't own.
    /// - Use [`SizerItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSizerItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_sizer_item.html) for more details.
    #[doc(alias = "wxSizerItem")]
    #[doc(alias = "SizerItem")]
    class SizerItem
        = SizerItemInRust<true>(wxSizerItem) impl
        SizerItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> SizerItemInRust<OWNED> {
    /// Construct a sizer item for tracking a spacer.
    ///
    /// See [C++ `wxSizerItem::wxSizerItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sizer_item.html#ab07c608bd56283df5847c1e9bd4ebfa9).
    pub fn new_with_int<O: ObjectMethods>(
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> SizerItemInRust<OWNED> {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItemInRust(ffi::wxSizerItem_new(
                width, height, proportion, flag, border, user_data,
            ))
        }
    }
    /// Construct a sizer item for tracking a window.
    ///
    /// See [C++ `wxSizerItem::wxSizerItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sizer_item.html#a4c858f9b3ae6e1d9e96602959d5d7ff2).
    pub fn new_with_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        window: Option<&W>,
        flags: &S,
    ) -> SizerItemInRust<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItemInRust(ffi::wxSizerItem_new1(window, flags))
        }
    }
    ///
    /// See [C++ `wxSizerItem::wxSizerItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sizer_item.html#aaefc97a23300b948bab559e4e89638b1).
    pub fn new_with_window_int<W: WindowMethods, O: ObjectMethods>(
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> SizerItemInRust<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItemInRust(ffi::wxSizerItem_new2(
                window, proportion, flag, border, user_data,
            ))
        }
    }
    /// Construct a sizer item for tracking a subsizer.
    ///
    /// See [C++ `wxSizerItem::wxSizerItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sizer_item.html#af321a97190675a193212131d5f11523f).
    pub fn new_with_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        sizer: Option<&S>,
        flags: &S2,
    ) -> SizerItemInRust<OWNED> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItemInRust(ffi::wxSizerItem_new3(sizer, flags))
        }
    }
    ///
    /// See [C++ `wxSizerItem::wxSizerItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sizer_item.html#a8678d88740bc5a9244338fd345502284).
    pub fn new_with_sizer_int<S: SizerMethods, O: ObjectMethods>(
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> SizerItemInRust<OWNED> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItemInRust(ffi::wxSizerItem_new4(
                sizer, proportion, flag, border, user_data,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SizerItemInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SizerItemInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SizerItemInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SizerItemInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSizerItem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SizerItemInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSlider
wxwidgets! {
    /// A slider is a control with a handle which can be pulled back and forth to change the value.
    /// - [`Slider`] represents a C++ `wxSlider` class instance which your code has ownership, [`SliderInRust`]`<false>` represents one which don't own.
    /// - Use [`Slider`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSlider` class's documentation](https://docs.wxwidgets.org/3.2/classwx_slider.html) for more details.
    #[doc(alias = "wxSlider")]
    #[doc(alias = "Slider")]
    class Slider
        = SliderInRust<true>(wxSlider) impl
        SliderMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SliderInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxSlider::wxSlider()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_slider.html#a2173af74dec187f971f43ff76ce5fda4).
    pub fn new_2step() -> SliderInRust<OWNED> {
        unsafe { SliderInRust(ffi::wxSlider_new()) }
    }
    /// Constructor, creating and showing a slider.
    ///
    /// See [C++ `wxSlider::wxSlider()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_slider.html#a891b43da8ecd9709fdac3ccadc23903f).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: c_int,
        min_value: c_int,
        max_value: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> SliderInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SliderInRust(ffi::wxSlider_new1(
                parent, id, value, min_value, max_value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SliderInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SliderInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: SliderInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SliderInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: SliderInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SliderInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: SliderInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SliderInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SliderInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SliderInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSlider_CLASSINFO()) }
    }
}

// wxSpinButton
wxwidgets! {
    /// A wxSpinButton has two small up and down (or left and right) arrow buttons.
    /// - [`SpinButton`] represents a C++ `wxSpinButton` class instance which your code has ownership, [`SpinButtonInRust`]`<false>` represents one which don't own.
    /// - Use [`SpinButton`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSpinButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_spin_button.html) for more details.
    #[doc(alias = "wxSpinButton")]
    #[doc(alias = "SpinButton")]
    class SpinButton
        = SpinButtonInRust<true>(wxSpinButton) impl
        SpinButtonMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinButtonInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxSpinButton::wxSpinButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_spin_button.html#aa4eba752e564f360bcc58b3f54ccc513).
    pub fn new_2step() -> SpinButtonInRust<OWNED> {
        unsafe { SpinButtonInRust(ffi::wxSpinButton_new()) }
    }
    /// Constructor, creating and showing a spin button.
    ///
    /// See [C++ `wxSpinButton::wxSpinButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_spin_button.html#a3b586bd26f28c503a5e313c85c64ec67).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SpinButtonInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SpinButtonInRust(ffi::wxSpinButton_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SpinButtonInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SpinButtonInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: SpinButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinButtonInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: SpinButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinButtonInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: SpinButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinButtonInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SpinButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinButtonInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSpinButton_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for SpinButtonInRust<OWNED> {
    /// Scrollbar creation function called by the spin button constructor.
    ///
    /// See [C++ `wxSpinButton::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_spin_button.html#a49a34a60952c5f9319da9379887ca10e).
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSpinButton_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxSpinCtrl
wxwidgets! {
    /// wxSpinCtrl combines wxTextCtrl and wxSpinButton in one control.
    /// - [`SpinCtrl`] represents a C++ `wxSpinCtrl` class instance which your code has ownership, [`SpinCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`SpinCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSpinCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_spin_ctrl.html) for more details.
    #[doc(alias = "wxSpinCtrl")]
    #[doc(alias = "SpinCtrl")]
    class SpinCtrl
        = SpinCtrlInRust<true>(wxSpinCtrl) impl
        SpinCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinCtrlInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxSpinCtrl::wxSpinCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_spin_ctrl.html#ac5defa94e938dd00380f551502c20a4e).
    pub fn new_2step() -> SpinCtrlInRust<OWNED> {
        unsafe { SpinCtrlInRust(ffi::wxSpinCtrl_new()) }
    }
    /// Constructor, creating and showing a spin control.
    ///
    /// See [C++ `wxSpinCtrl::wxSpinCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_spin_ctrl.html#ae14fbff54acea597904bdf583fa13c0f).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        min: c_int,
        max: c_int,
        initial: c_int,
        name: &str,
    ) -> SpinCtrlInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SpinCtrlInRust(ffi::wxSpinCtrl_new1(
                parent, id, value, pos, size, style, min, max, initial, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SpinCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SpinCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: SpinCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: SpinCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: SpinCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SpinCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSpinCtrl_CLASSINFO()) }
    }
}

// wxSpinCtrlDouble
wxwidgets! {
    /// wxSpinCtrlDouble combines wxTextCtrl and wxSpinButton in one control and displays a real number.
    /// - [`SpinCtrlDouble`] represents a C++ `wxSpinCtrlDouble` class instance which your code has ownership, [`SpinCtrlDoubleInRust`]`<false>` represents one which don't own.
    /// - Use [`SpinCtrlDouble`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSpinCtrlDouble` class's documentation](https://docs.wxwidgets.org/3.2/classwx_spin_ctrl_double.html) for more details.
    #[doc(alias = "wxSpinCtrlDouble")]
    #[doc(alias = "SpinCtrlDouble")]
    class SpinCtrlDouble
        = SpinCtrlDoubleInRust<true>(wxSpinCtrlDouble) impl
        SpinCtrlDoubleMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinCtrlDoubleInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxSpinCtrlDouble::wxSpinCtrlDouble()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_spin_ctrl_double.html#a7448457351183b00b4393b38f0f992b2).
    pub fn new_2step() -> SpinCtrlDoubleInRust<OWNED> {
        unsafe { SpinCtrlDoubleInRust(ffi::wxSpinCtrlDouble_new()) }
    }
    /// Constructor, creating and showing a spin control.
    ///
    /// See [C++ `wxSpinCtrlDouble::wxSpinCtrlDouble()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_spin_ctrl_double.html#afd85d7da42e6e994e653af5d2efce0bd).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        min: c_double,
        max: c_double,
        initial: c_double,
        inc: c_double,
        name: &str,
    ) -> SpinCtrlDoubleInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SpinCtrlDoubleInRust(ffi::wxSpinCtrlDouble_new1(
                parent, id, value, pos, size, style, min, max, initial, inc, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SpinCtrlDoubleInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: SpinCtrlDoubleInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: SpinCtrlDoubleInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: SpinCtrlDoubleInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SpinCtrlDoubleInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinCtrlDoubleInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSpinCtrlDouble_CLASSINFO()) }
    }
}

// wxSpinDoubleEvent
wxwidgets! {
    /// This event class is used for the events generated by wxSpinCtrlDouble.
    /// - [`SpinDoubleEvent`] represents a C++ `wxSpinDoubleEvent` class instance which your code has ownership, [`SpinDoubleEventInRust`]`<false>` represents one which don't own.
    /// - Use [`SpinDoubleEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSpinDoubleEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_spin_double_event.html) for more details.
    #[doc(alias = "wxSpinDoubleEvent")]
    #[doc(alias = "SpinDoubleEvent")]
    class SpinDoubleEvent
        = SpinDoubleEventInRust<true>(wxSpinDoubleEvent) impl
        SpinDoubleEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinDoubleEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxSpinDoubleEvent()
    /// The copy constructor.
    ///
    /// See [C++ `wxSpinDoubleEvent::wxSpinDoubleEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_spin_double_event.html#a53a641a6232fe880ca13fad00d136b62).
    pub fn new<S: SpinDoubleEventMethods>(event: &S) -> SpinDoubleEventInRust<OWNED> {
        unsafe {
            let event = event.as_ptr();
            SpinDoubleEventInRust(ffi::wxSpinDoubleEvent_new1(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SpinDoubleEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SpinDoubleEventInRust<OWNED>> for NotifyEventInRust<OWNED> {
    fn from(o: SpinDoubleEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinDoubleEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: SpinDoubleEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinDoubleEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: SpinDoubleEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinDoubleEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SpinDoubleEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinDoubleEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSpinDoubleEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SpinDoubleEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSpinEvent
wxwidgets! {
    /// This event class is used for the events generated by wxSpinButton and wxSpinCtrl.
    /// - [`SpinEvent`] represents a C++ `wxSpinEvent` class instance which your code has ownership, [`SpinEventInRust`]`<false>` represents one which don't own.
    /// - Use [`SpinEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSpinEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_spin_event.html) for more details.
    #[doc(alias = "wxSpinEvent")]
    #[doc(alias = "SpinEvent")]
    class SpinEvent
        = SpinEventInRust<true>(wxSpinEvent) impl
        SpinEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxSpinEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SpinEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SpinEventInRust<OWNED>> for NotifyEventInRust<OWNED> {
    fn from(o: SpinEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: SpinEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: SpinEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SpinEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSpinEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SpinEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSplashScreen
wxwidgets! {
    /// wxSplashScreen shows a window with a thin border, displaying a bitmap describing your application.
    /// - [`SplashScreen`] represents a C++ `wxSplashScreen` class instance which your code has ownership, [`SplashScreenInRust`]`<false>` represents one which don't own.
    /// - Use [`SplashScreen`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSplashScreen` class's documentation](https://docs.wxwidgets.org/3.2/classwx_splash_screen.html) for more details.
    #[doc(alias = "wxSplashScreen")]
    #[doc(alias = "SplashScreen")]
    class SplashScreen
        = SplashScreenInRust<true>(wxSplashScreen) impl
        SplashScreenMethods,
        FrameMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SplashScreenInRust<OWNED> {
    /// Construct the splash screen passing a bitmap, a style, a timeout, a window id, optional position and size, and a window style.
    ///
    /// See [C++ `wxSplashScreen::wxSplashScreen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_splash_screen.html#a05a1d1af1dac400c659d41bd033d8566).
    pub fn new<B: BitmapMethods, W: WindowMethods, P: PointMethods, S: SizeMethods>(
        bitmap: &B,
        splash_style: c_long,
        milliseconds: c_int,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
    ) -> SplashScreenInRust<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            SplashScreenInRust(ffi::wxSplashScreen_new(
                bitmap,
                splash_style,
                milliseconds,
                parent,
                id,
                pos,
                size,
                style,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SplashScreenInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SplashScreenInRust<OWNED>> for FrameInRust<OWNED> {
    fn from(o: SplashScreenInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenInRust<OWNED>> for TopLevelWindowInRust<OWNED> {
    fn from(o: SplashScreenInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenInRust<OWNED>> for NonOwnedWindowInRust<OWNED> {
    fn from(o: SplashScreenInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: SplashScreenInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: SplashScreenInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SplashScreenInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SplashScreenInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSplashScreen_CLASSINFO()) }
    }
}

// wxSplitterEvent
wxwidgets! {
    /// This class represents the events generated by a splitter control.
    /// - [`SplitterEvent`] represents a C++ `wxSplitterEvent` class instance which your code has ownership, [`SplitterEventInRust`]`<false>` represents one which don't own.
    /// - Use [`SplitterEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSplitterEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_splitter_event.html) for more details.
    #[doc(alias = "wxSplitterEvent")]
    #[doc(alias = "SplitterEvent")]
    class SplitterEvent
        = SplitterEventInRust<true>(wxSplitterEvent) impl
        SplitterEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SplitterEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxSplitterEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SplitterEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SplitterEventInRust<OWNED>> for NotifyEventInRust<OWNED> {
    fn from(o: SplitterEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: SplitterEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: SplitterEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SplitterEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SplitterEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSplitterEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SplitterEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSplitterWindow
wxwidgets! {
    /// This class manages up to two subwindows.
    /// - [`SplitterWindow`] represents a C++ `wxSplitterWindow` class instance which your code has ownership, [`SplitterWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`SplitterWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSplitterWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_splitter_window.html) for more details.
    #[doc(alias = "wxSplitterWindow")]
    #[doc(alias = "SplitterWindow")]
    class SplitterWindow
        = SplitterWindowInRust<true>(wxSplitterWindow) impl
        SplitterWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SplitterWindowInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxSplitterWindow::wxSplitterWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_splitter_window.html#a311c33909f1164ccdf9a11f5be45ecdc).
    pub fn new_2step() -> SplitterWindowInRust<OWNED> {
        unsafe { SplitterWindowInRust(ffi::wxSplitterWindow_new()) }
    }
    /// Constructor for creating the window.
    ///
    /// See [C++ `wxSplitterWindow::wxSplitterWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_splitter_window.html#aeefa297444ad5b968f3105af012c987e).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SplitterWindowInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SplitterWindowInRust(ffi::wxSplitterWindow_new1(
                parent, id, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SplitterWindowInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SplitterWindowInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: SplitterWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterWindowInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: SplitterWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterWindowInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SplitterWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SplitterWindowInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSplitterWindow_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for SplitterWindowInRust<OWNED> {
    /// Creation function, for two-step construction.
    ///
    /// See [C++ `wxSplitterWindow::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_splitter_window.html#a40bd4e468a9c71a837e8de40b4c983db).
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        point: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let point = point.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSplitterWindow_Create(self.as_ptr(), parent, id, point, size, style, name)
        }
    }
}

// wxStaticBitmap
wxwidgets! {
    /// A static bitmap control displays a bitmap.
    /// - [`StaticBitmap`] represents a C++ `wxStaticBitmap` class instance which your code has ownership, [`StaticBitmapInRust`]`<false>` represents one which don't own.
    /// - Use [`StaticBitmap`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxStaticBitmap` class's documentation](https://docs.wxwidgets.org/3.2/classwx_static_bitmap.html) for more details.
    #[doc(alias = "wxStaticBitmap")]
    #[doc(alias = "StaticBitmap")]
    class StaticBitmap
        = StaticBitmapInRust<true>(wxStaticBitmap) impl
        StaticBitmapMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBitmapInRust<OWNED> {
    //  ENUM: ScaleMode
    pub const Scale_None: c_int = 0;
    pub const Scale_Fill: c_int = 0 + 1;
    pub const Scale_AspectFit: c_int = 0 + 2;
    pub const Scale_AspectFill: c_int = 0 + 3;

    /// Default constructor.
    ///
    /// See [C++ `wxStaticBitmap::wxStaticBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_static_bitmap.html#a291d7a90496e62b907eae9e1b55bee9a).
    pub fn new_2step() -> StaticBitmapInRust<OWNED> {
        unsafe { StaticBitmapInRust(ffi::wxStaticBitmap_new()) }
    }
    /// Constructor, creating and showing a static bitmap control.
    ///
    /// See [C++ `wxStaticBitmap::wxStaticBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_static_bitmap.html#af23cde747ba13da14e80ea86bce3fa8b).
    pub fn new<W: WindowMethods, B: BitmapBundleMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &B,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticBitmapInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticBitmapInRust(ffi::wxStaticBitmap_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StaticBitmapInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StaticBitmapInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: StaticBitmapInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBitmapInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: StaticBitmapInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBitmapInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: StaticBitmapInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBitmapInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: StaticBitmapInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticBitmapInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxStaticBitmap_CLASSINFO()) }
    }
}

// wxStaticBox
wxwidgets! {
    /// A static box is a rectangle drawn around other windows to denote a logical grouping of items.
    /// - [`StaticBox`] represents a C++ `wxStaticBox` class instance which your code has ownership, [`StaticBoxInRust`]`<false>` represents one which don't own.
    /// - Use [`StaticBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxStaticBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_static_box.html) for more details.
    #[doc(alias = "wxStaticBox")]
    #[doc(alias = "StaticBox")]
    class StaticBox
        = StaticBoxInRust<true>(wxStaticBox) impl
        StaticBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBoxInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxStaticBox::wxStaticBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_static_box.html#aa96250d5fbd5864d041ef878def4e474).
    pub fn new_2step() -> StaticBoxInRust<OWNED> {
        unsafe { StaticBoxInRust(ffi::wxStaticBox_new()) }
    }
    /// Constructor, creating and showing a static box.
    ///
    /// See [C++ `wxStaticBox::wxStaticBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_static_box.html#a840d60b3a3102858924cb06ff5e5df16).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticBoxInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticBoxInRust(ffi::wxStaticBox_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    // BLOCKED: fn wxStaticBox2()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StaticBoxInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StaticBoxInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: StaticBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: StaticBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: StaticBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: StaticBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticBoxInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxStaticBox_CLASSINFO()) }
    }
}

// wxStaticBoxSizer
wxwidgets! {
    /// wxStaticBoxSizer is a sizer derived from wxBoxSizer but adds a static box around the sizer.
    /// - [`StaticBoxSizer`] represents a C++ `wxStaticBoxSizer` class instance which your code has ownership, [`StaticBoxSizerInRust`]`<false>` represents one which don't own.
    /// - Use [`StaticBoxSizer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxStaticBoxSizer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_static_box_sizer.html) for more details.
    #[doc(alias = "wxStaticBoxSizer")]
    #[doc(alias = "StaticBoxSizer")]
    class StaticBoxSizer
        = StaticBoxSizerInRust<true>(wxStaticBoxSizer) impl
        StaticBoxSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBoxSizerInRust<OWNED> {
    /// This constructor uses an already existing static box.
    ///
    /// See [C++ `wxStaticBoxSizer::wxStaticBoxSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_static_box_sizer.html#a8c2af376122e1093b95331ec1dd17ba5).
    pub fn new_with_staticbox<S: StaticBoxMethods>(
        box_: Option<&S>,
        orient: c_int,
    ) -> StaticBoxSizerInRust<OWNED> {
        unsafe {
            let box_ = match box_ {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            StaticBoxSizerInRust(ffi::wxStaticBoxSizer_new(box_, orient))
        }
    }
    /// This constructor creates a new static box with the given label and parent window.
    ///
    /// See [C++ `wxStaticBoxSizer::wxStaticBoxSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_static_box_sizer.html#a9f69e687c1c78bf70295ce5a72934412).
    pub fn new_with_int<W: WindowMethods>(
        orient: c_int,
        parent: Option<&W>,
        label: &str,
    ) -> StaticBoxSizerInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            StaticBoxSizerInRust(ffi::wxStaticBoxSizer_new1(orient, parent, label))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StaticBoxSizerInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StaticBoxSizerInRust<OWNED>> for BoxSizerInRust<OWNED> {
    fn from(o: StaticBoxSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxSizerInRust<OWNED>> for SizerInRust<OWNED> {
    fn from(o: StaticBoxSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxSizerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: StaticBoxSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticBoxSizerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxStaticBoxSizer_CLASSINFO()) }
    }
}

// wxStaticLine
wxwidgets! {
    /// A static line is just a line which may be used in a dialog to separate the groups of controls.
    /// - [`StaticLine`] represents a C++ `wxStaticLine` class instance which your code has ownership, [`StaticLineInRust`]`<false>` represents one which don't own.
    /// - Use [`StaticLine`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxStaticLine` class's documentation](https://docs.wxwidgets.org/3.2/classwx_static_line.html) for more details.
    #[doc(alias = "wxStaticLine")]
    #[doc(alias = "StaticLine")]
    class StaticLine
        = StaticLineInRust<true>(wxStaticLine) impl
        StaticLineMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticLineInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxStaticLine::wxStaticLine()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_static_line.html#a0b3436879b2193445a34bad6e2fc5086).
    pub fn new_2step() -> StaticLineInRust<OWNED> {
        unsafe { StaticLineInRust(ffi::wxStaticLine_new()) }
    }
    /// Constructor, creating and showing a static line.
    ///
    /// See [C++ `wxStaticLine::wxStaticLine()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_static_line.html#a9db24738fcc9f5a83a5052e3098fc470).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticLineInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticLineInRust(ffi::wxStaticLine_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StaticLineInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StaticLineInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: StaticLineInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticLineInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: StaticLineInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticLineInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: StaticLineInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticLineInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: StaticLineInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticLineInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxStaticLine_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for StaticLineInRust<OWNED> {
    /// Creates the static line for two-step construction.
    ///
    /// See [C++ `wxStaticLine::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_static_line.html#ac2e6c54b896563e2ff87da22a4361161).
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxStaticLine_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxStaticText
wxwidgets! {
    /// A static text control displays one or more lines of read-only text.
    /// - [`StaticText`] represents a C++ `wxStaticText` class instance which your code has ownership, [`StaticTextInRust`]`<false>` represents one which don't own.
    /// - Use [`StaticText`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxStaticText` class's documentation](https://docs.wxwidgets.org/3.2/classwx_static_text.html) for more details.
    #[doc(alias = "wxStaticText")]
    #[doc(alias = "StaticText")]
    class StaticText
        = StaticTextInRust<true>(wxStaticText) impl
        StaticTextMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticTextInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxStaticText::wxStaticText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_static_text.html#a9291a72fe2317f4a9e30c6eb7d02e014).
    pub fn new_2step() -> StaticTextInRust<OWNED> {
        unsafe { StaticTextInRust(ffi::wxStaticText_new()) }
    }
    /// Constructor, creating and showing a text control.
    ///
    /// See [C++ `wxStaticText::wxStaticText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_static_text.html#a726ca095a252614428459748e18320fb).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticTextInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticTextInRust(ffi::wxStaticText_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StaticTextInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StaticTextInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: StaticTextInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticTextInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: StaticTextInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticTextInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: StaticTextInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticTextInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: StaticTextInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticTextInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxStaticText_CLASSINFO()) }
    }
}

// wxStatusBar
wxwidgets! {
    /// A status bar is a narrow window that can be placed along the bottom of a frame to give small amounts of status information.
    /// - [`StatusBar`] represents a C++ `wxStatusBar` class instance which your code has ownership, [`StatusBarInRust`]`<false>` represents one which don't own.
    /// - Use [`StatusBar`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxStatusBar` class's documentation](https://docs.wxwidgets.org/3.2/classwx_status_bar.html) for more details.
    #[doc(alias = "wxStatusBar")]
    #[doc(alias = "StatusBar")]
    class StatusBar
        = StatusBarInRust<true>(wxStatusBar) impl
        StatusBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StatusBarInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxStatusBar::wxStatusBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_status_bar.html#a0518ffafb51b3e050df1a0477cd9e0c8).
    pub fn new_2step() -> StatusBarInRust<OWNED> {
        unsafe { StatusBarInRust(ffi::wxStatusBar_new()) }
    }
    /// Constructor, creating the window.
    ///
    /// See [C++ `wxStatusBar::wxStatusBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_status_bar.html#a0d828fb14054ba93ad3579b65c995943).
    pub fn new<W: WindowMethods>(
        parent: Option<&W>,
        id: c_int,
        style: c_long,
        name: &str,
    ) -> StatusBarInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let name = WxString::from(name);
            let name = name.as_ptr();
            StatusBarInRust(ffi::wxStatusBar_new1(parent, id, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StatusBarInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StatusBarInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: StatusBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StatusBarInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: StatusBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StatusBarInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: StatusBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StatusBarInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: StatusBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StatusBarInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxStatusBar_CLASSINFO()) }
    }
}

// wxStatusBarPane
wxwidgets! {
    /// A status bar pane data container used by wxStatusBar.
    /// - [`StatusBarPane`] represents a C++ `wxStatusBarPane` class instance which your code has ownership, [`StatusBarPaneInRust`]`<false>` represents one which don't own.
    /// - Use [`StatusBarPane`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxStatusBarPane` class's documentation](https://docs.wxwidgets.org/3.2/classwx_status_bar_pane.html) for more details.
    #[doc(alias = "wxStatusBarPane")]
    #[doc(alias = "StatusBarPane")]
    class StatusBarPane
        = StatusBarPaneInRust<true>(wxStatusBarPane) impl
        StatusBarPaneMethods
}
impl<const OWNED: bool> StatusBarPaneInRust<OWNED> {
    /// Constructs the pane with the given style and width.
    ///
    /// See [C++ `wxStatusBarPane::wxStatusBarPane()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_status_bar_pane.html#a09de0e3d124479f91b27048845ef6761).
    pub fn new(style: c_int, width: c_int) -> StatusBarPaneInRust<OWNED> {
        unsafe { StatusBarPaneInRust(ffi::wxStatusBarPane_new(style, width)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for StatusBarPaneInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for StatusBarPaneInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStatusBarPane_delete(self.0) }
        }
    }
}

// wxStdDialogButtonSizer
wxwidgets! {
    /// This class creates button layouts which conform to the standard button spacing and ordering defined by the platform or toolkit's user interface guidelines (if such things exist).
    /// - [`StdDialogButtonSizer`] represents a C++ `wxStdDialogButtonSizer` class instance which your code has ownership, [`StdDialogButtonSizerInRust`]`<false>` represents one which don't own.
    /// - Use [`StdDialogButtonSizer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxStdDialogButtonSizer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_std_dialog_button_sizer.html) for more details.
    #[doc(alias = "wxStdDialogButtonSizer")]
    #[doc(alias = "StdDialogButtonSizer")]
    class StdDialogButtonSizer
        = StdDialogButtonSizerInRust<true>(wxStdDialogButtonSizer) impl
        StdDialogButtonSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StdDialogButtonSizerInRust<OWNED> {
    /// Constructor for a wxStdDialogButtonSizer.
    ///
    /// See [C++ `wxStdDialogButtonSizer::wxStdDialogButtonSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_std_dialog_button_sizer.html#a468d2d4e9882c13caad28e06b2ddb873).
    pub fn new() -> StdDialogButtonSizerInRust<OWNED> {
        unsafe { StdDialogButtonSizerInRust(ffi::wxStdDialogButtonSizer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StdDialogButtonSizerInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StdDialogButtonSizerInRust<OWNED>> for BoxSizerInRust<OWNED> {
    fn from(o: StdDialogButtonSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StdDialogButtonSizerInRust<OWNED>> for SizerInRust<OWNED> {
    fn from(o: StdDialogButtonSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StdDialogButtonSizerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: StdDialogButtonSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StdDialogButtonSizerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxStdDialogButtonSizer_CLASSINFO()) }
    }
}

// wxStockPreferencesPage
wxwidgets! {
    /// Specialization of wxPreferencesPage useful for certain commonly used preferences page.
    /// - [`StockPreferencesPage`] represents a C++ `wxStockPreferencesPage` class instance which your code has ownership, [`StockPreferencesPageInRust`]`<false>` represents one which don't own.
    /// - Use [`StockPreferencesPage`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxStockPreferencesPage` class's documentation](https://docs.wxwidgets.org/3.2/classwx_stock_preferences_page.html) for more details.
    #[doc(alias = "wxStockPreferencesPage")]
    #[doc(alias = "StockPreferencesPage")]
    class StockPreferencesPage
        = StockPreferencesPageInRust<true>(wxStockPreferencesPage) impl
        StockPreferencesPageMethods,
        PreferencesPageMethods
}
impl<const OWNED: bool> StockPreferencesPageInRust<OWNED> {
    //  ENUM: Kind
    pub const Kind_General: c_int = 0;
    pub const Kind_Advanced: c_int = 0 + 1;

    // NOT_SUPPORTED: fn wxStockPreferencesPage()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for StockPreferencesPageInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StockPreferencesPageInRust<OWNED>> for PreferencesPageInRust<OWNED> {
    fn from(o: StockPreferencesPageInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for StockPreferencesPageInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStockPreferencesPage_delete(self.0) }
        }
    }
}

// wxSysColourChangedEvent
wxwidgets! {
    /// This class is used for system colour change events, which are generated when the user changes the colour settings or when the system theme changes (e.g.
    /// - [`SysColourChangedEvent`] represents a C++ `wxSysColourChangedEvent` class instance which your code has ownership, [`SysColourChangedEventInRust`]`<false>` represents one which don't own.
    /// - Use [`SysColourChangedEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSysColourChangedEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_sys_colour_changed_event.html) for more details.
    #[doc(alias = "wxSysColourChangedEvent")]
    #[doc(alias = "SysColourChangedEvent")]
    class SysColourChangedEvent
        = SysColourChangedEventInRust<true>(wxSysColourChangedEvent) impl
        SysColourChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SysColourChangedEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxSysColourChangedEvent::wxSysColourChangedEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_sys_colour_changed_event.html#a55442699b065591bccb95d0d73868a57).
    pub fn new() -> SysColourChangedEventInRust<OWNED> {
        unsafe { SysColourChangedEventInRust(ffi::wxSysColourChangedEvent_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SysColourChangedEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SysColourChangedEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: SysColourChangedEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SysColourChangedEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: SysColourChangedEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SysColourChangedEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxSysColourChangedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SysColourChangedEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSystemSettings
wxwidgets! {
    /// wxSystemSettings allows the application to ask for details about the system.
    /// - [`SystemSettings`] represents a C++ `wxSystemSettings` class instance which your code has ownership, [`SystemSettingsInRust`]`<false>` represents one which don't own.
    /// - Use [`SystemSettings`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxSystemSettings` class's documentation](https://docs.wxwidgets.org/3.2/classwx_system_settings.html) for more details.
    #[doc(alias = "wxSystemSettings")]
    #[doc(alias = "SystemSettings")]
    class SystemSettings
        = SystemSettingsInRust<true>(wxSystemSettings) impl
        SystemSettingsMethods
}
impl<const OWNED: bool> SystemSettingsInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxSystemSettings::wxSystemSettings()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_system_settings.html#a34c3d6ded6a697164682dbfb96481318).
    pub fn new() -> SystemSettingsInRust<OWNED> {
        unsafe { SystemSettingsInRust(ffi::wxSystemSettings_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SystemSettingsInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for SystemSettingsInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSystemSettings_delete(self.0) }
        }
    }
}

use super::*;

// wxPCXHandler
wxwidgets! {
    /// This is the image handler for the PCX format.
    /// - [`PCXHandler`] represents a C++ `wxPCXHandler` class instance which your code has ownership, [`PCXHandlerInRust`]`<false>` represents one which don't own.
    /// - Use [`PCXHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPCXHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_p_c_x_handler.html) for more details.
    #[doc(alias = "wxPCXHandler")]
    #[doc(alias = "PCXHandler")]
    class PCXHandler
        = PCXHandlerInRust<true>(wxPCXHandler) impl
        PCXHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PCXHandlerInRust<IN_RUST> {
    /// Default constructor for wxPCXHandler.
    ///
    /// See [C++ `wxPCXHandler::wxPCXHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_p_c_x_handler.html#a0e760d2c6f6e71285ea5ea4989a6a6ce).
    pub fn new() -> PCXHandlerInRust<IN_RUST> {
        unsafe { PCXHandlerInRust(ffi::wxPCXHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PCXHandlerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PCXHandlerInRust<IN_RUST>> for ImageHandlerInRust<IN_RUST> {
    fn from(o: PCXHandlerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PCXHandlerInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PCXHandlerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PCXHandlerInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPCXHandler_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for PCXHandlerInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPNGHandler
wxwidgets! {
    /// This is the image handler for the PNG format.
    /// - [`PNGHandler`] represents a C++ `wxPNGHandler` class instance which your code has ownership, [`PNGHandlerInRust`]`<false>` represents one which don't own.
    /// - Use [`PNGHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPNGHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_p_n_g_handler.html) for more details.
    #[doc(alias = "wxPNGHandler")]
    #[doc(alias = "PNGHandler")]
    class PNGHandler
        = PNGHandlerInRust<true>(wxPNGHandler) impl
        PNGHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PNGHandlerInRust<IN_RUST> {
    /// Default constructor for wxPNGHandler.
    ///
    /// See [C++ `wxPNGHandler::wxPNGHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_p_n_g_handler.html#ab23ae6e7eb76694201284def48b2d4f2).
    pub fn new() -> PNGHandlerInRust<IN_RUST> {
        unsafe { PNGHandlerInRust(ffi::wxPNGHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PNGHandlerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PNGHandlerInRust<IN_RUST>> for ImageHandlerInRust<IN_RUST> {
    fn from(o: PNGHandlerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PNGHandlerInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PNGHandlerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PNGHandlerInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPNGHandler_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for PNGHandlerInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPNMHandler
wxwidgets! {
    /// This is the image handler for the PNM format.
    /// - [`PNMHandler`] represents a C++ `wxPNMHandler` class instance which your code has ownership, [`PNMHandlerInRust`]`<false>` represents one which don't own.
    /// - Use [`PNMHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPNMHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_p_n_m_handler.html) for more details.
    #[doc(alias = "wxPNMHandler")]
    #[doc(alias = "PNMHandler")]
    class PNMHandler
        = PNMHandlerInRust<true>(wxPNMHandler) impl
        PNMHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PNMHandlerInRust<IN_RUST> {
    /// Default constructor for wxPNMHandler.
    ///
    /// See [C++ `wxPNMHandler::wxPNMHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_p_n_m_handler.html#ac9fc6a93ce1d2804807e8faa39acf146).
    pub fn new() -> PNMHandlerInRust<IN_RUST> {
        unsafe { PNMHandlerInRust(ffi::wxPNMHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PNMHandlerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PNMHandlerInRust<IN_RUST>> for ImageHandlerInRust<IN_RUST> {
    fn from(o: PNMHandlerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PNMHandlerInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PNMHandlerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PNMHandlerInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPNMHandler_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for PNMHandlerInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPaintDC
wxwidgets! {
    /// A wxPaintDC must be constructed if an application wishes to paint on the client area of a window from within an EVT_PAINT() event handler.
    /// - [`PaintDC`] represents a C++ `wxPaintDC` class instance which your code has ownership, [`PaintDCInRust`]`<false>` represents one which don't own.
    /// - Use [`PaintDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPaintDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_paint_d_c.html) for more details.
    #[doc(alias = "wxPaintDC")]
    #[doc(alias = "PaintDC")]
    class PaintDC
        = PaintDCInRust<true>(wxPaintDC) impl
        PaintDCMethods,
        ClientDCMethods,
        WindowDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PaintDCInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxPaintDC::wxPaintDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_paint_d_c.html#a36fa14a64759e36a31f1100b9710e71b).
    pub fn new<W: WindowMethods>(window: Option<&W>) -> PaintDCInRust<IN_RUST> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PaintDCInRust(ffi::wxPaintDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PaintDCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PaintDCInRust<IN_RUST>> for ClientDCInRust<IN_RUST> {
    fn from(o: PaintDCInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PaintDCInRust<IN_RUST>> for WindowDCInRust<IN_RUST> {
    fn from(o: PaintDCInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PaintDCInRust<IN_RUST>> for DCInRust<IN_RUST> {
    fn from(o: PaintDCInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PaintDCInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PaintDCInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PaintDCInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPaintDC_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for PaintDCInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPaintEvent
wxwidgets! {
    /// A paint event is sent when a window's contents needs to be repainted.
    /// - [`PaintEvent`] represents a C++ `wxPaintEvent` class instance which your code has ownership, [`PaintEventInRust`]`<false>` represents one which don't own.
    /// - Use [`PaintEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPaintEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_paint_event.html) for more details.
    #[doc(alias = "wxPaintEvent")]
    #[doc(alias = "PaintEvent")]
    class PaintEvent
        = PaintEventInRust<true>(wxPaintEvent) impl
        PaintEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PaintEventInRust<IN_RUST> {
    // BLOCKED: fn wxPaintEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PaintEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PaintEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: PaintEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PaintEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PaintEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PaintEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPaintEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for PaintEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPalette
wxwidgets! {
    /// A palette is a table that maps pixel values to RGB colours.
    /// - [`Palette`] represents a C++ `wxPalette` class instance which your code has ownership, [`PaletteInRust`]`<false>` represents one which don't own.
    /// - Use [`Palette`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPalette` class's documentation](https://docs.wxwidgets.org/3.2/classwx_palette.html) for more details.
    #[doc(alias = "wxPalette")]
    #[doc(alias = "Palette")]
    class Palette
        = PaletteInRust<true>(wxPalette) impl
        PaletteMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PaletteInRust<IN_RUST> {
    /// Default constructor.
    ///
    /// See [C++ `wxPalette::wxPalette()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_palette.html#aff9ca82ce8e82b801f6f119fc8887289).
    pub fn new() -> PaletteInRust<IN_RUST> {
        unsafe { PaletteInRust(ffi::wxPalette_new()) }
    }
    /// Copy constructor, uses Reference Counting.
    ///
    /// See [C++ `wxPalette::wxPalette()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_palette.html#ab6e5c7d9f2fbc763120378e111dda828).
    pub fn new_with_palette<P: PaletteMethods>(palette: &P) -> PaletteInRust<IN_RUST> {
        unsafe {
            let palette = palette.as_ptr();
            PaletteInRust(ffi::wxPalette_new1(palette))
        }
    }
    /// Creates a palette from arrays of size n, one for each red, blue or green component.
    ///
    /// See [C++ `wxPalette::wxPalette()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_palette.html#ac98e35c82e6e2e70bad9190a51067123).
    pub fn new_with_int(
        n: c_int,
        red: *const c_void,
        green: *const c_void,
        blue: *const c_void,
    ) -> PaletteInRust<IN_RUST> {
        unsafe { PaletteInRust(ffi::wxPalette_new2(n, red, green, blue)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PaletteInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PaletteInRust<IN_RUST>> for GDIObjectInRust<IN_RUST> {
    fn from(o: PaletteInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PaletteInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PaletteInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PaletteInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPalette_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for PaletteInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPanel
wxwidgets! {
    /// A panel is a window on which controls are placed.
    /// - [`Panel`] represents a C++ `wxPanel` class instance which your code has ownership, [`PanelInRust`]`<false>` represents one which don't own.
    /// - Use [`Panel`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPanel` class's documentation](https://docs.wxwidgets.org/3.2/classwx_panel.html) for more details.
    #[doc(alias = "wxPanel")]
    #[doc(alias = "Panel")]
    class Panel
        = PanelInRust<true>(wxPanel) impl
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PanelInRust<IN_RUST> {
    /// Default constructor.
    ///
    /// See [C++ `wxPanel::wxPanel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_panel.html#abacb5b24465e240cfdc04e069249cbbb).
    pub fn new_2step() -> PanelInRust<IN_RUST> {
        unsafe { PanelInRust(ffi::wxPanel_new()) }
    }
    /// Constructor.
    ///
    /// See [C++ `wxPanel::wxPanel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_panel.html#a5860a221ee88bd4ea6f0843112523890).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> PanelInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            PanelInRust(ffi::wxPanel_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for PanelInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PanelInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: PanelInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PanelInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: PanelInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PanelInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PanelInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PanelInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPanel_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> WindowMethods for PanelInRust<IN_RUST> {
    /// Used for two-step panel construction.
    ///
    /// See [C++ `wxPanel::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_panel.html#a3ff58c601a52262e03abf84d3896ff2f).
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
            ffi::wxPanel_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxPasswordEntryDialog
wxwidgets! {
    /// This class represents a dialog that requests a one-line password string from the user.
    /// - [`PasswordEntryDialog`] represents a C++ `wxPasswordEntryDialog` class instance which your code has ownership, [`PasswordEntryDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`PasswordEntryDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPasswordEntryDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_password_entry_dialog.html) for more details.
    #[doc(alias = "wxPasswordEntryDialog")]
    #[doc(alias = "PasswordEntryDialog")]
    class PasswordEntryDialog
        = PasswordEntryDialogInRust<true>(wxPasswordEntryDialog) impl
        PasswordEntryDialogMethods,
        TextEntryDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PasswordEntryDialogInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxPasswordEntryDialog::wxPasswordEntryDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_password_entry_dialog.html#a46055baef2b9fce8401e676c2915e743).
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        default_value: &str,
        style: c_long,
        pos: &P,
    ) -> PasswordEntryDialogInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let default_value = WxString::from(default_value);
            let default_value = default_value.as_ptr();
            let pos = pos.as_ptr();
            PasswordEntryDialogInRust(ffi::wxPasswordEntryDialog_new(
                parent,
                message,
                caption,
                default_value,
                style,
                pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for PasswordEntryDialogInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PasswordEntryDialogInRust<IN_RUST>>
    for TextEntryDialogInRust<IN_RUST>
{
    fn from(o: PasswordEntryDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PasswordEntryDialogInRust<IN_RUST>> for DialogInRust<IN_RUST> {
    fn from(o: PasswordEntryDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PasswordEntryDialogInRust<IN_RUST>>
    for TopLevelWindowInRust<IN_RUST>
{
    fn from(o: PasswordEntryDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PasswordEntryDialogInRust<IN_RUST>>
    for NonOwnedWindowInRust<IN_RUST>
{
    fn from(o: PasswordEntryDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PasswordEntryDialogInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: PasswordEntryDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PasswordEntryDialogInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: PasswordEntryDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PasswordEntryDialogInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PasswordEntryDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PasswordEntryDialogInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPasswordEntryDialog_CLASSINFO()) }
    }
}

// wxPen
wxwidgets! {
    /// A pen is a drawing tool for drawing outlines.
    /// - [`Pen`] represents a C++ `wxPen` class instance which your code has ownership, [`PenInRust`]`<false>` represents one which don't own.
    /// - Use [`Pen`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPen` class's documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html) for more details.
    #[doc(alias = "wxPen")]
    #[doc(alias = "Pen")]
    class Pen
        = PenInRust<true>(wxPen) impl
        PenMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PenInRust<IN_RUST> {
    /// Default constructor.
    ///
    /// See [C++ `wxPen::wxPen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#af08e2f8db0c78f9bbca31674ab61d5eb).
    pub fn new() -> PenInRust<IN_RUST> {
        unsafe { PenInRust(ffi::wxPen_new()) }
    }
    /// Creates a pen object using the specified pen description.
    ///
    /// See [C++ `wxPen::wxPen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#abb0a620b8513b8c36e9b9dc1b8f87001).
    pub fn new_with_peninfo(info: *const c_void) -> PenInRust<IN_RUST> {
        unsafe { PenInRust(ffi::wxPen_new1(info)) }
    }
    // NOT_SUPPORTED: fn wxPen2()
    /// Constructs a stippled pen from a stipple bitmap and a width.
    ///
    /// See [C++ `wxPen::wxPen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#ae2a92286436cd980de9f9360eb9d4def).
    pub fn new_with_bitmap<B: BitmapMethods>(stipple: &B, width: c_int) -> PenInRust<IN_RUST> {
        unsafe {
            let stipple = stipple.as_ptr();
            PenInRust(ffi::wxPen_new3(stipple, width))
        }
    }
    /// Copy constructor, uses Reference Counting.
    ///
    /// See [C++ `wxPen::wxPen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#a5ba7b9a179fa2887bf77099bbe2dddce).
    pub fn new_with_pen<P: PenMethods>(pen: &P) -> PenInRust<IN_RUST> {
        unsafe {
            let pen = pen.as_ptr();
            PenInRust(ffi::wxPen_new4(pen))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PenInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PenInRust<IN_RUST>> for GDIObjectInRust<IN_RUST> {
    fn from(o: PenInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PenInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PenInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PenInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPen_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for PenInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPenList
wxwidgets! {
    /// There is only one instance of this class: wxThePenList.
    /// - [`PenList`] represents a C++ `wxPenList` class instance which your code has ownership, [`PenListInRust`]`<false>` represents one which don't own.
    /// - Use [`PenList`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPenList` class's documentation](https://docs.wxwidgets.org/3.2/classwx_pen_list.html) for more details.
    #[doc(alias = "wxPenList")]
    #[doc(alias = "PenList")]
    class PenList
        = PenListInRust<true>(wxPenList) impl
        PenListMethods
}
impl<const IN_RUST: bool> PenListInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxPenList::wxPenList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen_list.html#af0244769e4e820f75e12e065b440f66d).
    pub fn new() -> PenListInRust<IN_RUST> {
        unsafe { PenListInRust(ffi::wxPenList_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PenListInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for PenListInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxPenList_delete(self.0) }
        }
    }
}

// wxPersistenceManager
wxwidgets! {
    /// Provides support for automatically saving and restoring object properties to persistent storage.
    /// - [`PersistenceManager`] represents a C++ `wxPersistenceManager` class instance which your code has ownership, [`PersistenceManagerInRust`]`<false>` represents one which don't own.
    /// - Use [`PersistenceManager`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPersistenceManager` class's documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html) for more details.
    #[doc(alias = "wxPersistenceManager")]
    #[doc(alias = "PersistenceManager")]
    class PersistenceManager
        = PersistenceManagerInRust<true>(wxPersistenceManager) impl
        PersistenceManagerMethods
}
impl<const IN_RUST: bool> PersistenceManagerInRust<IN_RUST> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PersistenceManagerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for PersistenceManagerInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxPersistenceManager_delete(self.0) }
        }
    }
}

// wxPickerBase
wxwidgets! {
    /// Base abstract class for all pickers which support an auxiliary text control.
    /// - [`PickerBase`] represents a C++ `wxPickerBase` class instance which your code has ownership, [`PickerBaseInRust`]`<false>` represents one which don't own.
    /// - Use [`PickerBase`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPickerBase` class's documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html) for more details.
    #[doc(alias = "wxPickerBase")]
    #[doc(alias = "PickerBase")]
    class PickerBase
        = PickerBaseInRust<true>(wxPickerBase) impl
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PickerBaseInRust<IN_RUST> {
    // BLOCKED: fn wxPickerBase()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for PickerBaseInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PickerBaseInRust<IN_RUST>> for ControlInRust<IN_RUST> {
    fn from(o: PickerBaseInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PickerBaseInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: PickerBaseInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PickerBaseInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: PickerBaseInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PickerBaseInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PickerBaseInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PickerBaseInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPickerBase_CLASSINFO()) }
    }
}

// wxPoint
wxwidgets! {
    /// A wxPoint is a useful data structure for graphics operations.
    /// - [`Point`] represents a C++ `wxPoint` class instance which your code has ownership, [`PointInRust`]`<false>` represents one which don't own.
    /// - Use [`Point`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPoint` class's documentation](https://docs.wxwidgets.org/3.2/classwx_point.html) for more details.
    #[doc(alias = "wxPoint")]
    #[doc(alias = "Point")]
    class Point
        = PointInRust<true>(wxPoint) impl
        PointMethods
}
impl<const IN_RUST: bool> PointInRust<IN_RUST> {
    /// Constructs a point.
    ///
    /// See [C++ `wxPoint::wxPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_point.html#a561759249aa13713d92eaa310e746772).
    pub fn new() -> PointInRust<IN_RUST> {
        unsafe { PointInRust(ffi::wxPoint_new()) }
    }
    /// Initializes the point object with the given x and y coordinates.
    ///
    /// See [C++ `wxPoint::wxPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_point.html#a35d450cf9a580947188a16c5dadebc34).
    pub fn new_with_int(x: c_int, y: c_int) -> PointInRust<IN_RUST> {
        unsafe { PointInRust(ffi::wxPoint_new1(x, y)) }
    }
    /// Converts the given wxRealPoint (with floating point coordinates) to a wxPoint instance.
    ///
    /// See [C++ `wxPoint::wxPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_point.html#ad6f9c02b56357d8dcfb0bc135dcab639).
    pub fn new_with_realpoint<R: RealPointMethods>(pt: &R) -> PointInRust<IN_RUST> {
        unsafe {
            let pt = pt.as_ptr();
            PointInRust(ffi::wxPoint_new2(pt))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PointInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for PointInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxPoint_delete(self.0) }
        }
    }
}

// wxPopupTransientWindow
wxwidgets! {
    /// A wxPopupWindow which disappears automatically when the user clicks mouse outside it or if it loses focus in any other way.
    /// - [`PopupTransientWindow`] represents a C++ `wxPopupTransientWindow` class instance which your code has ownership, [`PopupTransientWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`PopupTransientWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPopupTransientWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_popup_transient_window.html) for more details.
    #[doc(alias = "wxPopupTransientWindow")]
    #[doc(alias = "PopupTransientWindow")]
    class PopupTransientWindow
        = PopupTransientWindowInRust<true>(wxPopupTransientWindow) impl
        PopupTransientWindowMethods,
        PopupWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PopupTransientWindowInRust<IN_RUST> {
    /// Default constructor.
    ///
    /// See [C++ `wxPopupTransientWindow::wxPopupTransientWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_popup_transient_window.html#a79282a961a2356c2a6fcb492767732f1).
    pub fn new_2step() -> PopupTransientWindowInRust<IN_RUST> {
        unsafe { PopupTransientWindowInRust(ffi::wxPopupTransientWindow_new()) }
    }
    /// Constructor.
    ///
    /// See [C++ `wxPopupTransientWindow::wxPopupTransientWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_popup_transient_window.html#a1958dc2db49d85b12de4b7e249dc7ddc).
    pub fn new<W: WindowMethods>(
        parent: Option<&W>,
        flags: c_int,
    ) -> PopupTransientWindowInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PopupTransientWindowInRust(ffi::wxPopupTransientWindow_new1(parent, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for PopupTransientWindowInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PopupTransientWindowInRust<IN_RUST>> for PopupWindowInRust<IN_RUST> {
    fn from(o: PopupTransientWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PopupTransientWindowInRust<IN_RUST>>
    for NonOwnedWindowInRust<IN_RUST>
{
    fn from(o: PopupTransientWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PopupTransientWindowInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: PopupTransientWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PopupTransientWindowInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: PopupTransientWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PopupTransientWindowInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PopupTransientWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PopupTransientWindowInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPopupTransientWindow_CLASSINFO()) }
    }
}

// wxPopupWindow
wxwidgets! {
    /// A special kind of top level window used for popup menus, combobox popups and such.
    /// - [`PopupWindow`] represents a C++ `wxPopupWindow` class instance which your code has ownership, [`PopupWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`PopupWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPopupWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_popup_window.html) for more details.
    #[doc(alias = "wxPopupWindow")]
    #[doc(alias = "PopupWindow")]
    class PopupWindow
        = PopupWindowInRust<true>(wxPopupWindow) impl
        PopupWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PopupWindowInRust<IN_RUST> {
    /// Default constructor.
    ///
    /// See [C++ `wxPopupWindow::wxPopupWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_popup_window.html#a067444f3532f95541e0c445a916af9dd).
    pub fn new_2step() -> PopupWindowInRust<IN_RUST> {
        unsafe { PopupWindowInRust(ffi::wxPopupWindow_new()) }
    }
    /// Constructor.
    ///
    /// See [C++ `wxPopupWindow::wxPopupWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_popup_window.html#ab31a3724d9682f6870bd17f71f99b95c).
    pub fn new<W: WindowMethods>(parent: Option<&W>, flags: c_int) -> PopupWindowInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PopupWindowInRust(ffi::wxPopupWindow_new1(parent, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for PopupWindowInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PopupWindowInRust<IN_RUST>> for NonOwnedWindowInRust<IN_RUST> {
    fn from(o: PopupWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PopupWindowInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: PopupWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PopupWindowInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: PopupWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PopupWindowInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PopupWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PopupWindowInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPopupWindow_CLASSINFO()) }
    }
}

// wxPreferencesEditor
wxwidgets! {
    /// Manage preferences dialog.
    /// - [`PreferencesEditor`] represents a C++ `wxPreferencesEditor` class instance which your code has ownership, [`PreferencesEditorInRust`]`<false>` represents one which don't own.
    /// - Use [`PreferencesEditor`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPreferencesEditor` class's documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_editor.html) for more details.
    #[doc(alias = "wxPreferencesEditor")]
    #[doc(alias = "PreferencesEditor")]
    class PreferencesEditor
        = PreferencesEditorInRust<true>(wxPreferencesEditor) impl
        PreferencesEditorMethods
}
impl<const IN_RUST: bool> PreferencesEditorInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxPreferencesEditor::wxPreferencesEditor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_editor.html#a763bc5d60aef3ebf4ee296fb14634de3).
    pub fn new(title: &str) -> PreferencesEditorInRust<IN_RUST> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            PreferencesEditorInRust(ffi::wxPreferencesEditor_new(title))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PreferencesEditorInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for PreferencesEditorInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxPreferencesEditor_delete(self.0) }
        }
    }
}

// wxPreferencesPage
wxwidgets! {
    /// One page of preferences dialog.
    /// - [`PreferencesPage`] represents a C++ `wxPreferencesPage` class instance which your code has ownership, [`PreferencesPageInRust`]`<false>` represents one which don't own.
    /// - Use [`PreferencesPage`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPreferencesPage` class's documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_page.html) for more details.
    #[doc(alias = "wxPreferencesPage")]
    #[doc(alias = "PreferencesPage")]
    class PreferencesPage
        = PreferencesPageInRust<true>(wxPreferencesPage) impl
        PreferencesPageMethods
}
impl<const IN_RUST: bool> PreferencesPageInRust<IN_RUST> {
    // BLOCKED: fn wxPreferencesPage()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PreferencesPageInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for PreferencesPageInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxPreferencesPage_delete(self.0) }
        }
    }
}

// wxPropertySheetDialog
wxwidgets! {
    /// This class represents a property sheet dialog: a tabbed dialog for showing settings.
    /// - [`PropertySheetDialog`] represents a C++ `wxPropertySheetDialog` class instance which your code has ownership, [`PropertySheetDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`PropertySheetDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxPropertySheetDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html) for more details.
    #[doc(alias = "wxPropertySheetDialog")]
    #[doc(alias = "PropertySheetDialog")]
    class PropertySheetDialog
        = PropertySheetDialogInRust<true>(wxPropertySheetDialog) impl
        PropertySheetDialogMethods,
        DialogMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> PropertySheetDialogInRust<IN_RUST> {
    /// Default constructor.
    ///
    /// See [C++ `wxPropertySheetDialog::wxPropertySheetDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#a124dbdcb716e3157aad47db24d127550).
    pub fn new_2step() -> PropertySheetDialogInRust<IN_RUST> {
        unsafe { PropertySheetDialogInRust(ffi::wxPropertySheetDialog_new()) }
    }
    /// Constructor.
    ///
    /// See [C++ `wxPropertySheetDialog::wxPropertySheetDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#a3e9f920aa2df1b16761f5be1320f1268).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> PropertySheetDialogInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            PropertySheetDialogInRust(ffi::wxPropertySheetDialog_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for PropertySheetDialogInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<PropertySheetDialogInRust<IN_RUST>> for DialogInRust<IN_RUST> {
    fn from(o: PropertySheetDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PropertySheetDialogInRust<IN_RUST>>
    for TopLevelWindowInRust<IN_RUST>
{
    fn from(o: PropertySheetDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PropertySheetDialogInRust<IN_RUST>>
    for NonOwnedWindowInRust<IN_RUST>
{
    fn from(o: PropertySheetDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PropertySheetDialogInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: PropertySheetDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PropertySheetDialogInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: PropertySheetDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<PropertySheetDialogInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: PropertySheetDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for PropertySheetDialogInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxPropertySheetDialog_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> TopLevelWindowMethods for PropertySheetDialogInRust<IN_RUST> {
    /// Call this from your own Create function, before adding buttons and pages.
    ///
    /// See [C++ `wxPropertySheetDialog::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#ad96015ab13fc1e2c364660fafd4121e0).
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        title: &str,
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
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxPropertySheetDialog_Create(
                self.as_ptr(),
                parent,
                id,
                title,
                pos,
                size,
                style,
                name,
            )
        }
    }
}

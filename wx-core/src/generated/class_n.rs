use super::*;

// wxNativeFontInfo
wxwidgets! {
    /// wxNativeFontInfo is platform-specific font representation: this class should be considered as an opaque font description only used by the native functions, the user code can only get the objects of this type from somewhere and pass it somewhere else (possibly save them somewhere using ToString() and restore them using FromString())
    /// - [`NativeFontInfo`] represents a C++ `wxNativeFontInfo` class instance which your code has ownership, [`NativeFontInfoInRust`]`<false>` represents one which don't own.
    /// - Use [`NativeFontInfo`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxNativeFontInfo` class's documentation](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html) for more details.
    #[doc(alias = "wxNativeFontInfo")]
    #[doc(alias = "NativeFontInfo")]
    class NativeFontInfo
        = NativeFontInfoInRust<true>(wxNativeFontInfo) impl
        NativeFontInfoMethods
}
impl<const OWNED: bool> NativeFontInfoInRust<OWNED> {
    ///
    /// See [C++ `wxNativeFontInfo::wxNativeFontInfo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a1925fc4457120e9a1c9dbb3b2bd69f1c).
    pub fn new() -> NativeFontInfoInRust<OWNED> {
        unsafe { NativeFontInfoInRust(ffi::wxNativeFontInfo_new()) }
    }
    ///
    /// See [C++ `wxNativeFontInfo::wxNativeFontInfo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_native_font_info.html#a1b1e2d352f84671443e652dc829d77a3).
    pub fn new_with_nativefontinfo<N: NativeFontInfoMethods>(
        info: &N,
    ) -> NativeFontInfoInRust<OWNED> {
        unsafe {
            let info = info.as_ptr();
            NativeFontInfoInRust(ffi::wxNativeFontInfo_new1(info))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for NativeFontInfoInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for NativeFontInfoInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxNativeFontInfo_delete(self.0) }
        }
    }
}

// wxNavigationKeyEvent
wxwidgets! {
    /// This event class contains information about navigation events, generated by navigation keys such as tab and page down.
    /// - [`NavigationKeyEvent`] represents a C++ `wxNavigationKeyEvent` class instance which your code has ownership, [`NavigationKeyEventInRust`]`<false>` represents one which don't own.
    /// - Use [`NavigationKeyEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxNavigationKeyEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html) for more details.
    #[doc(alias = "wxNavigationKeyEvent")]
    #[doc(alias = "NavigationKeyEvent")]
    class NavigationKeyEvent
        = NavigationKeyEventInRust<true>(wxNavigationKeyEvent) impl
        NavigationKeyEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> NavigationKeyEventInRust<OWNED> {
    //  ENUM: wxNavigationKeyEventFlags
    pub const IsBackward: c_int = 0x0000;
    pub const IsForward: c_int = 0x0001;
    pub const WinChange: c_int = 0x0002;
    pub const FromTab: c_int = 0x0004;

    ///
    /// See [C++ `wxNavigationKeyEvent::wxNavigationKeyEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html#a87080163d24b140012f2f4ce2b48f977).
    pub fn new() -> NavigationKeyEventInRust<OWNED> {
        unsafe { NavigationKeyEventInRust(ffi::wxNavigationKeyEvent_new()) }
    }
    ///
    /// See [C++ `wxNavigationKeyEvent::wxNavigationKeyEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_navigation_key_event.html#ac54bb8927f61f701a4e6a354ae4d938e).
    pub fn new_with_navigationkeyevent<N: NavigationKeyEventMethods>(
        event: &N,
    ) -> NavigationKeyEventInRust<OWNED> {
        unsafe {
            let event = event.as_ptr();
            NavigationKeyEventInRust(ffi::wxNavigationKeyEvent_new1(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for NavigationKeyEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<NavigationKeyEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: NavigationKeyEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NavigationKeyEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: NavigationKeyEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NavigationKeyEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxNavigationKeyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for NavigationKeyEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxNonOwnedWindow
wxwidgets! {
    /// Common base class for all non-child windows.
    /// - [`NonOwnedWindow`] represents a C++ `wxNonOwnedWindow` class instance which your code has ownership, [`NonOwnedWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`NonOwnedWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxNonOwnedWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_non_owned_window.html) for more details.
    #[doc(alias = "wxNonOwnedWindow")]
    #[doc(alias = "NonOwnedWindow")]
    class NonOwnedWindow
        = NonOwnedWindowInRust<true>(wxNonOwnedWindow) impl
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NonOwnedWindowInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for NonOwnedWindowInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<NonOwnedWindowInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: NonOwnedWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NonOwnedWindowInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: NonOwnedWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NonOwnedWindowInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: NonOwnedWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NonOwnedWindowInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxNonOwnedWindow_CLASSINFO()) }
    }
}

// wxNotebook
wxwidgets! {
    /// This class represents a notebook control, which manages multiple windows with associated tabs.
    /// - [`Notebook`] represents a C++ `wxNotebook` class instance which your code has ownership, [`NotebookInRust`]`<false>` represents one which don't own.
    /// - Use [`Notebook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxNotebook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_notebook.html) for more details.
    #[doc(alias = "wxNotebook")]
    #[doc(alias = "Notebook")]
    class Notebook
        = NotebookInRust<true>(wxNotebook) impl
        NotebookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NotebookInRust<OWNED> {
    /// Constructs a notebook control.
    ///
    /// See [C++ `wxNotebook::wxNotebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_notebook.html#a7bd68c88ae0ac9b5bfa4d60ef3e0b067).
    pub fn new_2step() -> NotebookInRust<OWNED> {
        unsafe { NotebookInRust(ffi::wxNotebook_new()) }
    }
    /// Constructs a notebook control.
    ///
    /// See [C++ `wxNotebook::wxNotebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_notebook.html#a3f096bce4ab17440ce49c1bf761d074e).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> NotebookInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            NotebookInRust(ffi::wxNotebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for NotebookInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<NotebookInRust<OWNED>> for BookCtrlBaseInRust<OWNED> {
    fn from(o: NotebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: NotebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: NotebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: NotebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: NotebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NotebookInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxNotebook_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for NotebookInRust<OWNED> {
    /// Creates a notebook control.
    ///
    /// See [C++ `wxNotebook::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_notebook.html#a6ba4f58ec00e3c192bcb856b1244b09f).
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
            ffi::wxNotebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxNotificationMessage
wxwidgets! {
    /// This class allows showing the user a message non intrusively.
    /// - [`NotificationMessage`] represents a C++ `wxNotificationMessage` class instance which your code has ownership, [`NotificationMessageInRust`]`<false>` represents one which don't own.
    /// - Use [`NotificationMessage`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxNotificationMessage` class's documentation](https://docs.wxwidgets.org/3.2/classwx_notification_message.html) for more details.
    #[doc(alias = "wxNotificationMessage")]
    #[doc(alias = "NotificationMessage")]
    class NotificationMessage
        = NotificationMessageInRust<true>(wxNotificationMessage) impl
        NotificationMessageMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NotificationMessageInRust<OWNED> {
    //  ENUM: @38
    pub const Timeout_Auto: c_int = -1;
    pub const Timeout_Never: c_int = 0;

    /// Default constructor, use SetParent(), SetTitle() and SetMessage() to initialize the object before showing it.
    ///
    /// See [C++ `wxNotificationMessage::wxNotificationMessage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#a7799f9358f0d90f8d9ebc675c2da4782).
    pub fn new() -> NotificationMessageInRust<OWNED> {
        unsafe { NotificationMessageInRust(ffi::wxNotificationMessage_new()) }
    }
    /// Create a notification object with the given attributes.
    ///
    /// See [C++ `wxNotificationMessage::wxNotificationMessage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_notification_message.html#ad7702e56ced878ea5b0b0e3f9a6a727c).
    pub fn new_with_str<W: WindowMethods>(
        title: &str,
        message: &str,
        parent: Option<&W>,
        flags: c_int,
    ) -> NotificationMessageInRust<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            NotificationMessageInRust(ffi::wxNotificationMessage_new1(
                title, message, parent, flags,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for NotificationMessageInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<NotificationMessageInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: NotificationMessageInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotificationMessageInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: NotificationMessageInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NotificationMessageInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxNotificationMessage_CLASSINFO()) }
    }
}

// wxNotifyEvent
wxwidgets! {
    /// This class is not used by the event handlers by itself, but is a base class for other event classes (such as wxBookCtrlEvent).
    /// - [`NotifyEvent`] represents a C++ `wxNotifyEvent` class instance which your code has ownership, [`NotifyEventInRust`]`<false>` represents one which don't own.
    /// - Use [`NotifyEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxNotifyEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_notify_event.html) for more details.
    #[doc(alias = "wxNotifyEvent")]
    #[doc(alias = "NotifyEvent")]
    class NotifyEvent
        = NotifyEventInRust<true>(wxNotifyEvent) impl
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> NotifyEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxNotifyEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for NotifyEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<NotifyEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: NotifyEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotifyEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: NotifyEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotifyEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: NotifyEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NotifyEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxNotifyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for NotifyEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxNumberEntryDialog
wxwidgets! {
    /// This class represents a dialog that requests a numeric input from the user.
    /// - [`NumberEntryDialog`] represents a C++ `wxNumberEntryDialog` class instance which your code has ownership, [`NumberEntryDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`NumberEntryDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxNumberEntryDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_number_entry_dialog.html) for more details.
    #[doc(alias = "wxNumberEntryDialog")]
    #[doc(alias = "NumberEntryDialog")]
    class NumberEntryDialog
        = NumberEntryDialogInRust<true>(wxNumberEntryDialog) impl
        NumberEntryDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NumberEntryDialogInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxNumberEntryDialog::wxNumberEntryDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_number_entry_dialog.html#aebe799ab025b3b31b1d5e1de45e582a5).
    pub fn new_2step() -> NumberEntryDialogInRust<OWNED> {
        unsafe { NumberEntryDialogInRust(ffi::wxNumberEntryDialog_new()) }
    }
    /// Constructor.
    ///
    /// See [C++ `wxNumberEntryDialog::wxNumberEntryDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_number_entry_dialog.html#a35f2a9529c9d126dbb8196698cb93186).
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        prompt: &str,
        caption: &str,
        value: c_long,
        min: c_long,
        max: c_long,
        pos: &P,
    ) -> NumberEntryDialogInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let prompt = WxString::from(prompt);
            let prompt = prompt.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let pos = pos.as_ptr();
            NumberEntryDialogInRust(ffi::wxNumberEntryDialog_new1(
                parent, message, prompt, caption, value, min, max, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for NumberEntryDialogInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<NumberEntryDialogInRust<OWNED>> for DialogInRust<OWNED> {
    fn from(o: NumberEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogInRust<OWNED>> for TopLevelWindowInRust<OWNED> {
    fn from(o: NumberEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogInRust<OWNED>> for NonOwnedWindowInRust<OWNED> {
    fn from(o: NumberEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: NumberEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: NumberEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: NumberEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NumberEntryDialogInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxNumberEntryDialog_CLASSINFO()) }
    }
}

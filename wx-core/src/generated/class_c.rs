use super::*;

// wxCalculateLayoutEvent
wxwidgets! {
    /// This event is sent by wxLayoutAlgorithm to calculate the amount of the remaining client area that the window should occupy.
    /// - [`CalculateLayoutEvent`] represents a C++ `wxCalculateLayoutEvent` class instance which your code has ownership, [`CalculateLayoutEventInRust`]`<false>` represents one which don't own.
    /// - Use [`CalculateLayoutEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCalculateLayoutEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_calculate_layout_event.html) for more details.
    #[doc(alias = "wxCalculateLayoutEvent")]
    #[doc(alias = "CalculateLayoutEvent")]
    class CalculateLayoutEvent
        = CalculateLayoutEventInRust<true>(wxCalculateLayoutEvent) impl
        CalculateLayoutEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CalculateLayoutEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxCalculateLayoutEvent::wxCalculateLayoutEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calculate_layout_event.html#a132da85194408f1ee9e57929a63e4af0).
    pub fn new(id: c_int) -> CalculateLayoutEventInRust<OWNED> {
        unsafe { CalculateLayoutEventInRust(ffi::wxCalculateLayoutEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CalculateLayoutEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CalculateLayoutEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: CalculateLayoutEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalculateLayoutEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CalculateLayoutEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CalculateLayoutEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCalculateLayoutEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CalculateLayoutEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCalendarCtrl
wxwidgets! {
    /// The calendar control allows the user to pick a date.
    /// - [`CalendarCtrl`] represents a C++ `wxCalendarCtrl` class instance which your code has ownership, [`CalendarCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`CalendarCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCalendarCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html) for more details.
    #[doc(alias = "wxCalendarCtrl")]
    #[doc(alias = "CalendarCtrl")]
    class CalendarCtrl
        = CalendarCtrlInRust<true>(wxCalendarCtrl) impl
        CalendarCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CalendarCtrlInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxCalendarCtrl::wxCalendarCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#ab25f66cdb24c24f0e21ccb4761a5aff1).
    pub fn new_2step() -> CalendarCtrlInRust<OWNED> {
        unsafe { CalendarCtrlInRust(ffi::wxCalendarCtrl_new()) }
    }
    /// Does the same as Create() method.
    ///
    /// See [C++ `wxCalendarCtrl::wxCalendarCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#aaa594799b72c7b7fbbd80346d202f582).
    pub fn new<W: WindowMethods, D: DateTimeMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        date: &D,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> CalendarCtrlInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let date = date.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CalendarCtrlInRust(ffi::wxCalendarCtrl_new1(
                parent, id, date, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for CalendarCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CalendarCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: CalendarCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: CalendarCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: CalendarCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CalendarCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CalendarCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCalendarCtrl_CLASSINFO()) }
    }
}

// wxCalendarDateAttr
wxwidgets! {
    /// wxCalendarDateAttr is a custom attributes for a calendar date.
    /// - [`CalendarDateAttr`] represents a C++ `wxCalendarDateAttr` class instance which your code has ownership, [`CalendarDateAttrInRust`]`<false>` represents one which don't own.
    /// - Use [`CalendarDateAttr`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCalendarDateAttr` class's documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html) for more details.
    #[doc(alias = "wxCalendarDateAttr")]
    #[doc(alias = "CalendarDateAttr")]
    class CalendarDateAttr
        = CalendarDateAttrInRust<true>(wxCalendarDateAttr) impl
        CalendarDateAttrMethods
}
impl<const OWNED: bool> CalendarDateAttrInRust<OWNED> {
    // NOT_SUPPORTED: fn wxCalendarDateAttr()
    // NOT_SUPPORTED: fn wxCalendarDateAttr1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CalendarDateAttrInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for CalendarDateAttrInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCalendarDateAttr_delete(self.0) }
        }
    }
}

// wxCalendarEvent
wxwidgets! {
    /// The wxCalendarEvent class is used together with wxCalendarCtrl.
    /// - [`CalendarEvent`] represents a C++ `wxCalendarEvent` class instance which your code has ownership, [`CalendarEventInRust`]`<false>` represents one which don't own.
    /// - Use [`CalendarEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCalendarEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_event.html) for more details.
    #[doc(alias = "wxCalendarEvent")]
    #[doc(alias = "CalendarEvent")]
    class CalendarEvent
        = CalendarEventInRust<true>(wxCalendarEvent) impl
        CalendarEventMethods,
        DateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CalendarEventInRust<OWNED> {
    ///
    /// See [C++ `wxCalendarEvent::wxCalendarEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_event.html#af4042d0201f1f5a593ec298b908187f8).
    pub fn new() -> CalendarEventInRust<OWNED> {
        unsafe { CalendarEventInRust(ffi::wxCalendarEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxCalendarEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CalendarEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CalendarEventInRust<OWNED>> for DateEventInRust<OWNED> {
    fn from(o: CalendarEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: CalendarEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: CalendarEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CalendarEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CalendarEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCalendarEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CalendarEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCaret
wxwidgets! {
    /// A caret is a blinking cursor showing the position where the typed text will appear.
    /// - [`Caret`] represents a C++ `wxCaret` class instance which your code has ownership, [`CaretInRust`]`<false>` represents one which don't own.
    /// - Use [`Caret`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCaret` class's documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html) for more details.
    #[doc(alias = "wxCaret")]
    #[doc(alias = "Caret")]
    class Caret
        = CaretInRust<true>(wxCaret) impl
        CaretMethods
}
impl<const OWNED: bool> CaretInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxCaret::wxCaret()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a07b320d7d5296d378fa8d6350e2ecf22).
    pub fn new() -> CaretInRust<OWNED> {
        unsafe { CaretInRust(ffi::wxCaret_new()) }
    }
    /// Creates a caret with the given size (in pixels) and associates it with the window.
    ///
    /// See [C++ `wxCaret::wxCaret()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a69612cbfbe2bd14a244b9c347db5e142).
    pub fn new_with_window_int<W: WindowMethods>(
        window: Option<&W>,
        width: c_int,
        height: c_int,
    ) -> CaretInRust<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            CaretInRust(ffi::wxCaret_new1(window, width, height))
        }
    }
    ///
    /// See [C++ `wxCaret::wxCaret()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a60b70ae60ae73f9e3c86bfb08c628e64).
    pub fn new_with_window_size<W: WindowMethods, S: SizeMethods>(
        window: Option<&W>,
        size: &S,
    ) -> CaretInRust<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            CaretInRust(ffi::wxCaret_new2(window, size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CaretInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for CaretInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCaret_delete(self.0) }
        }
    }
}

// wxCheckBox
wxwidgets! {
    /// A checkbox is a labelled box which by default is either on (checkmark is visible) or off (no checkmark).
    /// - [`CheckBox`] represents a C++ `wxCheckBox` class instance which your code has ownership, [`CheckBoxInRust`]`<false>` represents one which don't own.
    /// - Use [`CheckBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCheckBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html) for more details.
    #[doc(alias = "wxCheckBox")]
    #[doc(alias = "CheckBox")]
    class CheckBox
        = CheckBoxInRust<true>(wxCheckBox) impl
        CheckBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CheckBoxInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxCheckBox::wxCheckBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#a2b239bc0db88dcae8b1dfe2ac3e7f96f).
    pub fn new_2step() -> CheckBoxInRust<OWNED> {
        unsafe { CheckBoxInRust(ffi::wxCheckBox_new()) }
    }
    /// Constructor, creating and showing a checkbox.
    ///
    /// See [C++ `wxCheckBox::wxCheckBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#a5ab183aef8c69afd5ca12de0ce41dbc4).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CheckBoxInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CheckBoxInRust(ffi::wxCheckBox_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for CheckBoxInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CheckBoxInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: CheckBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckBoxInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: CheckBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckBoxInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: CheckBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckBoxInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CheckBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CheckBoxInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCheckBox_CLASSINFO()) }
    }
}

// wxCheckListBox
wxwidgets! {
    /// A wxCheckListBox is like a wxListBox, but allows items to be checked or unchecked.
    /// - [`CheckListBox`] represents a C++ `wxCheckListBox` class instance which your code has ownership, [`CheckListBoxInRust`]`<false>` represents one which don't own.
    /// - Use [`CheckListBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCheckListBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html) for more details.
    #[doc(alias = "wxCheckListBox")]
    #[doc(alias = "CheckListBox")]
    class CheckListBox
        = CheckListBoxInRust<true>(wxCheckListBox) impl
        CheckListBoxMethods,
        // ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CheckListBoxInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxCheckListBox::wxCheckListBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#a7ef1af0cad44ed9b4ab99daa3469e10b).
    pub fn new_2step() -> CheckListBoxInRust<OWNED> {
        unsafe { CheckListBoxInRust(ffi::wxCheckListBox_new()) }
    }
    // NOT_SUPPORTED: fn wxCheckListBox1()
    /// Constructor, creating and showing a list box.
    ///
    /// See [C++ `wxCheckListBox::wxCheckListBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#abe40b7afe1da92affd48c6522f02c762).
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CheckListBoxInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CheckListBoxInRust(ffi::wxCheckListBox_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for CheckListBoxInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CheckListBoxInRust<OWNED>> for ListBoxInRust<OWNED> {
    fn from(o: CheckListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: CheckListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: CheckListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: CheckListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CheckListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CheckListBoxInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCheckListBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxCheckListBox
impl<const OWNED: bool> ItemContainerMethods for CheckListBoxInRust<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxCheckListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for CheckListBoxInRust<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxCheckListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ListBoxMethods for CheckListBoxInRust<OWNED> {
    // NOT_SUPPORTED: fn Create()
    ///
    /// See [C++ `wxCheckListBox::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#a1ff3c075762c1be1321c5d6e9a71bd1e).
    fn create_arraystring<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
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
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCheckListBox_Create1(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
}

// wxChildFocusEvent
wxwidgets! {
    /// A child focus event is sent to a (parent-)window when one of its child windows gains focus, so that the window could restore the focus back to its corresponding child if it loses it now and regains later.
    /// - [`ChildFocusEvent`] represents a C++ `wxChildFocusEvent` class instance which your code has ownership, [`ChildFocusEventInRust`]`<false>` represents one which don't own.
    /// - Use [`ChildFocusEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxChildFocusEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_child_focus_event.html) for more details.
    #[doc(alias = "wxChildFocusEvent")]
    #[doc(alias = "ChildFocusEvent")]
    class ChildFocusEvent
        = ChildFocusEventInRust<true>(wxChildFocusEvent) impl
        ChildFocusEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ChildFocusEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxChildFocusEvent::wxChildFocusEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_child_focus_event.html#ad630ddc1fb95a86a74efdbe04e6105b6).
    pub fn new<W: WindowMethods>(win: Option<&W>) -> ChildFocusEventInRust<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ChildFocusEventInRust(ffi::wxChildFocusEvent_new(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ChildFocusEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ChildFocusEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: ChildFocusEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChildFocusEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: ChildFocusEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChildFocusEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ChildFocusEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ChildFocusEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxChildFocusEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ChildFocusEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxChoice
wxwidgets! {
    /// A choice item is used to select one of a list of strings.
    /// - [`Choice`] represents a C++ `wxChoice` class instance which your code has ownership, [`ChoiceInRust`]`<false>` represents one which don't own.
    /// - Use [`Choice`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxChoice` class's documentation](https://docs.wxwidgets.org/3.2/classwx_choice.html) for more details.
    #[doc(alias = "wxChoice")]
    #[doc(alias = "Choice")]
    class Choice
        = ChoiceInRust<true>(wxChoice) impl
        ChoiceMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ChoiceInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxChoice::wxChoice()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choice.html#abe4fa74cf09a21608e90cd55ca96fabd).
    pub fn new_2step() -> ChoiceInRust<OWNED> {
        unsafe { ChoiceInRust(ffi::wxChoice_new()) }
    }
    // NOT_SUPPORTED: fn wxChoice1()
    /// Constructor, creating and showing a choice.
    ///
    /// See [C++ `wxChoice::wxChoice()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choice.html#a2bb4542d8803a4c91de478831f6ed560).
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ChoiceInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ChoiceInRust(ffi::wxChoice_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ChoiceInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ChoiceInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ChoiceInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoiceInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ChoiceInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoiceInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ChoiceInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoiceInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ChoiceInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ChoiceInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxChoice_CLASSINFO()) }
    }
}
// Mix-in(s) to wxChoice
impl<const OWNED: bool> ItemContainerMethods for ChoiceInRust<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxChoice_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for ChoiceInRust<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxChoice_AsItemContainer(self.as_ptr()) }
    }
}

// wxChoicebook
wxwidgets! {
    /// wxChoicebook is a class similar to wxNotebook, but uses a wxChoice control to show the labels instead of the tabs.
    /// - [`Choicebook`] represents a C++ `wxChoicebook` class instance which your code has ownership, [`ChoicebookInRust`]`<false>` represents one which don't own.
    /// - Use [`Choicebook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxChoicebook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_choicebook.html) for more details.
    #[doc(alias = "wxChoicebook")]
    #[doc(alias = "Choicebook")]
    class Choicebook
        = ChoicebookInRust<true>(wxChoicebook) impl
        ChoicebookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ChoicebookInRust<OWNED> {
    /// Constructs a choicebook control.
    ///
    /// See [C++ `wxChoicebook::wxChoicebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choicebook.html#aa06ad50928a63e7067e9e496e4fbcf08).
    pub fn new_2step() -> ChoicebookInRust<OWNED> {
        unsafe { ChoicebookInRust(ffi::wxChoicebook_new()) }
    }
    ///
    /// See [C++ `wxChoicebook::wxChoicebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choicebook.html#ab4054d2f57aebdc7d6991798c4a3376f).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ChoicebookInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ChoicebookInRust(ffi::wxChoicebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ChoicebookInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ChoicebookInRust<OWNED>> for BookCtrlBaseInRust<OWNED> {
    fn from(o: ChoicebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ChoicebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ChoicebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ChoicebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ChoicebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ChoicebookInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxChoicebook_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for ChoicebookInRust<OWNED> {
    /// Create the choicebook control that has already been constructed with the default constructor.
    ///
    /// See [C++ `wxChoicebook::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choicebook.html#a101f60164662715ef3a95a9ce3709037).
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
            ffi::wxChoicebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxClientDC
wxwidgets! {
    /// wxClientDC is primarily useful for obtaining information about the window from outside EVT_PAINT() handler.
    /// - [`ClientDC`] represents a C++ `wxClientDC` class instance which your code has ownership, [`ClientDCInRust`]`<false>` represents one which don't own.
    /// - Use [`ClientDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxClientDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_client_d_c.html) for more details.
    #[doc(alias = "wxClientDC")]
    #[doc(alias = "ClientDC")]
    class ClientDC
        = ClientDCInRust<true>(wxClientDC) impl
        ClientDCMethods,
        WindowDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> ClientDCInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxClientDC::wxClientDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_client_d_c.html#ac2deeb91c3d7f8dd755e6f6166159501).
    pub fn new<W: WindowMethods>(window: Option<&W>) -> ClientDCInRust<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ClientDCInRust(ffi::wxClientDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClientDCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ClientDCInRust<OWNED>> for WindowDCInRust<OWNED> {
    fn from(o: ClientDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClientDCInRust<OWNED>> for DCInRust<OWNED> {
    fn from(o: ClientDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClientDCInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ClientDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ClientDCInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxClientDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ClientDCInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxClipboard
wxwidgets! {
    /// A class for manipulating the clipboard.
    /// - [`Clipboard`] represents a C++ `wxClipboard` class instance which your code has ownership, [`ClipboardInRust`]`<false>` represents one which don't own.
    /// - Use [`Clipboard`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxClipboard` class's documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html) for more details.
    #[doc(alias = "wxClipboard")]
    #[doc(alias = "Clipboard")]
    class Clipboard
        = ClipboardInRust<true>(wxClipboard) impl
        ClipboardMethods,
        ObjectMethods
}
impl<const OWNED: bool> ClipboardInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxClipboard::wxClipboard()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#a8d61a457ae71f52f718e0225ba3e8bb4).
    pub fn new() -> ClipboardInRust<OWNED> {
        unsafe { ClipboardInRust(ffi::wxClipboard_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClipboardInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ClipboardInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ClipboardInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ClipboardInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxClipboard_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ClipboardInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxClipboardTextEvent
wxwidgets! {
    /// This class represents the events generated by a control (typically a wxTextCtrl but other windows can generate these events as well) when its content gets copied or cut to, or pasted from the clipboard.
    /// - [`ClipboardTextEvent`] represents a C++ `wxClipboardTextEvent` class instance which your code has ownership, [`ClipboardTextEventInRust`]`<false>` represents one which don't own.
    /// - Use [`ClipboardTextEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxClipboardTextEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard_text_event.html) for more details.
    #[doc(alias = "wxClipboardTextEvent")]
    #[doc(alias = "ClipboardTextEvent")]
    class ClipboardTextEvent
        = ClipboardTextEventInRust<true>(wxClipboardTextEvent) impl
        ClipboardTextEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ClipboardTextEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxClipboardTextEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClipboardTextEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ClipboardTextEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: ClipboardTextEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClipboardTextEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: ClipboardTextEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClipboardTextEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ClipboardTextEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ClipboardTextEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxClipboardTextEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ClipboardTextEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCloseEvent
wxwidgets! {
    /// This event class contains information about window and session close events.
    /// - [`CloseEvent`] represents a C++ `wxCloseEvent` class instance which your code has ownership, [`CloseEventInRust`]`<false>` represents one which don't own.
    /// - Use [`CloseEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCloseEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_close_event.html) for more details.
    #[doc(alias = "wxCloseEvent")]
    #[doc(alias = "CloseEvent")]
    class CloseEvent
        = CloseEventInRust<true>(wxCloseEvent) impl
        CloseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CloseEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxCloseEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CloseEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CloseEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: CloseEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CloseEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CloseEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CloseEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCloseEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CloseEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCollapsiblePane
wxwidgets! {
    /// A collapsible pane is a container with an embedded button-like control which can be used by the user to collapse or expand the pane's contents.
    /// - [`CollapsiblePane`] represents a C++ `wxCollapsiblePane` class instance which your code has ownership, [`CollapsiblePaneInRust`]`<false>` represents one which don't own.
    /// - Use [`CollapsiblePane`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCollapsiblePane` class's documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html) for more details.
    #[doc(alias = "wxCollapsiblePane")]
    #[doc(alias = "CollapsiblePane")]
    class CollapsiblePane
        = CollapsiblePaneInRust<true>(wxCollapsiblePane) impl
        CollapsiblePaneMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CollapsiblePaneInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxCollapsiblePane::wxCollapsiblePane()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#afe29ca6c7b230a05d63022c3adb7348a).
    pub fn new_2step() -> CollapsiblePaneInRust<OWNED> {
        unsafe { CollapsiblePaneInRust(ffi::wxCollapsiblePane_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// See [C++ `wxCollapsiblePane::wxCollapsiblePane()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#a920561522d4b28c08b8d693047c3aa38).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CollapsiblePaneInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CollapsiblePaneInRust(ffi::wxCollapsiblePane_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for CollapsiblePaneInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CollapsiblePaneInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: CollapsiblePaneInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: CollapsiblePaneInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: CollapsiblePaneInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CollapsiblePaneInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CollapsiblePaneInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCollapsiblePane_CLASSINFO()) }
    }
}

// wxCollapsiblePaneEvent
wxwidgets! {
    /// This event class is used for the events generated by wxCollapsiblePane.
    /// - [`CollapsiblePaneEvent`] represents a C++ `wxCollapsiblePaneEvent` class instance which your code has ownership, [`CollapsiblePaneEventInRust`]`<false>` represents one which don't own.
    /// - Use [`CollapsiblePaneEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCollapsiblePaneEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane_event.html) for more details.
    #[doc(alias = "wxCollapsiblePaneEvent")]
    #[doc(alias = "CollapsiblePaneEvent")]
    class CollapsiblePaneEvent
        = CollapsiblePaneEventInRust<true>(wxCollapsiblePaneEvent) impl
        CollapsiblePaneEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CollapsiblePaneEventInRust<OWNED> {
    /// The constructor is not normally used by the user code.
    ///
    /// See [C++ `wxCollapsiblePaneEvent::wxCollapsiblePaneEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane_event.html#a4b2a4c27e9908f892be38971b0ddf555).
    pub fn new<O: ObjectMethods>(
        generator: Option<&O>,
        id: c_int,
        collapsed: bool,
    ) -> CollapsiblePaneEventInRust<OWNED> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            CollapsiblePaneEventInRust(ffi::wxCollapsiblePaneEvent_new(generator, id, collapsed))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CollapsiblePaneEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CollapsiblePaneEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: CollapsiblePaneEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: CollapsiblePaneEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CollapsiblePaneEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CollapsiblePaneEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCollapsiblePaneEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CollapsiblePaneEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColour
wxwidgets! {
    /// A colour is an object representing a combination of Red, Green, and Blue (RGB) intensity values and an Alpha value, and is used to determine drawing colours.
    /// - [`Colour`] represents a C++ `wxColour` class instance which your code has ownership, [`ColourInRust`]`<false>` represents one which don't own.
    /// - Use [`Colour`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColour` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html) for more details.
    #[doc(alias = "wxColour")]
    #[doc(alias = "Colour")]
    class Colour
        = ColourInRust<true>(wxColour) impl
        ColourMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxColour::wxColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#aac96e7922132d672a1f83d59ecf07343).
    pub fn new() -> ColourInRust<OWNED> {
        unsafe { ColourInRust(ffi::wxColour_new()) }
    }
    // NOT_SUPPORTED: fn wxColour1()
    ///
    /// See [C++ `wxColour::wxColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#ae114ff88fe3a07477d111baa01c1b325).
    pub fn new_with_str(colour_name: &str) -> ColourInRust<OWNED> {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            ColourInRust(ffi::wxColour_new2(colour_name))
        }
    }
    // NOT_SUPPORTED: fn wxColour3()
    /// Copy constructor.
    ///
    /// See [C++ `wxColour::wxColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a0fbc51432a57424a0538d0932af6bf78).
    pub fn new_with_colour<C: ColourMethods>(colour: &C) -> ColourInRust<OWNED> {
        unsafe {
            let colour = colour.as_ptr();
            ColourInRust(ffi::wxColour_new4(colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ColourInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ColourInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxColour_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ColourInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColourData
wxwidgets! {
    /// This class holds a variety of information related to colour dialogs.
    /// - [`ColourData`] represents a C++ `wxColourData` class instance which your code has ownership, [`ColourDataInRust`]`<false>` represents one which don't own.
    /// - Use [`ColourData`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColourData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html) for more details.
    #[doc(alias = "wxColourData")]
    #[doc(alias = "ColourData")]
    class ColourData
        = ColourDataInRust<true>(wxColourData) impl
        ColourDataMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourDataInRust<OWNED> {
    //  ENUM: @6
    pub const NUM_CUSTOM: c_int = 16;

    /// Constructor.
    ///
    /// See [C++ `wxColourData::wxColourData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#a970e44b3f6ee89a4f4621bd76eb3738c).
    pub fn new() -> ColourDataInRust<OWNED> {
        unsafe { ColourDataInRust(ffi::wxColourData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourDataInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ColourDataInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ColourDataInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourDataInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxColourData_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ColourDataInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColourDatabase
wxwidgets! {
    /// wxWidgets maintains a database of standard RGB colours for a predefined set of named colours.
    /// - [`ColourDatabase`] represents a C++ `wxColourDatabase` class instance which your code has ownership, [`ColourDatabaseInRust`]`<false>` represents one which don't own.
    /// - Use [`ColourDatabase`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColourDatabase` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour_database.html) for more details.
    #[doc(alias = "wxColourDatabase")]
    #[doc(alias = "ColourDatabase")]
    class ColourDatabase
        = ColourDatabaseInRust<true>(wxColourDatabase) impl
        ColourDatabaseMethods
}
impl<const OWNED: bool> ColourDatabaseInRust<OWNED> {
    /// Constructs the colour database.
    ///
    /// See [C++ `wxColourDatabase::wxColourDatabase()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_database.html#a66269122cb725da348b3a1c396002e41).
    pub fn new() -> ColourDatabaseInRust<OWNED> {
        unsafe { ColourDatabaseInRust(ffi::wxColourDatabase_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourDatabaseInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for ColourDatabaseInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxColourDatabase_delete(self.0) }
        }
    }
}

// wxColourDialog
wxwidgets! {
    /// This class represents the colour chooser dialog.
    /// - [`ColourDialog`] represents a C++ `wxColourDialog` class instance which your code has ownership, [`ColourDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`ColourDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColourDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour_dialog.html) for more details.
    #[doc(alias = "wxColourDialog")]
    #[doc(alias = "ColourDialog")]
    class ColourDialog
        = ColourDialogInRust<true>(wxColourDialog) impl
        ColourDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourDialogInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxColourDialog::wxColourDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_dialog.html#a357b4f19cd3757c6f74c44cd49a1d90e).
    pub fn new<W: WindowMethods, C: ColourDataMethods>(
        parent: Option<&W>,
        data: Option<&C>,
    ) -> ColourDialogInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ColourDialogInRust(ffi::wxColourDialog_new(parent, data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ColourDialogInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ColourDialogInRust<OWNED>> for DialogInRust<OWNED> {
    fn from(o: ColourDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogInRust<OWNED>> for TopLevelWindowInRust<OWNED> {
    fn from(o: ColourDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogInRust<OWNED>> for NonOwnedWindowInRust<OWNED> {
    fn from(o: ColourDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ColourDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ColourDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ColourDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourDialogInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxColourDialog_CLASSINFO()) }
    }
}

// wxColourPickerCtrl
wxwidgets! {
    /// This control allows the user to select a colour.
    /// - [`ColourPickerCtrl`] represents a C++ `wxColourPickerCtrl` class instance which your code has ownership, [`ColourPickerCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`ColourPickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColourPickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html) for more details.
    #[doc(alias = "wxColourPickerCtrl")]
    #[doc(alias = "ColourPickerCtrl")]
    class ColourPickerCtrl
        = ColourPickerCtrlInRust<true>(wxColourPickerCtrl) impl
        ColourPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourPickerCtrlInRust<OWNED> {
    ///
    /// See [C++ `wxColourPickerCtrl::wxColourPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html#a610182eb11286c6df2fbd195b98868cd).
    pub fn new_2step() -> ColourPickerCtrlInRust<OWNED> {
        unsafe { ColourPickerCtrlInRust(ffi::wxColourPickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// See [C++ `wxColourPickerCtrl::wxColourPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html#a3763437a33dd948c3ae0d7a2210b7c0e).
    pub fn new<
        W: WindowMethods,
        C: ColourMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        colour: &C,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ColourPickerCtrlInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let colour = colour.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ColourPickerCtrlInRust(ffi::wxColourPickerCtrl_new1(
                parent, id, colour, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ColourPickerCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlInRust<OWNED>> for PickerBaseInRust<OWNED> {
    fn from(o: ColourPickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ColourPickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ColourPickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ColourPickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ColourPickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourPickerCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxColourPickerCtrl_CLASSINFO()) }
    }
}

// wxColourPickerEvent
wxwidgets! {
    /// This event class is used for the events generated by wxColourPickerCtrl.
    /// - [`ColourPickerEvent`] represents a C++ `wxColourPickerEvent` class instance which your code has ownership, [`ColourPickerEventInRust`]`<false>` represents one which don't own.
    /// - Use [`ColourPickerEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColourPickerEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html) for more details.
    #[doc(alias = "wxColourPickerEvent")]
    #[doc(alias = "ColourPickerEvent")]
    class ColourPickerEvent
        = ColourPickerEventInRust<true>(wxColourPickerEvent) impl
        ColourPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourPickerEventInRust<OWNED> {
    ///
    /// See [C++ `wxColourPickerEvent::wxColourPickerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html#afa8edf53f69e85cbaa6a1f2e91c22f2e).
    pub fn new() -> ColourPickerEventInRust<OWNED> {
        unsafe { ColourPickerEventInRust(ffi::wxColourPickerEvent_new()) }
    }
    /// The constructor is not normally used by the user code.
    ///
    /// See [C++ `wxColourPickerEvent::wxColourPickerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html#ab9d9e7ef4204c0dc7511eddf4fc4eb0c).
    pub fn new_with_object<O: ObjectMethods, C: ColourMethods>(
        generator: Option<&O>,
        id: c_int,
        colour: &C,
    ) -> ColourPickerEventInRust<OWNED> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let colour = colour.as_ptr();
            ColourPickerEventInRust(ffi::wxColourPickerEvent_new1(generator, id, colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourPickerEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ColourPickerEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: ColourPickerEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: ColourPickerEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ColourPickerEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourPickerEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxColourPickerEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ColourPickerEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxComboBox
wxwidgets! {
    /// A combobox is like a combination of an edit control and a listbox.
    /// - [`ComboBox`] represents a C++ `wxComboBox` class instance which your code has ownership, [`ComboBoxInRust`]`<false>` represents one which don't own.
    /// - Use [`ComboBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxComboBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html) for more details.
    #[doc(alias = "wxComboBox")]
    #[doc(alias = "ComboBox")]
    class ComboBox
        = ComboBoxInRust<true>(wxComboBox) impl
        ComboBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ComboBoxInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxComboBox::wxComboBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#aac1f4d94c40a0a675dd5b2c72376057a).
    pub fn new_2step() -> ComboBoxInRust<OWNED> {
        unsafe { ComboBoxInRust(ffi::wxComboBox_new()) }
    }
    // NOT_SUPPORTED: fn wxComboBox1()
    /// Constructor, creating and showing a combobox.
    ///
    /// See [C++ `wxComboBox::wxComboBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#ade11ed7d2b64bd1f50cdb34f162e120f).
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ComboBoxInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ComboBoxInRust(ffi::wxComboBox_new2(
                parent, id, value, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ComboBoxInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ComboBoxInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboBoxInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboBoxInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboBoxInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ComboBoxInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxComboBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxComboBox
impl<const OWNED: bool> ItemContainerMethods for ComboBoxInRust<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for ComboBoxInRust<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TextEntryMethods for ComboBoxInRust<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsTextEntry(self.as_ptr()) }
    }
}

// wxComboCtrl
wxwidgets! {
    /// A combo control is a generic combobox that allows totally custom popup.
    /// - [`ComboCtrl`] represents a C++ `wxComboCtrl` class instance which your code has ownership, [`ComboCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`ComboCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxComboCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html) for more details.
    #[doc(alias = "wxComboCtrl")]
    #[doc(alias = "ComboCtrl")]
    class ComboCtrl
        = ComboCtrlInRust<true>(wxComboCtrl) impl
        ComboCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ComboCtrlInRust<OWNED> {
    //  ENUM: @9
    pub const ShowBelow: c_int = 0x0000;
    pub const ShowAbove: c_int = 0x0001;
    pub const CanDeferShow: c_int = 0x0002;

    /// Default constructor.
    ///
    /// See [C++ `wxComboCtrl::wxComboCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#af31b505e73047689dc4d2948eaa13a3d).
    pub fn new_2step() -> ComboCtrlInRust<OWNED> {
        unsafe { ComboCtrlInRust(ffi::wxComboCtrl_new()) }
    }
    /// Constructor, creating and showing a combo control.
    ///
    /// See [C++ `wxComboCtrl::wxComboCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a0f9d6b21d3728f135dbe0d01ee4bf865).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ComboCtrlInRust<OWNED> {
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
            ComboCtrlInRust(ffi::wxComboCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ComboCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ComboCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ComboCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ComboCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ComboCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ComboCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ComboCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxComboCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxComboCtrl
impl<const OWNED: bool> TextEntryMethods for ComboCtrlInRust<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxComboCtrl_AsTextEntry(self.as_ptr()) }
    }
}

// wxComboPopup
wxwidgets! {
    /// In order to use a custom popup with wxComboCtrl, an interface class must be derived from wxComboPopup.
    /// - [`ComboPopup`] represents a C++ `wxComboPopup` class instance which your code has ownership, [`ComboPopupInRust`]`<false>` represents one which don't own.
    /// - Use [`ComboPopup`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxComboPopup` class's documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html) for more details.
    #[doc(alias = "wxComboPopup")]
    #[doc(alias = "ComboPopup")]
    class ComboPopup
        = ComboPopupInRust<true>(wxComboPopup) impl
        ComboPopupMethods
}
impl<const OWNED: bool> ComboPopupInRust<OWNED> {
    // BLOCKED: fn wxComboPopup()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ComboPopupInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for ComboPopupInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxComboPopup_delete(self.0) }
        }
    }
}

// wxCommand
wxwidgets! {
    /// wxCommand is a base class for modelling an application command, which is an action usually performed by selecting a menu item, pressing a toolbar button or any other means provided by the application to change the data or view.
    /// - [`Command`] represents a C++ `wxCommand` class instance which your code has ownership, [`CommandInRust`]`<false>` represents one which don't own.
    /// - Use [`Command`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCommand` class's documentation](https://docs.wxwidgets.org/3.2/classwx_command.html) for more details.
    #[doc(alias = "wxCommand")]
    #[doc(alias = "Command")]
    class Command
        = CommandInRust<true>(wxCommand) impl
        CommandMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandInRust<OWNED> {
    // BLOCKED: fn wxCommand()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CommandInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CommandInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CommandInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCommand_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CommandInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCommandEvent
wxwidgets! {
    /// This event class contains information about command events, which originate from a variety of simple controls.
    /// - [`CommandEvent`] represents a C++ `wxCommandEvent` class instance which your code has ownership, [`CommandEventInRust`]`<false>` represents one which don't own.
    /// - Use [`CommandEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCommandEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html) for more details.
    #[doc(alias = "wxCommandEvent")]
    #[doc(alias = "CommandEvent")]
    class CommandEvent
        = CommandEventInRust<true>(wxCommandEvent) impl
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxCommandEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CommandEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CommandEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: CommandEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CommandEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCommandEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CommandEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCommandLinkButton
wxwidgets! {
    /// Objects of this class are similar in appearance to the normal wxButtons but are similar to the links in a web page in functionality.
    /// - [`CommandLinkButton`] represents a C++ `wxCommandLinkButton` class instance which your code has ownership, [`CommandLinkButtonInRust`]`<false>` represents one which don't own.
    /// - Use [`CommandLinkButton`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCommandLinkButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html) for more details.
    #[doc(alias = "wxCommandLinkButton")]
    #[doc(alias = "CommandLinkButton")]
    class CommandLinkButton
        = CommandLinkButtonInRust<true>(wxCommandLinkButton) impl
        CommandLinkButtonMethods,
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandLinkButtonInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxCommandLinkButton::wxCommandLinkButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#a05be41a7e1fd702cee7662e1a6ae9293).
    pub fn new_2step() -> CommandLinkButtonInRust<OWNED> {
        unsafe { CommandLinkButtonInRust(ffi::wxCommandLinkButton_new()) }
    }
    /// Constructor really creating a command Link button.
    ///
    /// See [C++ `wxCommandLinkButton::wxCommandLinkButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#a193413ad8afa7895bdb0ef133a454234).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        main_label: &str,
        note: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CommandLinkButtonInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            let note = WxString::from(note);
            let note = note.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CommandLinkButtonInRust(ffi::wxCommandLinkButton_new1(
                parent, id, main_label, note, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for CommandLinkButtonInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CommandLinkButtonInRust<OWNED>> for ButtonInRust<OWNED> {
    fn from(o: CommandLinkButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonInRust<OWNED>> for AnyButtonInRust<OWNED> {
    fn from(o: CommandLinkButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: CommandLinkButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: CommandLinkButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: CommandLinkButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CommandLinkButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandLinkButtonInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCommandLinkButton_CLASSINFO()) }
    }
}

// wxCommandProcessor
wxwidgets! {
    /// wxCommandProcessor is a class that maintains a history of wxCommands, with undo/redo functionality built-in.
    /// - [`CommandProcessor`] represents a C++ `wxCommandProcessor` class instance which your code has ownership, [`CommandProcessorInRust`]`<false>` represents one which don't own.
    /// - Use [`CommandProcessor`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCommandProcessor` class's documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html) for more details.
    #[doc(alias = "wxCommandProcessor")]
    #[doc(alias = "CommandProcessor")]
    class CommandProcessor
        = CommandProcessorInRust<true>(wxCommandProcessor) impl
        CommandProcessorMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandProcessorInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxCommandProcessor::wxCommandProcessor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a8f0b55885b6ea95037e81bbe76e28d74).
    pub fn new(max_commands: c_int) -> CommandProcessorInRust<OWNED> {
        unsafe { CommandProcessorInRust(ffi::wxCommandProcessor_new(max_commands)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CommandProcessorInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CommandProcessorInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CommandProcessorInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandProcessorInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCommandProcessor_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CommandProcessorInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxContextMenuEvent
wxwidgets! {
    /// This class is used for context menu events, sent to give the application a chance to show a context (popup) menu for a wxWindow.
    /// - [`ContextMenuEvent`] represents a C++ `wxContextMenuEvent` class instance which your code has ownership, [`ContextMenuEventInRust`]`<false>` represents one which don't own.
    /// - Use [`ContextMenuEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxContextMenuEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_context_menu_event.html) for more details.
    #[doc(alias = "wxContextMenuEvent")]
    #[doc(alias = "ContextMenuEvent")]
    class ContextMenuEvent
        = ContextMenuEventInRust<true>(wxContextMenuEvent) impl
        ContextMenuEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ContextMenuEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxContextMenuEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ContextMenuEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ContextMenuEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: ContextMenuEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextMenuEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: ContextMenuEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextMenuEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ContextMenuEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ContextMenuEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxContextMenuEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ContextMenuEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxControl
wxwidgets! {
    /// This is the base class for a control or "widget".
    /// - [`Control`] represents a C++ `wxControl` class instance which your code has ownership, [`ControlInRust`]`<false>` represents one which don't own.
    /// - Use [`Control`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxControl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_control.html) for more details.
    #[doc(alias = "wxControl")]
    #[doc(alias = "Control")]
    class Control
        = ControlInRust<true>(wxControl) impl
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ControlInRust<OWNED> {
    /// Constructs a control.
    ///
    /// See [C++ `wxControl::wxControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#adb8f3edf807efa9159de826bf92d6a44).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ControlInRust<OWNED> {
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
            ControlInRust(ffi::wxControl_new(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    /// Default constructor to allow 2-phase creation.
    ///
    /// See [C++ `wxControl::wxControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#a08428de2ba5cc988a86fe17071d49522).
    pub fn new_2step() -> ControlInRust<OWNED> {
        unsafe { ControlInRust(ffi::wxControl_new1()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ControlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ControlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ControlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ControlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ControlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ControlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxControl_CLASSINFO()) }
    }
}

// wxControlWithItems
wxwidgets! {
    /// This is convenience class that derives from both wxControl and wxItemContainer.
    /// - [`ControlWithItems`] represents a C++ `wxControlWithItems` class instance which your code has ownership, [`ControlWithItemsInRust`]`<false>` represents one which don't own.
    /// - Use [`ControlWithItems`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxControlWithItems` class's documentation](https://docs.wxwidgets.org/3.2/classwx_control_with_items.html) for more details.
    #[doc(alias = "wxControlWithItems")]
    #[doc(alias = "ControlWithItems")]
    class ControlWithItems
        = ControlWithItemsInRust<true>(wxControlWithItems) impl
        ControlWithItemsMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ControlWithItemsInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ControlWithItemsInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ControlWithItemsInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ControlWithItemsInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlWithItemsInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ControlWithItemsInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlWithItemsInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ControlWithItemsInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlWithItemsInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ControlWithItemsInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ControlWithItemsInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxControlWithItems_CLASSINFO()) }
    }
}
// Mix-in(s) to wxControlWithItems
impl<const OWNED: bool> ItemContainerMethods for ControlWithItemsInRust<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxControlWithItems_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for ControlWithItemsInRust<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxControlWithItems_AsItemContainer(self.as_ptr()) }
    }
}

// wxCursor
wxwidgets! {
    /// A cursor is a small bitmap usually used for denoting where the mouse pointer is, with a picture that might indicate the interpretation of a mouse click.
    /// - [`Cursor`] represents a C++ `wxCursor` class instance which your code has ownership, [`CursorInRust`]`<false>` represents one which don't own.
    /// - Use [`Cursor`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCursor` class's documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html) for more details.
    #[doc(alias = "wxCursor")]
    #[doc(alias = "Cursor")]
    class Cursor
        = CursorInRust<true>(wxCursor) impl
        CursorMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> CursorInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxCursor::wxCursor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html#a21735e923430faf941c440c67f13859d).
    pub fn new() -> CursorInRust<OWNED> {
        unsafe { CursorInRust(ffi::wxCursor_new()) }
    }
    // NOT_SUPPORTED: fn wxCursor1()
    // NOT_SUPPORTED: fn wxCursor2()
    // NOT_SUPPORTED: fn wxCursor3()
    /// Constructs a cursor from a wxImage.
    ///
    /// See [C++ `wxCursor::wxCursor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html#adaf6e394c7306a3999bdc314d7b307a7).
    pub fn new_with_image<I: ImageMethods>(image: &I) -> CursorInRust<OWNED> {
        unsafe {
            let image = image.as_ptr();
            CursorInRust(ffi::wxCursor_new4(image))
        }
    }
    /// Constructs a cursor from XPM data.
    ///
    /// See [C++ `wxCursor::wxCursor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html#abb43549ff688898c79f48112ff85f7d7).
    pub fn new_with_char(xpm_data: *const c_void) -> CursorInRust<OWNED> {
        unsafe { CursorInRust(ffi::wxCursor_new5(xpm_data)) }
    }
    /// Copy constructor, uses reference counting.
    ///
    /// See [C++ `wxCursor::wxCursor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html#a20f3123d9d012427ae1f614e10b9cfb9).
    pub fn new_with_cursor<C: CursorMethods>(cursor: &C) -> CursorInRust<OWNED> {
        unsafe {
            let cursor = cursor.as_ptr();
            CursorInRust(ffi::wxCursor_new6(cursor))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CursorInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CursorInRust<OWNED>> for GDIObjectInRust<OWNED> {
    fn from(o: CursorInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CursorInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: CursorInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CursorInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxCursor_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CursorInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCustomDataObject
wxwidgets! {
    /// wxCustomDataObject is a specialization of wxDataObjectSimple for some application-specific data in arbitrary (either custom or one of the standard ones).
    /// - [`CustomDataObject`] represents a C++ `wxCustomDataObject` class instance which your code has ownership, [`CustomDataObjectInRust`]`<false>` represents one which don't own.
    /// - Use [`CustomDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCustomDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html) for more details.
    #[doc(alias = "wxCustomDataObject")]
    #[doc(alias = "CustomDataObject")]
    class CustomDataObject
        = CustomDataObjectInRust<true>(wxCustomDataObject) impl
        CustomDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> CustomDataObjectInRust<OWNED> {
    /// The constructor accepts a format argument which specifies the (single) format supported by this object.
    ///
    /// See [C++ `wxCustomDataObject::wxCustomDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html#a667ea5eae3e91095d79cb6fe9e548695).
    pub fn new<D: DataFormatMethods>(format: &D) -> CustomDataObjectInRust<OWNED> {
        unsafe {
            let format = format.as_ptr();
            CustomDataObjectInRust(ffi::wxCustomDataObject_new(format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CustomDataObjectInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CustomDataObjectInRust<OWNED>> for DataObjectSimpleInRust<OWNED> {
    fn from(o: CustomDataObjectInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CustomDataObjectInRust<OWNED>> for DataObjectInRust<OWNED> {
    fn from(o: CustomDataObjectInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for CustomDataObjectInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCustomDataObject_delete(self.0) }
        }
    }
}

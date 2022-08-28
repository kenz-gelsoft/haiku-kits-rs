use super::*;

// wxFileCtrl
wxwidgets! {
    /// This control allows the user to select a file.
    ///
    /// [See `wxFileCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html)
    #[doc(alias = "wxFileCtrl")]
    #[doc(alias = "FileCtrl")]
    class FileCtrl
        = FileCtrlIsOwned<true>(wxFileCtrl) impl
        FileCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileCtrlIsOwned<OWNED> {
    ///
    /// [See `wxFileCtrl::wxFileCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#ad0f3935004f16b380571ead2d1b4c04c)
    pub fn new_2step() -> FileCtrlIsOwned<OWNED> {
        unsafe { FileCtrlIsOwned(ffi::wxFileCtrl_new()) }
    }
    /// Constructs the window.
    ///
    /// [See `wxFileCtrl::wxFileCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#a3a4611ef0c3e9087e3768869c53ab5cc)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        default_directory: &str,
        default_filename: &str,
        wild_card: &str,
        style: c_long,
        pos: &P,
        size: &S,
        name: &str,
    ) -> FileCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let default_directory = WxString::from(default_directory);
            let default_directory = default_directory.as_ptr();
            let default_filename = WxString::from(default_filename);
            let default_filename = default_filename.as_ptr();
            let wild_card = WxString::from(wild_card);
            let wild_card = wild_card.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FileCtrlIsOwned(ffi::wxFileCtrl_new1(
                parent,
                id,
                default_directory,
                default_filename,
                wild_card,
                style,
                pos,
                size,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for FileCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FileCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: FileCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FileCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FileCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileCtrl_CLASSINFO()) }
    }
}

// wxFileCtrlEvent
wxwidgets! {
    /// A file control event holds information about events associated with wxFileCtrl objects.
    ///
    /// [See `wxFileCtrlEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_ctrl_event.html)
    #[doc(alias = "wxFileCtrlEvent")]
    #[doc(alias = "FileCtrlEvent")]
    class FileCtrlEvent
        = FileCtrlEventIsOwned<true>(wxFileCtrlEvent) impl
        FileCtrlEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileCtrlEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxFileCtrlEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileCtrlEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FileCtrlEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: FileCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileCtrlEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FileCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileCtrlEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileCtrlEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileCtrlEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FileCtrlEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileDataObject
wxwidgets! {
    /// wxFileDataObject is a specialization of wxDataObject for file names.
    ///
    /// [See `wxFileDataObject`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_data_object.html)
    #[doc(alias = "wxFileDataObject")]
    #[doc(alias = "FileDataObject")]
    class FileDataObject
        = FileDataObjectIsOwned<true>(wxFileDataObject) impl
        FileDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> FileDataObjectIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxFileDataObject::wxFileDataObject()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_data_object.html#a7c7cb54a6cf9114de7dec67755ac749e)
    pub fn new() -> FileDataObjectIsOwned<OWNED> {
        unsafe { FileDataObjectIsOwned(ffi::wxFileDataObject_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileDataObjectIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FileDataObjectIsOwned<OWNED>> for DataObjectSimpleIsOwned<OWNED> {
    fn from(o: FileDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: FileDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for FileDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFileDataObject_delete(self.0) }
        }
    }
}

// wxFileDialog
wxwidgets! {
    /// This class represents the file chooser dialog.
    ///
    /// [See `wxFileDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html)
    #[doc(alias = "wxFileDialog")]
    #[doc(alias = "FileDialog")]
    class FileDialog
        = FileDialogIsOwned<true>(wxFileDialog) impl
        FileDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileDialogIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxFileDialog::wxFileDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#af3ff2981229bd2f892df0fa96fb9265d)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        message: &str,
        default_dir: &str,
        default_file: &str,
        wildcard: &str,
        style: c_long,
        pos: &P,
        size: &S,
        name: &str,
    ) -> FileDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let default_dir = WxString::from(default_dir);
            let default_dir = default_dir.as_ptr();
            let default_file = WxString::from(default_file);
            let default_file = default_file.as_ptr();
            let wildcard = WxString::from(wildcard);
            let wildcard = wildcard.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FileDialogIsOwned(ffi::wxFileDialog_new(
                parent,
                message,
                default_dir,
                default_file,
                wildcard,
                style,
                pos,
                size,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for FileDialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileDialog_CLASSINFO()) }
    }
}

// wxFileDirPickerEvent
wxwidgets! {
    /// This event class is used for the events generated by wxFilePickerCtrl and by wxDirPickerCtrl.
    ///
    /// [See `wxFileDirPickerEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_dir_picker_event.html)
    #[doc(alias = "wxFileDirPickerEvent")]
    #[doc(alias = "FileDirPickerEvent")]
    class FileDirPickerEvent
        = FileDirPickerEventIsOwned<true>(wxFileDirPickerEvent) impl
        FileDirPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileDirPickerEventIsOwned<OWNED> {
    ///
    /// [See `wxFileDirPickerEvent::wxFileDirPickerEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_dir_picker_event.html#a311f6e70d669ca9ef6f4425c7778f215)
    pub fn new() -> FileDirPickerEventIsOwned<OWNED> {
        unsafe { FileDirPickerEventIsOwned(ffi::wxFileDirPickerEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxFileDirPickerEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileDirPickerEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FileDirPickerEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: FileDirPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDirPickerEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FileDirPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDirPickerEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileDirPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileDirPickerEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileDirPickerEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FileDirPickerEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileDropTarget
wxwidgets! {
    /// This is a drop target which accepts files (dragged from File Manager or Explorer).
    ///
    /// [See `wxFileDropTarget`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_drop_target.html)
    #[doc(alias = "wxFileDropTarget")]
    #[doc(alias = "FileDropTarget")]
    class FileDropTarget
        = FileDropTargetIsOwned<true>(wxFileDropTarget) impl
        FileDropTargetMethods,
        DropTargetMethods
}
impl<const OWNED: bool> FileDropTargetIsOwned<OWNED> {
    // BLOCKED: fn wxFileDropTarget()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileDropTargetIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FileDropTargetIsOwned<OWNED>> for DropTargetIsOwned<OWNED> {
    fn from(o: FileDropTargetIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for FileDropTargetIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFileDropTarget_delete(self.0) }
        }
    }
}

// wxFileHistory
wxwidgets! {
    /// The wxFileHistory encapsulates a user interface convenience, the list of most recently visited files as shown on a menu (usually the File menu).
    ///
    /// [See `wxFileHistory`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_history.html)
    #[doc(alias = "wxFileHistory")]
    #[doc(alias = "FileHistory")]
    class FileHistory
        = FileHistoryIsOwned<true>(wxFileHistory) impl
        FileHistoryMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileHistoryIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxFileHistory::wxFileHistory()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_history.html#a30e3a3a1f92fc253cc0fc69eb6f27fd8)
    pub fn new(max_files: usize, id_base: c_int) -> FileHistoryIsOwned<OWNED> {
        unsafe { FileHistoryIsOwned(ffi::wxFileHistory_new(max_files, id_base)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileHistoryIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FileHistoryIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileHistoryIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileHistoryIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileHistory_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FileHistoryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFilePickerCtrl
wxwidgets! {
    /// This control allows the user to select a file.
    ///
    /// [See `wxFilePickerCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html)
    #[doc(alias = "wxFilePickerCtrl")]
    #[doc(alias = "FilePickerCtrl")]
    class FilePickerCtrl
        = FilePickerCtrlIsOwned<true>(wxFilePickerCtrl) impl
        FilePickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FilePickerCtrlIsOwned<OWNED> {
    ///
    /// [See `wxFilePickerCtrl::wxFilePickerCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html#afe16cc740765fb8ec68c9df250a07812)
    pub fn new_2step() -> FilePickerCtrlIsOwned<OWNED> {
        unsafe { FilePickerCtrlIsOwned(ffi::wxFilePickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// [See `wxFilePickerCtrl::wxFilePickerCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html#a2f9d8631a622d88cf820719a33879a4f)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        path: &str,
        message: &str,
        wildcard: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> FilePickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let path = WxString::from(path);
            let path = path.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let wildcard = WxString::from(wildcard);
            let wildcard = wildcard.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FilePickerCtrlIsOwned(ffi::wxFilePickerCtrl_new1(
                parent, id, path, message, wildcard, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for FilePickerCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FilePickerCtrlIsOwned<OWNED>> for PickerBaseIsOwned<OWNED> {
    fn from(o: FilePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FilePickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: FilePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FilePickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FilePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FilePickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FilePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FilePickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FilePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FilePickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFilePickerCtrl_CLASSINFO()) }
    }
}

// wxFindDialogEvent
wxwidgets! {
    /// wxFindReplaceDialog events.
    ///
    /// [See `wxFindDialogEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_find_dialog_event.html)
    #[doc(alias = "wxFindDialogEvent")]
    #[doc(alias = "FindDialogEvent")]
    class FindDialogEvent
        = FindDialogEventIsOwned<true>(wxFindDialogEvent) impl
        FindDialogEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FindDialogEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxFindDialogEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FindDialogEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FindDialogEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: FindDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindDialogEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FindDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindDialogEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FindDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FindDialogEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFindDialogEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FindDialogEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFindReplaceData
wxwidgets! {
    /// wxFindReplaceData holds the data for wxFindReplaceDialog.
    ///
    /// [See `wxFindReplaceData`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_find_replace_data.html)
    #[doc(alias = "wxFindReplaceData")]
    #[doc(alias = "FindReplaceData")]
    class FindReplaceData
        = FindReplaceDataIsOwned<true>(wxFindReplaceData) impl
        FindReplaceDataMethods,
        ObjectMethods
}
impl<const OWNED: bool> FindReplaceDataIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxFindReplaceData()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FindReplaceDataIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FindReplaceDataIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FindReplaceDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FindReplaceDataIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFindReplaceData_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FindReplaceDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFindReplaceDialog
wxwidgets! {
    /// wxFindReplaceDialog is a standard modeless dialog which is used to allow the user to search for some text (and possibly replace it with something else).
    ///
    /// [See `wxFindReplaceDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html)
    #[doc(alias = "wxFindReplaceDialog")]
    #[doc(alias = "FindReplaceDialog")]
    class FindReplaceDialog
        = FindReplaceDialogIsOwned<true>(wxFindReplaceDialog) impl
        FindReplaceDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FindReplaceDialogIsOwned<OWNED> {
    ///
    /// [See `wxFindReplaceDialog::wxFindReplaceDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html#a10601867d5793096323ce0979f7993cd)
    pub fn new_2step() -> FindReplaceDialogIsOwned<OWNED> {
        unsafe { FindReplaceDialogIsOwned(ffi::wxFindReplaceDialog_new()) }
    }
    /// After using default constructor Create() must be called.
    ///
    /// [See `wxFindReplaceDialog::wxFindReplaceDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html#a27c6a7da82dd935ec7a9812ea6bf73c3)
    pub fn new<W: WindowMethods, F: FindReplaceDataMethods>(
        parent: Option<&W>,
        data: Option<&F>,
        title: &str,
        style: c_int,
    ) -> FindReplaceDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            FindReplaceDialogIsOwned(ffi::wxFindReplaceDialog_new1(parent, data, title, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for FindReplaceDialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FindReplaceDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFindReplaceDialog_CLASSINFO()) }
    }
}

// wxFlexGridSizer
wxwidgets! {
    /// A flex grid sizer is a sizer which lays out its children in a two-dimensional table with all table fields in one row having the same height and all fields in one column having the same width, but all rows or all columns are not necessarily the same height or width as in the wxGridSizer.
    ///
    /// [See `wxFlexGridSizer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html)
    #[doc(alias = "wxFlexGridSizer")]
    #[doc(alias = "FlexGridSizer")]
    class FlexGridSizer
        = FlexGridSizerIsOwned<true>(wxFlexGridSizer) impl
        FlexGridSizerMethods,
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FlexGridSizerIsOwned<OWNED> {
    /// wxFlexGridSizer constructors.
    ///
    /// [See `wxFlexGridSizer::wxFlexGridSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a1b4a8cb492c6d89e3e3755c9b8d31f03)
    pub fn new_with_int_int(cols: c_int, vgap: c_int, hgap: c_int) -> FlexGridSizerIsOwned<OWNED> {
        unsafe { FlexGridSizerIsOwned(ffi::wxFlexGridSizer_new(cols, vgap, hgap)) }
    }
    ///
    /// [See `wxFlexGridSizer::wxFlexGridSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a4c2fef6d9eca9c1d3ee3ee0ef41a4307)
    pub fn new_with_size<S: SizeMethods>(cols: c_int, gap: &S) -> FlexGridSizerIsOwned<OWNED> {
        unsafe {
            let gap = gap.as_ptr();
            FlexGridSizerIsOwned(ffi::wxFlexGridSizer_new1(cols, gap))
        }
    }
    ///
    /// [See `wxFlexGridSizer::wxFlexGridSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a12b3b94cf1fe8ea687c74c84b8eb892f)
    pub fn new_with_int_int_int(
        rows: c_int,
        cols: c_int,
        vgap: c_int,
        hgap: c_int,
    ) -> FlexGridSizerIsOwned<OWNED> {
        unsafe { FlexGridSizerIsOwned(ffi::wxFlexGridSizer_new2(rows, cols, vgap, hgap)) }
    }
    ///
    /// [See `wxFlexGridSizer::wxFlexGridSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a49c35e0580fec338a47c5f0f348515e7)
    pub fn new_with_int_size<S: SizeMethods>(
        rows: c_int,
        cols: c_int,
        gap: &S,
    ) -> FlexGridSizerIsOwned<OWNED> {
        unsafe {
            let gap = gap.as_ptr();
            FlexGridSizerIsOwned(ffi::wxFlexGridSizer_new3(rows, cols, gap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for FlexGridSizerIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FlexGridSizerIsOwned<OWNED>> for GridSizerIsOwned<OWNED> {
    fn from(o: FlexGridSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FlexGridSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: FlexGridSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FlexGridSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FlexGridSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FlexGridSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFlexGridSizer_CLASSINFO()) }
    }
}

// wxFocusEvent
wxwidgets! {
    /// A focus event is sent when a window's focus changes.
    ///
    /// [See `wxFocusEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_focus_event.html)
    #[doc(alias = "wxFocusEvent")]
    #[doc(alias = "FocusEvent")]
    class FocusEvent
        = FocusEventIsOwned<true>(wxFocusEvent) impl
        FocusEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FocusEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxFocusEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FocusEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FocusEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FocusEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FocusEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FocusEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FocusEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFocusEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FocusEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFont
wxwidgets! {
    /// A font is an object which determines the appearance of text.
    ///
    /// [See `wxFont`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font.html)
    #[doc(alias = "wxFont")]
    #[doc(alias = "Font")]
    class Font
        = FontIsOwned<true>(wxFont) impl
        FontMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontIsOwned<OWNED> {
    /// Default ctor.
    ///
    /// [See `wxFont::wxFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font.html#af183c2709f2b8f03e8fe88f28826612c)
    pub fn new() -> FontIsOwned<OWNED> {
        unsafe { FontIsOwned(ffi::wxFont_new()) }
    }
    /// Copy constructor, uses reference counting.
    ///
    /// [See `wxFont::wxFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font.html#ad77ff719ea7cf27f32d1de7e14dc76c1)
    pub fn new_with_font<F: FontMethods>(font: &F) -> FontIsOwned<OWNED> {
        unsafe {
            let font = font.as_ptr();
            FontIsOwned(ffi::wxFont_new1(font))
        }
    }
    /// Creates a font object using the specified font description.
    ///
    /// [See `wxFont::wxFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font.html#a019f22fdd833bf6bfc16f7b795a68a45)
    pub fn new_with_fontinfo(font_info: *const c_void) -> FontIsOwned<OWNED> {
        unsafe { FontIsOwned(ffi::wxFont_new2(font_info)) }
    }
    // NOT_SUPPORTED: fn wxFont3()
    // NOT_SUPPORTED: fn wxFont4()
    /// Constructor from font description string.
    ///
    /// [See `wxFont::wxFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font.html#a9d43775aaeafc628064b0e1b63730567)
    pub fn new_with_str(native_info_string: &str) -> FontIsOwned<OWNED> {
        unsafe {
            let native_info_string = WxString::from(native_info_string);
            let native_info_string = native_info_string.as_ptr();
            FontIsOwned(ffi::wxFont_new5(native_info_string))
        }
    }
    /// Construct font from a native font info structure.
    ///
    /// [See `wxFont::wxFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font.html#a17d85e1cac77bf812182915f3a017976)
    pub fn new_with_nativefontinfo<N: NativeFontInfoMethods>(
        native_info: &N,
    ) -> FontIsOwned<OWNED> {
        unsafe {
            let native_info = native_info.as_ptr();
            FontIsOwned(ffi::wxFont_new6(native_info))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FontIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: FontIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FontIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FontIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFont_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FontIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFontData
wxwidgets! {
    /// This class holds a variety of information related to font dialogs.
    ///
    /// [See `wxFontData`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_data.html)
    #[doc(alias = "wxFontData")]
    #[doc(alias = "FontData")]
    class FontData
        = FontDataIsOwned<true>(wxFontData) impl
        FontDataMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontDataIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxFontData::wxFontData()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a7475bad933f864387b91f41ec26efd44)
    pub fn new() -> FontDataIsOwned<OWNED> {
        unsafe { FontDataIsOwned(ffi::wxFontData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontDataIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FontDataIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FontDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FontDataIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFontData_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FontDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFontDialog
wxwidgets! {
    /// This class represents the font chooser dialog.
    ///
    /// [See `wxFontDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html)
    #[doc(alias = "wxFontDialog")]
    #[doc(alias = "FontDialog")]
    class FontDialog
        = FontDialogIsOwned<true>(wxFontDialog) impl
        FontDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontDialogIsOwned<OWNED> {
    /// Default ctor.
    ///
    /// [See `wxFontDialog::wxFontDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html#acf0f377d6df63ea86e72df3972d5c1c2)
    pub fn new_2step() -> FontDialogIsOwned<OWNED> {
        unsafe { FontDialogIsOwned(ffi::wxFontDialog_new()) }
    }
    // BLOCKED: fn wxFontDialog1()
    /// Constructor.
    ///
    /// [See `wxFontDialog::wxFontDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html#a1dafd790fcdea244ceca846c6e62ab2e)
    pub fn new<W: WindowMethods, F: FontDataMethods>(
        parent: Option<&W>,
        data: &F,
    ) -> FontDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = data.as_ptr();
            FontDialogIsOwned(ffi::wxFontDialog_new2(parent, data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for FontDialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FontDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFontDialog_CLASSINFO()) }
    }
}

// wxFontEnumerator
wxwidgets! {
    /// wxFontEnumerator enumerates either all available fonts on the system or only the ones with given attributes - either only fixed-width (suited for use in programs such as terminal emulators and the like) or the fonts available in the given encoding).
    ///
    /// [See `wxFontEnumerator`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html)
    #[doc(alias = "wxFontEnumerator")]
    #[doc(alias = "FontEnumerator")]
    class FontEnumerator
        = FontEnumeratorIsOwned<true>(wxFontEnumerator) impl
        FontEnumeratorMethods
}
impl<const OWNED: bool> FontEnumeratorIsOwned<OWNED> {
    ///
    /// [See `wxFontEnumerator::wxFontEnumerator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html#a4ea35d567889f590df8ea37d48b3bc98)
    pub fn new() -> FontEnumeratorIsOwned<OWNED> {
        unsafe { FontEnumeratorIsOwned(ffi::wxFontEnumerator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontEnumeratorIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for FontEnumeratorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFontEnumerator_delete(self.0) }
        }
    }
}

// wxFontList
wxwidgets! {
    /// A font list is a list containing all fonts which have been created.
    ///
    /// [See `wxFontList`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_list.html)
    #[doc(alias = "wxFontList")]
    #[doc(alias = "FontList")]
    class FontList
        = FontListIsOwned<true>(wxFontList) impl
        FontListMethods
}
impl<const OWNED: bool> FontListIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxFontList::wxFontList()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_list.html#a1ca7f23958bc81f12893d1602b3a037d)
    pub fn new() -> FontListIsOwned<OWNED> {
        unsafe { FontListIsOwned(ffi::wxFontList_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontListIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for FontListIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFontList_delete(self.0) }
        }
    }
}

// wxFontMapper
wxwidgets! {
    /// wxFontMapper manages user-definable correspondence between logical font names and the fonts present on the machine.
    ///
    /// [See `wxFontMapper`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html)
    #[doc(alias = "wxFontMapper")]
    #[doc(alias = "FontMapper")]
    class FontMapper
        = FontMapperIsOwned<true>(wxFontMapper) impl
        FontMapperMethods
}
impl<const OWNED: bool> FontMapperIsOwned<OWNED> {
    /// Default ctor.
    ///
    /// [See `wxFontMapper::wxFontMapper()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html#a9eb3ae0550d0c858ba994a7d7a020441)
    pub fn new() -> FontMapperIsOwned<OWNED> {
        unsafe { FontMapperIsOwned(ffi::wxFontMapper_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontMapperIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for FontMapperIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFontMapper_delete(self.0) }
        }
    }
}

// wxFontPickerCtrl
wxwidgets! {
    /// This control allows the user to select a font.
    ///
    /// [See `wxFontPickerCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html)
    #[doc(alias = "wxFontPickerCtrl")]
    #[doc(alias = "FontPickerCtrl")]
    class FontPickerCtrl
        = FontPickerCtrlIsOwned<true>(wxFontPickerCtrl) impl
        FontPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontPickerCtrlIsOwned<OWNED> {
    ///
    /// [See `wxFontPickerCtrl::wxFontPickerCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#a391e4a0d0935941152501cce7d492fb6)
    pub fn new_2step() -> FontPickerCtrlIsOwned<OWNED> {
        unsafe { FontPickerCtrlIsOwned(ffi::wxFontPickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// [See `wxFontPickerCtrl::wxFontPickerCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#a840bd203045d1be1249cef9348839951)
    pub fn new<
        W: WindowMethods,
        F: FontMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        font: &F,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> FontPickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let font = font.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FontPickerCtrlIsOwned(ffi::wxFontPickerCtrl_new1(
                parent, id, font, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for FontPickerCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FontPickerCtrlIsOwned<OWNED>> for PickerBaseIsOwned<OWNED> {
    fn from(o: FontPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: FontPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FontPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FontPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FontPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FontPickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFontPickerCtrl_CLASSINFO()) }
    }
}

// wxFontPickerEvent
wxwidgets! {
    /// This event class is used for the events generated by wxFontPickerCtrl.
    ///
    /// [See `wxFontPickerEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_picker_event.html)
    #[doc(alias = "wxFontPickerEvent")]
    #[doc(alias = "FontPickerEvent")]
    class FontPickerEvent
        = FontPickerEventIsOwned<true>(wxFontPickerEvent) impl
        FontPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontPickerEventIsOwned<OWNED> {
    /// The constructor is not normally used by the user code.
    ///
    /// [See `wxFontPickerEvent::wxFontPickerEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_font_picker_event.html#a59e707304d36f12937605e1bff5df798)
    pub fn new<O: ObjectMethods, F: FontMethods>(
        generator: Option<&O>,
        id: c_int,
        font: &F,
    ) -> FontPickerEventIsOwned<OWNED> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let font = font.as_ptr();
            FontPickerEventIsOwned(ffi::wxFontPickerEvent_new(generator, id, font))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontPickerEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FontPickerEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: FontPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FontPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FontPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FontPickerEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFontPickerEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FontPickerEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFrame
wxwidgets! {
    /// A frame is a window whose size and position can (usually) be changed by the user.
    ///
    /// [See `wxFrame`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_frame.html)
    #[doc(alias = "wxFrame")]
    #[doc(alias = "Frame")]
    class Frame
        = FrameIsOwned<true>(wxFrame) impl
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FrameIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxFrame::wxFrame()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_frame.html#af80368ba23c71c5d947c3178b8fe10fc)
    pub fn new_2step() -> FrameIsOwned<OWNED> {
        unsafe { FrameIsOwned(ffi::wxFrame_new()) }
    }
    /// Constructor, creating the window.
    ///
    /// [See `wxFrame::wxFrame()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_frame.html#a01b53ac2d4a5e6b0773ecbcf7b5f6af8)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> FrameIsOwned<OWNED> {
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
            FrameIsOwned(ffi::wxFrame_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for FrameIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<FrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: FrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: FrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFrame_CLASSINFO()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for FrameIsOwned<OWNED> {
    /// Used in two-step frame construction.
    ///
    /// [See `wxFrame::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_frame.html#a6541d1aab71fc90041bfdde6e8705add)
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
            ffi::wxFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for FrameIsOwned<OWNED> {
    /// Centres the frame on the display.
    ///
    /// [See `wxFrame::Centre()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_frame.html#a39b18ed552aabaf2a1bc4af7cc924a0f)
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxFrame_Centre(self.as_ptr(), direction) }
    }
}

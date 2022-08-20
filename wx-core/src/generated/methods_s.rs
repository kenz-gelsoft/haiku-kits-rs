use super::*;

// wxSearchCtrl
pub trait SearchCtrlMethods: TextCtrlMethods {
    // DTOR: fn ~wxSearchCtrl()
    fn get_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxSearchCtrl_GetMenu(self.as_ptr())) }
    }
    fn is_search_button_visible(&self) -> bool {
        unsafe { ffi::wxSearchCtrl_IsSearchButtonVisible(self.as_ptr()) }
    }
    fn is_cancel_button_visible(&self) -> bool {
        unsafe { ffi::wxSearchCtrl_IsCancelButtonVisible(self.as_ptr()) }
    }
    fn set_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSearchCtrl_SetMenu(self.as_ptr(), menu)
        }
    }
    fn show_cancel_button(&self, show: bool) {
        unsafe { ffi::wxSearchCtrl_ShowCancelButton(self.as_ptr(), show) }
    }
    fn show_search_button(&self, show: bool) {
        unsafe { ffi::wxSearchCtrl_ShowSearchButton(self.as_ptr(), show) }
    }
    fn set_descriptive_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxSearchCtrl_SetDescriptiveText(self.as_ptr(), text)
        }
    }
    fn get_descriptive_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxSearchCtrl_GetDescriptiveText(self.as_ptr())).into() }
    }
}

// wxSettableHeaderColumn
pub trait SettableHeaderColumnMethods: HeaderColumnMethods {
    fn set_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxSettableHeaderColumn_SetTitle(self.as_ptr(), title)
        }
    }
    fn set_bitmap<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxSettableHeaderColumn_SetBitmap(self.as_ptr(), bitmap)
        }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_SetWidth(self.as_ptr(), width) }
    }
    fn set_min_width(&self, min_width: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_SetMinWidth(self.as_ptr(), min_width) }
    }
    fn set_alignment(&self, align: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_SetAlignment(self.as_ptr(), align) }
    }
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_SetFlags(self.as_ptr(), flags) }
    }
    fn change_flag(&self, flag: c_int, set: bool) {
        unsafe { ffi::wxSettableHeaderColumn_ChangeFlag(self.as_ptr(), flag, set) }
    }
    fn set_flag(&self, flag: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_SetFlag(self.as_ptr(), flag) }
    }
    fn clear_flag(&self, flag: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_ClearFlag(self.as_ptr(), flag) }
    }
    fn toggle_flag(&self, flag: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_ToggleFlag(self.as_ptr(), flag) }
    }
    fn set_resizeable(&self, resizable: bool) {
        unsafe { ffi::wxSettableHeaderColumn_SetResizeable(self.as_ptr(), resizable) }
    }
    fn set_sortable(&self, sortable: bool) {
        unsafe { ffi::wxSettableHeaderColumn_SetSortable(self.as_ptr(), sortable) }
    }
    fn set_reorderable(&self, reorderable: bool) {
        unsafe { ffi::wxSettableHeaderColumn_SetReorderable(self.as_ptr(), reorderable) }
    }
    fn set_hidden(&self, hidden: bool) {
        unsafe { ffi::wxSettableHeaderColumn_SetHidden(self.as_ptr(), hidden) }
    }
    fn unset_as_sort_key(&self) {
        unsafe { ffi::wxSettableHeaderColumn_UnsetAsSortKey(self.as_ptr()) }
    }
    fn set_sort_order(&self, ascending: bool) {
        unsafe { ffi::wxSettableHeaderColumn_SetSortOrder(self.as_ptr(), ascending) }
    }
    fn toggle_sort_order(&self) {
        unsafe { ffi::wxSettableHeaderColumn_ToggleSortOrder(self.as_ptr()) }
    }
}

// wxSize
pub trait SizeMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator/()
    // BLOCKED: fn operator/1()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*1()
    // BLOCKED: fn operator*2()
    // BLOCKED: fn operator*3()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator/=1()
    // BLOCKED: fn operator*=()
    // BLOCKED: fn operator*=1()
    fn dec_by_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxSize_DecBy(self.as_ptr(), pt)
        }
    }
    fn dec_by_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecBy1(self.as_ptr(), size)
        }
    }
    fn dec_by_int_int(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxSize_DecBy2(self.as_ptr(), dx, dy) }
    }
    fn dec_by_int(&self, d: c_int) {
        unsafe { ffi::wxSize_DecBy3(self.as_ptr(), d) }
    }
    fn dec_to<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecTo(self.as_ptr(), size)
        }
    }
    fn dec_to_if_specified<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecToIfSpecified(self.as_ptr(), size)
        }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxSize_GetHeight(self.as_ptr()) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxSize_GetWidth(self.as_ptr()) }
    }
    fn inc_by_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxSize_IncBy(self.as_ptr(), pt)
        }
    }
    fn inc_by_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_IncBy1(self.as_ptr(), size)
        }
    }
    fn inc_by_int_int(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxSize_IncBy2(self.as_ptr(), dx, dy) }
    }
    fn inc_by_int(&self, d: c_int) {
        unsafe { ffi::wxSize_IncBy3(self.as_ptr(), d) }
    }
    fn inc_to<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_IncTo(self.as_ptr(), size)
        }
    }
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxSize_IsFullySpecified(self.as_ptr()) }
    }
    // BLOCKED: fn Scale()
    fn set(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxSize_Set(self.as_ptr(), width, height) }
    }
    fn set_defaults<S: SizeMethods>(&self, size_default: &S) {
        unsafe {
            let size_default = size_default.as_ptr();
            ffi::wxSize_SetDefaults(self.as_ptr(), size_default)
        }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxSize_SetHeight(self.as_ptr(), height) }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxSize_SetWidth(self.as_ptr(), width) }
    }
}

// wxSizer
pub trait SizerMethods: ObjectMethods {
    // DTOR: fn ~wxSizer()
    fn add_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        &self,
        window: Option<&W>,
        flags: &S,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItem::option_from(ffi::wxSizer_Add(self.as_ptr(), window, flags))
        }
    }
    fn add_window_int<W: WindowMethods, O: ObjectMethods>(
        &self,
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Add1(
                self.as_ptr(),
                window,
                proportion,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn add_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        &self,
        sizer: Option<&S>,
        flags: &S2,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItem::option_from(ffi::wxSizer_Add2(self.as_ptr(), sizer, flags))
        }
    }
    fn add_sizer_int<S: SizerMethods, O: ObjectMethods>(
        &self,
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Add3(
                self.as_ptr(),
                sizer,
                proportion,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn add_int_int<O: ObjectMethods>(
        &self,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Add4(
                self.as_ptr(),
                width,
                height,
                proportion,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn add_int_sizerflags<S: SizerFlagsMethods>(
        &self,
        width: c_int,
        height: c_int,
        flags: &S,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let flags = flags.as_ptr();
            SizerItem::option_from(ffi::wxSizer_Add5(self.as_ptr(), width, height, flags))
        }
    }
    fn add_sizeritem<S: SizerItemMethods>(
        &self,
        item: Option<&S>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Add6(self.as_ptr(), item))
        }
    }
    fn add_spacer(&self, size: c_int) -> Option<SizerItemIsOwned<false>> {
        unsafe { SizerItem::option_from(ffi::wxSizer_AddSpacer(self.as_ptr(), size)) }
    }
    fn add_stretch_spacer(&self, prop: c_int) -> Option<SizerItemIsOwned<false>> {
        unsafe { SizerItem::option_from(ffi::wxSizer_AddStretchSpacer(self.as_ptr(), prop)) }
    }
    fn calc_min(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxSizer_CalcMin(self.as_ptr())) }
    }
    fn clear(&self, delete_windows: bool) {
        unsafe { ffi::wxSizer_Clear(self.as_ptr(), delete_windows) }
    }
    fn compute_fitting_client_size<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxSizer_ComputeFittingClientSize(self.as_ptr(), window))
        }
    }
    fn compute_fitting_window_size<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxSizer_ComputeFittingWindowSize(self.as_ptr(), window))
        }
    }
    fn detach_window<W: WindowMethods>(&self, window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Detach(self.as_ptr(), window)
        }
    }
    fn detach_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Detach1(self.as_ptr(), sizer)
        }
    }
    fn detach_int(&self, index: c_int) -> bool {
        unsafe { ffi::wxSizer_Detach2(self.as_ptr(), index) }
    }
    fn fit<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxSizer_Fit(self.as_ptr(), window))
        }
    }
    fn fit_inside<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_FitInside(self.as_ptr(), window)
        }
    }
    fn inform_first_direction(
        &self,
        direction: c_int,
        size: c_int,
        available_other_dir: c_int,
    ) -> bool {
        unsafe {
            ffi::wxSizer_InformFirstDirection(self.as_ptr(), direction, size, available_other_dir)
        }
    }
    fn get_children(&self) -> SizerItemListIsOwned<false> {
        unsafe { SizerItemListIsOwned::from_ptr(ffi::wxSizer_GetChildren(self.as_ptr())) }
    }
    // BLOCKED: fn GetChildren1()
    fn get_containing_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxSizer_GetContainingWindow(self.as_ptr())) }
    }
    fn set_containing_window<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetContainingWindow(self.as_ptr(), window)
        }
    }
    fn get_item_count(&self) -> usize {
        unsafe { ffi::wxSizer_GetItemCount(self.as_ptr()) }
    }
    fn get_item_window<W: WindowMethods>(
        &self,
        window: Option<&W>,
        recursive: bool,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_GetItem(self.as_ptr(), window, recursive))
        }
    }
    fn get_item_sizer<S: SizerMethods>(
        &self,
        sizer: Option<&S>,
        recursive: bool,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_GetItem1(self.as_ptr(), sizer, recursive))
        }
    }
    fn get_item_sz(&self, index: usize) -> Option<SizerItemIsOwned<false>> {
        unsafe { SizerItem::option_from(ffi::wxSizer_GetItem2(self.as_ptr(), index)) }
    }
    fn get_item_by_id(&self, id: c_int, recursive: bool) -> Option<SizerItemIsOwned<false>> {
        unsafe { SizerItem::option_from(ffi::wxSizer_GetItemById(self.as_ptr(), id, recursive)) }
    }
    fn get_min_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxSizer_GetMinSize(self.as_ptr())) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxSizer_GetPosition(self.as_ptr())) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxSizer_GetSize(self.as_ptr())) }
    }
    fn hide_window<W: WindowMethods>(&self, window: Option<&W>, recursive: bool) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Hide(self.as_ptr(), window, recursive)
        }
    }
    fn hide_sizer<S: SizerMethods>(&self, sizer: Option<&S>, recursive: bool) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Hide1(self.as_ptr(), sizer, recursive)
        }
    }
    fn hide_sz(&self, index: usize) -> bool {
        unsafe { ffi::wxSizer_Hide2(self.as_ptr(), index) }
    }
    fn insert_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        &self,
        index: usize,
        window: Option<&W>,
        flags: &S,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItem::option_from(ffi::wxSizer_Insert(self.as_ptr(), index, window, flags))
        }
    }
    fn insert_window_int<W: WindowMethods, O: ObjectMethods>(
        &self,
        index: usize,
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Insert1(
                self.as_ptr(),
                index,
                window,
                proportion,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn insert_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        &self,
        index: usize,
        sizer: Option<&S>,
        flags: &S2,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItem::option_from(ffi::wxSizer_Insert2(self.as_ptr(), index, sizer, flags))
        }
    }
    fn insert_sizer_int<S: SizerMethods, O: ObjectMethods>(
        &self,
        index: usize,
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Insert3(
                self.as_ptr(),
                index,
                sizer,
                proportion,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn insert_int_int<O: ObjectMethods>(
        &self,
        index: usize,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Insert4(
                self.as_ptr(),
                index,
                width,
                height,
                proportion,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn insert_int_sizerflags<S: SizerFlagsMethods>(
        &self,
        index: usize,
        width: c_int,
        height: c_int,
        flags: &S,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let flags = flags.as_ptr();
            SizerItem::option_from(ffi::wxSizer_Insert5(
                self.as_ptr(),
                index,
                width,
                height,
                flags,
            ))
        }
    }
    fn insert_sizeritem<S: SizerItemMethods>(
        &self,
        index: usize,
        item: Option<&S>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Insert6(self.as_ptr(), index, item))
        }
    }
    fn insert_spacer(&self, index: usize, size: c_int) -> Option<SizerItemIsOwned<false>> {
        unsafe { SizerItem::option_from(ffi::wxSizer_InsertSpacer(self.as_ptr(), index, size)) }
    }
    fn insert_stretch_spacer(&self, index: usize, prop: c_int) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            SizerItem::option_from(ffi::wxSizer_InsertStretchSpacer(self.as_ptr(), index, prop))
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxSizer_IsEmpty(self.as_ptr()) }
    }
    fn is_shown_window<W: WindowMethods>(&self, window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_IsShown(self.as_ptr(), window)
        }
    }
    fn is_shown_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_IsShown1(self.as_ptr(), sizer)
        }
    }
    fn is_shown_sz(&self, index: usize) -> bool {
        unsafe { ffi::wxSizer_IsShown2(self.as_ptr(), index) }
    }
    fn layout(&self) {
        unsafe { ffi::wxSizer_Layout(self.as_ptr()) }
    }
    fn prepend_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        &self,
        window: Option<&W>,
        flags: &S,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItem::option_from(ffi::wxSizer_Prepend(self.as_ptr(), window, flags))
        }
    }
    fn prepend_window_int<W: WindowMethods, O: ObjectMethods>(
        &self,
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Prepend1(
                self.as_ptr(),
                window,
                proportion,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn prepend_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        &self,
        sizer: Option<&S>,
        flags: &S2,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItem::option_from(ffi::wxSizer_Prepend2(self.as_ptr(), sizer, flags))
        }
    }
    fn prepend_sizer_int<S: SizerMethods, O: ObjectMethods>(
        &self,
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Prepend3(
                self.as_ptr(),
                sizer,
                proportion,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn prepend_int_int<O: ObjectMethods>(
        &self,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Prepend4(
                self.as_ptr(),
                width,
                height,
                proportion,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn prepend_int_sizerflags<S: SizerFlagsMethods>(
        &self,
        width: c_int,
        height: c_int,
        flags: &S,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let flags = flags.as_ptr();
            SizerItem::option_from(ffi::wxSizer_Prepend5(self.as_ptr(), width, height, flags))
        }
    }
    fn prepend_sizeritem<S: SizerItemMethods>(
        &self,
        item: Option<&S>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxSizer_Prepend6(self.as_ptr(), item))
        }
    }
    fn prepend_spacer(&self, size: c_int) -> Option<SizerItemIsOwned<false>> {
        unsafe { SizerItem::option_from(ffi::wxSizer_PrependSpacer(self.as_ptr(), size)) }
    }
    fn prepend_stretch_spacer(&self, prop: c_int) -> Option<SizerItemIsOwned<false>> {
        unsafe { SizerItem::option_from(ffi::wxSizer_PrependStretchSpacer(self.as_ptr(), prop)) }
    }
    fn reposition_children<S: SizeMethods>(&self, min_size: &S) {
        unsafe {
            let min_size = min_size.as_ptr();
            ffi::wxSizer_RepositionChildren(self.as_ptr(), min_size)
        }
    }
    // BLOCKED: fn Remove()
    fn remove_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Remove1(self.as_ptr(), sizer)
        }
    }
    fn remove_int(&self, index: c_int) -> bool {
        unsafe { ffi::wxSizer_Remove2(self.as_ptr(), index) }
    }
    fn replace_window<W: WindowMethods, W2: WindowMethods>(
        &self,
        oldwin: Option<&W>,
        newwin: Option<&W2>,
        recursive: bool,
    ) -> bool {
        unsafe {
            let oldwin = match oldwin {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let newwin = match newwin {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Replace(self.as_ptr(), oldwin, newwin, recursive)
        }
    }
    fn replace_sizer<S: SizerMethods, S2: SizerMethods>(
        &self,
        oldsz: Option<&S>,
        newsz: Option<&S2>,
        recursive: bool,
    ) -> bool {
        unsafe {
            let oldsz = match oldsz {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let newsz = match newsz {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Replace1(self.as_ptr(), oldsz, newsz, recursive)
        }
    }
    fn replace_sz<S: SizerItemMethods>(&self, index: usize, newitem: Option<&S>) -> bool {
        unsafe {
            let newitem = match newitem {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Replace2(self.as_ptr(), index, newitem)
        }
    }
    fn set_dimension_int(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxSizer_SetDimension(self.as_ptr(), x, y, width, height) }
    }
    fn set_dimension_point<P: PointMethods, S: SizeMethods>(&self, pos: &P, size: &S) {
        unsafe {
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            ffi::wxSizer_SetDimension1(self.as_ptr(), pos, size)
        }
    }
    fn set_item_min_size_window_int<W: WindowMethods>(
        &self,
        window: Option<&W>,
        width: c_int,
        height: c_int,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetItemMinSize(self.as_ptr(), window, width, height)
        }
    }
    fn set_item_min_size_window_size<W: WindowMethods, S: SizeMethods>(
        &self,
        window: Option<&W>,
        size: &S,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            ffi::wxSizer_SetItemMinSize1(self.as_ptr(), window, size)
        }
    }
    fn set_item_min_size_sizer_int<S: SizerMethods>(
        &self,
        sizer: Option<&S>,
        width: c_int,
        height: c_int,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetItemMinSize2(self.as_ptr(), sizer, width, height)
        }
    }
    fn set_item_min_size_sizer_size<S: SizerMethods, S2: SizeMethods>(
        &self,
        sizer: Option<&S>,
        size: &S2,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            ffi::wxSizer_SetItemMinSize3(self.as_ptr(), sizer, size)
        }
    }
    fn set_item_min_size_sz_int(&self, index: usize, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxSizer_SetItemMinSize4(self.as_ptr(), index, width, height) }
    }
    fn set_item_min_size_sz_size<S: SizeMethods>(&self, index: usize, size: &S) -> bool {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSizer_SetItemMinSize5(self.as_ptr(), index, size)
        }
    }
    fn set_min_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSizer_SetMinSize(self.as_ptr(), size)
        }
    }
    fn set_min_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxSizer_SetMinSize1(self.as_ptr(), width, height) }
    }
    fn set_size_hints<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetSizeHints(self.as_ptr(), window)
        }
    }
    // BLOCKED: fn SetVirtualSizeHints()
    fn show_window<W: WindowMethods>(
        &self,
        window: Option<&W>,
        show: bool,
        recursive: bool,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Show(self.as_ptr(), window, show, recursive)
        }
    }
    fn show_sizer<S: SizerMethods>(&self, sizer: Option<&S>, show: bool, recursive: bool) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Show1(self.as_ptr(), sizer, show, recursive)
        }
    }
    fn show_sz(&self, index: usize, show: bool) -> bool {
        unsafe { ffi::wxSizer_Show2(self.as_ptr(), index, show) }
    }
    fn show_items(&self, show: bool) {
        unsafe { ffi::wxSizer_ShowItems(self.as_ptr(), show) }
    }
}

// wxSizerFlags
pub trait SizerFlagsMethods: WxRustMethods {
    fn align(&self, alignment: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Align(self.as_ptr(), alignment);
            &self
        }
    }
    fn border_int(&self, direction: c_int, borderinpixels: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Border(self.as_ptr(), direction, borderinpixels);
            &self
        }
    }
    fn border(&self, direction: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Border1(self.as_ptr(), direction);
            &self
        }
    }
    fn bottom(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Bottom(self.as_ptr());
            &self
        }
    }
    fn center(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Center(self.as_ptr());
            &self
        }
    }
    fn centre(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Centre(self.as_ptr());
            &self
        }
    }
    fn center_horizontal(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CenterHorizontal(self.as_ptr());
            &self
        }
    }
    fn center_vertical(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CenterVertical(self.as_ptr());
            &self
        }
    }
    fn centre_horizontal(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CentreHorizontal(self.as_ptr());
            &self
        }
    }
    fn centre_vertical(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CentreVertical(self.as_ptr());
            &self
        }
    }
    fn double_border(&self, direction: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_DoubleBorder(self.as_ptr(), direction);
            &self
        }
    }
    fn double_horz_border(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_DoubleHorzBorder(self.as_ptr());
            &self
        }
    }
    fn expand(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Expand(self.as_ptr());
            &self
        }
    }
    fn fixed_min_size(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_FixedMinSize(self.as_ptr());
            &self
        }
    }
    fn reserve_space_even_if_hidden(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_ReserveSpaceEvenIfHidden(self.as_ptr());
            &self
        }
    }
    fn left(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Left(self.as_ptr());
            &self
        }
    }
    fn proportion(&self, proportion: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Proportion(self.as_ptr(), proportion);
            &self
        }
    }
    fn right(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Right(self.as_ptr());
            &self
        }
    }
    fn shaped(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Shaped(self.as_ptr());
            &self
        }
    }
    fn top(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Top(self.as_ptr());
            &self
        }
    }
    fn triple_border(&self, direction: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_TripleBorder(self.as_ptr(), direction);
            &self
        }
    }
    fn disable_consistency_checks() {
        unsafe { ffi::wxSizerFlags_DisableConsistencyChecks() }
    }
    fn get_default_border() -> c_int {
        unsafe { ffi::wxSizerFlags_GetDefaultBorder() }
    }
    // NOT_SUPPORTED: fn GetDefaultBorderFractional()
}

// wxSizerItem
pub trait SizerItemMethods: ObjectMethods {
    // DTOR: fn ~wxSizerItem()
    fn assign_window<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizerItem_AssignWindow(self.as_ptr(), window)
        }
    }
    fn assign_sizer<S: SizerMethods>(&self, sizer: Option<&S>) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizerItem_AssignSizer(self.as_ptr(), sizer)
        }
    }
    fn assign_spacer_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSizerItem_AssignSpacer(self.as_ptr(), size)
        }
    }
    fn assign_spacer_int(&self, w: c_int, h: c_int) {
        unsafe { ffi::wxSizerItem_AssignSpacer1(self.as_ptr(), w, h) }
    }
    fn calc_min(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxSizerItem_CalcMin(self.as_ptr())) }
    }
    fn delete_windows(&self) {
        unsafe { ffi::wxSizerItem_DeleteWindows(self.as_ptr()) }
    }
    fn detach_sizer(&self) {
        unsafe { ffi::wxSizerItem_DetachSizer(self.as_ptr()) }
    }
    fn get_border(&self) -> c_int {
        unsafe { ffi::wxSizerItem_GetBorder(self.as_ptr()) }
    }
    fn get_flag(&self) -> c_int {
        unsafe { ffi::wxSizerItem_GetFlag(self.as_ptr()) }
    }
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxSizerItem_GetId(self.as_ptr()) }
    }
    fn get_min_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxSizerItem_GetMinSize(self.as_ptr())) }
    }
    fn set_min_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSizerItem_SetMinSize(self.as_ptr(), size)
        }
    }
    fn set_min_size_int(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxSizerItem_SetMinSize1(self.as_ptr(), x, y) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxSizerItem_GetPosition(self.as_ptr())) }
    }
    fn get_proportion(&self) -> c_int {
        unsafe { ffi::wxSizerItem_GetProportion(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetRatio()
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxSizerItem_GetRect(self.as_ptr())) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxSizerItem_GetSize(self.as_ptr())) }
    }
    fn get_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxSizerItem_GetSizer(self.as_ptr())) }
    }
    fn get_spacer(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxSizerItem_GetSpacer(self.as_ptr())) }
    }
    fn get_user_data(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxSizerItem_GetUserData(self.as_ptr())) }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxSizerItem_GetWindow(self.as_ptr())) }
    }
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxSizerItem_IsShown(self.as_ptr()) }
    }
    fn is_sizer(&self) -> bool {
        unsafe { ffi::wxSizerItem_IsSizer(self.as_ptr()) }
    }
    fn is_spacer(&self) -> bool {
        unsafe { ffi::wxSizerItem_IsSpacer(self.as_ptr()) }
    }
    fn is_window(&self) -> bool {
        unsafe { ffi::wxSizerItem_IsWindow(self.as_ptr()) }
    }
    fn set_border(&self, border: c_int) {
        unsafe { ffi::wxSizerItem_SetBorder(self.as_ptr(), border) }
    }
    fn set_dimension<P: PointMethods, S: SizeMethods>(&self, pos: &P, size: &S) {
        unsafe {
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            ffi::wxSizerItem_SetDimension(self.as_ptr(), pos, size)
        }
    }
    fn set_flag(&self, flag: c_int) {
        unsafe { ffi::wxSizerItem_SetFlag(self.as_ptr(), flag) }
    }
    fn set_id(&self, id: c_int) {
        unsafe { ffi::wxSizerItem_SetId(self.as_ptr(), id) }
    }
    fn set_init_size(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxSizerItem_SetInitSize(self.as_ptr(), x, y) }
    }
    fn set_proportion(&self, proportion: c_int) {
        unsafe { ffi::wxSizerItem_SetProportion(self.as_ptr(), proportion) }
    }
    fn set_ratio(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxSizerItem_SetRatio(self.as_ptr(), width, height) }
    }
    // BLOCKED: fn SetRatio1()
    // NOT_SUPPORTED: fn SetRatio2()
    // BLOCKED: fn SetSizer()
    // BLOCKED: fn SetSpacer()
    fn set_user_data<O: ObjectMethods>(&self, user_data: Option<&O>) {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizerItem_SetUserData(self.as_ptr(), user_data)
        }
    }
    // BLOCKED: fn SetWindow()
    fn show(&self, show: bool) {
        unsafe { ffi::wxSizerItem_Show(self.as_ptr(), show) }
    }
}

// wxSlider
pub trait SliderMethods: ControlMethods {
    // DTOR: fn ~wxSlider()
    fn clear_sel(&self) {
        unsafe { ffi::wxSlider_ClearSel(self.as_ptr()) }
    }
    fn clear_ticks(&self) {
        unsafe { ffi::wxSlider_ClearTicks(self.as_ptr()) }
    }
    fn create_int<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: c_int,
        min_value: c_int,
        max_value: c_int,
        point: &P,
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
            let point = point.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSlider_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                min_value,
                max_value,
                point,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_line_size(&self) -> c_int {
        unsafe { ffi::wxSlider_GetLineSize(self.as_ptr()) }
    }
    fn get_max(&self) -> c_int {
        unsafe { ffi::wxSlider_GetMax(self.as_ptr()) }
    }
    fn get_min(&self) -> c_int {
        unsafe { ffi::wxSlider_GetMin(self.as_ptr()) }
    }
    fn get_page_size(&self) -> c_int {
        unsafe { ffi::wxSlider_GetPageSize(self.as_ptr()) }
    }
    fn get_sel_end(&self) -> c_int {
        unsafe { ffi::wxSlider_GetSelEnd(self.as_ptr()) }
    }
    fn get_sel_start(&self) -> c_int {
        unsafe { ffi::wxSlider_GetSelStart(self.as_ptr()) }
    }
    fn get_thumb_length(&self) -> c_int {
        unsafe { ffi::wxSlider_GetThumbLength(self.as_ptr()) }
    }
    fn get_tick_freq(&self) -> c_int {
        unsafe { ffi::wxSlider_GetTickFreq(self.as_ptr()) }
    }
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxSlider_GetValue(self.as_ptr()) }
    }
    fn set_line_size(&self, line_size: c_int) {
        unsafe { ffi::wxSlider_SetLineSize(self.as_ptr(), line_size) }
    }
    fn set_min(&self, min_value: c_int) {
        unsafe { ffi::wxSlider_SetMin(self.as_ptr(), min_value) }
    }
    fn set_max(&self, max_value: c_int) {
        unsafe { ffi::wxSlider_SetMax(self.as_ptr(), max_value) }
    }
    fn set_page_size(&self, page_size: c_int) {
        unsafe { ffi::wxSlider_SetPageSize(self.as_ptr(), page_size) }
    }
    fn set_range(&self, min_value: c_int, max_value: c_int) {
        unsafe { ffi::wxSlider_SetRange(self.as_ptr(), min_value, max_value) }
    }
    fn set_selection(&self, start_pos: c_int, end_pos: c_int) {
        unsafe { ffi::wxSlider_SetSelection(self.as_ptr(), start_pos, end_pos) }
    }
    fn set_thumb_length(&self, len: c_int) {
        unsafe { ffi::wxSlider_SetThumbLength(self.as_ptr(), len) }
    }
    fn set_tick(&self, tick_pos: c_int) {
        unsafe { ffi::wxSlider_SetTick(self.as_ptr(), tick_pos) }
    }
    fn set_tick_freq(&self, freq: c_int) {
        unsafe { ffi::wxSlider_SetTickFreq(self.as_ptr(), freq) }
    }
    fn set_value(&self, value: c_int) {
        unsafe { ffi::wxSlider_SetValue(self.as_ptr(), value) }
    }
}

// wxSpinButton
pub trait SpinButtonMethods: ControlMethods {
    // DTOR: fn ~wxSpinButton()
    fn get_increment(&self) -> c_int {
        unsafe { ffi::wxSpinButton_GetIncrement(self.as_ptr()) }
    }
    fn get_max(&self) -> c_int {
        unsafe { ffi::wxSpinButton_GetMax(self.as_ptr()) }
    }
    fn get_min(&self) -> c_int {
        unsafe { ffi::wxSpinButton_GetMin(self.as_ptr()) }
    }
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxSpinButton_GetValue(self.as_ptr()) }
    }
    fn set_increment(&self, value: c_int) {
        unsafe { ffi::wxSpinButton_SetIncrement(self.as_ptr(), value) }
    }
    fn set_range(&self, min: c_int, max: c_int) {
        unsafe { ffi::wxSpinButton_SetRange(self.as_ptr(), min, max) }
    }
    fn set_value(&self, value: c_int) {
        unsafe { ffi::wxSpinButton_SetValue(self.as_ptr(), value) }
    }
}

// wxSpinCtrl
pub trait SpinCtrlMethods: ControlMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
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
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSpinCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                style,
                min,
                max,
                initial,
                name,
            )
        }
    }
    fn get_base(&self) -> c_int {
        unsafe { ffi::wxSpinCtrl_GetBase(self.as_ptr()) }
    }
    fn get_max(&self) -> c_int {
        unsafe { ffi::wxSpinCtrl_GetMax(self.as_ptr()) }
    }
    fn get_min(&self) -> c_int {
        unsafe { ffi::wxSpinCtrl_GetMin(self.as_ptr()) }
    }
    fn get_text_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxSpinCtrl_GetTextValue(self.as_ptr())).into() }
    }
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxSpinCtrl_GetValue(self.as_ptr()) }
    }
    fn get_increment(&self) -> c_int {
        unsafe { ffi::wxSpinCtrl_GetIncrement(self.as_ptr()) }
    }
    fn set_base(&self, base: c_int) -> bool {
        unsafe { ffi::wxSpinCtrl_SetBase(self.as_ptr(), base) }
    }
    fn set_range(&self, min_val: c_int, max_val: c_int) {
        unsafe { ffi::wxSpinCtrl_SetRange(self.as_ptr(), min_val, max_val) }
    }
    fn set_selection(&self, from: c_long, to: c_long) {
        unsafe { ffi::wxSpinCtrl_SetSelection(self.as_ptr(), from, to) }
    }
    fn set_value_str(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxSpinCtrl_SetValue(self.as_ptr(), text)
        }
    }
    fn set_value_int(&self, value: c_int) {
        unsafe { ffi::wxSpinCtrl_SetValue1(self.as_ptr(), value) }
    }
    fn set_increment(&self, value: c_int) {
        unsafe { ffi::wxSpinCtrl_SetIncrement(self.as_ptr(), value) }
    }
}

// wxSpinCtrlDouble
pub trait SpinCtrlDoubleMethods: ControlMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
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
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSpinCtrlDouble_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                style,
                min,
                max,
                initial,
                inc,
                name,
            )
        }
    }
    fn get_digits(&self) -> c_uint {
        unsafe { ffi::wxSpinCtrlDouble_GetDigits(self.as_ptr()) }
    }
    fn get_increment(&self) -> c_double {
        unsafe { ffi::wxSpinCtrlDouble_GetIncrement(self.as_ptr()) }
    }
    fn get_max(&self) -> c_double {
        unsafe { ffi::wxSpinCtrlDouble_GetMax(self.as_ptr()) }
    }
    fn get_min(&self) -> c_double {
        unsafe { ffi::wxSpinCtrlDouble_GetMin(self.as_ptr()) }
    }
    fn get_text_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxSpinCtrlDouble_GetTextValue(self.as_ptr())).into() }
    }
    fn get_value(&self) -> c_double {
        unsafe { ffi::wxSpinCtrlDouble_GetValue(self.as_ptr()) }
    }
    fn set_digits(&self, digits: c_uint) {
        unsafe { ffi::wxSpinCtrlDouble_SetDigits(self.as_ptr(), digits) }
    }
    fn set_increment(&self, inc: c_double) {
        unsafe { ffi::wxSpinCtrlDouble_SetIncrement(self.as_ptr(), inc) }
    }
    fn set_range(&self, min_val: c_double, max_val: c_double) {
        unsafe { ffi::wxSpinCtrlDouble_SetRange(self.as_ptr(), min_val, max_val) }
    }
    fn set_value_str(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxSpinCtrlDouble_SetValue(self.as_ptr(), text)
        }
    }
    fn set_value_double(&self, value: c_double) {
        unsafe { ffi::wxSpinCtrlDouble_SetValue1(self.as_ptr(), value) }
    }
}

// wxStaticBitmap
pub trait StaticBitmapMethods: ControlMethods {
    fn create_bitmapbundle<
        W: WindowMethods,
        B: BitmapBundleMethods,
        P: PointMethods,
        S: SizeMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &B,
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
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxStaticBitmap_Create(self.as_ptr(), parent, id, label, pos, size, style, name)
        }
    }
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxStaticBitmap_GetBitmap(self.as_ptr())) }
    }
    fn get_icon(&self) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxStaticBitmap_GetIcon(self.as_ptr())) }
    }
    fn set_bitmap<B: BitmapBundleMethods>(&self, label: &B) {
        unsafe {
            let label = label.as_ptr();
            ffi::wxStaticBitmap_SetBitmap(self.as_ptr(), label)
        }
    }
    fn set_icon<I: IconMethods>(&self, label: &I) {
        unsafe {
            let label = label.as_ptr();
            ffi::wxStaticBitmap_SetIcon(self.as_ptr(), label)
        }
    }
    // NOT_SUPPORTED: fn SetScaleMode()
    // NOT_SUPPORTED: fn GetScaleMode()
}

// wxStaticBox
pub trait StaticBoxMethods: ControlMethods {
    // DTOR: fn ~wxStaticBox()
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxStaticBox_Create(self.as_ptr(), parent, id, label, pos, size, style, name)
        }
    }
    // BLOCKED: fn Create1()
}

// wxStaticBoxSizer
pub trait StaticBoxSizerMethods: BoxSizerMethods {
    fn get_static_box(&self) -> WeakRef<StaticBox> {
        unsafe { WeakRef::<StaticBox>::from(ffi::wxStaticBoxSizer_GetStaticBox(self.as_ptr())) }
    }
}

// wxStaticText
pub trait StaticTextMethods: ControlMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxStaticText_Create(self.as_ptr(), parent, id, label, pos, size, style, name)
        }
    }
    fn is_ellipsized(&self) -> bool {
        unsafe { ffi::wxStaticText_IsEllipsized(self.as_ptr()) }
    }
    fn wrap(&self, width: c_int) {
        unsafe { ffi::wxStaticText_Wrap(self.as_ptr(), width) }
    }
}
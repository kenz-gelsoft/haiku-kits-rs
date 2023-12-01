#include "generated.h"

extern "C" {

// CLASS: BView
BArchivable * BView_Instantiate(BMessage * archive) {
    return BView::Instantiate(archive);
}
void BView_AttachedToWindow(BView * self) {
    return self->AttachedToWindow();
}
void BView_AllAttached(BView * self) {
    return self->AllAttached();
}
void BView_DetachedFromWindow(BView * self) {
    return self->DetachedFromWindow();
}
void BView_AllDetached(BView * self) {
    return self->AllDetached();
}
void BView_Draw(BView * self, BRect* update_rect) {
    return self->Draw(*update_rect);
}
void BView_MouseDown(BView * self, BPoint* where_) {
    return self->MouseDown(*where_);
}
void BView_MouseUp(BView * self, BPoint* where_) {
    return self->MouseUp(*where_);
}
void BView_MouseMoved(BView * self, BPoint* where_, uint32 code, const BMessage * drag_message) {
    return self->MouseMoved(*where_, code, drag_message);
}
void BView_WindowActivated(BView * self, bool active) {
    return self->WindowActivated(active);
}
void BView_KeyDown(BView * self, const char * bytes, int32 num_bytes) {
    return self->KeyDown(bytes, num_bytes);
}
void BView_KeyUp(BView * self, const char * bytes, int32 num_bytes) {
    return self->KeyUp(bytes, num_bytes);
}
void BView_Pulse(BView * self) {
    return self->Pulse();
}
void BView_FrameMoved(BView * self, BPoint* new_position) {
    return self->FrameMoved(*new_position);
}
void BView_FrameResized(BView * self, float new_width, float new_height) {
    return self->FrameResized(new_width, new_height);
}
void BView_TargetedByScrollView(BView * self, BScrollView * scroll_view) {
    return self->TargetedByScrollView(scroll_view);
}
void BView_DrawAfterChildren(BView * self, BRect* update_rect) {
    return self->DrawAfterChildren(*update_rect);
}
void BView_AddChild(BView * self, BView * child, BView * before) {
    return self->AddChild(child, before);
}
bool BView_AddChild1(BView * self, BLayoutItem * child) {
    return self->AddChild(child);
}
bool BView_RemoveChild(BView * self, BView * child) {
    return self->RemoveChild(child);
}
int32 BView_CountChildren(const BView * self) {
    return self->CountChildren();
}
BView * BView_ChildAt(const BView * self, int32 index) {
    return self->ChildAt(index);
}
BView * BView_NextSibling(const BView * self) {
    return self->NextSibling();
}
BView * BView_PreviousSibling(const BView * self) {
    return self->PreviousSibling();
}
bool BView_RemoveSelf(BView * self) {
    return self->RemoveSelf();
}
BView * BView_FindView(const BView * self, const char * name) {
    return self->FindView(name);
}
BView * BView_Parent(const BView * self) {
    return self->Parent();
}
void BView_BeginRectTracking(BView * self, BRect* start_rect, uint32 style) {
    return self->BeginRectTracking(*start_rect, style);
}
void BView_EndRectTracking(BView * self) {
    return self->EndRectTracking();
}
void BView_GetMouse(BView * self, BPoint * location, uint32 * buttons, bool check_message_queue) {
    return self->GetMouse(location, buttons, check_message_queue);
}
void BView_DragMessage(BView * self, BMessage * message, BRect* drag_rect, BHandler * reply_to) {
    return self->DragMessage(message, *drag_rect, reply_to);
}
void BView_DragMessage1(BView * self, BMessage * message, BBitmap * bitmap, BPoint* offset, BHandler * reply_to) {
    return self->DragMessage(message, bitmap, *offset, reply_to);
}
status_t BView_SetEventMask(BView * self, uint32 mask, uint32 options) {
    return self->SetEventMask(mask, options);
}
uint32 BView_EventMask(BView * self) {
    return self->EventMask();
}
status_t BView_SetMouseEventMask(BView * self, uint32 mask, uint32 options) {
    return self->SetMouseEventMask(mask, options);
}
void BView_ScrollBy(BView * self, float dh, float dv) {
    return self->ScrollBy(dh, dv);
}
void BView_ScrollTo(BView * self, float x, float y) {
    return self->ScrollTo(x, y);
}
void BView_ScrollTo1(BView * self, BPoint* where_) {
    return self->ScrollTo(*where_);
}
void BView_MakeFocus(BView * self, bool focus) {
    return self->MakeFocus(focus);
}
void BView_ConvertToScreen(const BView * self, BPoint * point) {
    return self->ConvertToScreen(point);
}
void BView_ConvertFromScreen(const BView * self, BPoint * point) {
    return self->ConvertFromScreen(point);
}
void BView_ConvertToScreen2(const BView * self, BRect * rect) {
    return self->ConvertToScreen(rect);
}
void BView_ConvertFromScreen2(const BView * self, BRect * rect) {
    return self->ConvertFromScreen(rect);
}
void BView_ConvertToParent(const BView * self, BPoint * point) {
    return self->ConvertToParent(point);
}
void BView_ConvertFromParent(const BView * self, BPoint * point) {
    return self->ConvertFromParent(point);
}
void BView_ConvertToParent2(const BView * self, BRect * rect) {
    return self->ConvertToParent(rect);
}
void BView_ConvertFromParent2(const BView * self, BRect * rect) {
    return self->ConvertFromParent(rect);
}
void BView_SetFlags(BView * self, uint32 flags) {
    return self->SetFlags(flags);
}
uint32 BView_Flags(const BView * self) {
    return self->Flags();
}
void BView_GetClippingRegion(const BView * self, BRegion * region) {
    return self->GetClippingRegion(region);
}
void BView_ConstrainClippingRegion(BView * self, BRegion * region) {
    return self->ConstrainClippingRegion(region);
}
void BView_ClipToPicture(BView * self, BPicture * picture, BPoint* where_, bool sync) {
    return self->ClipToPicture(picture, *where_, sync);
}
void BView_ClipToInversePicture(BView * self, BPicture * picture, BPoint* where_, bool sync) {
    return self->ClipToInversePicture(picture, *where_, sync);
}
void BView_ClipToRect(BView * self, BRect* rect) {
    return self->ClipToRect(*rect);
}
void BView_ClipToInverseRect(BView * self, BRect* rect) {
    return self->ClipToInverseRect(*rect);
}
void BView_ClipToShape(BView * self, BShape * shape) {
    return self->ClipToShape(shape);
}
void BView_ClipToInverseShape(BView * self, BShape * shape) {
    return self->ClipToInverseShape(shape);
}
void BView_GetBlendingMode(const BView * self, source_alpha * src_alpha, alpha_function * alpha_func) {
    return self->GetBlendingMode(src_alpha, alpha_func);
}
void BView_SetPenSize(BView * self, float size) {
    return self->SetPenSize(size);
}
float BView_PenSize(const BView * self) {
    return self->PenSize();
}
bool BView_HasDefaultColors(const BView * self) {
    return self->HasDefaultColors();
}
bool BView_HasSystemColors(const BView * self) {
    return self->HasSystemColors();
}
void BView_AdoptParentColors(BView * self) {
    return self->AdoptParentColors();
}
void BView_AdoptSystemColors(BView * self) {
    return self->AdoptSystemColors();
}
void BView_AdoptViewColors(BView * self, BView * view) {
    return self->AdoptViewColors(view);
}
float BView_LineMiterLimit(const BView * self) {
    return self->LineMiterLimit();
}
void BView_SetFillRule(BView * self, int32 rule) {
    return self->SetFillRule(rule);
}
int32 BView_FillRule(const BView * self) {
    return self->FillRule();
}
void BView_SetOrigin(BView * self, BPoint* where_) {
    return self->SetOrigin(*where_);
}
void BView_SetOrigin1(BView * self, float x, float y) {
    return self->SetOrigin(x, y);
}
BPoint *BView_Origin(const BView * self) {
    return new BPoint(self->Origin());
}
void BView_TranslateBy(BView * self, double x, double y) {
    return self->TranslateBy(x, y);
}
void BView_ScaleBy(BView * self, double x, double y) {
    return self->ScaleBy(x, y);
}
void BView_RotateBy(BView * self, double angle_radians) {
    return self->RotateBy(angle_radians);
}
void BView_PushState(BView * self) {
    return self->PushState();
}
void BView_PopState(BView * self) {
    return self->PopState();
}
void BView_MovePenTo(BView * self, BPoint* pt) {
    return self->MovePenTo(*pt);
}
void BView_MovePenTo1(BView * self, float x, float y) {
    return self->MovePenTo(x, y);
}
void BView_MovePenBy(BView * self, float x, float y) {
    return self->MovePenBy(x, y);
}
BPoint *BView_PenLocation(const BView * self) {
    return new BPoint(self->PenLocation());
}
void BView_SetFont(BView * self, const BFont * font, uint32 mask) {
    return self->SetFont(font, mask);
}
void BView_GetFont(const BView * self, BFont * font) {
    return self->GetFont(font);
}
void BView_TruncateString(const BView * self, BString * in_out, uint32 mode, float width) {
    return self->TruncateString(in_out, mode, width);
}
float BView_StringWidth(const BView * self, const char * string) {
    return self->StringWidth(string);
}
float BView_StringWidth1(const BView * self, const char * string, int32 length) {
    return self->StringWidth(string, length);
}
void BView_SetFontSize(BView * self, float size) {
    return self->SetFontSize(size);
}
void BView_ForceFontAliasing(BView * self, bool enable) {
    return self->ForceFontAliasing(enable);
}
void BView_GetFontHeight(const BView * self, font_height * height) {
    return self->GetFontHeight(height);
}
void BView_SetScale(const BView * self, float scale) {
    return self->SetScale(scale);
}
float BView_Scale(const BView * self) {
    return self->Scale();
}
void BView_SetViewBitmap(BView * self, const BBitmap * bitmap, BRect* src_rect, BRect* dst_rect, uint32 follow_flags, uint32 options) {
    return self->SetViewBitmap(bitmap, *src_rect, *dst_rect, follow_flags, options);
}
void BView_SetViewBitmap1(BView * self, const BBitmap * bitmap, uint32 follow_flags, uint32 options) {
    return self->SetViewBitmap(bitmap, follow_flags, options);
}
void BView_ClearViewBitmap(BView * self) {
    return self->ClearViewBitmap();
}
status_t BView_SetViewOverlay(BView * self, const BBitmap * overlay, BRect* src_rect, BRect* dst_rect, rgb_color * color_key, uint32 follow_flags, uint32 options) {
    return self->SetViewOverlay(overlay, *src_rect, *dst_rect, color_key, follow_flags, options);
}
status_t BView_SetViewOverlay1(BView * self, const BBitmap * overlay, rgb_color * color_key, uint32 follow_flags, uint32 options) {
    return self->SetViewOverlay(overlay, color_key, follow_flags, options);
}
void BView_ClearViewOverlay(BView * self) {
    return self->ClearViewOverlay();
}
void BView_BeginLineArray(BView * self, int32 count) {
    return self->BeginLineArray(count);
}
void BView_EndLineArray(BView * self) {
    return self->EndLineArray();
}
void BView_FillPolygon3(BView * self, const BPolygon * polygon, const BGradient * gradient) {
    return self->FillPolygon(polygon, *gradient);
}
void BView_FillPolygon4(BView * self, const BPoint * point_array, int32 num_points, const BGradient * gradient) {
    return self->FillPolygon(point_array, num_points, *gradient);
}
void BView_FillPolygon5(BView * self, const BPoint * point_array, int32 num_points, BRect* bounds, const BGradient * gradient) {
    return self->FillPolygon(point_array, num_points, *bounds, *gradient);
}
void BView_FillTriangle2(BView * self, BPoint* point1, BPoint* point2, BPoint* point3, const BGradient * gradient) {
    return self->FillTriangle(*point1, *point2, *point3, *gradient);
}
void BView_FillTriangle3(BView * self, BPoint* point1, BPoint* point2, BPoint* point3, BRect* bounds, const BGradient * gradient) {
    return self->FillTriangle(*point1, *point2, *point3, *bounds, *gradient);
}
void BView_FillRect1(BView * self, BRect* rect, const BGradient * gradient) {
    return self->FillRect(*rect, *gradient);
}
void BView_FillRegion1(BView * self, BRegion * rectegion, const BGradient * gradient) {
    return self->FillRegion(rectegion, *gradient);
}
void BView_InvertRect(BView * self, BRect* rect) {
    return self->InvertRect(*rect);
}
void BView_FillRoundRect1(BView * self, BRect* rect, float x_radius, float y_radius, const BGradient * gradient) {
    return self->FillRoundRect(*rect, x_radius, y_radius, *gradient);
}
void BView_FillEllipse2(BView * self, BPoint* center, float x_radius, float y_radius, const BGradient * gradient) {
    return self->FillEllipse(*center, x_radius, y_radius, *gradient);
}
void BView_FillEllipse3(BView * self, BRect* rect, const BGradient * gradient) {
    return self->FillEllipse(*rect, *gradient);
}
void BView_FillArc2(BView * self, BPoint* center, float x_radius, float y_radius, float start_angle, float arc_angle, const BGradient * gradient) {
    return self->FillArc(*center, x_radius, y_radius, start_angle, arc_angle, *gradient);
}
void BView_FillArc3(BView * self, BRect* rect, float start_angle, float arc_angle, const BGradient * gradient) {
    return self->FillArc(*rect, start_angle, arc_angle, *gradient);
}
void BView_FillBezier1(BView * self, BPoint * control_points, const BGradient * gradient) {
    return self->FillBezier(control_points, *gradient);
}
void BView_FillShape1(BView * self, BShape * shape, const BGradient * gradient) {
    return self->FillShape(shape, *gradient);
}
void BView_CopyBits(BView * self, BRect* src, BRect* dst) {
    return self->CopyBits(*src, *dst);
}
void BView_DrawBitmapAsync(BView * self, const BBitmap * a_bitmap, BRect* bitmap_rect, BRect* view_rect, uint32 options) {
    return self->DrawBitmapAsync(a_bitmap, *bitmap_rect, *view_rect, options);
}
void BView_DrawBitmapAsync1(BView * self, const BBitmap * a_bitmap, BRect* bitmap_rect, BRect* view_rect) {
    return self->DrawBitmapAsync(a_bitmap, *bitmap_rect, *view_rect);
}
void BView_DrawBitmapAsync2(BView * self, const BBitmap * a_bitmap, BRect* view_rect) {
    return self->DrawBitmapAsync(a_bitmap, *view_rect);
}
void BView_DrawBitmapAsync3(BView * self, const BBitmap * a_bitmap, BPoint* where_) {
    return self->DrawBitmapAsync(a_bitmap, *where_);
}
void BView_DrawBitmapAsync4(BView * self, const BBitmap * a_bitmap) {
    return self->DrawBitmapAsync(a_bitmap);
}
void BView_DrawBitmap(BView * self, const BBitmap * a_bitmap, BRect* bitmap_rect, BRect* view_rect, uint32 options) {
    return self->DrawBitmap(a_bitmap, *bitmap_rect, *view_rect, options);
}
void BView_DrawBitmap1(BView * self, const BBitmap * a_bitmap, BRect* bitmap_rect, BRect* view_rect) {
    return self->DrawBitmap(a_bitmap, *bitmap_rect, *view_rect);
}
void BView_DrawBitmap2(BView * self, const BBitmap * a_bitmap, BRect* view_rect) {
    return self->DrawBitmap(a_bitmap, *view_rect);
}
void BView_DrawBitmap3(BView * self, const BBitmap * a_bitmap, BPoint* where_) {
    return self->DrawBitmap(a_bitmap, *where_);
}
void BView_DrawBitmap4(BView * self, const BBitmap * a_bitmap) {
    return self->DrawBitmap(a_bitmap);
}
void BView_DrawTiledBitmapAsync(BView * self, const BBitmap * a_bitmap, BRect* view_rect, BPoint* phase) {
    return self->DrawTiledBitmapAsync(a_bitmap, *view_rect, *phase);
}
void BView_DrawTiledBitmap(BView * self, const BBitmap * a_bitmap, BRect* view_rect, BPoint* phase) {
    return self->DrawTiledBitmap(a_bitmap, *view_rect, *phase);
}
void BView_DrawChar(BView * self, char a_char) {
    return self->DrawChar(a_char);
}
void BView_DrawChar1(BView * self, char a_char, BPoint* location) {
    return self->DrawChar(a_char, *location);
}
void BView_DrawString(BView * self, const char * string, escapement_delta * delta) {
    return self->DrawString(string, delta);
}
void BView_DrawString2(BView * self, const char * string, int32 length, escapement_delta * delta) {
    return self->DrawString(string, length, delta);
}
void BView_DrawString4(BView * self, const char * string, const BPoint * locations, int32 location_count) {
    return self->DrawString(string, locations, location_count);
}
void BView_DrawString5(BView * self, const char * string, int32 length, const BPoint * locations, int32 location_count) {
    return self->DrawString(string, length, locations, location_count);
}
void BView_Invalidate(BView * self, BRect* inval_rect) {
    return self->Invalidate(*inval_rect);
}
void BView_Invalidate1(BView * self, const BRegion * inval_region) {
    return self->Invalidate(inval_region);
}
void BView_Invalidate2(BView * self) {
    return self->Invalidate();
}
void BView_DelayedInvalidate(BView * self, bigtime_t delay) {
    return self->DelayedInvalidate(delay);
}
void BView_DelayedInvalidate1(BView * self, bigtime_t delay, BRect* inval_rect) {
    return self->DelayedInvalidate(delay, *inval_rect);
}
void BView_SetDiskMode(BView * self, char * filename, long offset) {
    return self->SetDiskMode(filename, offset);
}
void BView_BeginPicture(BView * self, BPicture * a_picture) {
    return self->BeginPicture(a_picture);
}
void BView_AppendToPicture(BView * self, BPicture * a_picture) {
    return self->AppendToPicture(a_picture);
}
BPicture * BView_EndPicture(BView * self) {
    return self->EndPicture();
}
void BView_DrawPicture(BView * self, const BPicture * a_picture) {
    return self->DrawPicture(a_picture);
}
void BView_DrawPicture1(BView * self, const BPicture * a_picture, BPoint* where_) {
    return self->DrawPicture(a_picture, *where_);
}
void BView_DrawPicture2(BView * self, const char * filename, long offset, BPoint* where_) {
    return self->DrawPicture(filename, offset, *where_);
}
void BView_DrawPictureAsync(BView * self, const BPicture * a_picture) {
    return self->DrawPictureAsync(a_picture);
}
void BView_DrawPictureAsync1(BView * self, const BPicture * a_picture, BPoint* where_) {
    return self->DrawPictureAsync(a_picture, *where_);
}
void BView_DrawPictureAsync2(BView * self, const char * filename, long offset, BPoint* where_) {
    return self->DrawPictureAsync(filename, offset, *where_);
}
void BView_BeginLayer(BView * self, uint8 opacity) {
    return self->BeginLayer(opacity);
}
void BView_EndLayer(BView * self) {
    return self->EndLayer();
}
void BView_MoveBy(BView * self, float dh, float dv) {
    return self->MoveBy(dh, dv);
}
void BView_MoveTo(BView * self, BPoint* where_) {
    return self->MoveTo(*where_);
}
void BView_MoveTo1(BView * self, float x, float y) {
    return self->MoveTo(x, y);
}
void BView_ResizeBy(BView * self, float dh, float dv) {
    return self->ResizeBy(dh, dv);
}
void BView_ResizeTo(BView * self, float width, float height) {
    return self->ResizeTo(width, height);
}
void BView_ResizeTo1(BView * self, BSize* size) {
    return self->ResizeTo(*size);
}
BSize *BView_MinSize(BView * self) {
    return new BSize(self->MinSize());
}
BSize *BView_MaxSize(BView * self) {
    return new BSize(self->MaxSize());
}
BSize *BView_PreferredSize(BView * self) {
    return new BSize(self->PreferredSize());
}
void BView_SetExplicitMinSize(BView * self, BSize* size) {
    return self->SetExplicitMinSize(*size);
}
void BView_SetExplicitMaxSize(BView * self, BSize* size) {
    return self->SetExplicitMaxSize(*size);
}
void BView_SetExplicitPreferredSize(BView * self, BSize* size) {
    return self->SetExplicitPreferredSize(*size);
}
void BView_SetExplicitSize(BView * self, BSize* size) {
    return self->SetExplicitSize(*size);
}
BSize *BView_ExplicitMinSize(const BView * self) {
    return new BSize(self->ExplicitMinSize());
}
BSize *BView_ExplicitMaxSize(const BView * self) {
    return new BSize(self->ExplicitMaxSize());
}
BSize *BView_ExplicitPreferredSize(const BView * self) {
    return new BSize(self->ExplicitPreferredSize());
}
bool BView_HasHeightForWidth(BView * self) {
    return self->HasHeightForWidth();
}
void BView_GetHeightForWidth(BView * self, float width, float * min, float * max, float * preferred) {
    return self->GetHeightForWidth(width, min, max, preferred);
}
void BView_InvalidateLayout(BView * self, bool descendants) {
    return self->InvalidateLayout(descendants);
}
void BView_SetLayout(BView * self, BLayout * layout) {
    return self->SetLayout(layout);
}
BLayout * BView_GetLayout(const BView * self) {
    return self->GetLayout();
}
void BView_EnableLayoutInvalidation(BView * self) {
    return self->EnableLayoutInvalidation();
}
void BView_DisableLayoutInvalidation(BView * self) {
    return self->DisableLayoutInvalidation();
}
bool BView_IsLayoutInvalidationDisabled(BView * self) {
    return self->IsLayoutInvalidationDisabled();
}
bool BView_IsLayoutValid(const BView * self) {
    return self->IsLayoutValid();
}
void BView_ResetLayoutInvalidation(BView * self) {
    return self->ResetLayoutInvalidation();
}
BLayoutContext * BView_LayoutContext(const BView * self) {
    return self->LayoutContext();
}
void BView_Layout(BView * self, bool force) {
    return self->Layout(force);
}
void BView_Relayout(BView * self) {
    return self->Relayout();
}
void BView_SetToolTip(BView * self, const char * text) {
    return self->SetToolTip(text);
}
void BView_SetToolTip1(BView * self, BToolTip * tip) {
    return self->SetToolTip(tip);
}
BToolTip * BView_ToolTip(const BView * self) {
    return self->ToolTip();
}
void BView_ShowToolTip(BView * self, BToolTip * tip) {
    return self->ShowToolTip(tip);
}
void BView_HideToolTip(BView * self) {
    return self->HideToolTip();
}
BView *BView_new(BMessage * archive) {
    return new BView(archive);
}
BView *BView_new1(BRect* frame, const char * name, uint32 resizing_mode, uint32 flags) {
    return new BView(*frame, name, resizing_mode, flags);
}
BView *BView_new2(const char * name, uint32 flags, BLayout * layout) {
    return new BView(name, flags, layout);
}
BRect *BView_Bounds(const BView * self) {
    return new BRect(self->Bounds());
}
void BView_Flush(const BView * self) {
    return self->Flush();
}
BRect *BView_Frame(const BView * self) {
    return new BRect(self->Frame());
}
void BView_GetPreferredSize(BView * self, float * _width, float * _height) {
    return self->GetPreferredSize(_width, _height);
}
void BView_Hide(BView * self) {
    return self->Hide();
}
bool BView_IsFocus(const BView * self) {
    return self->IsFocus();
}
bool BView_IsHidden(const BView * self) {
    return self->IsHidden();
}
bool BView_IsHidden1(const BView * self, const BView * looking_from) {
    return self->IsHidden(looking_from);
}
bool BView_IsPrinting(const BView * self) {
    return self->IsPrinting();
}
BPoint *BView_LeftTop(const BView * self) {
    return new BPoint(self->LeftTop());
}
void BView_ResizeToPreferred(BView * self) {
    return self->ResizeToPreferred();
}
uint32 BView_ResizingMode(const BView * self) {
    return self->ResizingMode();
}
void BView_SetResizingMode(BView * self, uint32 mode) {
    return self->SetResizingMode(mode);
}
void BView_SetViewCursor(BView * self, const BCursor * cursor, bool sync) {
    return self->SetViewCursor(cursor, sync);
}
void BView_Show(BView * self) {
    return self->Show();
}
void BView_Sync(const BView * self) {
    return self->Sync();
}
BWindow * BView_Window(const BView * self) {
    return self->Window();
}

} // extern "C"


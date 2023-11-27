#include "generated.h"

extern "C" {

// CLASS: BMessage
void BMessage_delete(BMessage *self) {
    delete self;
}
status_t BMessage_GetInfo1(const BMessage * self, const char * name, type_code * type_found, int32 * count_found) {
    return self->GetInfo(name, type_found, count_found);
}
status_t BMessage_GetInfo2(const BMessage * self, const char * name, type_code * type_found, bool * fixed_size) {
    return self->GetInfo(name, type_found, fixed_size);
}
status_t BMessage_GetInfo3(const BMessage * self, const char * name, type_code * type_found, int32 * count_found, bool * fixed_size) {
    return self->GetInfo(name, type_found, count_found, fixed_size);
}
bool BMessage_IsEmpty(const BMessage * self) {
    return self->IsEmpty();
}
bool BMessage_IsSystem(const BMessage * self) {
    return self->IsSystem();
}
bool BMessage_IsReply(const BMessage * self) {
    return self->IsReply();
}
void BMessage_PrintToStream(const BMessage * self) {
    return self->PrintToStream();
}
status_t BMessage_Rename(BMessage * self, const char * old_entry, const char * new_entry) {
    return self->Rename(old_entry, new_entry);
}
bool BMessage_WasDelivered(const BMessage * self) {
    return self->WasDelivered();
}
bool BMessage_IsSourceWaiting(const BMessage * self) {
    return self->IsSourceWaiting();
}
bool BMessage_IsSourceRemote(const BMessage * self) {
    return self->IsSourceRemote();
}
const BMessage * BMessage_Previous(const BMessage * self) {
    return self->Previous();
}
bool BMessage_WasDropped(const BMessage * self) {
    return self->WasDropped();
}
status_t BMessage_SendReply(BMessage * self, uint32 command, BHandler * reply_to) {
    return self->SendReply(command, reply_to);
}
status_t BMessage_SendReply1(BMessage * self, BMessage * reply, BHandler * reply_to, bigtime_t timeout) {
    return self->SendReply(reply, reply_to, timeout);
}
status_t BMessage_SendReply3(BMessage * self, uint32 command, BMessage * reply_to_reply) {
    return self->SendReply(command, reply_to_reply);
}
status_t BMessage_SendReply4(BMessage * self, BMessage * reply, BMessage * reply_to_reply, bigtime_t send_timeout, bigtime_t reply_timeout) {
    return self->SendReply(reply, reply_to_reply, send_timeout, reply_timeout);
}
status_t BMessage_Flatten1(const BMessage * self, BDataIO * stream, ssize_t * size) {
    return self->Flatten(stream, size);
}
status_t BMessage_Unflatten(BMessage * self, const char * flat_buffer) {
    return self->Unflatten(flat_buffer);
}
status_t BMessage_Unflatten1(BMessage * self, BDataIO * stream) {
    return self->Unflatten(stream);
}
status_t BMessage_AddSpecifier(BMessage * self, const char * property) {
    return self->AddSpecifier(property);
}
status_t BMessage_AddSpecifier1(BMessage * self, const char * property, int32 index) {
    return self->AddSpecifier(property, index);
}
status_t BMessage_AddSpecifier2(BMessage * self, const char * property, int32 index, int32 range) {
    return self->AddSpecifier(property, index, range);
}
status_t BMessage_AddSpecifier3(BMessage * self, const char * property, const char * name) {
    return self->AddSpecifier(property, name);
}
status_t BMessage_AddSpecifier4(BMessage * self, const BMessage * specifier) {
    return self->AddSpecifier(specifier);
}
status_t BMessage_SetCurrentSpecifier(BMessage * self, int32 index) {
    return self->SetCurrentSpecifier(index);
}
status_t BMessage_GetCurrentSpecifier(const BMessage * self, int32 * index, BMessage * specifier, int32 * what, const char ** property) {
    return self->GetCurrentSpecifier(index, specifier, what, property);
}
bool BMessage_HasSpecifiers(const BMessage * self) {
    return self->HasSpecifiers();
}
status_t BMessage_PopSpecifier(BMessage * self) {
    return self->PopSpecifier();
}
status_t BMessage_AddAlignment(BMessage * self, const char * name, const BAlignment * alignment) {
    return self->AddAlignment(name, *alignment);
}
status_t BMessage_AddString(BMessage * self, const char * name, const char * string) {
    return self->AddString(name, string);
}
status_t BMessage_AddString1(BMessage * self, const char * name, const BString * string) {
    return self->AddString(name, *string);
}
status_t BMessage_AddStrings(BMessage * self, const char * name, const BStringList * list) {
    return self->AddStrings(name, *list);
}
status_t BMessage_AddInt8(BMessage * self, const char * name, int8 value) {
    return self->AddInt8(name, value);
}
status_t BMessage_AddUInt8(BMessage * self, const char * name, uint8 value) {
    return self->AddUInt8(name, value);
}
status_t BMessage_AddInt16(BMessage * self, const char * name, int16 value) {
    return self->AddInt16(name, value);
}
status_t BMessage_AddUInt16(BMessage * self, const char * name, uint16 value) {
    return self->AddUInt16(name, value);
}
status_t BMessage_AddInt32(BMessage * self, const char * name, int32 value) {
    return self->AddInt32(name, value);
}
status_t BMessage_AddUInt32(BMessage * self, const char * name, uint32 value) {
    return self->AddUInt32(name, value);
}
status_t BMessage_AddInt64(BMessage * self, const char * name, int64 value) {
    return self->AddInt64(name, value);
}
status_t BMessage_AddUInt64(BMessage * self, const char * name, uint64 value) {
    return self->AddUInt64(name, value);
}
status_t BMessage_AddBool(BMessage * self, const char * name, bool value) {
    return self->AddBool(name, value);
}
status_t BMessage_AddFloat(BMessage * self, const char * name, float value) {
    return self->AddFloat(name, value);
}
status_t BMessage_AddDouble(BMessage * self, const char * name, double value) {
    return self->AddDouble(name, value);
}
status_t BMessage_AddPointer(BMessage * self, const char * name, const void * pointer) {
    return self->AddPointer(name, pointer);
}
status_t BMessage_AddRef(BMessage * self, const char * name, const entry_ref * ref_) {
    return self->AddRef(name, ref_);
}
status_t BMessage_AddNodeRef(BMessage * self, const char * name, const node_ref * ref_) {
    return self->AddNodeRef(name, ref_);
}
status_t BMessage_AddMessage(BMessage * self, const char * name, const BMessage * message) {
    return self->AddMessage(name, message);
}
status_t BMessage_AddFlat1(BMessage * self, const char * name, const BFlattenable * object, int32 count) {
    return self->AddFlat(name, object, count);
}
status_t BMessage_Append(BMessage * self, const BMessage * message) {
    return self->Append(*message);
}
status_t BMessage_RemoveData(BMessage * self, const char * name, int32 index) {
    return self->RemoveData(name, index);
}
status_t BMessage_RemoveName(BMessage * self, const char * name) {
    return self->RemoveName(name);
}
status_t BMessage_MakeEmpty(BMessage * self) {
    return self->MakeEmpty();
}
status_t BMessage_FindAlignment(const BMessage * self, const char * name, BAlignment * alignment) {
    return self->FindAlignment(name, alignment);
}
status_t BMessage_FindAlignment1(const BMessage * self, const char * name, int32 index, BAlignment * alignment) {
    return self->FindAlignment(name, index, alignment);
}
status_t BMessage_FindRect(const BMessage * self, const char * name, BRect * rect) {
    return self->FindRect(name, rect);
}
status_t BMessage_FindRect1(const BMessage * self, const char * name, int32 index, BRect * rect) {
    return self->FindRect(name, index, rect);
}
status_t BMessage_FindPoint(const BMessage * self, const char * name, BPoint * point) {
    return self->FindPoint(name, point);
}
status_t BMessage_FindPoint1(const BMessage * self, const char * name, int32 index, BPoint * point) {
    return self->FindPoint(name, index, point);
}
status_t BMessage_FindSize(const BMessage * self, const char * name, BSize * size) {
    return self->FindSize(name, size);
}
status_t BMessage_FindSize1(const BMessage * self, const char * name, int32 index, BSize * size) {
    return self->FindSize(name, index, size);
}
status_t BMessage_FindString(const BMessage * self, const char * name, const char ** string) {
    return self->FindString(name, string);
}
status_t BMessage_FindString1(const BMessage * self, const char * name, int32 index, const char ** string) {
    return self->FindString(name, index, string);
}
status_t BMessage_FindString2(const BMessage * self, const char * name, BString * string) {
    return self->FindString(name, string);
}
status_t BMessage_FindString3(const BMessage * self, const char * name, int32 index, BString * string) {
    return self->FindString(name, index, string);
}
status_t BMessage_FindStrings(const BMessage * self, const char * name, BStringList * list) {
    return self->FindStrings(name, list);
}
status_t BMessage_FindInt8(const BMessage * self, const char * name, int8 * value) {
    return self->FindInt8(name, value);
}
status_t BMessage_FindInt81(const BMessage * self, const char * name, int32 index, int8 * value) {
    return self->FindInt8(name, index, value);
}
status_t BMessage_FindUInt8(const BMessage * self, const char * name, uint8 * value) {
    return self->FindUInt8(name, value);
}
status_t BMessage_FindUInt81(const BMessage * self, const char * name, int32 index, uint8 * value) {
    return self->FindUInt8(name, index, value);
}
status_t BMessage_FindInt16(const BMessage * self, const char * name, int16 * value) {
    return self->FindInt16(name, value);
}
status_t BMessage_FindInt161(const BMessage * self, const char * name, int32 index, int16 * value) {
    return self->FindInt16(name, index, value);
}
status_t BMessage_FindUInt16(const BMessage * self, const char * name, uint16 * value) {
    return self->FindUInt16(name, value);
}
status_t BMessage_FindUInt161(const BMessage * self, const char * name, int32 index, uint16 * value) {
    return self->FindUInt16(name, index, value);
}
status_t BMessage_FindInt321(const BMessage * self, const char * name, int32 index, int32 * value) {
    return self->FindInt32(name, index, value);
}
status_t BMessage_FindUInt32(const BMessage * self, const char * name, uint32 * value) {
    return self->FindUInt32(name, value);
}
status_t BMessage_FindUInt321(const BMessage * self, const char * name, int32 index, uint32 * value) {
    return self->FindUInt32(name, index, value);
}
status_t BMessage_FindInt64(const BMessage * self, const char * name, int64 * value) {
    return self->FindInt64(name, value);
}
status_t BMessage_FindInt641(const BMessage * self, const char * name, int32 index, int64 * value) {
    return self->FindInt64(name, index, value);
}
status_t BMessage_FindUInt64(const BMessage * self, const char * name, uint64 * value) {
    return self->FindUInt64(name, value);
}
status_t BMessage_FindUInt641(const BMessage * self, const char * name, int32 index, uint64 * value) {
    return self->FindUInt64(name, index, value);
}
status_t BMessage_FindBool(const BMessage * self, const char * name, bool * value) {
    return self->FindBool(name, value);
}
status_t BMessage_FindBool1(const BMessage * self, const char * name, int32 index, bool * value) {
    return self->FindBool(name, index, value);
}
status_t BMessage_FindFloat(const BMessage * self, const char * name, float * value) {
    return self->FindFloat(name, value);
}
status_t BMessage_FindFloat1(const BMessage * self, const char * name, int32 index, float * value) {
    return self->FindFloat(name, index, value);
}
status_t BMessage_FindDouble(const BMessage * self, const char * name, double * value) {
    return self->FindDouble(name, value);
}
status_t BMessage_FindDouble1(const BMessage * self, const char * name, int32 index, double * value) {
    return self->FindDouble(name, index, value);
}
status_t BMessage_FindColor(const BMessage * self, const char * name, rgb_color * value) {
    return self->FindColor(name, value);
}
status_t BMessage_FindColor1(const BMessage * self, const char * name, int32 index, rgb_color * value) {
    return self->FindColor(name, index, value);
}
status_t BMessage_FindPointer(const BMessage * self, const char * name, void ** pointer) {
    return self->FindPointer(name, pointer);
}
status_t BMessage_FindPointer1(const BMessage * self, const char * name, int32 index, void ** pointer) {
    return self->FindPointer(name, index, pointer);
}
status_t BMessage_FindMessenger(const BMessage * self, const char * name, BMessenger * messenger) {
    return self->FindMessenger(name, messenger);
}
status_t BMessage_FindMessenger1(const BMessage * self, const char * name, int32 index, BMessenger * messenger) {
    return self->FindMessenger(name, index, messenger);
}
status_t BMessage_FindRef(const BMessage * self, const char * name, entry_ref * ref_) {
    return self->FindRef(name, ref_);
}
status_t BMessage_FindRef1(const BMessage * self, const char * name, int32 index, entry_ref * ref_) {
    return self->FindRef(name, index, ref_);
}
status_t BMessage_FindNodeRef(const BMessage * self, const char * name, node_ref * ref_) {
    return self->FindNodeRef(name, ref_);
}
status_t BMessage_FindNodeRef1(const BMessage * self, const char * name, int32 index, node_ref * ref_) {
    return self->FindNodeRef(name, index, ref_);
}
status_t BMessage_FindMessage(const BMessage * self, const char * name, BMessage * message) {
    return self->FindMessage(name, message);
}
status_t BMessage_FindMessage1(const BMessage * self, const char * name, int32 index, BMessage * message) {
    return self->FindMessage(name, index, message);
}
status_t BMessage_FindFlat(const BMessage * self, const char * name, BFlattenable * object) {
    return self->FindFlat(name, object);
}
status_t BMessage_FindFlat1(const BMessage * self, const char * name, int32 index, BFlattenable * object) {
    return self->FindFlat(name, index, object);
}
status_t BMessage_ReplaceAlignment(BMessage * self, const char * name, const BAlignment * alignment) {
    return self->ReplaceAlignment(name, *alignment);
}
status_t BMessage_ReplaceAlignment1(BMessage * self, const char * name, int32 index, const BAlignment * alignment) {
    return self->ReplaceAlignment(name, index, *alignment);
}
status_t BMessage_ReplaceString(BMessage * self, const char * name, const char * string) {
    return self->ReplaceString(name, string);
}
status_t BMessage_ReplaceString1(BMessage * self, const char * name, int32 index, const char * string) {
    return self->ReplaceString(name, index, string);
}
status_t BMessage_ReplaceString2(BMessage * self, const char * name, const BString * string) {
    return self->ReplaceString(name, *string);
}
status_t BMessage_ReplaceString3(BMessage * self, const char * name, int32 index, const BString * string) {
    return self->ReplaceString(name, index, *string);
}
status_t BMessage_ReplaceInt8(BMessage * self, const char * name, int8 value) {
    return self->ReplaceInt8(name, value);
}
status_t BMessage_ReplaceInt81(BMessage * self, const char * name, int32 index, int8 value) {
    return self->ReplaceInt8(name, index, value);
}
status_t BMessage_ReplaceUInt8(BMessage * self, const char * name, uint8 value) {
    return self->ReplaceUInt8(name, value);
}
status_t BMessage_ReplaceUInt81(BMessage * self, const char * name, int32 index, uint8 value) {
    return self->ReplaceUInt8(name, index, value);
}
status_t BMessage_ReplaceInt16(BMessage * self, const char * name, int16 value) {
    return self->ReplaceInt16(name, value);
}
status_t BMessage_ReplaceInt161(BMessage * self, const char * name, int32 index, int16 value) {
    return self->ReplaceInt16(name, index, value);
}
status_t BMessage_ReplaceUInt16(BMessage * self, const char * name, uint16 value) {
    return self->ReplaceUInt16(name, value);
}
status_t BMessage_ReplaceUInt161(BMessage * self, const char * name, int32 index, uint16 value) {
    return self->ReplaceUInt16(name, index, value);
}
status_t BMessage_ReplaceInt32(BMessage * self, const char * name, int32 value) {
    return self->ReplaceInt32(name, value);
}
status_t BMessage_ReplaceInt321(BMessage * self, const char * name, int32 index, int32 value) {
    return self->ReplaceInt32(name, index, value);
}
status_t BMessage_ReplaceUInt32(BMessage * self, const char * name, uint32 value) {
    return self->ReplaceUInt32(name, value);
}
status_t BMessage_ReplaceUInt321(BMessage * self, const char * name, int32 index, uint32 value) {
    return self->ReplaceUInt32(name, index, value);
}
status_t BMessage_ReplaceInt64(BMessage * self, const char * name, int64 value) {
    return self->ReplaceInt64(name, value);
}
status_t BMessage_ReplaceInt641(BMessage * self, const char * name, int32 index, int64 value) {
    return self->ReplaceInt64(name, index, value);
}
status_t BMessage_ReplaceUInt64(BMessage * self, const char * name, uint64 value) {
    return self->ReplaceUInt64(name, value);
}
status_t BMessage_ReplaceUInt641(BMessage * self, const char * name, int32 index, uint64 value) {
    return self->ReplaceUInt64(name, index, value);
}
status_t BMessage_ReplaceBool(BMessage * self, const char * name, bool a_boolean) {
    return self->ReplaceBool(name, a_boolean);
}
status_t BMessage_ReplaceBool1(BMessage * self, const char * name, int32 index, bool value) {
    return self->ReplaceBool(name, index, value);
}
status_t BMessage_ReplaceFloat(BMessage * self, const char * name, float value) {
    return self->ReplaceFloat(name, value);
}
status_t BMessage_ReplaceFloat1(BMessage * self, const char * name, int32 index, float value) {
    return self->ReplaceFloat(name, index, value);
}
status_t BMessage_ReplaceDouble(BMessage * self, const char * name, double value) {
    return self->ReplaceDouble(name, value);
}
status_t BMessage_ReplaceDouble1(BMessage * self, const char * name, int32 index, double value) {
    return self->ReplaceDouble(name, index, value);
}
status_t BMessage_ReplacePointer(BMessage * self, const char * name, const void * pointer) {
    return self->ReplacePointer(name, pointer);
}
status_t BMessage_ReplacePointer1(BMessage * self, const char * name, int32 index, const void * pointer) {
    return self->ReplacePointer(name, index, pointer);
}
status_t BMessage_ReplaceRef(BMessage * self, const char * name, const entry_ref * ref_) {
    return self->ReplaceRef(name, ref_);
}
status_t BMessage_ReplaceRef1(BMessage * self, const char * name, int32 index, const entry_ref * ref_) {
    return self->ReplaceRef(name, index, ref_);
}
status_t BMessage_ReplaceNodeRef(BMessage * self, const char * name, const node_ref * ref_) {
    return self->ReplaceNodeRef(name, ref_);
}
status_t BMessage_ReplaceNodeRef1(BMessage * self, const char * name, int32 index, const node_ref * ref_) {
    return self->ReplaceNodeRef(name, index, ref_);
}
status_t BMessage_ReplaceMessage(BMessage * self, const char * name, const BMessage * message) {
    return self->ReplaceMessage(name, message);
}
status_t BMessage_ReplaceMessage1(BMessage * self, const char * name, int32 index, const BMessage * message) {
    return self->ReplaceMessage(name, index, message);
}
status_t BMessage_ReplaceFlat(BMessage * self, const char * name, BFlattenable * object) {
    return self->ReplaceFlat(name, object);
}
status_t BMessage_ReplaceFlat1(BMessage * self, const char * name, int32 index, BFlattenable * object) {
    return self->ReplaceFlat(name, index, object);
}
bool BMessage_HasSameData(const BMessage * self, const BMessage * other, bool ignore_field_order, bool deep) {
    return self->HasSameData(*other, ignore_field_order, deep);
}
bool BMessage_HasAlignment(const BMessage * self, const char * name, int32 n) {
    return self->HasAlignment(name, n);
}
bool BMessage_HasRect(const BMessage * self, const char * name, int32 n) {
    return self->HasRect(name, n);
}
bool BMessage_HasPoint(const BMessage * self, const char * name, int32 n) {
    return self->HasPoint(name, n);
}
bool BMessage_HasSize(const BMessage * self, const char * name, int32 n) {
    return self->HasSize(name, n);
}
bool BMessage_HasString(const BMessage * self, const char * name, int32 n) {
    return self->HasString(name, n);
}
bool BMessage_HasInt8(const BMessage * self, const char * name, int32 n) {
    return self->HasInt8(name, n);
}
bool BMessage_HasUInt8(const BMessage * self, const char * name, int32 n) {
    return self->HasUInt8(name, n);
}
bool BMessage_HasInt16(const BMessage * self, const char * name, int32 n) {
    return self->HasInt16(name, n);
}
bool BMessage_HasUInt16(const BMessage * self, const char * name, int32 n) {
    return self->HasUInt16(name, n);
}
bool BMessage_HasInt32(const BMessage * self, const char * name, int32 n) {
    return self->HasInt32(name, n);
}
bool BMessage_HasUInt32(const BMessage * self, const char * name, int32 n) {
    return self->HasUInt32(name, n);
}
bool BMessage_HasInt64(const BMessage * self, const char * name, int32 n) {
    return self->HasInt64(name, n);
}
bool BMessage_HasUInt64(const BMessage * self, const char * name, int32 n) {
    return self->HasUInt64(name, n);
}
bool BMessage_HasBool(const BMessage * self, const char * name, int32 n) {
    return self->HasBool(name, n);
}
bool BMessage_HasFloat(const BMessage * self, const char * name, int32 n) {
    return self->HasFloat(name, n);
}
bool BMessage_HasDouble(const BMessage * self, const char * name, int32 n) {
    return self->HasDouble(name, n);
}
bool BMessage_HasColor(const BMessage * self, const char * name, int32 n) {
    return self->HasColor(name, n);
}
bool BMessage_HasPointer(const BMessage * self, const char * name, int32 n) {
    return self->HasPointer(name, n);
}
bool BMessage_HasMessenger(const BMessage * self, const char * name, int32 n) {
    return self->HasMessenger(name, n);
}
bool BMessage_HasRef(const BMessage * self, const char * name, int32 n) {
    return self->HasRef(name, n);
}
bool BMessage_HasNodeRef(const BMessage * self, const char * name, int32 n) {
    return self->HasNodeRef(name, n);
}
bool BMessage_HasMessage(const BMessage * self, const char * name, int32 n) {
    return self->HasMessage(name, n);
}
bool BMessage_HasFlat(const BMessage * self, const char * name, const BFlattenable * object) {
    return self->HasFlat(name, object);
}
bool BMessage_HasFlat1(const BMessage * self, const char * name, int32 n, const BFlattenable * object) {
    return self->HasFlat(name, n, object);
}
const char *BMessage_FindString4(const BMessage * self, const char * name, int32 n) {
    return self->FindString(name, n);
}
int8 BMessage_FindInt82(const BMessage * self, const char * name, int32 n) {
    return self->FindInt8(name, n);
}
int16 BMessage_FindInt162(const BMessage * self, const char * name, int32 n) {
    return self->FindInt16(name, n);
}
int64 BMessage_FindInt642(const BMessage * self, const char * name, int32 n) {
    return self->FindInt64(name, n);
}
bool BMessage_FindBool2(const BMessage * self, const char * name, int32 n) {
    return self->FindBool(name, n);
}
float BMessage_FindFloat2(const BMessage * self, const char * name, int32 n) {
    return self->FindFloat(name, n);
}
double BMessage_FindDouble2(const BMessage * self, const char * name, int32 n) {
    return self->FindDouble(name, n);
}
bool BMessage_GetBool(const BMessage * self, const char * name, bool default_value) {
    return self->GetBool(name, default_value);
}
bool BMessage_GetBool1(const BMessage * self, const char * name, int32 index, bool default_value) {
    return self->GetBool(name, index, default_value);
}
int8 BMessage_GetInt8(const BMessage * self, const char * name, int8 default_value) {
    return self->GetInt8(name, default_value);
}
int8 BMessage_GetInt81(const BMessage * self, const char * name, int32 index, int8 default_value) {
    return self->GetInt8(name, index, default_value);
}
uint8 BMessage_GetUInt8(const BMessage * self, const char * name, uint8 default_value) {
    return self->GetUInt8(name, default_value);
}
uint8 BMessage_GetUInt81(const BMessage * self, const char * name, int32 index, uint8 default_value) {
    return self->GetUInt8(name, index, default_value);
}
int16 BMessage_GetInt16(const BMessage * self, const char * name, int16 default_value) {
    return self->GetInt16(name, default_value);
}
int16 BMessage_GetInt161(const BMessage * self, const char * name, int32 index, int16 default_value) {
    return self->GetInt16(name, index, default_value);
}
uint16 BMessage_GetUInt16(const BMessage * self, const char * name, uint16 default_value) {
    return self->GetUInt16(name, default_value);
}
uint16 BMessage_GetUInt161(const BMessage * self, const char * name, int32 index, uint16 default_value) {
    return self->GetUInt16(name, index, default_value);
}
int32 BMessage_GetInt32(const BMessage * self, const char * name, int32 default_value) {
    return self->GetInt32(name, default_value);
}
int32 BMessage_GetInt321(const BMessage * self, const char * name, int32 index, int32 default_value) {
    return self->GetInt32(name, index, default_value);
}
uint32 BMessage_GetUInt32(const BMessage * self, const char * name, uint32 default_value) {
    return self->GetUInt32(name, default_value);
}
uint32 BMessage_GetUInt321(const BMessage * self, const char * name, int32 index, uint32 default_value) {
    return self->GetUInt32(name, index, default_value);
}
int64 BMessage_GetInt64(const BMessage * self, const char * name, int64 default_value) {
    return self->GetInt64(name, default_value);
}
int64 BMessage_GetInt641(const BMessage * self, const char * name, int32 index, int64 default_value) {
    return self->GetInt64(name, index, default_value);
}
uint64 BMessage_GetUInt64(const BMessage * self, const char * name, uint64 default_value) {
    return self->GetUInt64(name, default_value);
}
uint64 BMessage_GetUInt641(const BMessage * self, const char * name, int32 index, uint64 default_value) {
    return self->GetUInt64(name, index, default_value);
}
float BMessage_GetFloat(const BMessage * self, const char * name, float default_value) {
    return self->GetFloat(name, default_value);
}
float BMessage_GetFloat1(const BMessage * self, const char * name, int32 index, float default_value) {
    return self->GetFloat(name, index, default_value);
}
double BMessage_GetDouble(const BMessage * self, const char * name, double default_value) {
    return self->GetDouble(name, default_value);
}
double BMessage_GetDouble1(const BMessage * self, const char * name, int32 index, double default_value) {
    return self->GetDouble(name, index, default_value);
}
const void * BMessage_GetPointer(const BMessage * self, const char * name, int32 index, const void * default_value) {
    return self->GetPointer(name, index, default_value);
}
const void * BMessage_GetPointer1(const BMessage * self, const char * name, const void * default_value) {
    return self->GetPointer(name, default_value);
}
const char *BMessage_GetString(const BMessage * self, const char * name, const char * default_value) {
    return self->GetString(name, default_value);
}
const char *BMessage_GetString1(const BMessage * self, const char * name, int32 index, const char * default_value) {
    return self->GetString(name, index, default_value);
}
status_t BMessage_SetBool(BMessage * self, const char * name, bool value) {
    return self->SetBool(name, value);
}
status_t BMessage_SetInt8(BMessage * self, const char * name, int8 value) {
    return self->SetInt8(name, value);
}
status_t BMessage_SetUInt8(BMessage * self, const char * name, uint8 value) {
    return self->SetUInt8(name, value);
}
status_t BMessage_SetInt16(BMessage * self, const char * name, int16 value) {
    return self->SetInt16(name, value);
}
status_t BMessage_SetUInt16(BMessage * self, const char * name, uint16 value) {
    return self->SetUInt16(name, value);
}
status_t BMessage_SetInt32(BMessage * self, const char * name, int32 value) {
    return self->SetInt32(name, value);
}
status_t BMessage_SetUInt32(BMessage * self, const char * name, uint32 value) {
    return self->SetUInt32(name, value);
}
status_t BMessage_SetInt64(BMessage * self, const char * name, int64 value) {
    return self->SetInt64(name, value);
}
status_t BMessage_SetUInt64(BMessage * self, const char * name, uint64 value) {
    return self->SetUInt64(name, value);
}
status_t BMessage_SetPointer(BMessage * self, const char * name, const void * value) {
    return self->SetPointer(name, value);
}
status_t BMessage_SetString(BMessage * self, const char * name, const char * string) {
    return self->SetString(name, string);
}
status_t BMessage_SetString1(BMessage * self, const char * name, const BString * string) {
    return self->SetString(name, *string);
}
status_t BMessage_SetFloat(BMessage * self, const char * name, float value) {
    return self->SetFloat(name, value);
}
status_t BMessage_SetDouble(BMessage * self, const char * name, double value) {
    return self->SetDouble(name, value);
}
status_t BMessage_SetAlignment(BMessage * self, const char * name, const BAlignment * value) {
    return self->SetAlignment(name, *value);
}
status_t BMessage_SetPoint(BMessage * self, const char * name, const BPoint * value) {
    return self->SetPoint(name, *value);
}
status_t BMessage_SetRect(BMessage * self, const char * name, const BRect * value) {
    return self->SetRect(name, *value);
}
status_t BMessage_SetSize(BMessage * self, const char * name, const BSize * value) {
    return self->SetSize(name, *value);
}
BMessage *BMessage_new() {
    return new BMessage();
}
BMessage *BMessage_new1(const BMessage * other) {
    return new BMessage(*other);
}
BMessage *BMessage_new2(uint32 what) {
    return new BMessage(what);
}

} // extern "C"


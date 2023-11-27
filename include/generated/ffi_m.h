#pragma once

#include <Message.h>

extern "C" {

// CLASS: BMessage
void BMessage_delete(BMessage *self);
status_t BMessage_GetInfo1(const BMessage * self, const char * name, type_code * type_found, int32 * count_found);
status_t BMessage_GetInfo2(const BMessage * self, const char * name, type_code * type_found, bool * fixed_size);
status_t BMessage_GetInfo3(const BMessage * self, const char * name, type_code * type_found, int32 * count_found, bool * fixed_size);
bool BMessage_IsEmpty(const BMessage * self);
bool BMessage_IsSystem(const BMessage * self);
bool BMessage_IsReply(const BMessage * self);
void BMessage_PrintToStream(const BMessage * self);
status_t BMessage_Rename(BMessage * self, const char * old_entry, const char * new_entry);
bool BMessage_WasDelivered(const BMessage * self);
bool BMessage_IsSourceWaiting(const BMessage * self);
bool BMessage_IsSourceRemote(const BMessage * self);
const BMessage * BMessage_Previous(const BMessage * self);
bool BMessage_WasDropped(const BMessage * self);
status_t BMessage_SendReply(BMessage * self, uint32 command, BHandler * reply_to);
status_t BMessage_SendReply1(BMessage * self, BMessage * reply, BHandler * reply_to, bigtime_t timeout);
status_t BMessage_SendReply3(BMessage * self, uint32 command, BMessage * reply_to_reply);
status_t BMessage_SendReply4(BMessage * self, BMessage * reply, BMessage * reply_to_reply, bigtime_t send_timeout, bigtime_t reply_timeout);
status_t BMessage_Flatten1(const BMessage * self, BDataIO * stream, ssize_t * size);
status_t BMessage_Unflatten(BMessage * self, const char * flat_buffer);
status_t BMessage_Unflatten1(BMessage * self, BDataIO * stream);
status_t BMessage_AddSpecifier(BMessage * self, const char * property);
status_t BMessage_AddSpecifier1(BMessage * self, const char * property, int32 index);
status_t BMessage_AddSpecifier2(BMessage * self, const char * property, int32 index, int32 range);
status_t BMessage_AddSpecifier3(BMessage * self, const char * property, const char * name);
status_t BMessage_AddSpecifier4(BMessage * self, const BMessage * specifier);
status_t BMessage_SetCurrentSpecifier(BMessage * self, int32 index);
status_t BMessage_GetCurrentSpecifier(const BMessage * self, int32 * index, BMessage * specifier, int32 * what, const char ** property);
bool BMessage_HasSpecifiers(const BMessage * self);
status_t BMessage_PopSpecifier(BMessage * self);
status_t BMessage_AddAlignment(BMessage * self, const char * name, const BAlignment * alignment);
status_t BMessage_AddString(BMessage * self, const char * name, const char * string);
status_t BMessage_AddString1(BMessage * self, const char * name, const BString * string);
status_t BMessage_AddStrings(BMessage * self, const char * name, const BStringList * list);
status_t BMessage_AddInt8(BMessage * self, const char * name, int8 value);
status_t BMessage_AddUInt8(BMessage * self, const char * name, uint8 value);
status_t BMessage_AddInt16(BMessage * self, const char * name, int16 value);
status_t BMessage_AddUInt16(BMessage * self, const char * name, uint16 value);
status_t BMessage_AddInt32(BMessage * self, const char * name, int32 value);
status_t BMessage_AddUInt32(BMessage * self, const char * name, uint32 value);
status_t BMessage_AddInt64(BMessage * self, const char * name, int64 value);
status_t BMessage_AddUInt64(BMessage * self, const char * name, uint64 value);
status_t BMessage_AddBool(BMessage * self, const char * name, bool value);
status_t BMessage_AddDouble(BMessage * self, const char * name, double value);
status_t BMessage_AddPointer(BMessage * self, const char * name, const void * pointer);
status_t BMessage_AddRef(BMessage * self, const char * name, const entry_ref * ref_);
status_t BMessage_AddNodeRef(BMessage * self, const char * name, const node_ref * ref_);
status_t BMessage_AddMessage(BMessage * self, const char * name, const BMessage * message);
status_t BMessage_AddFlat1(BMessage * self, const char * name, const BFlattenable * object, int32 count);
status_t BMessage_Append(BMessage * self, const BMessage * message);
status_t BMessage_RemoveData(BMessage * self, const char * name, int32 index);
status_t BMessage_RemoveName(BMessage * self, const char * name);
status_t BMessage_MakeEmpty(BMessage * self);
status_t BMessage_FindAlignment(const BMessage * self, const char * name, BAlignment * alignment);
status_t BMessage_FindAlignment1(const BMessage * self, const char * name, int32 index, BAlignment * alignment);
status_t BMessage_FindRect(const BMessage * self, const char * name, BRect * rect);
status_t BMessage_FindRect1(const BMessage * self, const char * name, int32 index, BRect * rect);
status_t BMessage_FindPoint(const BMessage * self, const char * name, BPoint * point);
status_t BMessage_FindPoint1(const BMessage * self, const char * name, int32 index, BPoint * point);
status_t BMessage_FindSize(const BMessage * self, const char * name, BSize * size);
status_t BMessage_FindSize1(const BMessage * self, const char * name, int32 index, BSize * size);
status_t BMessage_FindString(const BMessage * self, const char * name, const char ** string);
status_t BMessage_FindString1(const BMessage * self, const char * name, int32 index, const char ** string);
status_t BMessage_FindString2(const BMessage * self, const char * name, BString * string);
status_t BMessage_FindString3(const BMessage * self, const char * name, int32 index, BString * string);
status_t BMessage_FindStrings(const BMessage * self, const char * name, BStringList * list);
status_t BMessage_FindInt8(const BMessage * self, const char * name, int8 * value);
status_t BMessage_FindInt81(const BMessage * self, const char * name, int32 index, int8 * value);
status_t BMessage_FindUInt8(const BMessage * self, const char * name, uint8 * value);
status_t BMessage_FindUInt81(const BMessage * self, const char * name, int32 index, uint8 * value);
status_t BMessage_FindInt16(const BMessage * self, const char * name, int16 * value);
status_t BMessage_FindInt161(const BMessage * self, const char * name, int32 index, int16 * value);
status_t BMessage_FindUInt16(const BMessage * self, const char * name, uint16 * value);
status_t BMessage_FindUInt161(const BMessage * self, const char * name, int32 index, uint16 * value);
status_t BMessage_FindInt321(const BMessage * self, const char * name, int32 index, int32 * value);
status_t BMessage_FindUInt32(const BMessage * self, const char * name, uint32 * value);
status_t BMessage_FindUInt321(const BMessage * self, const char * name, int32 index, uint32 * value);
status_t BMessage_FindInt64(const BMessage * self, const char * name, int64 * value);
status_t BMessage_FindInt641(const BMessage * self, const char * name, int32 index, int64 * value);
status_t BMessage_FindUInt64(const BMessage * self, const char * name, uint64 * value);
status_t BMessage_FindUInt641(const BMessage * self, const char * name, int32 index, uint64 * value);
status_t BMessage_FindBool(const BMessage * self, const char * name, bool * value);
status_t BMessage_FindBool1(const BMessage * self, const char * name, int32 index, bool * value);
status_t BMessage_FindFloat(const BMessage * self, const char * name, float * value);
status_t BMessage_FindFloat1(const BMessage * self, const char * name, int32 index, float * value);
status_t BMessage_FindDouble(const BMessage * self, const char * name, double * value);
status_t BMessage_FindDouble1(const BMessage * self, const char * name, int32 index, double * value);
status_t BMessage_FindColor(const BMessage * self, const char * name, rgb_color * value);
status_t BMessage_FindColor1(const BMessage * self, const char * name, int32 index, rgb_color * value);
status_t BMessage_FindPointer(const BMessage * self, const char * name, void ** pointer);
status_t BMessage_FindPointer1(const BMessage * self, const char * name, int32 index, void ** pointer);
status_t BMessage_FindMessenger(const BMessage * self, const char * name, BMessenger * messenger);
status_t BMessage_FindMessenger1(const BMessage * self, const char * name, int32 index, BMessenger * messenger);
status_t BMessage_FindRef(const BMessage * self, const char * name, entry_ref * ref_);
status_t BMessage_FindRef1(const BMessage * self, const char * name, int32 index, entry_ref * ref_);
status_t BMessage_FindNodeRef(const BMessage * self, const char * name, node_ref * ref_);
status_t BMessage_FindNodeRef1(const BMessage * self, const char * name, int32 index, node_ref * ref_);
status_t BMessage_FindMessage(const BMessage * self, const char * name, BMessage * message);
status_t BMessage_FindMessage1(const BMessage * self, const char * name, int32 index, BMessage * message);
status_t BMessage_FindFlat(const BMessage * self, const char * name, BFlattenable * object);
status_t BMessage_FindFlat1(const BMessage * self, const char * name, int32 index, BFlattenable * object);
status_t BMessage_ReplaceAlignment(BMessage * self, const char * name, const BAlignment * alignment);
status_t BMessage_ReplaceAlignment1(BMessage * self, const char * name, int32 index, const BAlignment * alignment);
status_t BMessage_ReplaceString(BMessage * self, const char * name, const char * string);
status_t BMessage_ReplaceString1(BMessage * self, const char * name, int32 index, const char * string);
status_t BMessage_ReplaceString2(BMessage * self, const char * name, const BString * string);
status_t BMessage_ReplaceString3(BMessage * self, const char * name, int32 index, const BString * string);
status_t BMessage_ReplaceInt8(BMessage * self, const char * name, int8 value);
status_t BMessage_ReplaceInt81(BMessage * self, const char * name, int32 index, int8 value);
status_t BMessage_ReplaceUInt8(BMessage * self, const char * name, uint8 value);
status_t BMessage_ReplaceUInt81(BMessage * self, const char * name, int32 index, uint8 value);
status_t BMessage_ReplaceInt16(BMessage * self, const char * name, int16 value);
status_t BMessage_ReplaceInt161(BMessage * self, const char * name, int32 index, int16 value);
status_t BMessage_ReplaceUInt16(BMessage * self, const char * name, uint16 value);
status_t BMessage_ReplaceUInt161(BMessage * self, const char * name, int32 index, uint16 value);
status_t BMessage_ReplaceInt32(BMessage * self, const char * name, int32 value);
status_t BMessage_ReplaceInt321(BMessage * self, const char * name, int32 index, int32 value);
status_t BMessage_ReplaceUInt32(BMessage * self, const char * name, uint32 value);
status_t BMessage_ReplaceUInt321(BMessage * self, const char * name, int32 index, uint32 value);
status_t BMessage_ReplaceInt64(BMessage * self, const char * name, int64 value);
status_t BMessage_ReplaceInt641(BMessage * self, const char * name, int32 index, int64 value);
status_t BMessage_ReplaceUInt64(BMessage * self, const char * name, uint64 value);
status_t BMessage_ReplaceUInt641(BMessage * self, const char * name, int32 index, uint64 value);
status_t BMessage_ReplaceBool(BMessage * self, const char * name, bool a_boolean);
status_t BMessage_ReplaceBool1(BMessage * self, const char * name, int32 index, bool value);
status_t BMessage_ReplaceDouble(BMessage * self, const char * name, double value);
status_t BMessage_ReplaceDouble1(BMessage * self, const char * name, int32 index, double value);
status_t BMessage_ReplacePointer(BMessage * self, const char * name, const void * pointer);
status_t BMessage_ReplacePointer1(BMessage * self, const char * name, int32 index, const void * pointer);
status_t BMessage_ReplaceRef(BMessage * self, const char * name, const entry_ref * ref_);
status_t BMessage_ReplaceRef1(BMessage * self, const char * name, int32 index, const entry_ref * ref_);
status_t BMessage_ReplaceNodeRef(BMessage * self, const char * name, const node_ref * ref_);
status_t BMessage_ReplaceNodeRef1(BMessage * self, const char * name, int32 index, const node_ref * ref_);
status_t BMessage_ReplaceMessage(BMessage * self, const char * name, const BMessage * message);
status_t BMessage_ReplaceMessage1(BMessage * self, const char * name, int32 index, const BMessage * message);
status_t BMessage_ReplaceFlat(BMessage * self, const char * name, BFlattenable * object);
status_t BMessage_ReplaceFlat1(BMessage * self, const char * name, int32 index, BFlattenable * object);
bool BMessage_HasSameData(const BMessage * self, const BMessage * other, bool ignore_field_order, bool deep);
bool BMessage_HasAlignment(const BMessage * self, const char * name, int32 n);
bool BMessage_HasRect(const BMessage * self, const char * name, int32 n);
bool BMessage_HasPoint(const BMessage * self, const char * name, int32 n);
bool BMessage_HasSize(const BMessage * self, const char * name, int32 n);
bool BMessage_HasString(const BMessage * self, const char * name, int32 n);
bool BMessage_HasInt8(const BMessage * self, const char * name, int32 n);
bool BMessage_HasUInt8(const BMessage * self, const char * name, int32 n);
bool BMessage_HasInt16(const BMessage * self, const char * name, int32 n);
bool BMessage_HasUInt16(const BMessage * self, const char * name, int32 n);
bool BMessage_HasInt32(const BMessage * self, const char * name, int32 n);
bool BMessage_HasUInt32(const BMessage * self, const char * name, int32 n);
bool BMessage_HasInt64(const BMessage * self, const char * name, int32 n);
bool BMessage_HasUInt64(const BMessage * self, const char * name, int32 n);
bool BMessage_HasBool(const BMessage * self, const char * name, int32 n);
bool BMessage_HasFloat(const BMessage * self, const char * name, int32 n);
bool BMessage_HasDouble(const BMessage * self, const char * name, int32 n);
bool BMessage_HasColor(const BMessage * self, const char * name, int32 n);
bool BMessage_HasPointer(const BMessage * self, const char * name, int32 n);
bool BMessage_HasMessenger(const BMessage * self, const char * name, int32 n);
bool BMessage_HasRef(const BMessage * self, const char * name, int32 n);
bool BMessage_HasNodeRef(const BMessage * self, const char * name, int32 n);
bool BMessage_HasMessage(const BMessage * self, const char * name, int32 n);
bool BMessage_HasFlat(const BMessage * self, const char * name, const BFlattenable * object);
bool BMessage_HasFlat1(const BMessage * self, const char * name, int32 n, const BFlattenable * object);
const char *BMessage_FindString4(const BMessage * self, const char * name, int32 n);
int8 BMessage_FindInt82(const BMessage * self, const char * name, int32 n);
int16 BMessage_FindInt162(const BMessage * self, const char * name, int32 n);
int64 BMessage_FindInt642(const BMessage * self, const char * name, int32 n);
bool BMessage_FindBool2(const BMessage * self, const char * name, int32 n);
double BMessage_FindDouble2(const BMessage * self, const char * name, int32 n);
bool BMessage_GetBool(const BMessage * self, const char * name, bool default_value);
bool BMessage_GetBool1(const BMessage * self, const char * name, int32 index, bool default_value);
int8 BMessage_GetInt8(const BMessage * self, const char * name, int8 default_value);
int8 BMessage_GetInt81(const BMessage * self, const char * name, int32 index, int8 default_value);
uint8 BMessage_GetUInt8(const BMessage * self, const char * name, uint8 default_value);
uint8 BMessage_GetUInt81(const BMessage * self, const char * name, int32 index, uint8 default_value);
int16 BMessage_GetInt16(const BMessage * self, const char * name, int16 default_value);
int16 BMessage_GetInt161(const BMessage * self, const char * name, int32 index, int16 default_value);
uint16 BMessage_GetUInt16(const BMessage * self, const char * name, uint16 default_value);
uint16 BMessage_GetUInt161(const BMessage * self, const char * name, int32 index, uint16 default_value);
int32 BMessage_GetInt32(const BMessage * self, const char * name, int32 default_value);
int32 BMessage_GetInt321(const BMessage * self, const char * name, int32 index, int32 default_value);
uint32 BMessage_GetUInt32(const BMessage * self, const char * name, uint32 default_value);
uint32 BMessage_GetUInt321(const BMessage * self, const char * name, int32 index, uint32 default_value);
int64 BMessage_GetInt64(const BMessage * self, const char * name, int64 default_value);
int64 BMessage_GetInt641(const BMessage * self, const char * name, int32 index, int64 default_value);
uint64 BMessage_GetUInt64(const BMessage * self, const char * name, uint64 default_value);
uint64 BMessage_GetUInt641(const BMessage * self, const char * name, int32 index, uint64 default_value);
double BMessage_GetDouble(const BMessage * self, const char * name, double default_value);
double BMessage_GetDouble1(const BMessage * self, const char * name, int32 index, double default_value);
const void * BMessage_GetPointer(const BMessage * self, const char * name, int32 index, const void * default_value);
const void * BMessage_GetPointer1(const BMessage * self, const char * name, const void * default_value);
const char *BMessage_GetString(const BMessage * self, const char * name, const char * default_value);
const char *BMessage_GetString1(const BMessage * self, const char * name, int32 index, const char * default_value);
status_t BMessage_SetBool(BMessage * self, const char * name, bool value);
status_t BMessage_SetInt8(BMessage * self, const char * name, int8 value);
status_t BMessage_SetUInt8(BMessage * self, const char * name, uint8 value);
status_t BMessage_SetInt16(BMessage * self, const char * name, int16 value);
status_t BMessage_SetUInt16(BMessage * self, const char * name, uint16 value);
status_t BMessage_SetInt32(BMessage * self, const char * name, int32 value);
status_t BMessage_SetUInt32(BMessage * self, const char * name, uint32 value);
status_t BMessage_SetInt64(BMessage * self, const char * name, int64 value);
status_t BMessage_SetUInt64(BMessage * self, const char * name, uint64 value);
status_t BMessage_SetPointer(BMessage * self, const char * name, const void * value);
status_t BMessage_SetString(BMessage * self, const char * name, const char * string);
status_t BMessage_SetString1(BMessage * self, const char * name, const BString * string);
status_t BMessage_SetDouble(BMessage * self, const char * name, double value);
status_t BMessage_SetAlignment(BMessage * self, const char * name, const BAlignment * value);
status_t BMessage_SetPoint(BMessage * self, const char * name, const BPoint * value);
status_t BMessage_SetRect(BMessage * self, const char * name, const BRect * value);
status_t BMessage_SetSize(BMessage * self, const char * name, const BSize * value);
BMessage *BMessage_new();
BMessage *BMessage_new1(const BMessage * other);
BMessage *BMessage_new2(uint32 what);

} // extern "C"


#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

// FIXME: workaround for windows (LLP64)
#![allow(overflowing_literals)]

use std::os::raw::{c_int, c_long};

use crate::manual::*;

//  ENUM: diff_type
pub const B_HOURS_DIFF: c_int = 0;
pub const B_MINUTES_DIFF: c_int = 0 + 1;
pub const B_SECONDS_DIFF: c_int = 0 + 2;
pub const B_MILLISECONDS_DIFF: c_int = 0 + 3;
pub const B_MICROSECONDS_DIFF: c_int = 0 + 4;
//  ENUM: time_type
pub const B_GMT_TIME: c_int = 0;
pub const B_LOCAL_TIME: c_int = 0 + 1;

//  ENUM: BHttpStatusClass
pub const Invalid: c_int = 000;
pub const Informational: c_int = 100;
pub const Success: c_int = 200;
pub const Redirection: c_int = 300;
pub const ClientError: c_int = 400;
pub const ServerError: c_int = 500;
//  ENUM: BHttpStatusCode
pub const Unknown: c_int = 0;
pub const Continue: c_int = 100;
pub const SwitchingProtocols: c_int = 100 + 1;
pub const Ok: c_int = 200;
pub const Created: c_int = 200 + 1;
pub const Accepted: c_int = 200 + 2;
pub const NonAuthoritativeInformation: c_int = 200 + 3;
pub const NoContent: c_int = 200 + 4;
pub const ResetContent: c_int = 200 + 5;
pub const PartialContent: c_int = 200 + 6;
pub const MultipleChoice: c_int = 300;
pub const MovedPermanently: c_int = 300 + 1;
pub const Found: c_int = 300 + 2;
pub const SeeOther: c_int = 300 + 3;
pub const NotModified: c_int = 300 + 4;
pub const UseProxy: c_int = 300 + 5;
pub const TemporaryRedirect: c_int = 307;
pub const PermanentRedirect: c_int = 307 + 1;
pub const BadRequest: c_int = 400;
pub const Unauthorized: c_int = 400 + 1;
pub const PaymentRequired: c_int = 400 + 2;
pub const Forbidden: c_int = 400 + 3;
pub const NotFound: c_int = 400 + 4;
pub const MethodNotAllowed: c_int = 400 + 5;
pub const NotAcceptable: c_int = 400 + 6;
pub const ProxyAuthenticationRequired: c_int = 400 + 7;
pub const RequestTimeout: c_int = 400 + 8;
pub const Conflict: c_int = 400 + 9;
pub const Gone: c_int = 400 + 10;
pub const LengthRequired: c_int = 400 + 11;
pub const PreconditionFailed: c_int = 400 + 12;
pub const RequestEntityTooLarge: c_int = 400 + 13;
pub const RequestUriTooLarge: c_int = 400 + 14;
pub const UnsupportedMediaType: c_int = 400 + 15;
pub const RequestedRangeNotSatisfiable: c_int = 400 + 16;
pub const ExpectationFailed: c_int = 400 + 17;
pub const InternalServerError: c_int = 500;
pub const NotImplemented: c_int = 500 + 1;
pub const BadGateway: c_int = 500 + 2;
pub const ServiceUnavailable: c_int = 500 + 3;
pub const GatewayTimeout: c_int = 500 + 4;
//  ENUM: BHttpTimeFormat
pub const RFC1123: c_int = 0;
pub const RFC850: c_int = 0 + 1;
pub const AscTime: c_int = 0 + 2;

//  ENUM: @81
pub const HttpStatus: c_int = 0x5f485354 /* '_HST' */;
pub const HttpFields: c_int = 0x5f484846 /* '_HHF' */;
pub const CertificateError: c_int = 0x5f434552 /* '_CER' */;
pub const HttpRedirect: c_int = 0x5f485245 /* '_HRE' */;
//  ENUM: @82
pub const HostNameResolved: c_int = 0x5f4e4852 /* '_NHR' */;
pub const ConnectionOpened: c_int = 0x5f4e434f /* '_NCO' */;
pub const UploadProgress: c_int = 0x5f4e5550 /* '_NUP' */;
pub const ResponseStarted: c_int = 0x5f4e5253 /* '_NRS' */;
pub const DownloadProgress: c_int = 0x5f4e4450 /* '_NDP' */;
pub const BytesWritten: c_int = 0x5f4e4257 /* '_NBW' */;
pub const RequestCompleted: c_int = 0x5f4e5243 /* '_NRC' */;
pub const DebugMessage: c_int = 0x5f4e4442 /* '_NDB' */;

//  ENUM: @83
pub const DebugInfo: c_int = 0x5f444249 /* '_DBI' */;
pub const DebugWarning: c_int = 0x5f444257 /* '_DBW' */;
pub const DebugError: c_int = 0x5f444245 /* '_DBE' */;

//  ENUM: BJobState
pub const B_JOB_STATE_WAITING_TO_RUN: c_int = 0;
pub const B_JOB_STATE_STARTED: c_int = 0 + 1;
pub const B_JOB_STATE_IN_PROGRESS: c_int = 0 + 2;
pub const B_JOB_STATE_SUCCEEDED: c_int = 0 + 3;
pub const B_JOB_STATE_FAILED: c_int = 0 + 4;
pub const B_JOB_STATE_ABORTED: c_int = 0 + 5;

//  ENUM: @0
pub const B_ABOUT_REQUESTED: c_int = 0x5f414252 /* '_ABR' */;
pub const B_WINDOW_ACTIVATED: c_int = 0x5f414354 /* '_ACT' */;
pub const B_APP_ACTIVATED: c_int = 0x5f414354 /* '_ACT' */;
pub const B_ARGV_RECEIVED: c_int = 0x5f415247 /* '_ARG' */;
pub const B_QUIT_REQUESTED: c_int = 0x5f515251 /* '_QRQ' */;
pub const B_CLOSE_REQUESTED: c_int = 0x5f515251 /* '_QRQ' */;
pub const B_CANCEL: c_int = 0x5f434e43 /* '_CNC' */;
pub const B_INVALIDATE: c_int = 0x5f49564c /* '_IVL' */;
pub const B_KEY_DOWN: c_int = 0x5f4b5944 /* '_KYD' */;
pub const B_KEY_UP: c_int = 0x5f4b5955 /* '_KYU' */;
pub const B_UNMAPPED_KEY_DOWN: c_int = 0x5f554b44 /* '_UKD' */;
pub const B_UNMAPPED_KEY_UP: c_int = 0x5f554b55 /* '_UKU' */;
pub const B_KEY_MAP_LOADED: c_int = 0x5f4b4d4c /* '_KML' */;
pub const B_LAYOUT_WINDOW: c_int = 0x5f4c4159 /* '_LAY' */;
pub const B_MODIFIERS_CHANGED: c_int = 0x5f4d4348 /* '_MCH' */;
pub const B_MINIMIZE: c_int = 0x5f574d4e /* '_WMN' */;
pub const B_MOUSE_DOWN: c_int = 0x5f4d444e /* '_MDN' */;
pub const B_MOUSE_MOVED: c_int = 0x5f4d4d56 /* '_MMV' */;
pub const B_MOUSE_ENTER_EXIT: c_int = 0x5f4d4558 /* '_MEX' */;
pub const B_MOUSE_IDLE: c_int = 0x5f4d5349 /* '_MSI' */;
pub const B_MOUSE_UP: c_int = 0x5f4d5550 /* '_MUP' */;
pub const B_MOUSE_WHEEL_CHANGED: c_int = 0x5f4d5743 /* '_MWC' */;
pub const B_OPEN_IN_WORKSPACE: c_int = 0x5f4f5753 /* '_OWS' */;
pub const B_PACKAGE_UPDATE: c_int = 0x5f504b55 /* '_PKU' */;
pub const B_PRINTER_CHANGED: c_int = 0x5f504348 /* '_PCH' */;
pub const B_PULSE: c_int = 0x5f50554c /* '_PUL' */;
pub const B_READY_TO_RUN: c_int = 0x5f525452 /* '_RTR' */;
pub const B_REFS_RECEIVED: c_int = 0x5f525243 /* '_RRC' */;
pub const B_RELEASE_OVERLAY_LOCK: c_int = 0x5f524f56 /* '_ROV' */;
pub const B_ACQUIRE_OVERLAY_LOCK: c_int = 0x5f414f56 /* '_AOV' */;
pub const B_SCREEN_CHANGED: c_int = 0x5f534348 /* '_SCH' */;
pub const B_VALUE_CHANGED: c_int = 0x5f564348 /* '_VCH' */;
pub const B_TRANSLATOR_ADDED: c_int = 0x5f415254 /* '_ART' */;
pub const B_TRANSLATOR_REMOVED: c_int = 0x5f525254 /* '_RRT' */;
pub const B_DELETE_TRANSLATOR: c_int = 0x5f445254 /* '_DRT' */;
pub const B_VIEW_MOVED: c_int = 0x5f564d56 /* '_VMV' */;
pub const B_VIEW_RESIZED: c_int = 0x5f565253 /* '_VRS' */;
pub const B_WINDOW_MOVED: c_int = 0x5f574d56 /* '_WMV' */;
pub const B_WINDOW_RESIZED: c_int = 0x5f575253 /* '_WRS' */;
pub const B_WORKSPACES_CHANGED: c_int = 0x5f574347 /* '_WCG' */;
pub const B_WORKSPACE_ACTIVATED: c_int = 0x5f574143 /* '_WAC' */;
pub const B_ZOOM: c_int = 0x5f575a4d /* '_WZM' */;
pub const B_COLORS_UPDATED: c_int = 0x5f434c55 /* '_CLU' */;
pub const B_FONTS_UPDATED: c_int = 0x5f464e55 /* '_FNU' */;
pub const B_TRACKER_ADDON_MESSAGE: c_int = 0x5f54414d /* '_TAM' */;
pub const _APP_MENU_: c_int = 0x5f414d4e /* '_AMN' */;
pub const _BROWSER_MENUS_: c_int = 0x5f42524d /* '_BRM' */;
pub const _MENU_EVENT_: c_int = 0x5f4d4556 /* '_MEV' */;
pub const _PING_: c_int = 0x5f50424c /* '_PBL' */;
pub const _QUIT_: c_int = 0x5f514954 /* '_QIT' */;
pub const _VOLUME_MOUNTED_: c_int = 0x5f4e564c /* '_NVL' */;
pub const _VOLUME_UNMOUNTED_: c_int = 0x5f56524d /* '_VRM' */;
pub const _MESSAGE_DROPPED_: c_int = 0x5f4d4450 /* '_MDP' */;
pub const _DISPOSE_DRAG_: c_int = 0x5f445044 /* '_DPD' */;
pub const _MENUS_DONE_: c_int = 0x5f4d4e44 /* '_MND' */;
pub const _SHOW_DRAG_HANDLES_: c_int = 0x5f534448 /* '_SDH' */;
pub const _EVENTS_PENDING_: c_int = 0x5f455650 /* '_EVP' */;
pub const _UPDATE_: c_int = 0x5f555044 /* '_UPD' */;
pub const _UPDATE_IF_NEEDED_: c_int = 0x5f55504e /* '_UPN' */;
pub const _PRINTER_INFO_: c_int = 0x5f50494e /* '_PIN' */;
pub const _SETUP_PRINTER_: c_int = 0x5f535550 /* '_SUP' */;
pub const _SELECT_PRINTER_: c_int = 0x5f50534c /* '_PSL' */;
//  ENUM: @1
pub const B_SET_PROPERTY: c_int = 0x50534554 /* 'PSET' */;
pub const B_GET_PROPERTY: c_int = 0x50474554 /* 'PGET' */;
pub const B_CREATE_PROPERTY: c_int = 0x50435254 /* 'PCRT' */;
pub const B_DELETE_PROPERTY: c_int = 0x5044454c /* 'PDEL' */;
pub const B_COUNT_PROPERTIES: c_int = 0x50434e54 /* 'PCNT' */;
pub const B_EXECUTE_PROPERTY: c_int = 0x50455845 /* 'PEXE' */;
pub const B_GET_SUPPORTED_SUITES: c_int = 0x53554954 /* 'SUIT' */;
pub const B_UNDO: c_int = 0x554e444f /* 'UNDO' */;
pub const B_REDO: c_int = 0x5245444f /* 'REDO' */;
pub const B_CUT: c_int = 0x43435554 /* 'CCUT' */;
pub const B_COPY: c_int = 0x434f5059 /* 'COPY' */;
pub const B_PASTE: c_int = 0x50535445 /* 'PSTE' */;
pub const B_SELECT_ALL: c_int = 0x53414c4c /* 'SALL' */;
pub const B_SAVE_REQUESTED: c_int = 0x53415645 /* 'SAVE' */;
pub const B_MESSAGE_NOT_UNDERSTOOD: c_int = 0x4d4e4f54 /* 'MNOT' */;
pub const B_NO_REPLY: c_int = 0x4e4f4e45 /* 'NONE' */;
pub const B_REPLY: c_int = 0x52504c59 /* 'RPLY' */;
pub const B_SIMPLE_DATA: c_int = 0x44415441 /* 'DATA' */;
pub const B_MIME_DATA: c_int = 0x4d494d45 /* 'MIME' */;
pub const B_ARCHIVED_OBJECT: c_int = 0x41524356 /* 'ARCV' */;
pub const B_UPDATE_STATUS_BAR: c_int = 0x53425550 /* 'SBUP' */;
pub const B_RESET_STATUS_BAR: c_int = 0x53425253 /* 'SBRS' */;
pub const B_NODE_MONITOR: c_int = 0x4e444d4e /* 'NDMN' */;
pub const B_QUERY_UPDATE: c_int = 0x51555044 /* 'QUPD' */;
pub const B_ENDORSABLE: c_int = 0x454e444f /* 'ENDO' */;
pub const B_COPY_TARGET: c_int = 0x44444350 /* 'DDCP' */;
pub const B_MOVE_TARGET: c_int = 0x44444d56 /* 'DDMV' */;
pub const B_TRASH_TARGET: c_int = 0x4444524d /* 'DDRM' */;
pub const B_LINK_TARGET: c_int = 0x44444c4e /* 'DDLN' */;
pub const B_INPUT_DEVICES_CHANGED: c_int = 0x49444348 /* 'IDCH' */;
pub const B_INPUT_METHOD_EVENT: c_int = 0x494d4556 /* 'IMEV' */;
pub const B_WINDOW_MOVE_TO: c_int = 0x57444d54 /* 'WDMT' */;
pub const B_WINDOW_MOVE_BY: c_int = 0x57444d42 /* 'WDMB' */;
pub const B_SILENT_RELAUNCH: c_int = 0x4152454c /* 'AREL' */;
pub const B_OBSERVER_NOTICE_CHANGE: c_int = 0x4e544348 /* 'NTCH' */;
pub const B_CONTROL_INVOKED: c_int = 0x4349564b /* 'CIVK' */;
pub const B_CONTROL_MODIFIED: c_int = 0x434d4f44 /* 'CMOD' */;

//  ENUM: @2
pub const B_CLIPBOARD_CHANGED: c_int = 0x434c4348 /* 'CLCH' */;

//  ENUM: BCursorID
pub const B_CURSOR_ID_SYSTEM_DEFAULT: c_int = 1;
pub const B_CURSOR_ID_CONTEXT_MENU: c_int = 3;
pub const B_CURSOR_ID_COPY: c_int = 4;
pub const B_CURSOR_ID_CREATE_LINK: c_int = 29;
pub const B_CURSOR_ID_CROSS_HAIR: c_int = 5;
pub const B_CURSOR_ID_FOLLOW_LINK: c_int = 6;
pub const B_CURSOR_ID_GRAB: c_int = 7;
pub const B_CURSOR_ID_GRABBING: c_int = 8;
pub const B_CURSOR_ID_HELP: c_int = 9;
pub const B_CURSOR_ID_I_BEAM: c_int = 2;
pub const B_CURSOR_ID_I_BEAM_HORIZONTAL: c_int = 10;
pub const B_CURSOR_ID_MOVE: c_int = 11;
pub const B_CURSOR_ID_NO_CURSOR: c_int = 12;
pub const B_CURSOR_ID_NOT_ALLOWED: c_int = 13;
pub const B_CURSOR_ID_PROGRESS: c_int = 14;
pub const B_CURSOR_ID_RESIZE_NORTH: c_int = 15;
pub const B_CURSOR_ID_RESIZE_EAST: c_int = 16;
pub const B_CURSOR_ID_RESIZE_SOUTH: c_int = 17;
pub const B_CURSOR_ID_RESIZE_WEST: c_int = 18;
pub const B_CURSOR_ID_RESIZE_NORTH_EAST: c_int = 19;
pub const B_CURSOR_ID_RESIZE_NORTH_WEST: c_int = 20;
pub const B_CURSOR_ID_RESIZE_SOUTH_EAST: c_int = 21;
pub const B_CURSOR_ID_RESIZE_SOUTH_WEST: c_int = 22;
pub const B_CURSOR_ID_RESIZE_NORTH_SOUTH: c_int = 23;
pub const B_CURSOR_ID_RESIZE_EAST_WEST: c_int = 24;
pub const B_CURSOR_ID_RESIZE_NORTH_EAST_SOUTH_WEST: c_int = 25;
pub const B_CURSOR_ID_RESIZE_NORTH_WEST_SOUTH_EAST: c_int = 26;
pub const B_CURSOR_ID_ZOOM_IN: c_int = 27;
pub const B_CURSOR_ID_ZOOM_OUT: c_int = 28;

pub const B_OBSERVE_ORIGINAL_WHAT: &str = "be:observe_orig_what";
pub const B_OBSERVE_WHAT_CHANGE: &str = "be:observe_change_what";

//  ENUM: BKeyPurpose
pub const B_KEY_PURPOSE_ANY: c_int = 0;
pub const B_KEY_PURPOSE_GENERIC: c_int = 0 + 1;
pub const B_KEY_PURPOSE_KEYRING: c_int = 0 + 2;
pub const B_KEY_PURPOSE_WEB: c_int = 0 + 3;
pub const B_KEY_PURPOSE_NETWORK: c_int = 0 + 4;
pub const B_KEY_PURPOSE_VOLUME: c_int = 0 + 5;
//  ENUM: BKeyType
pub const B_KEY_TYPE_ANY: c_int = 0;
pub const B_KEY_TYPE_GENERIC: c_int = 0 + 1;
pub const B_KEY_TYPE_PASSWORD: c_int = 0 + 2;
pub const B_KEY_TYPE_CERTIFICATE: c_int = 0 + 3;

pub const B_LOOPER_PORT_DEFAULT_CAPACITY: c_int = 200;

pub const B_FIELD_NAME_LENGTH: c_int = 255;
pub const B_PROPERTY_NAME_LENGTH: c_int = 255;
//  ENUM: @3
pub const B_NO_SPECIFIER: c_int = 0;
pub const B_DIRECT_SPECIFIER: c_int = 1;
pub const B_INDEX_SPECIFIER: c_int = 1 + 1;
pub const B_REVERSE_INDEX_SPECIFIER: c_int = 1 + 2;
pub const B_RANGE_SPECIFIER: c_int = 1 + 3;
pub const B_REVERSE_RANGE_SPECIFIER: c_int = 1 + 4;
pub const B_NAME_SPECIFIER: c_int = 1 + 5;
pub const B_ID_SPECIFIER: c_int = 1 + 6;
pub const B_SPECIFIERS_END: c_int = 128;

//  ENUM: filter_result
pub const B_SKIP_MESSAGE: c_int = 0;
pub const B_DISPATCH_MESSAGE: c_int = 0 + 1;
//  ENUM: message_delivery
pub const B_ANY_DELIVERY: c_int = 0;
pub const B_DROPPED_DELIVERY: c_int = 0 + 1;
pub const B_PROGRAMMED_DELIVERY: c_int = 0 + 2;
//  ENUM: message_source
pub const B_ANY_SOURCE: c_int = 0;
pub const B_REMOTE_SOURCE: c_int = 0 + 1;
pub const B_LOCAL_SOURCE: c_int = 0 + 2;

//  ENUM: notification_type
pub const B_INFORMATION_NOTIFICATION: c_int = 0;
pub const B_IMPORTANT_NOTIFICATION: c_int = 0 + 1;
pub const B_ERROR_NOTIFICATION: c_int = 0 + 2;
pub const B_PROGRESS_NOTIFICATION: c_int = 0 + 3;

//  ENUM: value_kind
pub const B_COMMAND_KIND: c_int = 0;
pub const B_TYPE_CODE_KIND: c_int = 1;

pub const _B_APP_INFO_RESERVED1_: c_int = (0x10000000);
pub const B_ARGV_ONLY: c_int = (0x8);
pub const B_BACKGROUND_APP: c_int = (0x4);
pub const B_EXCLUSIVE_LAUNCH: c_int = (0x2);
pub const B_LAUNCH_MASK: c_int = (0x3);
pub const B_MULTIPLE_LAUNCH: c_int = (0x1);
pub const B_SINGLE_LAUNCH: c_int = (0x0);
//  ENUM: @5
pub const B_REQUEST_LAUNCHED: c_int = 0x00000001;
pub const B_REQUEST_QUIT: c_int = 0x00000002;
pub const B_REQUEST_ACTIVATED: c_int = 0x00000004;
//  ENUM: @6
pub const B_SOME_APP_LAUNCHED: c_int = 0x42524153 /* 'BRAS' */;
pub const B_SOME_APP_QUIT: c_int = 0x42524151 /* 'BRAQ' */;
pub const B_SOME_APP_ACTIVATED: c_int = 0x42524157 /* 'BRAW' */;

pub const B_CURRENT_FS_API_VERSION: &str = "/v1";
pub const B_STAT_SIZE_INSECURE: c_int = 0x2000;
pub const B_VNODE_DONT_CREATE_SPECIAL_SUB_NODE: c_int = 0x02;
pub const B_VNODE_PUBLISH_REMOVED: c_int = 0x01;
pub const FS_WRITE_FSINFO_NAME: c_int = 0x0001;

pub const B_USB_MODULE_NAME: &str = "bus_managers/usb/v3";
pub const USB_ISO_ASAP: c_int = 0x01;

pub const USB_REQTYPE_DEVICE_IN: c_int = 0x80;
pub const USB_REQTYPE_DEVICE_OUT: c_int = 0x00;
pub const USB_REQTYPE_ENDPOINT_IN: c_int = 0x82;
pub const USB_REQTYPE_ENDPOINT_OUT: c_int = 0x02;
pub const USB_REQTYPE_INTERFACE_IN: c_int = 0x81;
pub const USB_REQTYPE_INTERFACE_OUT: c_int = 0x01;
pub const USB_REQTYPE_OTHER_IN: c_int = 0x83;
pub const USB_REQTYPE_OTHER_OUT: c_int = 0x03;
pub const USB_REQTYPE_CLASS: c_int = 0x20;
pub const USB_REQTYPE_MASK: c_int = 0x9f;
pub const USB_REQTYPE_RESERVED: c_int = 0x60;
pub const USB_REQTYPE_STANDARD: c_int = 0x00;
pub const USB_REQTYPE_VENDOR: c_int = 0x40;
pub const USB_REQUEST_CLEAR_FEATURE: c_int = 1;
pub const USB_REQUEST_GET_CONFIGURATION: c_int = 8;
pub const USB_REQUEST_GET_DESCRIPTOR: c_int = 6;
pub const USB_REQUEST_GET_INTERFACE: c_int = 10;
pub const USB_REQUEST_GET_STATUS: c_int = 0;
pub const USB_REQUEST_SET_ADDRESS: c_int = 5;
pub const USB_REQUEST_SET_CONFIGURATION: c_int = 9;
pub const USB_REQUEST_SET_DESCRIPTOR: c_int = 7;
pub const USB_REQUEST_SET_FEATURE: c_int = 3;
pub const USB_REQUEST_SET_INTERFACE: c_int = 11;
pub const USB_REQUEST_SYNCH_FRAME: c_int = 12;
pub const USB_DESCRIPTOR_CONFIGURATION: c_int = 0x02;
pub const USB_DESCRIPTOR_DEVICE: c_int = 0x01;
pub const USB_DESCRIPTOR_ENDPOINT: c_int = 0x05;
pub const USB_DESCRIPTOR_INTERFACE: c_int = 0x04;
pub const USB_DESCRIPTOR_STRING: c_int = 0x03;
pub const USB_FEATURE_DEVICE_REMOTE_WAKEUP: c_int = 1;
pub const USB_FEATURE_ENDPOINT_HALT: c_int = 0;
pub const USB_ENDPOINT_ATTR_BULK: c_int = 0x02;
pub const USB_ENDPOINT_ATTR_CONTROL: c_int = 0x00;
pub const USB_ENDPOINT_ATTR_INTERRUPT: c_int = 0x03;
pub const USB_ENDPOINT_ATTR_ISOCHRONOUS: c_int = 0x01;
pub const USB_ENDPOINT_ATTR_MASK: c_int = 0x03;
pub const USB_ENDPOINT_ADDR_DIR_IN: c_int = 0x80;
pub const USB_ENDPOINT_ADDR_DIR_OUT: c_int = 0x00;
pub const USB_DESCRIPTOR_CS_CONFIGURATION: c_int = 0x22;
pub const USB_DESCRIPTOR_CS_DEVICE: c_int = 0x21;
pub const USB_DESCRIPTOR_CS_ENDPOINT: c_int = 0x25;
pub const USB_DESCRIPTOR_CS_INTERFACE: c_int = 0x24;
pub const USB_DESCRIPTOR_CS_STRING: c_int = 0x23;
pub const USB_ENDPOINT_ADDR_DIR_MASK: c_int = 0x80;
pub const USB_ENDPOINT_ATTR_ADAPTIVE_SYNCHRO: c_int = 0x08;
pub const USB_ENDPOINT_ATTR_ASYNCRONOUS: c_int = 0x04;
pub const USB_ENDPOINT_ATTR_DATA_USAGE: c_int = 0x00;
pub const USB_ENDPOINT_ATTR_FEEDBACK_USAGE: c_int = 0x10;
pub const USB_ENDPOINT_ATTR_IMPLICIT_USAGE: c_int = 0x20;
pub const USB_ENDPOINT_ATTR_NO_SYNCHRONIZE: c_int = 0x00;
pub const USB_ENDPOINT_ATTR_SYNCHRONIZE_MASK: c_int = 0x0C;
pub const USB_ENDPOINT_ATTR_SYNCHRONOUS: c_int = 0x0C;
pub const USB_ENDPOINT_ATTR_USAGE_MASK: c_int = 0x30;

//  ENUM: direct_buffer_state
pub const B_DIRECT_MODE_MASK: c_int = 15;
pub const B_DIRECT_START: c_int = 0;
pub const B_DIRECT_STOP: c_int = 1;
pub const B_DIRECT_MODIFY: c_int = 2;
pub const B_CLIPPING_MODIFIED: c_int = 16;
pub const B_BUFFER_RESIZED: c_int = 32;
pub const B_BUFFER_MOVED: c_int = 64;
pub const B_BUFFER_RESET: c_int = 128;
//  ENUM: direct_driver_state
pub const B_DRIVER_CHANGED: c_int = 0x0001;
pub const B_MODE_CHANGED: c_int = 0x0002;

pub const B_GS_CUR_API_VERSION: c_int = B_BEOS_VERSION;
pub const B_GS_INVALID_SOUND: c_int = ((gs_id)-1);
pub const B_GS_MAIN_SOUND: c_int = ((gs_id)-2);
pub const B_GS_MIN_API_VERSION: c_int = 0x100;
//  ENUM: @9
pub const B_GS_BAD_HANDLE: c_int = -99999;
pub const B_GS_NO_SOUNDS: c_int = -99999 + 1;
pub const B_GS_NO_HARDWARE: c_int = -99999 + 2;
pub const B_GS_ALREADY_COMMITTED: c_int = -99999 + 3;
pub const B_GS_READ_ONLY_VALUE: c_int = -99999 + 4;
//  ENUM: gs_attributes
pub const B_GS_NO_ATTRIBUTE: c_int = 0;
pub const B_GS_MAIN_GAIN: c_int = 1;
pub const B_GS_CD_THROUGH_GAIN: c_int = 1 + 1;
pub const B_GS_GAIN: c_int = 128;
pub const B_GS_PAN: c_int = 128 + 1;
pub const B_GS_SAMPLING_RATE: c_int = 128 + 2;
pub const B_GS_LOOPING: c_int = 128 + 3;
pub const B_GS_FIRST_PRIVATE_ATTRIBUTE: c_int = 90000;
pub const B_GS_FIRST_USER_ATTRIBUTE: c_int = 100000;

//  ENUM: @10
pub const B_ENABLE_VIEW_DRAWING: c_int = 0x0001;
pub const B_ENABLE_DEBUGGER: c_int = 0x0002;

//  ENUM: alert_type
pub const B_EMPTY_ALERT: c_int = 0;
pub const B_INFO_ALERT: c_int = 0 + 1;
pub const B_IDEA_ALERT: c_int = 0 + 2;
pub const B_WARNING_ALERT: c_int = 0 + 3;
pub const B_STOP_ALERT: c_int = 0 + 4;
//  ENUM: button_spacing
pub const B_EVEN_SPACING: c_int = 0;
pub const B_OFFSET_SPACING: c_int = 0 + 1;

pub const B_ANY_BYTES_PER_ROW: c_int = -1;
//  ENUM: @11
pub const B_BITMAP_CLEAR_TO_WHITE: c_int = 0x00000001;
pub const B_BITMAP_ACCEPTS_VIEWS: c_int = 0x00000002;
pub const B_BITMAP_IS_AREA: c_int = 0x00000004;
pub const B_BITMAP_IS_LOCKED: c_int = 0x00000008 | B_BITMAP_IS_AREA;
pub const B_BITMAP_IS_CONTIGUOUS: c_int = 0x00000010 | B_BITMAP_IS_LOCKED;
pub const B_BITMAP_IS_OFFSCREEN: c_int = 0x00000020;
pub const B_BITMAP_WILL_OVERLAY: c_int = 0x00000040 | B_BITMAP_IS_OFFSCREEN;
pub const B_BITMAP_RESERVE_OVERLAY_CHANNEL: c_int = 0x00000080;
pub const B_BITMAP_NO_SERVER_LINK: c_int = 0x00000100;

//  ENUM: color_control_layout
pub const B_CELLS_4x64: c_int = 4;
pub const B_CELLS_8x32: c_int = 8;
pub const B_CELLS_16x16: c_int = 16;
pub const B_CELLS_32x8: c_int = 32;
pub const B_CELLS_64x4: c_int = 64;

//  ENUM: @12
pub const B_CONTROL_OFF: c_int = 0;
pub const B_CONTROL_ON: c_int = 1;
pub const B_CONTROL_PARTIALLY_ON: c_int = 2;

//  ENUM: deskbar_location
pub const B_DESKBAR_TOP: c_int = 0;
pub const B_DESKBAR_BOTTOM: c_int = 0 + 1;
pub const B_DESKBAR_LEFT_TOP: c_int = 0 + 2;
pub const B_DESKBAR_RIGHT_TOP: c_int = 0 + 3;
pub const B_DESKBAR_LEFT_BOTTOM: c_int = 0 + 4;
pub const B_DESKBAR_RIGHT_BOTTOM: c_int = 0 + 5;

pub const B_FONT_FAMILY_LENGTH: c_int = 63;
pub const B_FONT_STYLE_LENGTH: c_int = 63;
//  ENUM: @17
pub const B_CHAR_SPACING: c_int = 0;
pub const B_STRING_SPACING: c_int = 1;
pub const B_BITMAP_SPACING: c_int = 2;
pub const B_FIXED_SPACING: c_int = 3;
//  ENUM: @18
pub const B_DISABLE_ANTIALIASING: c_int = 0x00000001;
pub const B_FORCE_ANTIALIASING: c_int = 0x00000002;
//  ENUM: @19
pub const B_NO_TRUNCATION: c_int = ~0U;
pub const B_TRUNCATE_END: c_int = 0;
pub const B_TRUNCATE_BEGINNING: c_int = 1;
pub const B_TRUNCATE_MIDDLE: c_int = 2;
pub const B_TRUNCATE_SMART: c_int = 3;
//  ENUM: @20
pub const B_UNICODE_UTF8: c_int = 0;
pub const B_ISO_8859_1: c_int = 1;
pub const B_ISO_8859_2: c_int = 2;
pub const B_ISO_8859_3: c_int = 3;
pub const B_ISO_8859_4: c_int = 4;
pub const B_ISO_8859_5: c_int = 5;
pub const B_ISO_8859_6: c_int = 6;
pub const B_ISO_8859_7: c_int = 7;
pub const B_ISO_8859_8: c_int = 8;
pub const B_ISO_8859_9: c_int = 9;
pub const B_ISO_8859_10: c_int = 10;
pub const B_MACINTOSH_ROMAN: c_int = 11;
//  ENUM: @21
pub const B_HAS_TUNED_FONT: c_int = 0x0001;
pub const B_IS_FIXED: c_int = 0x0002;
//  ENUM: @22
pub const B_ITALIC_FACE: c_int = 0x0001;
pub const B_UNDERSCORE_FACE: c_int = 0x0002;
pub const B_NEGATIVE_FACE: c_int = 0x0004;
pub const B_OUTLINED_FACE: c_int = 0x0008;
pub const B_STRIKEOUT_FACE: c_int = 0x0010;
pub const B_BOLD_FACE: c_int = 0x0020;
pub const B_REGULAR_FACE: c_int = 0x0040;
pub const B_CONDENSED_FACE: c_int = 0x0080;
pub const B_LIGHT_FACE: c_int = 0x0100;
pub const B_HEAVY_FACE: c_int = 0x0200;
//  ENUM: font_direction
pub const B_FONT_LEFT_TO_RIGHT: c_int = 0;
pub const B_FONT_RIGHT_TO_LEFT: c_int = 1;
//  ENUM: font_file_format
pub const B_TRUETYPE_WINDOWS: c_int = 0;
pub const B_POSTSCRIPT_TYPE1_WINDOWS: c_int = 1;
//  ENUM: font_metric_mode
pub const B_SCREEN_METRIC: c_int = 0;
pub const B_PRINTING_METRIC: c_int = 1;

//  ENUM: @29
pub const B_VIEWS_SUPPORT_DRAW_BITMAP: c_int = 0x1;
pub const B_BITMAPS_SUPPORT_ATTACHED_VIEWS: c_int = 0x2;
pub const B_BITMAPS_SUPPORT_OVERLAY: c_int = 0x4;
//  ENUM: @30
pub const B_8_BIT_640x480: c_int = 0x00000001;
pub const B_8_BIT_800x600: c_int = 0x00000002;
pub const B_8_BIT_1024x768: c_int = 0x00000004;
pub const B_8_BIT_1280x1024: c_int = 0x00000008;
pub const B_8_BIT_1600x1200: c_int = 0x00000010;
pub const B_16_BIT_640x480: c_int = 0x00000020;
pub const B_16_BIT_800x600: c_int = 0x00000040;
pub const B_16_BIT_1024x768: c_int = 0x00000080;
pub const B_16_BIT_1280x1024: c_int = 0x00000100;
pub const B_16_BIT_1600x1200: c_int = 0x00000200;
pub const B_32_BIT_640x480: c_int = 0x00000400;
pub const B_32_BIT_800x600: c_int = 0x00000800;
pub const B_32_BIT_1024x768: c_int = 0x00001000;
pub const B_32_BIT_1280x1024: c_int = 0x00002000;
pub const B_32_BIT_1600x1200: c_int = 0x00004000;
pub const B_8_BIT_1152x900: c_int = 0x00008000;
pub const B_16_BIT_1152x900: c_int = 0x00010000;
pub const B_32_BIT_1152x900: c_int = 0x00020000;
pub const B_15_BIT_640x480: c_int = 0x00040000;
pub const B_15_BIT_800x600: c_int = 0x00080000;
pub const B_15_BIT_1024x768: c_int = 0x00100000;
pub const B_15_BIT_1280x1024: c_int = 0x00200000;
pub const B_15_BIT_1600x1200: c_int = 0x00400000;
pub const B_15_BIT_1152x900: c_int = 0x00800000;
pub const B_8_BIT_640x400: c_int = 0x80000000;
//  ENUM: alpha_function
pub const B_ALPHA_OVERLAY: c_int = 0;
pub const B_ALPHA_COMPOSITE: c_int = 0 + 1;
pub const B_ALPHA_COMPOSITE_SOURCE_OVER: c_int = B_ALPHA_COMPOSITE;
pub const B_ALPHA_COMPOSITE_SOURCE_IN: c_int = B_ALPHA_COMPOSITE + 1;
pub const B_ALPHA_COMPOSITE_SOURCE_OUT: c_int = B_ALPHA_COMPOSITE + 2;
pub const B_ALPHA_COMPOSITE_SOURCE_ATOP: c_int = B_ALPHA_COMPOSITE + 3;
pub const B_ALPHA_COMPOSITE_DESTINATION_OVER: c_int = B_ALPHA_COMPOSITE + 4;
pub const B_ALPHA_COMPOSITE_DESTINATION_IN: c_int = B_ALPHA_COMPOSITE + 5;
pub const B_ALPHA_COMPOSITE_DESTINATION_OUT: c_int = B_ALPHA_COMPOSITE + 6;
pub const B_ALPHA_COMPOSITE_DESTINATION_ATOP: c_int = B_ALPHA_COMPOSITE + 7;
pub const B_ALPHA_COMPOSITE_XOR: c_int = B_ALPHA_COMPOSITE + 8;
pub const B_ALPHA_COMPOSITE_CLEAR: c_int = B_ALPHA_COMPOSITE + 9;
pub const B_ALPHA_COMPOSITE_DIFFERENCE: c_int = B_ALPHA_COMPOSITE + 10;
pub const B_ALPHA_COMPOSITE_LIGHTEN: c_int = B_ALPHA_COMPOSITE + 11;
pub const B_ALPHA_COMPOSITE_DARKEN: c_int = B_ALPHA_COMPOSITE + 12;
//  ENUM: buffer_layout
pub const B_BUFFER_NONINTERLEAVED: c_int = 1;
//  ENUM: buffer_orientation
pub const B_BUFFER_TOP_TO_BOTTOM: c_int = 0;
pub const B_BUFFER_BOTTOM_TO_TOP: c_int = 0 + 1;
//  ENUM: color_space
pub const B_NO_COLOR_SPACE: c_int = 0x0000;
pub const B_RGBA64: c_int = 0x2012;
pub const B_RGB48: c_int = 0x0011;
pub const B_RGB32: c_int = 0x0008;
pub const B_RGBA32: c_int = 0x2008;
pub const B_RGB24: c_int = 0x0003;
pub const B_RGB16: c_int = 0x0005;
pub const B_RGB15: c_int = 0x0010;
pub const B_RGBA15: c_int = 0x2010;
pub const B_CMAP8: c_int = 0x0004;
pub const B_GRAY8: c_int = 0x0002;
pub const B_GRAY1: c_int = 0x0001;
pub const B_RGBA64_BIG: c_int = 0x3012;
pub const B_RGB48_BIG: c_int = 0x1011;
pub const B_RGB32_BIG: c_int = 0x1008;
pub const B_RGBA32_BIG: c_int = 0x3008;
pub const B_RGB24_BIG: c_int = 0x1003;
pub const B_RGB16_BIG: c_int = 0x1005;
pub const B_RGB15_BIG: c_int = 0x1010;
pub const B_RGBA15_BIG: c_int = 0x3010;
pub const B_RGBA64_LITTLE: c_int = B_RGBA64;
pub const B_RGB48_LITTLE: c_int = B_RGB48;
pub const B_RGB32_LITTLE: c_int = B_RGB32;
pub const B_RGBA32_LITTLE: c_int = B_RGBA32;
pub const B_RGB24_LITTLE: c_int = B_RGB24;
pub const B_RGB16_LITTLE: c_int = B_RGB16;
pub const B_RGB15_LITTLE: c_int = B_RGB15;
pub const B_RGBA15_LITTLE: c_int = B_RGBA15;
pub const B_YCbCr422: c_int = 0x4000;
pub const B_YCbCr411: c_int = 0x4001;
pub const B_YCbCr444: c_int = 0x4003;
pub const B_YCbCr420: c_int = 0x4004;
pub const B_YUV422: c_int = 0x4020;
pub const B_YUV411: c_int = 0x4021;
pub const B_YUV444: c_int = 0x4023;
pub const B_YUV420: c_int = 0x4024;
pub const B_YUV9: c_int = 0x402C;
pub const B_YUV12: c_int = 0x402D;
pub const B_UVL24: c_int = 0x4030;
pub const B_UVL32: c_int = 0x4031;
pub const B_UVLA32: c_int = 0x6031;
pub const B_LAB24: c_int = 0x4032;
pub const B_LAB32: c_int = 0x4033;
pub const B_LABA32: c_int = 0x6033;
pub const B_HSI24: c_int = 0x4040;
pub const B_HSI32: c_int = 0x4041;
pub const B_HSIA32: c_int = 0x6041;
pub const B_HSV24: c_int = 0x4042;
pub const B_HSV32: c_int = 0x4043;
pub const B_HSVA32: c_int = 0x6043;
pub const B_HLS24: c_int = 0x4044;
pub const B_HLS32: c_int = 0x4045;
pub const B_HLSA32: c_int = 0x6045;
pub const B_CMY24: c_int = 0xC001;
pub const B_CMY32: c_int = 0xC002;
pub const B_CMYA32: c_int = 0xE002;
pub const B_CMYK32: c_int = 0xC003;
pub const B_MONOCHROME_1_BIT: c_int = B_GRAY1;
pub const B_GRAYSCALE_8_BIT: c_int = B_GRAY8;
pub const B_COLOR_8_BIT: c_int = B_CMAP8;
pub const B_RGB_32_BIT: c_int = B_RGB32;
pub const B_RGB_16_BIT: c_int = B_RGB15;
pub const B_BIG_RGB_32_BIT: c_int = B_RGB32_BIG;
pub const B_BIG_RGB_16_BIT: c_int = B_RGB15_BIG;
//  ENUM: drawing_mode
pub const B_OP_COPY: c_int = 0;
pub const B_OP_OVER: c_int = 0 + 1;
pub const B_OP_ERASE: c_int = 0 + 2;
pub const B_OP_INVERT: c_int = 0 + 3;
pub const B_OP_ADD: c_int = 0 + 4;
pub const B_OP_SUBTRACT: c_int = 0 + 5;
pub const B_OP_BLEND: c_int = 0 + 6;
pub const B_OP_MIN: c_int = 0 + 7;
pub const B_OP_MAX: c_int = 0 + 8;
pub const B_OP_SELECT: c_int = 0 + 9;
pub const B_OP_ALPHA: c_int = 0 + 10;
//  ENUM: source_alpha
pub const B_PIXEL_ALPHA: c_int = 0;
pub const B_CONSTANT_ALPHA: c_int = 0 + 1;

//  ENUM: input_device_notification
pub const B_INPUT_DEVICE_ADDED: c_int = 0x0001;
pub const B_INPUT_DEVICE_STARTED: c_int = 0x0002;
pub const B_INPUT_DEVICE_STOPPED: c_int = 0x0004;
pub const B_INPUT_DEVICE_REMOVED: c_int = 0x0008;
//  ENUM: input_device_type
pub const B_POINTING_DEVICE: c_int = 0;
pub const B_KEYBOARD_DEVICE: c_int = 1;
pub const B_UNDEFINED_DEVICE: c_int = 2;
//  ENUM: input_method_op
pub const B_INPUT_METHOD_STARTED: c_int = 0;
pub const B_INPUT_METHOD_STOPPED: c_int = 1;
pub const B_INPUT_METHOD_CHANGED: c_int = 2;
pub const B_INPUT_METHOD_LOCATION_REQUEST: c_int = 3;

pub const B_MAX_MOUSE_BUTTONS: c_int = 16;
//  SKIP: B_UTF8_BULLET
//  SKIP: B_UTF8_CLOSE_QUOTE
//  SKIP: B_UTF8_COPYRIGHT
//  SKIP: B_UTF8_ELLIPSIS
//  SKIP: B_UTF8_HIROSHI
//  SKIP: B_UTF8_OPEN_QUOTE
//  SKIP: B_UTF8_REGISTERED
//  SKIP: B_UTF8_SMILING_FACE
//  SKIP: B_UTF8_TRADEMARK
//  ENUM: @31
pub const B_BACKSPACE: c_int = 0x08;
pub const B_RETURN: c_int = 0x0a;
pub const B_ENTER: c_int = 0x0a;
pub const B_SPACE: c_int = 0x20;
pub const B_TAB: c_int = 0x09;
pub const B_ESCAPE: c_int = 0x1b;
pub const B_SUBSTITUTE: c_int = 0x1a;
pub const B_LEFT_ARROW: c_int = 0x1c;
pub const B_RIGHT_ARROW: c_int = 0x1d;
pub const B_UP_ARROW: c_int = 0x1e;
pub const B_DOWN_ARROW: c_int = 0x1f;
pub const B_INSERT: c_int = 0x05;
pub const B_DELETE: c_int = 0x7f;
pub const B_HOME: c_int = 0x01;
pub const B_END: c_int = 0x04;
pub const B_PAGE_UP: c_int = 0x0b;
pub const B_PAGE_DOWN: c_int = 0x0c;
pub const B_FUNCTION_KEY: c_int = 0x10;
pub const B_KATAKANA_HIRAGANA: c_int = 0xf2;
pub const B_HANKAKU_ZENKAKU: c_int = 0xf3;
pub const B_HANGUL: c_int = 0xf0;
pub const B_HANGUL_HANJA: c_int = 0xf1;
//  ENUM: @32
pub const B_F1_KEY: c_int = 0x02;
pub const B_F2_KEY: c_int = 0x03;
pub const B_F3_KEY: c_int = 0x04;
pub const B_F4_KEY: c_int = 0x05;
pub const B_F5_KEY: c_int = 0x06;
pub const B_F6_KEY: c_int = 0x07;
pub const B_F7_KEY: c_int = 0x08;
pub const B_F8_KEY: c_int = 0x09;
pub const B_F9_KEY: c_int = 0x0a;
pub const B_F10_KEY: c_int = 0x0b;
pub const B_F11_KEY: c_int = 0x0c;
pub const B_F12_KEY: c_int = 0x0d;
pub const B_PRINT_KEY: c_int = 0x0e;
pub const B_SCROLL_KEY: c_int = 0x0f;
pub const B_PAUSE_KEY: c_int = 0x10;
//  ENUM: @33
pub const B_CONTROL_TABLE: c_int = 0x00000001;
pub const B_OPTION_CAPS_SHIFT_TABLE: c_int = 0x00000002;
pub const B_OPTION_CAPS_TABLE: c_int = 0x00000004;
pub const B_OPTION_SHIFT_TABLE: c_int = 0x00000008;
pub const B_OPTION_TABLE: c_int = 0x00000010;
pub const B_CAPS_SHIFT_TABLE: c_int = 0x00000020;
pub const B_CAPS_TABLE: c_int = 0x00000040;
pub const B_SHIFT_TABLE: c_int = 0x00000080;
pub const B_NORMAL_TABLE: c_int = 0x00000100;
//  ENUM: @34
pub const B_SHIFT_KEY: c_int = 0x00000001;
pub const B_COMMAND_KEY: c_int = 0x00000002;
pub const B_CONTROL_KEY: c_int = 0x00000004;
pub const B_CAPS_LOCK: c_int = 0x00000008;
pub const B_SCROLL_LOCK: c_int = 0x00000010;
pub const B_NUM_LOCK: c_int = 0x00000020;
pub const B_OPTION_KEY: c_int = 0x00000040;
pub const B_MENU_KEY: c_int = 0x00000080;
pub const B_LEFT_SHIFT_KEY: c_int = 0x00000100;
pub const B_RIGHT_SHIFT_KEY: c_int = 0x00000200;
pub const B_LEFT_COMMAND_KEY: c_int = 0x00000400;
pub const B_RIGHT_COMMAND_KEY: c_int = 0x00000800;
pub const B_LEFT_CONTROL_KEY: c_int = 0x00001000;
pub const B_RIGHT_CONTROL_KEY: c_int = 0x00002000;
pub const B_LEFT_OPTION_KEY: c_int = 0x00004000;
pub const B_RIGHT_OPTION_KEY: c_int = 0x00008000;
//  ENUM: @35
pub const B_USE_DEFAULT_SPACING: c_int = -1002;
pub const B_USE_ITEM_SPACING: c_int = -1003;
pub const B_USE_ITEM_INSETS: c_int = B_USE_ITEM_SPACING;
pub const B_USE_HALF_ITEM_SPACING: c_int = -1004;
pub const B_USE_HALF_ITEM_INSETS: c_int = B_USE_HALF_ITEM_SPACING;
pub const B_USE_WINDOW_SPACING: c_int = -1005;
pub const B_USE_WINDOW_INSETS: c_int = B_USE_WINDOW_SPACING;
pub const B_USE_SMALL_SPACING: c_int = -1006;
pub const B_USE_SMALL_INSETS: c_int = B_USE_SMALL_SPACING;
pub const B_USE_CORNER_SPACING: c_int = -1007;
pub const B_USE_CORNER_INSETS: c_int = B_USE_CORNER_SPACING;
pub const B_USE_BIG_SPACING: c_int = -1008;
pub const B_USE_BIG_INSETS: c_int = B_USE_BIG_SPACING;
pub const B_USE_BORDER_SPACING: c_int = -1009;
pub const B_USE_BORDER_INSETS: c_int = B_USE_BORDER_SPACING;
//  ENUM: @36
pub const B_EVEN_ODD: c_int = 0;
pub const B_NONZERO: c_int = 0 + 1;
//  ENUM: @37
pub const B_INACTIVE_ICON_BITMAP: c_int = 0x00;
pub const B_ACTIVE_ICON_BITMAP: c_int = 0x01;
pub const B_PARTIALLY_ACTIVATE_ICON_BITMAP: c_int = 0x02;
pub const B_DISABLED_ICON_BITMAP: c_int = 0x80;
//  ENUM: @38
pub const B_KEEP_ICON_BITMAP: c_int = 0x0001;
//  ENUM: @39
pub const B_TRIM_ICON_BITMAP: c_int = 0x0100;
pub const B_TRIM_ICON_BITMAP_KEEP_ASPECT: c_int = 0x0200;
pub const B_CREATE_ACTIVE_ICON_BITMAP: c_int = 0x0400;
pub const B_CREATE_PARTIALLY_ACTIVE_ICON_BITMAP: c_int = 0x0800;
pub const B_CREATE_DISABLED_ICON_BITMAPS: c_int = 0x1000;
//  ENUM: alignment
pub const B_ALIGN_LEFT: c_int = 0;
pub const B_ALIGN_RIGHT: c_int = 0 + 1;
pub const B_ALIGN_CENTER: c_int = 0 + 2;
pub const B_ALIGN_HORIZONTAL_CENTER: c_int = B_ALIGN_CENTER;
pub const B_ALIGN_HORIZONTAL_UNSET: c_long = -1;
pub const B_ALIGN_USE_FULL_WIDTH: c_long = -2;
//  ENUM: bitmap_drawing_options
pub const B_FILTER_BITMAP_BILINEAR: c_int = 0x00000100;
pub const B_WAIT_FOR_RETRACE: c_int = 0x00000800;
//  ENUM: bitmap_tiling
pub const B_TILE_BITMAP_X: c_int = 0x00000001;
pub const B_TILE_BITMAP_Y: c_int = 0x00000002;
pub const B_TILE_BITMAP: c_int = 0x00000003;
//  ENUM: border_style
pub const B_PLAIN_BORDER: c_int = 0;
pub const B_FANCY_BORDER: c_int = 0 + 1;
pub const B_NO_BORDER: c_int = 0 + 2;
//  ENUM: button_width
pub const B_WIDTH_AS_USUAL: c_int = 0;
pub const B_WIDTH_FROM_WIDEST: c_int = 0 + 1;
pub const B_WIDTH_FROM_LABEL: c_int = 0 + 2;
//  ENUM: cap_mode
pub const B_ROUND_CAP: c_int = B_ROUND_JOIN;
pub const B_BUTT_CAP: c_int = B_BUTT_JOIN;
pub const B_SQUARE_CAP: c_int = B_SQUARE_JOIN;
//  ENUM: color_which
pub const B_NO_COLOR: c_int = 0;
pub const B_PANEL_BACKGROUND_COLOR: c_int = 1;
pub const B_PANEL_TEXT_COLOR: c_int = 10;
pub const B_DOCUMENT_BACKGROUND_COLOR: c_int = 11;
pub const B_DOCUMENT_TEXT_COLOR: c_int = 12;
pub const B_CONTROL_BACKGROUND_COLOR: c_int = 13;
pub const B_CONTROL_TEXT_COLOR: c_int = 14;
pub const B_CONTROL_BORDER_COLOR: c_int = 15;
pub const B_CONTROL_HIGHLIGHT_COLOR: c_int = 16;
pub const B_CONTROL_MARK_COLOR: c_int = 27;
pub const B_NAVIGATION_BASE_COLOR: c_int = 4;
pub const B_NAVIGATION_PULSE_COLOR: c_int = 17;
pub const B_SHINE_COLOR: c_int = 18;
pub const B_SHADOW_COLOR: c_int = 19;
pub const B_LINK_TEXT_COLOR: c_int = 33;
pub const B_LINK_HOVER_COLOR: c_int = 34;
pub const B_LINK_VISITED_COLOR: c_int = 35;
pub const B_LINK_ACTIVE_COLOR: c_int = 36;
pub const B_MENU_BACKGROUND_COLOR: c_int = 2;
pub const B_MENU_SELECTED_BACKGROUND_COLOR: c_int = 6;
pub const B_MENU_ITEM_TEXT_COLOR: c_int = 7;
pub const B_MENU_SELECTED_ITEM_TEXT_COLOR: c_int = 8;
pub const B_MENU_SELECTED_BORDER_COLOR: c_int = 9;
pub const B_LIST_BACKGROUND_COLOR: c_int = 28;
pub const B_LIST_SELECTED_BACKGROUND_COLOR: c_int = 29;
pub const B_LIST_ITEM_TEXT_COLOR: c_int = 30;
pub const B_LIST_SELECTED_ITEM_TEXT_COLOR: c_int = 31;
pub const B_SCROLL_BAR_THUMB_COLOR: c_int = 32;
pub const B_TOOL_TIP_BACKGROUND_COLOR: c_int = 20;
pub const B_TOOL_TIP_TEXT_COLOR: c_int = 21;
pub const B_STATUS_BAR_COLOR: c_int = 37;
pub const B_SUCCESS_COLOR: c_int = 100;
pub const B_FAILURE_COLOR: c_int = 101;
pub const B_WINDOW_TAB_COLOR: c_int = 3;
pub const B_WINDOW_TEXT_COLOR: c_int = 22;
pub const B_WINDOW_INACTIVE_TAB_COLOR: c_int = 23;
pub const B_WINDOW_INACTIVE_TEXT_COLOR: c_int = 24;
pub const B_WINDOW_BORDER_COLOR: c_int = 25;
pub const B_WINDOW_INACTIVE_BORDER_COLOR: c_int = 26;
pub const B_KEYBOARD_NAVIGATION_COLOR: c_int = B_NAVIGATION_BASE_COLOR;
pub const B_MENU_SELECTION_BACKGROUND_COLOR: c_int = B_MENU_SELECTED_BACKGROUND_COLOR;
pub const B_DESKTOP_COLOR: c_int = 5;
//  ENUM: join_mode
pub const B_ROUND_JOIN: c_int = 0;
pub const B_MITER_JOIN: c_int = 0 + 1;
pub const B_BEVEL_JOIN: c_int = 0 + 2;
pub const B_BUTT_JOIN: c_int = 0 + 3;
pub const B_SQUARE_JOIN: c_int = 0 + 4;
//  ENUM: mode_focus_follows_mouse
pub const B_NORMAL_FOCUS_FOLLOWS_MOUSE: c_int = 0;
pub const B_WARP_FOCUS_FOLLOWS_MOUSE: c_int = 1;
pub const B_INSTANT_WARP_FOCUS_FOLLOWS_MOUSE: c_int = 2;
//  ENUM: mode_mouse
pub const B_NORMAL_MOUSE: c_int = 0;
pub const B_CLICK_TO_FOCUS_MOUSE: c_int = -1;
pub const B_FOCUS_FOLLOWS_MOUSE: c_int = 1;
//  ENUM: orientation
pub const B_HORIZONTAL: c_int = 0;
pub const B_VERTICAL: c_int = 0 + 1;
//  ENUM: overlay_options
pub const B_OVERLAY_FILTER_HORIZONTAL: c_int = 0x00010000;
pub const B_OVERLAY_FILTER_VERTICAL: c_int = 0x00020000;
pub const B_OVERLAY_MIRROR: c_int = 0x00040000;
pub const B_OVERLAY_TRANSFER_CHANNEL: c_int = 0x00080000;
//  ENUM: vertical_alignment
pub const B_ALIGN_TOP: c_long = 0x10;
pub const B_ALIGN_MIDDLE: c_int = 0x20;
pub const B_ALIGN_BOTTOM: c_int = 0x30;
pub const B_ALIGN_VERTICAL_CENTER: c_int = B_ALIGN_MIDDLE;
pub const B_ALIGN_VERTICAL_UNSET: c_long = -1;
pub const B_ALIGN_NO_VERTICAL: c_int = B_ALIGN_VERTICAL_UNSET;
pub const B_ALIGN_USE_FULL_HEIGHT: c_long = -2;

//  ENUM: list_view_type
pub const B_SINGLE_SELECTION_LIST: c_int = 0;
pub const B_MULTIPLE_SELECTION_LIST: c_int = 0 + 1;

//  ENUM: menu_layout
pub const B_ITEMS_IN_ROW: c_int = 0;
pub const B_ITEMS_IN_COLUMN: c_int = 0 + 1;
pub const B_ITEMS_IN_MATRIX: c_int = 0 + 2;

//  ENUM: menu_bar_border
pub const B_BORDER_FRAME: c_int = 0;
pub const B_BORDER_CONTENTS: c_int = 0 + 1;
pub const B_BORDER_EACH_ITEM: c_int = 0 + 2;

//  ENUM: @40
pub const B_OPTION_CONTROL_VALUE: c_int = 0x5f424d56 /* '_BMV' */;

//  ENUM: @41
pub const B_ONE_STATE_BUTTON: c_int = 0;
pub const B_TWO_STATE_BUTTON: c_int = 0 + 1;

pub const B_H_SCROLL_BAR_HEIGHT: f32 = 14.0;
pub const B_V_SCROLL_BAR_WIDTH: f32 = 14.0;

//  ENUM: @43
pub const B_SIZE_UNSET: c_int = -2;
pub const B_SIZE_UNLIMITED: c_int = 1024 * 1024 * 1024;

//  ENUM: hash_mark_location
pub const B_HASH_MARKS_NONE: c_int = 0;
pub const B_HASH_MARKS_TOP: c_int = 1;
pub const B_HASH_MARKS_LEFT: c_int = 1;
pub const B_HASH_MARKS_BOTTOM: c_int = 2;
pub const B_HASH_MARKS_RIGHT: c_int = 2;
pub const B_HASH_MARKS_BOTH: c_int = 3;
//  ENUM: thumb_style
pub const B_BLOCK_THUMB: c_int = 0;
pub const B_TRIANGLE_THUMB: c_int = 0 + 1;

//  ENUM: tab_position
pub const B_TAB_FIRST: c_int = 999;
pub const B_TAB_FRONT: c_int = 999 + 1;
pub const B_TAB_ANY: c_int = 999 + 2;

//  ENUM: undo_state
pub const B_UNDO_UNAVAILABLE: c_int = 0;
pub const B_UNDO_TYPING: c_int = 0 + 1;
pub const B_UNDO_CUT: c_int = 0 + 2;
pub const B_UNDO_PASTE: c_int = 0 + 3;
pub const B_UNDO_CLEAR: c_int = 0 + 4;
pub const B_UNDO_DROP: c_int = 0 + 5;

pub const _RESIZE_MASK_: c_int = (0xffff);
pub const B_FOLLOW_ALL: c_int = B_FOLLOW_ALL_SIDES;
pub const B_FOLLOW_ALL_SIDES: c_int = _rule_(_VIEW_TOP_, _VIEW_LEFT_, _VIEW_BOTTOM_, _VIEW_RIGHT_);
pub const B_FOLLOW_BOTTOM: c_int = _rule_(_VIEW_BOTTOM_, 0, _VIEW_BOTTOM_, 0);
pub const B_FOLLOW_H_CENTER: c_int = _rule_(0, _VIEW_CENTER_, 0, _VIEW_CENTER_);
pub const B_FOLLOW_LEFT: c_int = _rule_(0, _VIEW_LEFT_, 0, _VIEW_LEFT_);
pub const B_FOLLOW_LEFT_RIGHT: c_int = _rule_(0, _VIEW_LEFT_, 0, _VIEW_RIGHT_);
pub const B_FOLLOW_LEFT_TOP: c_int = B_FOLLOW_TOP | B_FOLLOW_LEFT;
pub const B_FOLLOW_NONE: c_int = 0;
pub const B_FOLLOW_RIGHT: c_int = _rule_(0, _VIEW_RIGHT_, 0, _VIEW_RIGHT_);
pub const B_FOLLOW_TOP: c_int = _rule_(_VIEW_TOP_, 0, _VIEW_TOP_, 0);
pub const B_FOLLOW_TOP_BOTTOM: c_int = _rule_(_VIEW_TOP_, 0, _VIEW_BOTTOM_, 0);
pub const B_FOLLOW_V_CENTER: c_int = _rule_(_VIEW_CENTER_, 0, _VIEW_CENTER_, 0);
pub const B_MOUSE_BUTTON: c_int = (1 << ((n) - 1));
//  ENUM: @44
pub const B_PRIMARY_MOUSE_BUTTON: c_int = B_MOUSE_BUTTON(1);
pub const B_SECONDARY_MOUSE_BUTTON: c_int = B_MOUSE_BUTTON(2);
pub const B_TERTIARY_MOUSE_BUTTON: c_int = B_MOUSE_BUTTON(3);
//  ENUM: @45
pub const B_ENTERED_VIEW: c_int = 0;
pub const B_INSIDE_VIEW: c_int = 0 + 1;
pub const B_EXITED_VIEW: c_int = 0 + 2;
pub const B_OUTSIDE_VIEW: c_int = 0 + 3;
//  ENUM: @46
pub const B_POINTER_EVENTS: c_int = 0x00000001;
pub const B_KEYBOARD_EVENTS: c_int = 0x00000002;
//  ENUM: @47
pub const B_LOCK_WINDOW_FOCUS: c_int = 0x00000001;
pub const B_SUSPEND_VIEW_FOCUS: c_int = 0x00000002;
pub const B_NO_POINTER_HISTORY: c_int = 0x00000004;
pub const B_FULL_POINTER_HISTORY: c_int = 0x00000008;
//  ENUM: @48
pub const B_TRACK_WHOLE_RECT: c_int = 0;
pub const B_TRACK_RECT_CORNER: c_int = 0 + 1;
//  ENUM: @49
pub const B_FONT_FAMILY_AND_STYLE: c_int = 0x00000001;
pub const B_FONT_SIZE: c_int = 0x00000002;
pub const B_FONT_SHEAR: c_int = 0x00000004;
pub const B_FONT_ROTATION: c_int = 0x00000008;
pub const B_FONT_SPACING: c_int = 0x00000010;
pub const B_FONT_ENCODING: c_int = 0x00000020;
pub const B_FONT_FACE: c_int = 0x00000040;
pub const B_FONT_FLAGS: c_int = 0x00000080;
pub const B_FONT_FALSE_BOLD_WIDTH: c_int = 0x00000100;
pub const B_FONT_ALL: c_int = 0x000001FF;
//  ENUM: coordinate_space
pub const B_CURRENT_STATE_COORDINATES: c_int = 0;
pub const B_PREVIOUS_STATE_COORDINATES: c_int = 0 + 1;
pub const B_VIEW_COORDINATES: c_int = 0 + 2;
pub const B_PARENT_VIEW_DRAW_COORDINATES: c_int = 0 + 3;
pub const B_PARENT_VIEW_COORDINATES: c_int = 0 + 4;
pub const B_WINDOW_COORDINATES: c_int = 0 + 5;
pub const B_SCREEN_COORDINATES: c_int = 0 + 6;

pub const B_ALL_WORKSPACES: c_int = 0xffffffff;
pub const B_CURRENT_WORKSPACE: c_int = 0;
//  ENUM: @50
pub const B_NOT_MOVABLE: c_int = 0x00000001;
pub const B_NOT_CLOSABLE: c_int = 0x00000020;
pub const B_NOT_ZOOMABLE: c_int = 0x00000040;
pub const B_NOT_MINIMIZABLE: c_int = 0x00004000;
pub const B_NOT_RESIZABLE: c_int = 0x00000002;
pub const B_NOT_H_RESIZABLE: c_int = 0x00000004;
pub const B_NOT_V_RESIZABLE: c_int = 0x00000008;
pub const B_AVOID_FRONT: c_int = 0x00000080;
pub const B_AVOID_FOCUS: c_int = 0x00002000;
pub const B_WILL_ACCEPT_FIRST_CLICK: c_int = 0x00000010;
pub const B_OUTLINE_RESIZE: c_int = 0x00001000;
pub const B_NO_WORKSPACE_ACTIVATION: c_int = 0x00000100;
pub const B_NOT_ANCHORED_ON_ACTIVATE: c_int = 0x00020000;
pub const B_ASYNCHRONOUS_CONTROLS: c_int = 0x00080000;
pub const B_QUIT_ON_WINDOW_CLOSE: c_int = 0x00100000;
pub const B_SAME_POSITION_IN_ALL_WORKSPACES: c_int = 0x00200000;
pub const B_AUTO_UPDATE_SIZE_LIMITS: c_int = 0x00400000;
pub const B_CLOSE_ON_ESCAPE: c_int = 0x00800000;
pub const B_NO_SERVER_SIDE_WINDOW_MODIFIERS: c_int = 0x00000200;
//  ENUM: @51
pub const B_DO_NOT_RESIZE_TO_FIT: c_int = 0x0001;
pub const B_MOVE_IF_PARTIALLY_OFFSCREEN: c_int = 0x0002;
//  ENUM: window_alignment
pub const B_BYTE_ALIGNMENT: c_int = 0;
pub const B_PIXEL_ALIGNMENT: c_int = 1;
//  ENUM: window_feel
pub const B_NORMAL_WINDOW_FEEL: c_int = 0;
pub const B_MODAL_SUBSET_WINDOW_FEEL: c_int = 2;
pub const B_MODAL_APP_WINDOW_FEEL: c_int = 1;
pub const B_MODAL_ALL_WINDOW_FEEL: c_int = 3;
pub const B_FLOATING_SUBSET_WINDOW_FEEL: c_int = 5;
pub const B_FLOATING_APP_WINDOW_FEEL: c_int = 4;
pub const B_FLOATING_ALL_WINDOW_FEEL: c_int = 6;
//  ENUM: window_look
pub const B_BORDERED_WINDOW_LOOK: c_int = 20;
pub const B_NO_BORDER_WINDOW_LOOK: c_int = 19;
pub const B_TITLED_WINDOW_LOOK: c_int = 1;
pub const B_DOCUMENT_WINDOW_LOOK: c_int = 11;
pub const B_MODAL_WINDOW_LOOK: c_int = 3;
pub const B_FLOATING_WINDOW_LOOK: c_int = 7;
//  ENUM: window_type
pub const B_UNTYPED_WINDOW: c_int = 0;
pub const B_TITLED_WINDOW: c_int = 1;
pub const B_MODAL_WINDOW: c_int = 3;
pub const B_DOCUMENT_WINDOW: c_int = 11;
pub const B_BORDERED_WINDOW: c_int = 20;
pub const B_FLOATING_WINDOW: c_int = 21;

//  SKIP: B_CATALOG
//  SKIP: B_TRANSLATE
//  SKIP: B_TRANSLATE_ALL
//  SKIP: B_TRANSLATE_COMMENT
//  SKIP: B_TRANSLATE_CONTEXT
//  SKIP: B_TRANSLATE_ID
//  SKIP: B_TRANSLATE_MARK
//  SKIP: B_TRANSLATE_MARK_ALL
// NODEF: B_TRANSLATE_MARK_ALL_VOID
//  SKIP: B_TRANSLATE_MARK_COMMENT
// NODEF: B_TRANSLATE_MARK_COMMENT_VOID
//  SKIP: B_TRANSLATE_MARK_CONTEXT
// NODEF: B_TRANSLATE_MARK_CONTEXT_VOID
//  SKIP: B_TRANSLATE_MARK_ID
// NODEF: B_TRANSLATE_MARK_ID_VOID
//  SKIP: B_TRANSLATE_MARK_SYSTEM_NAME
// NODEF: B_TRANSLATE_MARK_SYSTEM_NAME_VOID
// NODEF: B_TRANSLATE_MARK_VOID
//  SKIP: B_TRANSLATE_NOCOLLECT
//  SKIP: B_TRANSLATE_NOCOLLECT_ALL
//  SKIP: B_TRANSLATE_NOCOLLECT_COMMENT
//  SKIP: B_TRANSLATE_NOCOLLECT_ID
//  SKIP: B_TRANSLATE_NOCOLLECT_SYSTEM_NAME
//  SKIP: B_TRANSLATE_SYSTEM_NAME
//  SKIP: B_TRANSLATION_SYSTEM_NAME_CONTEXT

pub const U_ICU_NAMESPACE: c_int = icu;
//  ENUM: collator_strengths
pub const B_COLLATE_DEFAULT: c_int = -1;
pub const B_COLLATE_PRIMARY: c_int = 1;
pub const B_COLLATE_SECONDARY: c_int = 1 + 1;
pub const B_COLLATE_TERTIARY: c_int = 1 + 2;
pub const B_COLLATE_QUATERNARY: c_int = 1 + 3;
pub const B_COLLATE_IDENTICAL: c_int = 127;

//   DUP: U_ICU_NAMESPACE

//   DUP: U_ICU_NAMESPACE
//  ENUM: BWeekday
pub const B_WEEKDAY_MONDAY: c_int = 1;
pub const B_WEEKDAY_TUESDAY: c_int = 1 + 1;
pub const B_WEEKDAY_WEDNESDAY: c_int = 1 + 2;
pub const B_WEEKDAY_THURSDAY: c_int = 1 + 3;
pub const B_WEEKDAY_FRIDAY: c_int = 1 + 4;
pub const B_WEEKDAY_SATURDAY: c_int = 1 + 5;
pub const B_WEEKDAY_SUNDAY: c_int = 1 + 6;

//  ENUM: BDateElement
pub const B_DATE_ELEMENT_INVALID: c_int = 0;
pub const B_DATE_ELEMENT_YEAR: c_int = 1 << 0;
pub const B_DATE_ELEMENT_MONTH: c_int = 1 << 1;
pub const B_DATE_ELEMENT_WEEKDAY: c_int = 1 << 2;
pub const B_DATE_ELEMENT_DAY: c_int = 1 << 3;
pub const B_DATE_ELEMENT_AM_PM: c_int = 1 << 4;
pub const B_DATE_ELEMENT_HOUR: c_int = 1 << 5;
pub const B_DATE_ELEMENT_MINUTE: c_int = 1 << 6;
pub const B_DATE_ELEMENT_SECOND: c_int = 1 << 7;
pub const B_DATE_ELEMENT_TIMEZONE: c_int = 1 << 8;

//   DUP: U_ICU_NAMESPACE

//  ENUM: @52
pub const B_CURRENCY_FIELD: c_int = 0;
pub const B_DECIMAL_SEPARATOR_FIELD: c_int = 0 + 1;
pub const B_EXPONENT_FIELD: c_int = 0 + 2;
pub const B_EXPONENT_SIGN_FIELD: c_int = 0 + 3;
pub const B_EXPONENT_SYMBOL_FIELD: c_int = 0 + 4;
pub const B_FRACTION_FIELD: c_int = 0 + 5;
pub const B_GROUPING_SEPARATOR_FIELD: c_int = 0 + 6;
pub const B_INTEGER_FIELD: c_int = 0 + 7;
pub const B_PERCENT_FIELD: c_int = 0 + 8;
pub const B_PERMILLE_FIELD: c_int = 0 + 9;
pub const B_SIGN_FIELD: c_int = 0 + 10;

//   DUP: U_ICU_NAMESPACE
//  ENUM: BDateFormatStyle
pub const B_FULL_DATE_FORMAT: c_int = 0;
pub const B_LONG_DATE_FORMAT: c_int = 0 + 1;
pub const B_MEDIUM_DATE_FORMAT: c_int = 0 + 2;
pub const B_SHORT_DATE_FORMAT: c_int = 0 + 3;
pub const B_DATE_FORMAT_STYLE_COUNT: c_int = 0 + 4;
//  ENUM: BMeasurementKind
pub const B_METRIC: c_int = 0;
pub const B_US: c_int = 0 + 1;
//  ENUM: BTimeFormatStyle
pub const B_FULL_TIME_FORMAT: c_int = 0;
pub const B_LONG_TIME_FORMAT: c_int = 0 + 1;
pub const B_MEDIUM_TIME_FORMAT: c_int = 0 + 2;
pub const B_SHORT_TIME_FORMAT: c_int = 0 + 3;
pub const B_TIME_FORMAT_STYLE_COUNT: c_int = 0 + 4;

//   DUP: U_ICU_NAMESPACE
//  ENUM: script_direction
pub const B_LEFT_TO_RIGHT: c_int = 0;
pub const B_RIGHT_TO_LEFT: c_int = 0 + 1;
pub const B_TOP_TO_BOTTOM: c_int = 0 + 2;

//  ENUM: @53
pub const B_LOCALE_CHANGED: c_int = 0x5f4c4343 /* '_LCC' */;

//  ENUM: country_strings
pub const B_COUNTRY_STRINGS_BASE: c_int = 0;
pub const B_DATE_TIME_FORMAT: c_int = B_COUNTRY_STRINGS_BASE;
pub const B_TIME_AM_PM_FORMAT: c_int = B_COUNTRY_STRINGS_BASE + 1;
pub const B_SHORT_DATE_TIME_FORMAT: c_int = B_COUNTRY_STRINGS_BASE + 2;
pub const B_SHORT_TIME_AM_PM_FORMAT: c_int = B_COUNTRY_STRINGS_BASE + 3;
pub const B_AM_STRING: c_int = B_COUNTRY_STRINGS_BASE + 4;
pub const B_PM_STRING: c_int = B_COUNTRY_STRINGS_BASE + 5;
pub const B_DATE_SEPARATOR: c_int = B_COUNTRY_STRINGS_BASE + 6;
pub const B_TIME_SEPARATOR: c_int = B_COUNTRY_STRINGS_BASE + 7;
pub const B_NUM_COUNTRY_STRINGS: c_int = B_COUNTRY_STRINGS_BASE + 8;
//  ENUM: language_strings
pub const B_LANGUAGE_STRINGS_BASE: c_int = 100;
pub const B_YESTERDAY_STRING: c_int = B_LANGUAGE_STRINGS_BASE;
pub const B_TODAY_STRING: c_int = B_LANGUAGE_STRINGS_BASE + 1;
pub const B_TOMORROW_STRING: c_int = B_LANGUAGE_STRINGS_BASE + 2;
pub const B_FUTURE_STRING: c_int = B_LANGUAGE_STRINGS_BASE + 3;
pub const B_DAY_1: c_int = B_LANGUAGE_STRINGS_BASE + 4;
pub const B_DAY_2: c_int = B_LANGUAGE_STRINGS_BASE + 5;
pub const B_DAY_3: c_int = B_LANGUAGE_STRINGS_BASE + 6;
pub const B_DAY_4: c_int = B_LANGUAGE_STRINGS_BASE + 7;
pub const B_DAY_5: c_int = B_LANGUAGE_STRINGS_BASE + 8;
pub const B_DAY_6: c_int = B_LANGUAGE_STRINGS_BASE + 9;
pub const B_DAY_7: c_int = B_LANGUAGE_STRINGS_BASE + 10;
pub const B_AB_DAY_1: c_int = B_LANGUAGE_STRINGS_BASE + 11;
pub const B_AB_DAY_2: c_int = B_LANGUAGE_STRINGS_BASE + 12;
pub const B_AB_DAY_3: c_int = B_LANGUAGE_STRINGS_BASE + 13;
pub const B_AB_DAY_4: c_int = B_LANGUAGE_STRINGS_BASE + 14;
pub const B_AB_DAY_5: c_int = B_LANGUAGE_STRINGS_BASE + 15;
pub const B_AB_DAY_6: c_int = B_LANGUAGE_STRINGS_BASE + 16;
pub const B_AB_DAY_7: c_int = B_LANGUAGE_STRINGS_BASE + 17;
pub const B_MON_1: c_int = B_LANGUAGE_STRINGS_BASE + 18;
pub const B_MON_2: c_int = B_LANGUAGE_STRINGS_BASE + 19;
pub const B_MON_3: c_int = B_LANGUAGE_STRINGS_BASE + 20;
pub const B_MON_4: c_int = B_LANGUAGE_STRINGS_BASE + 21;
pub const B_MON_5: c_int = B_LANGUAGE_STRINGS_BASE + 22;
pub const B_MON_6: c_int = B_LANGUAGE_STRINGS_BASE + 23;
pub const B_MON_7: c_int = B_LANGUAGE_STRINGS_BASE + 24;
pub const B_MON_8: c_int = B_LANGUAGE_STRINGS_BASE + 25;
pub const B_MON_9: c_int = B_LANGUAGE_STRINGS_BASE + 26;
pub const B_MON_10: c_int = B_LANGUAGE_STRINGS_BASE + 27;
pub const B_MON_11: c_int = B_LANGUAGE_STRINGS_BASE + 28;
pub const B_MON_12: c_int = B_LANGUAGE_STRINGS_BASE + 29;
pub const B_AB_MON_1: c_int = B_LANGUAGE_STRINGS_BASE + 30;
pub const B_AB_MON_2: c_int = B_LANGUAGE_STRINGS_BASE + 31;
pub const B_AB_MON_3: c_int = B_LANGUAGE_STRINGS_BASE + 32;
pub const B_AB_MON_4: c_int = B_LANGUAGE_STRINGS_BASE + 33;
pub const B_AB_MON_5: c_int = B_LANGUAGE_STRINGS_BASE + 34;
pub const B_AB_MON_6: c_int = B_LANGUAGE_STRINGS_BASE + 35;
pub const B_AB_MON_7: c_int = B_LANGUAGE_STRINGS_BASE + 36;
pub const B_AB_MON_8: c_int = B_LANGUAGE_STRINGS_BASE + 37;
pub const B_AB_MON_9: c_int = B_LANGUAGE_STRINGS_BASE + 38;
pub const B_AB_MON_10: c_int = B_LANGUAGE_STRINGS_BASE + 39;
pub const B_AB_MON_11: c_int = B_LANGUAGE_STRINGS_BASE + 40;
pub const B_AB_MON_12: c_int = B_LANGUAGE_STRINGS_BASE + 41;
pub const B_YES_EXPRESSION: c_int = B_LANGUAGE_STRINGS_BASE + 42;
pub const B_NO_EXPRESSION: c_int = B_LANGUAGE_STRINGS_BASE + 43;
pub const B_YES_STRING: c_int = B_LANGUAGE_STRINGS_BASE + 44;
pub const B_NO_STRING: c_int = B_LANGUAGE_STRINGS_BASE + 45;
pub const B_NUM_LANGUAGE_STRINGS: c_int = B_AB_MON_12 - B_LANGUAGE_STRINGS_BASE;
//  ENUM: other_locale_strings
pub const B_OTHER_STRINGS_BASE: c_int = 200;
pub const B_CODESET: c_int = B_OTHER_STRINGS_BASE;
pub const B_ERA: c_int = B_OTHER_STRINGS_BASE + 1;
pub const B_ERA_DATE_FORMAT: c_int = B_OTHER_STRINGS_BASE + 2;
pub const B_ERA_DATE_TIME_FORMAT: c_int = B_OTHER_STRINGS_BASE + 3;
pub const B_ERA_TIME_FORMAT: c_int = B_OTHER_STRINGS_BASE + 4;
pub const B_ALT_DIGITS: c_int = B_OTHER_STRINGS_BASE + 5;

//  ENUM: BNumberElement
pub const B_DECIMAL_SEPARATOR: c_int = 10;
pub const B_GROUPING_SEPARATOR: c_int = 10 + 1;

//   DUP: U_ICU_NAMESPACE

//   DUP: U_ICU_NAMESPACE

//   DUP: U_ICU_NAMESPACE
//  ENUM: time_unit_element
pub const B_TIME_UNIT_YEAR: c_int = 0;
pub const B_TIME_UNIT_MONTH: c_int = 0 + 1;
pub const B_TIME_UNIT_WEEK: c_int = 0 + 2;
pub const B_TIME_UNIT_DAY: c_int = 0 + 3;
pub const B_TIME_UNIT_HOUR: c_int = 0 + 4;
pub const B_TIME_UNIT_MINUTE: c_int = 0 + 5;
pub const B_TIME_UNIT_SECOND: c_int = 0 + 6;
pub const B_TIME_UNIT_LAST: c_int = B_TIME_UNIT_SECOND;
//  ENUM: time_unit_style
pub const B_TIME_UNIT_ABBREVIATED: c_int = 0;
pub const B_TIME_UNIT_FULL: c_int = 0 + 1;

//   DUP: U_ICU_NAMESPACE

//  ENUM: unicode_char_category
pub const B_UNICODE_UNASSIGNED: c_int = 0;
pub const B_UNICODE_GENERAL_OTHER_TYPES: c_int = 0;
pub const B_UNICODE_UPPERCASE_LETTER: c_int = 1;
pub const B_UNICODE_LOWERCASE_LETTER: c_int = 2;
pub const B_UNICODE_TITLECASE_LETTER: c_int = 3;
pub const B_UNICODE_MODIFIER_LETTER: c_int = 4;
pub const B_UNICODE_OTHER_LETTER: c_int = 5;
pub const B_UNICODE_NON_SPACING_MARK: c_int = 6;
pub const B_UNICODE_ENCLOSING_MARK: c_int = 7;
pub const B_UNICODE_COMBINING_SPACING_MARK: c_int = 8;
pub const B_UNICODE_DECIMAL_DIGIT_NUMBER: c_int = 9;
pub const B_UNICODE_LETTER_NUMBER: c_int = 10;
pub const B_UNICODE_OTHER_NUMBER: c_int = 11;
pub const B_UNICODE_SPACE_SEPARATOR: c_int = 12;
pub const B_UNICODE_LINE_SEPARATOR: c_int = 13;
pub const B_UNICODE_PARAGRAPH_SEPARATOR: c_int = 14;
pub const B_UNICODE_CONTROL_CHAR: c_int = 15;
pub const B_UNICODE_FORMAT_CHAR: c_int = 16;
pub const B_UNICODE_PRIVATE_USE_CHAR: c_int = 17;
pub const B_UNICODE_SURROGATE: c_int = 18;
pub const B_UNICODE_DASH_PUNCTUATION: c_int = 19;
pub const B_UNICODE_START_PUNCTUATION: c_int = 20;
pub const B_UNICODE_END_PUNCTUATION: c_int = 21;
pub const B_UNICODE_CONNECTOR_PUNCTUATION: c_int = 22;
pub const B_UNICODE_OTHER_PUNCTUATION: c_int = 23;
pub const B_UNICODE_MATH_SYMBOL: c_int = 24;
pub const B_UNICODE_CURRENCY_SYMBOL: c_int = 25;
pub const B_UNICODE_MODIFIER_SYMBOL: c_int = 26;
pub const B_UNICODE_OTHER_SYMBOL: c_int = 27;
pub const B_UNICODE_INITIAL_PUNCTUATION: c_int = 28;
pub const B_UNICODE_FINAL_PUNCTUATION: c_int = 29;
pub const B_UNICODE_CATEGORY_COUNT: c_int = 29 + 1;
//  ENUM: unicode_char_direction
pub const B_UNICODE_LEFT_TO_RIGHT: c_int = 0;
pub const B_UNICODE_RIGHT_TO_LEFT: c_int = 1;
pub const B_UNICODE_EUROPEAN_NUMBER: c_int = 2;
pub const B_UNICODE_EUROPEAN_NUMBER_SEPARATOR: c_int = 3;
pub const B_UNICODE_EUROPEAN_NUMBER_TERMINATOR: c_int = 4;
pub const B_UNICODE_ARABIC_NUMBER: c_int = 5;
pub const B_UNICODE_COMMON_NUMBER_SEPARATOR: c_int = 6;
pub const B_UNICODE_BLOCK_SEPARATOR: c_int = 7;
pub const B_UNICODE_SEGMENT_SEPARATOR: c_int = 8;
pub const B_UNICODE_WHITE_SPACE_NEUTRAL: c_int = 9;
pub const B_UNICODE_OTHER_NEUTRAL: c_int = 10;
pub const B_UNICODE_LEFT_TO_RIGHT_EMBEDDING: c_int = 11;
pub const B_UNICODE_LEFT_TO_RIGHT_OVERRIDE: c_int = 12;
pub const B_UNICODE_RIGHT_TO_LEFT_ARABIC: c_int = 13;
pub const B_UNICODE_RIGHT_TO_LEFT_EMBEDDING: c_int = 14;
pub const B_UNICODE_RIGHT_TO_LEFT_OVERRIDE: c_int = 15;
pub const B_UNICODE_POP_DIRECTIONAL_FORMAT: c_int = 16;
pub const B_UNICODE_DIR_NON_SPACING_MARK: c_int = 17;
pub const B_UNICODE_BOUNDARY_NEUTRAL: c_int = 18;
pub const B_UNICODE_DIRECTION_COUNT: c_int = 18 + 1;
//  ENUM: unicode_char_script
pub const B_UNICODE_NO_BLOCK: c_int = 0;
pub const B_UNICODE_BASIC_LATIN: c_int = 1;
pub const B_UNICODE_LATIN_1_SUPPLEMENT: c_int = 2;
pub const B_UNICODE_LATIN_EXTENDED_A: c_int = 3;
pub const B_UNICODE_LATIN_EXTENDED_B: c_int = 4;
pub const B_UNICODE_IPA_EXTENSIONS: c_int = 5;
pub const B_UNICODE_SPACING_MODIFIER_LETTERS: c_int = 6;
pub const B_UNICODE_COMBINING_DIACRITICAL_MARKS: c_int = 7;
pub const B_UNICODE_GREEK: c_int = 8;
pub const B_UNICODE_CYRILLIC: c_int = 9;
pub const B_UNICODE_ARMENIAN: c_int = 10;
pub const B_UNICODE_HEBREW: c_int = 11;
pub const B_UNICODE_ARABIC: c_int = 12;
pub const B_UNICODE_SYRIAC: c_int = 13;
pub const B_UNICODE_THAANA: c_int = 14;
pub const B_UNICODE_DEVANAGARI: c_int = 15;
pub const B_UNICODE_BENGALI: c_int = 16;
pub const B_UNICODE_GURMUKHI: c_int = 17;
pub const B_UNICODE_GUJARATI: c_int = 18;
pub const B_UNICODE_ORIYA: c_int = 19;
pub const B_UNICODE_TAMIL: c_int = 20;
pub const B_UNICODE_TELUGU: c_int = 21;
pub const B_UNICODE_KANNADA: c_int = 22;
pub const B_UNICODE_MALAYALAM: c_int = 23;
pub const B_UNICODE_SINHALA: c_int = 24;
pub const B_UNICODE_THAI: c_int = 25;
pub const B_UNICODE_LAO: c_int = 26;
pub const B_UNICODE_TIBETAN: c_int = 27;
pub const B_UNICODE_MYANMAR: c_int = 28;
pub const B_UNICODE_GEORGIAN: c_int = 29;
pub const B_UNICODE_HANGUL_JAMO: c_int = 30;
pub const B_UNICODE_ETHIOPIC: c_int = 31;
pub const B_UNICODE_CHEROKEE: c_int = 32;
pub const B_UNICODE_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS: c_int = 33;
pub const B_UNICODE_OGHAM: c_int = 34;
pub const B_UNICODE_RUNIC: c_int = 35;
pub const B_UNICODE_KHMER: c_int = 36;
pub const B_UNICODE_MONGOLIAN: c_int = 37;
pub const B_UNICODE_LATIN_EXTENDED_ADDITIONAL: c_int = 38;
pub const B_UNICODE_GREEK_EXTENDED: c_int = 39;
pub const B_UNICODE_GENERAL_PUNCTUATION: c_int = 40;
pub const B_UNICODE_SUPERSCRIPTS_AND_SUBSCRIPTS: c_int = 41;
pub const B_UNICODE_CURRENCY_SYMBOLS: c_int = 42;
pub const B_UNICODE_COMBINING_MARKS_FOR_SYMBOLS: c_int = 43;
pub const B_UNICODE_LETTERLIKE_SYMBOLS: c_int = 44;
pub const B_UNICODE_NUMBER_FORMS: c_int = 45;
pub const B_UNICODE_ARROWS: c_int = 46;
pub const B_UNICODE_MATHEMATICAL_OPERATORS: c_int = 47;
pub const B_UNICODE_MISCELLANEOUS_TECHNICAL: c_int = 48;
pub const B_UNICODE_CONTROL_PICTURES: c_int = 49;
pub const B_UNICODE_OPTICAL_CHARACTER_RECOGNITION: c_int = 50;
pub const B_UNICODE_ENCLOSED_ALPHANUMERICS: c_int = 51;
pub const B_UNICODE_BOX_DRAWING: c_int = 52;
pub const B_UNICODE_BLOCK_ELEMENTS: c_int = 53;
pub const B_UNICODE_GEOMETRIC_SHAPES: c_int = 54;
pub const B_UNICODE_MISCELLANEOUS_SYMBOLS: c_int = 55;
pub const B_UNICODE_DINGBATS: c_int = 56;
pub const B_UNICODE_BRAILLE_PATTERNS: c_int = 57;
pub const B_UNICODE_CJK_RADICALS_SUPPLEMENT: c_int = 58;
pub const B_UNICODE_KANGXI_RADICALS: c_int = 59;
pub const B_UNICODE_IDEOGRAPHIC_DESCRIPTION_CHARACTERS: c_int = 60;
pub const B_UNICODE_CJK_SYMBOLS_AND_PUNCTUATION: c_int = 61;
pub const B_UNICODE_HIRAGANA: c_int = 62;
pub const B_UNICODE_KATAKANA: c_int = 63;
pub const B_UNICODE_BOPOMOFO: c_int = 64;
pub const B_UNICODE_HANGUL_COMPATIBILITY_JAMO: c_int = 65;
pub const B_UNICODE_KANBUN: c_int = 66;
pub const B_UNICODE_BOPOMOFO_EXTENDED: c_int = 67;
pub const B_UNICODE_ENCLOSED_CJK_LETTERS_AND_MONTHS: c_int = 68;
pub const B_UNICODE_CJK_COMPATIBILITY: c_int = 69;
pub const B_UNICODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A: c_int = 70;
pub const B_UNICODE_CJK_UNIFIED_IDEOGRAPHS: c_int = 71;
pub const B_UNICODE_YI_SYLLABLES: c_int = 72;
pub const B_UNICODE_YI_RADICALS: c_int = 73;
pub const B_UNICODE_HANGUL_SYLLABLES: c_int = 74;
pub const B_UNICODE_HIGH_SURROGATES: c_int = 75;
pub const B_UNICODE_HIGH_PRIVATE_USE_SURROGATES: c_int = 76;
pub const B_UNICODE_LOW_SURROGATES: c_int = 77;
pub const B_UNICODE_PRIVATE_USE: c_int = 78;
pub const B_UNICODE_PRIVATE_USE_AREA: c_int = B_UNICODE_PRIVATE_USE;
pub const B_UNICODE_CJK_COMPATIBILITY_IDEOGRAPHS: c_int = 79;
pub const B_UNICODE_ALPHABETIC_PRESENTATION_FORMS: c_int = 80;
pub const B_UNICODE_ARABIC_PRESENTATION_FORMS_A: c_int = 81;
pub const B_UNICODE_COMBINING_HALF_MARKS: c_int = 82;
pub const B_UNICODE_CJK_COMPATIBILITY_FORMS: c_int = 83;
pub const B_UNICODE_SMALL_FORM_VARIANTS: c_int = 84;
pub const B_UNICODE_ARABIC_PRESENTATION_FORMS_B: c_int = 85;
pub const B_UNICODE_SPECIALS: c_int = 86;
pub const B_UNICODE_HALFWIDTH_AND_FULLWIDTH_FORMS: c_int = 87;
pub const B_UNICODE_OLD_ITALIC: c_int = 88;
pub const B_UNICODE_GOTHIC: c_int = 89;
pub const B_UNICODE_DESERET: c_int = 90;
pub const B_UNICODE_BYZANTINE_MUSICAL_SYMBOLS: c_int = 91;
pub const B_UNICODE_MUSICAL_SYMBOLS: c_int = 92;
pub const B_UNICODE_MATHEMATICAL_ALPHANUMERIC_SYMBOLS: c_int = 93;
pub const B_UNICODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B: c_int = 94;
pub const B_UNICODE_CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT: c_int = 95;
pub const B_UNICODE_TAGS: c_int = 96;
pub const B_UNICODE_CYRILLIC_SUPPLEMENTARY: c_int = 97;
pub const B_UNICODE_CYRILLIC_SUPPLEMENT: c_int = B_UNICODE_CYRILLIC_SUPPLEMENTARY;
pub const B_UNICODE_TAGALOG: c_int = 98;
pub const B_UNICODE_HANUNOO: c_int = 99;
pub const B_UNICODE_BUHID: c_int = 100;
pub const B_UNICODE_TAGBANWA: c_int = 101;
pub const B_UNICODE_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A: c_int = 102;
pub const B_UNICODE_SUPPLEMENTAL_ARROWS_A: c_int = 103;
pub const B_UNICODE_SUPPLEMENTAL_ARROWS_B: c_int = 104;
pub const B_UNICODE_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B: c_int = 105;
pub const B_UNICODE_SUPPLEMENTAL_MATHEMATICAL_OPERATORS: c_int = 106;
pub const B_UNICODE_KATAKANA_PHONETIC_EXTENSIONS: c_int = 107;
pub const B_UNICODE_VARIATION_SELECTORS: c_int = 108;
pub const B_UNICODE_SUPPLEMENTARY_PRIVATE_USE_AREA_A: c_int = 109;
pub const B_UNICODE_SUPPLEMENTARY_PRIVATE_USE_AREA_B: c_int = 110;
pub const B_UNICODE_LIMBU: c_int = 111;
pub const B_UNICODE_TAI_LE: c_int = 112;
pub const B_UNICODE_KHMER_SYMBOLS: c_int = 113;
pub const B_UNICODE_PHONETIC_EXTENSIONS: c_int = 114;
pub const B_UNICODE_MISCELLANEOUS_SYMBOLS_AND_ARROWS: c_int = 115;
pub const B_UNICODE_YIJING_HEXAGRAM_SYMBOLS: c_int = 116;
pub const B_UNICODE_LINEAR_B_SYLLABARY: c_int = 117;
pub const B_UNICODE_LINEAR_B_IDEOGRAMS: c_int = 118;
pub const B_UNICODE_AEGEAN_NUMBERS: c_int = 119;
pub const B_UNICODE_UGARITIC: c_int = 120;
pub const B_UNICODE_SHAVIAN: c_int = 121;
pub const B_UNICODE_OSMANYA: c_int = 122;
pub const B_UNICODE_CYPRIOT_SYLLABARY: c_int = 123;
pub const B_UNICODE_TAI_XUAN_JING_SYMBOLS: c_int = 124;
pub const B_UNICODE_VARIATION_SELECTORS_SUPPLEMENT: c_int = 125;
pub const B_UNICODE_ANCIENT_GREEK_MUSICAL_NOTATION: c_int = 126;
pub const B_UNICODE_ANCIENT_GREEK_NUMBERS: c_int = 127;
pub const B_UNICODE_ARABIC_SUPPLEMENT: c_int = 128;
pub const B_UNICODE_BUGINESE: c_int = 129;
pub const B_UNICODE_CJK_STROKES: c_int = 130;
pub const B_UNICODE_COMBINING_DIACRITICAL_MARKS_SUPPLEMENT: c_int = 131;
pub const B_UNICODE_COPTIC: c_int = 132;
pub const B_UNICODE_ETHIOPIC_EXTENDED: c_int = 133;
pub const B_UNICODE_ETHIOPIC_SUPPLEMENT: c_int = 134;
pub const B_UNICODE_GEORGIAN_SUPPLEMENT: c_int = 135;
pub const B_UNICODE_GLAGOLITIC: c_int = 136;
pub const B_UNICODE_KHAROSHTHI: c_int = 137;
pub const B_UNICODE_MODIFIER_TONE_LETTERS: c_int = 138;
pub const B_UNICODE_NEW_TAI_LUE: c_int = 139;
pub const B_UNICODE_OLD_PERSIAN: c_int = 140;
pub const B_UNICODE_PHONETIC_EXTENSIONS_SUPPLEMENT: c_int = 141;
pub const B_UNICODE_SUPPLEMENTAL_PUNCTUATION: c_int = 142;
pub const B_UNICODE_SYLOTI_NAGRI: c_int = 143;
pub const B_UNICODE_TIFINAGH: c_int = 144;
pub const B_UNICODE_VERTICAL_FORMS: c_int = 145;
pub const B_UNICODE_NKO: c_int = 146;
pub const B_UNICODE_BALINESE: c_int = 147;
pub const B_UNICODE_LATIN_EXTENDED_C: c_int = 148;
pub const B_UNICODE_LATIN_EXTENDED_D: c_int = 149;
pub const B_UNICODE_PHAGS_PA: c_int = 150;
pub const B_UNICODE_PHOENICIAN: c_int = 151;
pub const B_UNICODE_CUNEIFORM: c_int = 152;
pub const B_UNICODE_CUNEIFORM_NUMBERS_AND_PUNCTUATION: c_int = 153;
pub const B_UNICODE_COUNTING_ROD_NUMERALS: c_int = 154;
pub const B_UNICODE_SUNDANESE: c_int = 155;
pub const B_UNICODE_LEPCHA: c_int = 156;
pub const B_UNICODE_OL_CHIKI: c_int = 157;
pub const B_UNICODE_CYRILLIC_EXTENDED_A: c_int = 158;
pub const B_UNICODE_VAI: c_int = 159;
pub const B_UNICODE_CYRILLIC_EXTENDED_B: c_int = 160;
pub const B_UNICODE_SAURASHTRA: c_int = 161;
pub const B_UNICODE_KAYAH_LI: c_int = 162;
pub const B_UNICODE_REJANG: c_int = 163;
pub const B_UNICODE_CHAM: c_int = 164;
pub const B_UNICODE_ANCIENT_SYMBOLS: c_int = 165;
pub const B_UNICODE_PHAISTOS_DISC: c_int = 166;
pub const B_UNICODE_LYCIAN: c_int = 167;
pub const B_UNICODE_CARIAN: c_int = 168;
pub const B_UNICODE_LYDIAN: c_int = 169;
pub const B_UNICODE_MAHJONG_TILES: c_int = 170;
pub const B_UNICODE_DOMINO_TILES: c_int = 171;
pub const B_UNICODE_SAMARITAN: c_int = 172;
pub const B_UNICODE_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED: c_int = 173;
pub const B_UNICODE_TAI_THAM: c_int = 174;
pub const B_UNICODE_VEDIC_EXTENSIONS: c_int = 175;
pub const B_UNICODE_LISU: c_int = 176;
pub const B_UNICODE_BAMUM: c_int = 177;
pub const B_UNICODE_COMMON_INDIC_NUMBER_FORMS: c_int = 178;
pub const B_UNICODE_DEVANAGARI_EXTENDED: c_int = 179;
pub const B_UNICODE_HANGUL_JAMO_EXTENDED_A: c_int = 180;
pub const B_UNICODE_JAVANESE: c_int = 181;
pub const B_UNICODE_MYANMAR_EXTENDED_A: c_int = 182;
pub const B_UNICODE_TAI_VIET: c_int = 183;
pub const B_UNICODE_MEETEI_MAYEK: c_int = 184;
pub const B_UNICODE_HANGUL_JAMO_EXTENDED_B: c_int = 185;
pub const B_UNICODE_IMPERIAL_ARAMAIC: c_int = 186;
pub const B_UNICODE_OLD_SOUTH_ARABIAN: c_int = 187;
pub const B_UNICODE_AVESTAN: c_int = 188;
pub const B_UNICODE_INSCRIPTIONAL_PARTHIAN: c_int = 189;
pub const B_UNICODE_INSCRIPTIONAL_PAHLAVI: c_int = 190;
pub const B_UNICODE_OLD_TURKIC: c_int = 191;
pub const B_UNICODE_RUMI_NUMERAL_SYMBOLS: c_int = 192;
pub const B_UNICODE_KAITHI: c_int = 193;
pub const B_UNICODE_EGYPTIAN_HIEROGLYPHS: c_int = 194;
pub const B_UNICODE_ENCLOSED_ALPHANUMERIC_SUPPLEMENT: c_int = 195;
pub const B_UNICODE_ENCLOSED_IDEOGRAPHIC_SUPPLEMENT: c_int = 196;
pub const B_UNICODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C: c_int = 197;
pub const B_UNICODE_MANDAIC: c_int = 198;
pub const B_UNICODE_BATAK: c_int = 199;
pub const B_UNICODE_ETHIOPIC_EXTENDED_A: c_int = 200;
pub const B_UNICODE_BRAHMI: c_int = 201;
pub const B_UNICODE_BAMUM_SUPPLEMENT: c_int = 202;
pub const B_UNICODE_KANA_SUPPLEMENT: c_int = 203;
pub const B_UNICODE_PLAYING_CARDS: c_int = 204;
pub const B_UNICODE_MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS: c_int = 205;
pub const B_UNICODE_EMOTICONS: c_int = 206;
pub const B_UNICODE_TRANSPORT_AND_MAP_SYMBOLS: c_int = 207;
pub const B_UNICODE_ALCHEMICAL_SYMBOLS: c_int = 208;
pub const B_UNICODE_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D: c_int = 209;
pub const B_UNICODE_SCRIPT_COUNT: c_int = 210;
pub const B_UNICODE_NO_SCRIPT: c_int = B_UNICODE_SCRIPT_COUNT;
pub const B_UNICODE_INVALID_CODE: c_int = -1;
//  ENUM: unicode_east_asian_width
pub const B_UNICODE_EA_NEUTRAL: c_int = 0;
pub const B_UNICODE_EA_AMBIGUOUS: c_int = 0 + 1;
pub const B_UNICODE_EA_HALFWIDTH: c_int = 0 + 2;
pub const B_UNICODE_EA_FULLWIDTH: c_int = 0 + 3;
pub const B_UNICODE_EA_NARROW: c_int = 0 + 4;
pub const B_UNICODE_EA_WIDE: c_int = 0 + 5;
pub const B_UNICODE_EA_COUNT: c_int = 0 + 6;

pub const B_CHECK_CONTINUOSLY: c_int = 3;
pub const B_CHECK_CONTINUOUSLY: c_int = 3;
pub const B_CHECK_DAILY: c_int = 2;
pub const B_CHECK_NEVER: c_int = 0;
pub const B_CHECK_WEEKDAYS: c_int = 1;
pub const B_MAIL_ATTR_ACCOUNT: &str = "MAIL:account";
pub const B_MAIL_ATTR_ACCOUNT_ID: &str = "MAIL:account_id";
pub const B_MAIL_ATTR_BCC: &str = "MAIL:bcc";
pub const B_MAIL_ATTR_CC: &str = "MAIL:cc";
pub const B_MAIL_ATTR_CONTENT: &str = "MAIL:content_length";
pub const B_MAIL_ATTR_FLAGS: &str = "MAIL:flags";
pub const B_MAIL_ATTR_FROM: &str = "MAIL:from";
pub const B_MAIL_ATTR_HEADER: &str = "MAIL:header_length";
pub const B_MAIL_ATTR_MIME: &str = "MAIL:mime";
pub const B_MAIL_ATTR_NAME: &str = "MAIL:name";
pub const B_MAIL_ATTR_PRIORITY: &str = "MAIL:priority";
pub const B_MAIL_ATTR_READ: &str = "MAIL:read";
pub const B_MAIL_ATTR_RECIPIENTS: &str = "MAIL:recipients";
pub const B_MAIL_ATTR_REPLY: &str = "MAIL:reply";
pub const B_MAIL_ATTR_STATUS: &str = "MAIL:status";
pub const B_MAIL_ATTR_SUBJECT: &str = "MAIL:subject";
pub const B_MAIL_ATTR_THREAD: &str = "MAIL:thread";
pub const B_MAIL_ATTR_TO: &str = "MAIL:to";
pub const B_MAIL_ATTR_WHEN: &str = "MAIL:when";
pub const B_MAIL_BCC: &str = "Bcc: ";
pub const B_MAIL_CC: &str = "Cc: ";
pub const B_MAIL_DATE: &str = "Date: ";
pub const B_MAIL_FROM: &str = "From: ";
pub const B_MAIL_PRIORITY: &str = "Priority: ";
pub const B_MAIL_REPLY: &str = "Reply-To: ";
pub const B_MAIL_SUBJECT: &str = "Subject: ";
pub const B_MAIL_TO: &str = "To: ";
pub const B_MAIL_TYPE: &str = "text/x-email";
pub const B_MAX_HOST_NAME_LENGTH: c_int = 64;
pub const B_MAX_USER_NAME_LENGTH: c_int = 32;
pub const B_PARTIAL_MAIL_TYPE: &str = "text/x-partial-email";
//  ENUM: mail_flags
pub const B_MAIL_PENDING: c_int = 1;
pub const B_MAIL_SENT: c_int = 2;
pub const B_MAIL_SAVE: c_int = 4;
//  ENUM: read_flags
pub const B_UNREAD: c_int = 0;
pub const B_SEEN: c_int = 1;
pub const B_READ: c_int = 2;

pub const B_MAIL_NULL_CONVERSION: c_int = ((uint32) -1);
pub const B_MAIL_US_ASCII_CONVERSION: c_int = ((uint32) -3);
pub const B_MAIL_UTF8_CONVERSION: c_int = ((uint32) -2);
pub const BASE64_LINELENGTH: c_int = 76;
//  ENUM: mail_encoding
pub const base64: char = 0x62 /* 'b' */;
pub const quoted_printable: char = 0x71 /* 'q' */;
pub const seven_bit: char = 0x37 /* '7' */;
pub const eight_bit: char = 0x38 /* '8' */;
pub const uuencode: char = 0x75 /* 'u' */;
pub const null_encoding: c_int = 0;
pub const no_encoding: c_int = -1;

//  ENUM: component_type
pub const B_MAIL_PLAIN_TEXT_BODY: c_int = 0;
pub const B_MAIL_SIMPLE_ATTACHMENT: c_int = 0 + 1;
pub const B_MAIL_ATTRIBUTED_ATTACHMENT: c_int = 0 + 2;
pub const B_MAIL_MULTIPART_CONTAINER: c_int = 0 + 3;

//  ENUM: @54
pub const B_MAIL_BODY_FETCHED: c_int = 0x5f4d6266 /* '_Mbf' */;

//   DUP: B_MAIL_ATTR_ACCOUNT
//   DUP: B_MAIL_ATTR_THREAD
//  ENUM: mail_reply_to_mode
pub const B_MAIL_REPLY_TO: c_int = 0;
pub const B_MAIL_REPLY_TO_ALL: c_int = 0 + 1;
pub const B_MAIL_REPLY_TO_SENDER: c_int = 0 + 2;

//  ENUM: b_mail_status_window_option
pub const B_MAIL_SHOW_STATUS_WINDOW_NEVER: c_int = 0;
pub const B_MAIL_SHOW_STATUS_WINDOW_WHEN_SENDING: c_int = 1;
pub const B_MAIL_SHOW_STATUS_WINDOW_WHEN_ACTIVE: c_int = 2;
pub const B_MAIL_SHOW_STATUS_WINDOW_ALWAYS: c_int = 3;

//  ENUM: @57
pub const B_FLAVOR_IS_GLOBAL: c_long = 0x100000;
pub const B_FLAVOR_IS_LOCAL: c_long = 0x200000;

pub const B_MEDIA_ANY_QUALITY: f32 = 0.0;
pub const B_MEDIA_HIGH_QUALITY: f32 = 1.0;
pub const B_MEDIA_LOW_QUALITY: f32 = 0.1;
pub const B_MEDIA_MEDIUM_QUALITY: f32 = 0.5;
pub const B_MEDIA_NAME_LENGTH: c_int = 64;
//  ENUM: @58
pub const B_MEDIA_WILDCARD: c_int = 0x54525743 /* 'TRWC' */;
pub const B_MEDIA_NODE_CREATED: c_int = 0x54524941 /* 'TRIA' */;
pub const B_MEDIA_NODE_DELETED: c_int = 0x54524941 /* 'TRIA' */ + 1;
pub const B_MEDIA_CONNECTION_MADE: c_int = 0x54524941 /* 'TRIA' */ + 2;
pub const B_MEDIA_CONNECTION_BROKEN: c_int = 0x54524941 /* 'TRIA' */ + 3;
pub const B_MEDIA_BUFFER_CREATED: c_int = 0x54524941 /* 'TRIA' */ + 4;
pub const B_MEDIA_BUFFER_DELETED: c_int = 0x54524941 /* 'TRIA' */ + 5;
pub const B_MEDIA_TRANSPORT_STATE: c_int = 0x54524941 /* 'TRIA' */ + 6;
pub const B_MEDIA_PARAMETER_CHANGED: c_int = 0x54524941 /* 'TRIA' */ + 7;
pub const B_MEDIA_FORMAT_CHANGED: c_int = 0x54524941 /* 'TRIA' */ + 8;
pub const B_MEDIA_WEB_CHANGED: c_int = 0x54524941 /* 'TRIA' */ + 9;
pub const B_MEDIA_DEFAULT_CHANGED: c_int = 0x54524941 /* 'TRIA' */ + 10;
pub const B_MEDIA_NEW_PARAMETER_VALUE: c_int = 0x54524941 /* 'TRIA' */ + 11;
pub const B_MEDIA_NODE_STOPPED: c_int = 0x54524941 /* 'TRIA' */ + 12;
pub const B_MEDIA_FLAVORS_CHANGED: c_int = 0x54524941 /* 'TRIA' */ + 13;
pub const B_MEDIA_SERVER_STARTED: c_int = 0x54524941 /* 'TRIA' */ + 14;
pub const B_MEDIA_SERVER_QUIT: c_int = 0x54524941 /* 'TRIA' */ + 15;
//  ENUM: @59
pub const B_MEDIA_BIG_ENDIAN: c_int = 1;
pub const B_MEDIA_LITTLE_ENDIAN: c_int = 2;
pub const B_MEDIA_HOST_ENDIAN: c_int = B_MEDIA_LITTLE_ENDIAN;
//  ENUM: @60
pub const B_BIG_ENDIAN: c_int = 0;
pub const B_LITTLE_ENDIAN: c_int = 0 + 1;
//  ENUM: @61
pub const B_UNDEFINED_SAMPLES: c_int = 0;
pub const B_LINEAR_SAMPLES: c_int = 0 + 1;
pub const B_FLOAT_SAMPLES: c_int = 0 + 2;
pub const B_MULAW_SAMPLES: c_int = 0 + 3;
//  ENUM: media_display_flags
pub const B_F1_DOMINANT: c_int = 0x1;
pub const B_F2_DOMINANT: c_int = 0x2;
pub const B_TOP_SCANLINE_F1: c_int = 0x4;
pub const B_TOP_SCANLINE_F2: c_int = 0x8;
//  ENUM: media_flags
pub const B_MEDIA_FLAGS_VERSION: c_int = 1;
pub const B_MEDIA_FLAGS_PRIVATE: c_int = 0x40000000;
//  ENUM: media_format_family
pub const B_ANY_FORMAT_FAMILY: c_int = 0;
pub const B_BEOS_FORMAT_FAMILY: c_int = 1;
pub const B_QUICKTIME_FORMAT_FAMILY: c_int = 2;
pub const B_AVI_FORMAT_FAMILY: c_int = 3;
pub const B_ASF_FORMAT_FAMILY: c_int = 4;
pub const B_MPEG_FORMAT_FAMILY: c_int = 5;
pub const B_WAV_FORMAT_FAMILY: c_int = 6;
pub const B_AIFF_FORMAT_FAMILY: c_int = 7;
pub const B_AVR_FORMAT_FAMILY: c_int = 8;
pub const B_MISC_FORMAT_FAMILY: c_int = 99999;
//  ENUM: media_format_flags
pub const B_MEDIA_RETAINED_DATA: c_int = 0x1;
pub const B_MEDIA_MULTIPLE_BUFFERS: c_int = 0x2;
pub const B_MEDIA_CONTIGUOUS_BUFFER: c_int = 0x4;
pub const B_MEDIA_LINEAR_UPDATES: c_int = 0x8;
pub const B_MEDIA_MAUI_UNDEFINED_FLAGS: c_int = ~0xf;
//  ENUM: media_frame_flags
pub const B_MEDIA_KEY_FRAME: c_int = 0x1;
//  ENUM: media_multi_channels
pub const B_CHANNEL_LEFT: c_int = 0x00001;
pub const B_CHANNEL_RIGHT: c_int = 0x00002;
pub const B_CHANNEL_CENTER: c_int = 0x00004;
pub const B_CHANNEL_SUB: c_int = 0x00008;
pub const B_CHANNEL_REARLEFT: c_int = 0x00010;
pub const B_CHANNEL_REARRIGHT: c_int = 0x00020;
pub const B_CHANNEL_FRONT_LEFT_CENTER: c_int = 0x00040;
pub const B_CHANNEL_FRONT_RIGHT_CENTER: c_int = 0x00080;
pub const B_CHANNEL_BACK_CENTER: c_int = 0x00100;
pub const B_CHANNEL_SIDE_LEFT: c_int = 0x00200;
pub const B_CHANNEL_SIDE_RIGHT: c_int = 0x00400;
pub const B_CHANNEL_TOP_CENTER: c_int = 0x00800;
pub const B_CHANNEL_TOP_FRONT_LEFT: c_int = 0x01000;
pub const B_CHANNEL_TOP_FRONT_CENTER: c_int = 0x02000;
pub const B_CHANNEL_TOP_FRONT_RIGHT: c_int = 0x04000;
pub const B_CHANNEL_TOP_BACK_LEFT: c_int = 0x08000;
pub const B_CHANNEL_TOP_BACK_CENTER: c_int = 0x10000;
pub const B_CHANNEL_TOP_BACK_RIGHT: c_int = 0x20000;
//  ENUM: media_multi_matrix
pub const B_MATRIX_PROLOGIC_LR: c_int = 0x1;
pub const B_MATRIX_AMBISONIC_WXYZ: c_int = 0x4;
//  ENUM: media_producer_status
pub const B_DATA_NOT_AVAILABLE: c_int = 1;
pub const B_DATA_AVAILABLE: c_int = 2;
pub const B_PRODUCER_STOPPED: c_int = 3;
//  ENUM: media_realtime_flags
pub const B_MEDIA_REALTIME_ALLOCATOR: c_int = 0x1;
pub const B_MEDIA_REALTIME_AUDIO: c_int = 0x2;
pub const B_MEDIA_REALTIME_VIDEO: c_int = 0x4;
pub const B_MEDIA_REALTIME_ANYKIND: c_int = 0xffff;
//  ENUM: media_type
pub const B_MEDIA_NO_TYPE: c_int = -1;
pub const B_MEDIA_UNKNOWN_TYPE: c_int = 0;
pub const B_MEDIA_RAW_AUDIO: c_int = 1;
pub const B_MEDIA_RAW_VIDEO: c_int = 1 + 1;
pub const B_MEDIA_VBL: c_int = 1 + 2;
pub const B_MEDIA_TIMECODE: c_int = 1 + 3;
pub const B_MEDIA_MIDI: c_int = 1 + 4;
pub const B_MEDIA_TEXT: c_int = 1 + 5;
pub const B_MEDIA_HTML: c_int = 1 + 6;
pub const B_MEDIA_MULTISTREAM: c_int = 1 + 7;
pub const B_MEDIA_PARAMETERS: c_int = 1 + 8;
pub const B_MEDIA_ENCODED_AUDIO: c_int = 1 + 9;
pub const B_MEDIA_ENCODED_VIDEO: c_int = 1 + 10;
pub const B_MEDIA_PRIVATE: c_int = 90000;
pub const B_MEDIA_FIRST_USER_TYPE: c_int = 100000;
//  ENUM: node_kind
pub const B_BUFFER_PRODUCER: c_int = 0x1;
pub const B_BUFFER_CONSUMER: c_int = 0x2;
pub const B_TIME_SOURCE: c_int = 0x4;
pub const B_CONTROLLABLE: c_int = 0x8;
pub const B_FILE_INTERFACE: c_int = 0x10;
pub const B_ENTITY_INTERFACE: c_int = 0x20;
pub const B_PHYSICAL_INPUT: c_int = 0x10000;
pub const B_PHYSICAL_OUTPUT: c_int = 0x20000;
pub const B_SYSTEM_MIXER: c_int = 0x40000;
//  ENUM: video_orientation
pub const B_VIDEO_TOP_LEFT_RIGHT: c_int = 1;
pub const B_VIDEO_BOTTOM_LEFT_RIGHT: c_int = 1 + 1;

//  ENUM: @71
pub const B_MEDIA_FILE_REPLACE_MODE: c_int = 0x00000001;
pub const B_MEDIA_FILE_NO_READ_AHEAD: c_int = 0x00000002;
pub const B_MEDIA_FILE_UNBUFFERED: c_int = 0x00000006;
pub const B_MEDIA_FILE_BIG_BUFFERS: c_int = 0x00000008;

//  ENUM: beos_format
pub const B_BEOS_FORMAT_RAW_AUDIO: c_int = 0x72617761 /* 'rawa' */;
pub const B_BEOS_FORMAT_RAW_VIDEO: c_int = 0x72617776 /* 'rawv' */;
//  ENUM: media_file_accept_format_flags
pub const B_MEDIA_REJECT_WILDCARDS: c_int = 0x1;
//  ENUM: mpeg_id
pub const B_MPEG_ANY: c_int = 0;
pub const B_MPEG_1_AUDIO_LAYER_1: c_int = 0x101;
pub const B_MPEG_1_AUDIO_LAYER_2: c_int = 0x102;
pub const B_MPEG_1_AUDIO_LAYER_3: c_int = 0x103;
pub const B_MPEG_1_VIDEO: c_int = 0x111;
pub const B_MPEG_2_AUDIO_LAYER_1: c_int = 0x201;
pub const B_MPEG_2_AUDIO_LAYER_2: c_int = 0x202;
pub const B_MPEG_2_AUDIO_LAYER_3: c_int = 0x203;
pub const B_MPEG_2_VIDEO: c_int = 0x211;
pub const B_MPEG_2_5_AUDIO_LAYER_1: c_int = 0x301;
pub const B_MPEG_2_5_AUDIO_LAYER_2: c_int = 0x302;
pub const B_MPEG_2_5_AUDIO_LAYER_3: c_int = 0x303;

//  ENUM: bus_type
pub const B_ISA_BUS: c_int = 0;
pub const B_PCI_BUS: c_int = 0 + 1;
pub const B_PCMCIA_BUS: c_int = 0 + 2;
pub const B_UNKNOWN_BUS: c_int = 0x80;

//  ENUM: media_seek_type
pub const B_MEDIA_SEEK_CLOSEST_FORWARD: c_int = 1;
pub const B_MEDIA_SEEK_CLOSEST_BACKWARD: c_int = 2;
pub const B_MEDIA_SEEK_DIRECTION_MASK: c_int = 3;

//  ENUM: media_parameter_flags
pub const B_HIDDEN_PARAMETER: c_int = 1;
pub const B_ADVANCED_PARAMETER: c_int = 2;

//  ENUM: @74
pub const B_UNKNOWN_FILE: c_int = 0;
pub const B_AIFF_FILE: c_int = 0 + 1;
pub const B_WAVE_FILE: c_int = 0 + 2;
pub const B_UNIX_FILE: c_int = 0 + 3;

//  ENUM: timecode_type
pub const B_TIMECODE_DEFAULT: c_int = 0;
pub const B_TIMECODE_100: c_int = 0 + 1;
pub const B_TIMECODE_75: c_int = 0 + 2;
pub const B_TIMECODE_30: c_int = 0 + 3;
pub const B_TIMECODE_30_DROP_2: c_int = 0 + 4;
pub const B_TIMECODE_30_DROP_4: c_int = 0 + 5;
pub const B_TIMECODE_25: c_int = 0 + 6;
pub const B_TIMECODE_24: c_int = 0 + 7;
pub const B_TIMECODE_18: c_int = 0 + 8;

// NODEF: _MIDI_CONSTANTS_

pub const B_MIDI_EVENT: c_int = 0x4d494449 /* 'MIDI' */;
//  ENUM: BMidiOp
pub const B_MIDI_NO_OP: c_int = 0;
pub const B_MIDI_REGISTERED: c_int = 0 + 1;
pub const B_MIDI_UNREGISTERED: c_int = 0 + 2;
pub const B_MIDI_CONNECTED: c_int = 0 + 3;
pub const B_MIDI_DISCONNECTED: c_int = 0 + 4;
pub const B_MIDI_CHANGED_NAME: c_int = 0 + 5;
pub const B_MIDI_CHANGED_LATENCY: c_int = 0 + 6;
pub const B_MIDI_CHANGED_PROPERTIES: c_int = 0 + 7;

//  ENUM: @75
pub const B_NO_ADDRESS_RESOLUTION: c_int = 0x0001;
pub const B_UNCONFIGURED_ADDRESS_FAMILIES: c_int = 0x0002;

pub const B_NETWORK_IS_ENCRYPTED: c_int = 0x01;
pub const B_NETWORK_IS_PERSISTENT: c_int = 0x02;
//  ENUM: @76
pub const B_NETWORK_AUTHENTICATION_NONE: c_int = 0;
pub const B_NETWORK_AUTHENTICATION_WEP: c_int = 1;
pub const B_NETWORK_AUTHENTICATION_WPA: c_int = 2;
pub const B_NETWORK_AUTHENTICATION_WPA2: c_int = 3;
pub const B_NETWORK_AUTHENTICATION_EAP: c_int = 4;
//  ENUM: @77
pub const B_NETWORK_CIPHER_NONE: c_int = 0x01;
pub const B_NETWORK_CIPHER_WEP_40: c_int = 0x02;
pub const B_NETWORK_CIPHER_WEP_104: c_int = 0x04;
pub const B_NETWORK_CIPHER_TKIP: c_int = 0x08;
pub const B_NETWORK_CIPHER_CCMP: c_int = 0x10;
pub const B_NETWORK_CIPHER_AES_128_CMAC: c_int = 0x20;
//  ENUM: @78
pub const B_KEY_MODE_IEEE802_1X: c_int = 0x0001;
pub const B_KEY_MODE_PSK: c_int = 0x0002;
pub const B_KEY_MODE_NONE: c_int = 0x0004;
pub const B_KEY_MODE_FT_IEEE802_1X: c_int = 0x0020;
pub const B_KEY_MODE_FT_PSK: c_int = 0x0040;
pub const B_KEY_MODE_IEEE802_1X_SHA256: c_int = 0x0080;
pub const B_KEY_MODE_PSK_SHA256: c_int = 0x0100;
pub const B_KEY_MODE_WPS: c_int = 0x0200;
//  ENUM: @79
pub const B_NETWORK_EAP_ENCAPSULATION_NONE: c_int = 0x0000;
pub const B_NETWORK_EAP_ENCAPSULATION_PEAP: c_int = 0x0001;
pub const B_NETWORK_EAP_ENCAPSULATION_TLS: c_int = 0x0002;

pub const B_NETWORK_DEVICE_LINK_CHANGED: c_int = 0x0010;
pub const B_NETWORK_INTERFACE_ADDED: c_int = 0x0001;
pub const B_NETWORK_INTERFACE_CHANGED: c_int = 0x0003;
pub const B_NETWORK_INTERFACE_REMOVED: c_int = 0x0002;
pub const B_NETWORK_MONITOR: c_int = 0x5f4e544e /* '_NTN' */;
pub const B_NETWORK_WLAN_JOINED: c_int = 0x0100;
pub const B_NETWORK_WLAN_LEFT: c_int = 0x0200;
pub const B_NETWORK_WLAN_MESSAGE_INTEGRITY_FAILED: c_int = 0x0400;
pub const B_NETWORK_WLAN_SCANNED: c_int = 0x0300;
//  ENUM: @80
pub const B_WATCH_NETWORK_INTERFACE_CHANGES: c_int = 0x000f;
pub const B_WATCH_NETWORK_LINK_CHANGES: c_int = 0x00f0;
pub const B_WATCH_NETWORK_WLAN_CHANGES: c_int = 0x0f00;

//  ENUM: info_location
pub const B_USE_ATTRIBUTES: c_int = 0x1;
pub const B_USE_RESOURCES: c_int = 0x2;
pub const B_USE_BOTH_LOCATIONS: c_int = 0x3;
//  ENUM: info_variety
pub const B_DEVELOPMENT_VERSION: c_int = 0;
pub const B_ALPHA_VERSION: c_int = 0 + 1;
pub const B_BETA_VERSION: c_int = 0 + 2;
pub const B_GAMMA_VERSION: c_int = 0 + 3;
pub const B_GOLDEN_MASTER_VERSION: c_int = 0 + 4;
pub const B_FINAL_VERSION: c_int = 0 + 5;
//  ENUM: version_kind
pub const B_APP_VERSION_KIND: c_int = 0;
pub const B_SYSTEM_VERSION_KIND: c_int = 0 + 1;

pub const B_DISK_DEVICE_MAX_PARAMETER_SIZE: c_int = (32 * 1024);
pub const B_DISK_DEVICE_NAME_LENGTH: c_int = B_FILE_NAME_LENGTH;
pub const B_DISK_DEVICE_TYPE_LENGTH: c_int = B_FILE_NAME_LENGTH;
pub const B_DISK_SYSTEM_NAME_LENGTH: c_int = B_PATH_NAME_LENGTH;
//  ENUM: @84
pub const B_PARTITION_IS_DEVICE: c_int = 0x01;
pub const B_PARTITION_FILE_SYSTEM: c_int = 0x02;
pub const B_PARTITION_PARTITIONING_SYSTEM: c_int = 0x04;
pub const B_PARTITION_READ_ONLY: c_int = 0x08;
pub const B_PARTITION_MOUNTED: c_int = 0x10;
pub const B_PARTITION_BUSY: c_int = 0x20;
//  ENUM: @85
pub const B_PARTITION_VALID: c_int = 0;
pub const B_PARTITION_CORRUPT: c_int = 0 + 1;
pub const B_PARTITION_UNRECOGNIZED: c_int = 0 + 2;
pub const B_PARTITION_UNINITIALIZED: c_int = 0 + 3;
//  ENUM: @86
pub const B_PARTITION_CHANGED_OFFSET: c_int = 0x000001;
pub const B_PARTITION_CHANGED_SIZE: c_int = 0x000002;
pub const B_PARTITION_CHANGED_CONTENT_SIZE: c_int = 0x000004;
pub const B_PARTITION_CHANGED_BLOCK_SIZE: c_int = 0x000008;
pub const B_PARTITION_CHANGED_STATUS: c_int = 0x000010;
pub const B_PARTITION_CHANGED_FLAGS: c_int = 0x000020;
pub const B_PARTITION_CHANGED_VOLUME: c_int = 0x000040;
pub const B_PARTITION_CHANGED_NAME: c_int = 0x000080;
pub const B_PARTITION_CHANGED_CONTENT_NAME: c_int = 0x000100;
pub const B_PARTITION_CHANGED_TYPE: c_int = 0x000200;
pub const B_PARTITION_CHANGED_CONTENT_TYPE: c_int = 0x000400;
pub const B_PARTITION_CHANGED_PARAMETERS: c_int = 0x000800;
pub const B_PARTITION_CHANGED_CONTENT_PARAMETERS: c_int = 0x001000;
pub const B_PARTITION_CHANGED_CHILDREN: c_int = 0x002000;
pub const B_PARTITION_CHANGED_DESCENDANTS: c_int = 0x004000;
pub const B_PARTITION_CHANGED_DEFRAGMENTATION: c_int = 0x008000;
pub const B_PARTITION_CHANGED_CHECK: c_int = 0x010000;
pub const B_PARTITION_CHANGED_REPAIR: c_int = 0x020000;
pub const B_PARTITION_CHANGED_INITIALIZATION: c_int = 0x040000;
//  ENUM: @87
pub const B_DISK_DEVICE_REMOVABLE: c_int = 0x01;
pub const B_DISK_DEVICE_HAS_MEDIA: c_int = 0x02;
pub const B_DISK_DEVICE_READ_ONLY: c_int = 0x04;
pub const B_DISK_DEVICE_WRITE_ONCE: c_int = 0x08;
pub const B_DISK_DEVICE_IS_FILE: c_int = 0x10;
//  ENUM: @88
pub const B_DISK_SYSTEM_IS_FILE_SYSTEM: c_int = 0x000001;
pub const B_DISK_SYSTEM_SUPPORTS_CHECKING: c_int = 0x000002;
pub const B_DISK_SYSTEM_SUPPORTS_REPAIRING: c_int = 0x000004;
pub const B_DISK_SYSTEM_SUPPORTS_RESIZING: c_int = 0x000008;
pub const B_DISK_SYSTEM_SUPPORTS_MOVING: c_int = 0x000010;
pub const B_DISK_SYSTEM_SUPPORTS_SETTING_CONTENT_NAME: c_int = 0x000020;
pub const B_DISK_SYSTEM_SUPPORTS_SETTING_CONTENT_PARAMETERS: c_int = 0x000040;
pub const B_DISK_SYSTEM_SUPPORTS_INITIALIZING: c_int = 0x000080;
pub const B_DISK_SYSTEM_SUPPORTS_CONTENT_NAME: c_int = 0x000100;
pub const B_DISK_SYSTEM_SUPPORTS_DEFRAGMENTING: c_int = 0x001000;
pub const B_DISK_SYSTEM_SUPPORTS_DEFRAGMENTING_WHILE_MOUNTED: c_int = 0x002000;
pub const B_DISK_SYSTEM_SUPPORTS_CHECKING_WHILE_MOUNTED: c_int = 0x004000;
pub const B_DISK_SYSTEM_SUPPORTS_REPAIRING_WHILE_MOUNTED: c_int = 0x008000;
pub const B_DISK_SYSTEM_SUPPORTS_RESIZING_WHILE_MOUNTED: c_int = 0x010000;
pub const B_DISK_SYSTEM_SUPPORTS_MOVING_WHILE_MOUNTED: c_int = 0x020000;
pub const B_DISK_SYSTEM_SUPPORTS_SETTING_CONTENT_NAME_WHILE_MOUNTED: c_int = 0x040000;
pub const B_DISK_SYSTEM_SUPPORTS_SETTING_CONTENT_PARAMETERS_WHILE_MOUNTED: c_int = 0x080000;
pub const B_DISK_SYSTEM_SUPPORTS_WRITING: c_int = 0x100000;
pub const B_DISK_SYSTEM_SUPPORTS_RESIZING_CHILD: c_int = 0x001000;
pub const B_DISK_SYSTEM_SUPPORTS_MOVING_CHILD: c_int = 0x002000;
pub const B_DISK_SYSTEM_SUPPORTS_SETTING_NAME: c_int = 0x004000;
pub const B_DISK_SYSTEM_SUPPORTS_SETTING_TYPE: c_int = 0x008000;
pub const B_DISK_SYSTEM_SUPPORTS_SETTING_PARAMETERS: c_int = 0x010000;
pub const B_DISK_SYSTEM_SUPPORTS_CREATING_CHILD: c_int = 0x020000;
pub const B_DISK_SYSTEM_SUPPORTS_DELETING_CHILD: c_int = 0x040000;
pub const B_DISK_SYSTEM_SUPPORTS_NAME: c_int = 0x080000;
//  ENUM: @89
pub const B_DISK_DEVICE_JOB_BAD_TYPE: c_int = 0;
pub const B_DISK_DEVICE_JOB_DEFRAGMENT: c_int = 0 + 1;
pub const B_DISK_DEVICE_JOB_REPAIR: c_int = 0 + 2;
pub const B_DISK_DEVICE_JOB_RESIZE: c_int = 0 + 3;
pub const B_DISK_DEVICE_JOB_MOVE: c_int = 0 + 4;
pub const B_DISK_DEVICE_JOB_SET_NAME: c_int = 0 + 5;
pub const B_DISK_DEVICE_JOB_SET_CONTENT_NAME: c_int = 0 + 6;
pub const B_DISK_DEVICE_JOB_SET_TYPE: c_int = 0 + 7;
pub const B_DISK_DEVICE_JOB_SET_PARAMETERS: c_int = 0 + 8;
pub const B_DISK_DEVICE_JOB_SET_CONTENT_PARAMETERS: c_int = 0 + 9;
pub const B_DISK_DEVICE_JOB_INITIALIZE: c_int = 0 + 10;
pub const B_DISK_DEVICE_JOB_UNINITIALIZE: c_int = 0 + 11;
pub const B_DISK_DEVICE_JOB_CREATE: c_int = 0 + 12;
pub const B_DISK_DEVICE_JOB_DELETE: c_int = 0 + 13;
pub const B_DISK_DEVICE_JOB_SCAN: c_int = 0 + 14;
//  ENUM: @90
pub const B_DISK_DEVICE_JOB_UNINITIALIZED: c_int = 0;
pub const B_DISK_DEVICE_JOB_SCHEDULED: c_int = 0 + 1;
pub const B_DISK_DEVICE_JOB_IN_PROGRESS: c_int = 0 + 2;
pub const B_DISK_DEVICE_JOB_SUCCEEDED: c_int = 0 + 3;
pub const B_DISK_DEVICE_JOB_FAILED: c_int = 0 + 4;
pub const B_DISK_DEVICE_JOB_CANCELED: c_int = 0 + 5;
//  ENUM: @91
pub const B_DISK_DEVICE_JOB_CAN_CANCEL: c_int = 0x01;
pub const B_DISK_DEVICE_JOB_STOP_ON_CANCEL: c_int = 0x02;
pub const B_DISK_DEVICE_JOB_REVERSE_ON_CANCEL: c_int = 0x04;
pub const B_DISK_DEVICE_JOB_CAN_PAUSE: c_int = 0x08;
//  ENUM: B_PARAMETER_EDITOR_TYPE
pub const B_CREATE_PARAMETER_EDITOR: c_int = 0x01;
pub const B_INITIALIZE_PARAMETER_EDITOR: c_int = 0x04;
pub const B_DELETE_PARAMETER_EDITOR: c_int = 0x08;
pub const B_PROPERTIES_PARAMETER_EDITOR: c_int = 0x10;

//  ENUM: file_panel_button
pub const B_CANCEL_BUTTON: c_int = 0;
pub const B_DEFAULT_BUTTON: c_int = 0 + 1;
//  ENUM: file_panel_mode
pub const B_OPEN_PANEL: c_int = 0;
pub const B_SAVE_PANEL: c_int = 0 + 1;

//  ENUM: @92
pub const B_FIND_PATH_CREATE_DIRECTORY: c_int = 0x0001;
pub const B_FIND_PATH_CREATE_PARENT_DIRECTORY: c_int = 0x0002;
pub const B_FIND_PATH_EXISTING_ONLY: c_int = 0x0004;
pub const B_FIND_PATHS_SYSTEM_ONLY: c_int = 0x0010;
pub const B_FIND_PATHS_USER_ONLY: c_int = 0x0020;
//  ENUM: directory_which
pub const B_DESKTOP_DIRECTORY: c_int = 0;
pub const B_TRASH_DIRECTORY: c_int = 0 + 1;
pub const B_SYSTEM_DIRECTORY: c_int = 1000;
pub const B_SYSTEM_ADDONS_DIRECTORY: c_int = 1002;
pub const B_SYSTEM_BOOT_DIRECTORY: c_int = 1002 + 1;
pub const B_SYSTEM_FONTS_DIRECTORY: c_int = 1002 + 2;
pub const B_SYSTEM_LIB_DIRECTORY: c_int = 1002 + 3;
pub const B_SYSTEM_SERVERS_DIRECTORY: c_int = 1002 + 4;
pub const B_SYSTEM_APPS_DIRECTORY: c_int = 1002 + 5;
pub const B_SYSTEM_BIN_DIRECTORY: c_int = 1002 + 6;
pub const B_SYSTEM_DOCUMENTATION_DIRECTORY: c_int = 1010;
pub const B_SYSTEM_PREFERENCES_DIRECTORY: c_int = 1010 + 1;
pub const B_SYSTEM_TRANSLATORS_DIRECTORY: c_int = 1010 + 2;
pub const B_SYSTEM_MEDIA_NODES_DIRECTORY: c_int = 1010 + 3;
pub const B_SYSTEM_SOUNDS_DIRECTORY: c_int = 1010 + 4;
pub const B_SYSTEM_DATA_DIRECTORY: c_int = 1010 + 5;
pub const B_SYSTEM_DEVELOP_DIRECTORY: c_int = 1010 + 6;
pub const B_SYSTEM_PACKAGES_DIRECTORY: c_int = 1010 + 7;
pub const B_SYSTEM_HEADERS_DIRECTORY: c_int = 1010 + 8;
pub const B_SYSTEM_ETC_DIRECTORY: c_int = 2008;
pub const B_SYSTEM_SETTINGS_DIRECTORY: c_int = 2010;
pub const B_SYSTEM_LOG_DIRECTORY: c_int = 2012;
pub const B_SYSTEM_SPOOL_DIRECTORY: c_int = 2012 + 1;
pub const B_SYSTEM_TEMP_DIRECTORY: c_int = 2012 + 2;
pub const B_SYSTEM_VAR_DIRECTORY: c_int = 2012 + 3;
pub const B_SYSTEM_CACHE_DIRECTORY: c_int = 2020;
pub const B_SYSTEM_NONPACKAGED_DIRECTORY: c_int = 2023;
pub const B_SYSTEM_NONPACKAGED_ADDONS_DIRECTORY: c_int = 2023 + 1;
pub const B_SYSTEM_NONPACKAGED_TRANSLATORS_DIRECTORY: c_int = 2023 + 2;
pub const B_SYSTEM_NONPACKAGED_MEDIA_NODES_DIRECTORY: c_int = 2023 + 3;
pub const B_SYSTEM_NONPACKAGED_BIN_DIRECTORY: c_int = 2023 + 4;
pub const B_SYSTEM_NONPACKAGED_DATA_DIRECTORY: c_int = 2023 + 5;
pub const B_SYSTEM_NONPACKAGED_FONTS_DIRECTORY: c_int = 2023 + 6;
pub const B_SYSTEM_NONPACKAGED_SOUNDS_DIRECTORY: c_int = 2023 + 7;
pub const B_SYSTEM_NONPACKAGED_DOCUMENTATION_DIRECTORY: c_int = 2023 + 8;
pub const B_SYSTEM_NONPACKAGED_LIB_DIRECTORY: c_int = 2023 + 9;
pub const B_SYSTEM_NONPACKAGED_HEADERS_DIRECTORY: c_int = 2023 + 10;
pub const B_SYSTEM_NONPACKAGED_DEVELOP_DIRECTORY: c_int = 2023 + 11;
pub const B_USER_DIRECTORY: c_int = 3000;
pub const B_USER_CONFIG_DIRECTORY: c_int = 3000 + 1;
pub const B_USER_ADDONS_DIRECTORY: c_int = 3000 + 2;
pub const B_USER_BOOT_DIRECTORY: c_int = 3000 + 3;
pub const B_USER_FONTS_DIRECTORY: c_int = 3000 + 4;
pub const B_USER_LIB_DIRECTORY: c_int = 3000 + 5;
pub const B_USER_SETTINGS_DIRECTORY: c_int = 3000 + 6;
pub const B_USER_DESKBAR_DIRECTORY: c_int = 3000 + 7;
pub const B_USER_PRINTERS_DIRECTORY: c_int = 3000 + 8;
pub const B_USER_TRANSLATORS_DIRECTORY: c_int = 3000 + 9;
pub const B_USER_MEDIA_NODES_DIRECTORY: c_int = 3000 + 10;
pub const B_USER_SOUNDS_DIRECTORY: c_int = 3000 + 11;
pub const B_USER_DATA_DIRECTORY: c_int = 3000 + 12;
pub const B_USER_CACHE_DIRECTORY: c_int = 3000 + 13;
pub const B_USER_PACKAGES_DIRECTORY: c_int = 3000 + 14;
pub const B_USER_HEADERS_DIRECTORY: c_int = 3000 + 15;
pub const B_USER_NONPACKAGED_DIRECTORY: c_int = 3000 + 16;
pub const B_USER_NONPACKAGED_ADDONS_DIRECTORY: c_int = 3000 + 17;
pub const B_USER_NONPACKAGED_TRANSLATORS_DIRECTORY: c_int = 3000 + 18;
pub const B_USER_NONPACKAGED_MEDIA_NODES_DIRECTORY: c_int = 3000 + 19;
pub const B_USER_NONPACKAGED_BIN_DIRECTORY: c_int = 3000 + 20;
pub const B_USER_NONPACKAGED_DATA_DIRECTORY: c_int = 3000 + 21;
pub const B_USER_NONPACKAGED_FONTS_DIRECTORY: c_int = 3000 + 22;
pub const B_USER_NONPACKAGED_SOUNDS_DIRECTORY: c_int = 3000 + 23;
pub const B_USER_NONPACKAGED_DOCUMENTATION_DIRECTORY: c_int = 3000 + 24;
pub const B_USER_NONPACKAGED_LIB_DIRECTORY: c_int = 3000 + 25;
pub const B_USER_NONPACKAGED_HEADERS_DIRECTORY: c_int = 3000 + 26;
pub const B_USER_NONPACKAGED_DEVELOP_DIRECTORY: c_int = 3000 + 27;
pub const B_USER_DEVELOP_DIRECTORY: c_int = 3000 + 28;
pub const B_USER_DOCUMENTATION_DIRECTORY: c_int = 3000 + 29;
pub const B_USER_SERVERS_DIRECTORY: c_int = 3000 + 30;
pub const B_USER_APPS_DIRECTORY: c_int = 3000 + 31;
pub const B_USER_BIN_DIRECTORY: c_int = 3000 + 32;
pub const B_USER_PREFERENCES_DIRECTORY: c_int = 3000 + 33;
pub const B_USER_ETC_DIRECTORY: c_int = 3000 + 34;
pub const B_USER_LOG_DIRECTORY: c_int = 3000 + 35;
pub const B_USER_SPOOL_DIRECTORY: c_int = 3000 + 36;
pub const B_USER_VAR_DIRECTORY: c_int = 3000 + 37;
pub const B_APPS_DIRECTORY: c_int = 4000;
pub const B_PREFERENCES_DIRECTORY: c_int = 4000 + 1;
pub const B_UTILITIES_DIRECTORY: c_int = 4000 + 2;
pub const B_PACKAGE_LINKS_DIRECTORY: c_int = 4000 + 3;
pub const B_BEOS_DIRECTORY: c_int = 1000;
pub const B_BEOS_SYSTEM_DIRECTORY: c_int = 1000 + 1;
pub const B_BEOS_ADDONS_DIRECTORY: c_int = 1000 + 2;
pub const B_BEOS_BOOT_DIRECTORY: c_int = 1000 + 3;
pub const B_BEOS_FONTS_DIRECTORY: c_int = 1000 + 4;
pub const B_BEOS_LIB_DIRECTORY: c_int = 1000 + 5;
pub const B_BEOS_SERVERS_DIRECTORY: c_int = 1000 + 6;
pub const B_BEOS_APPS_DIRECTORY: c_int = 1000 + 7;
pub const B_BEOS_BIN_DIRECTORY: c_int = 1000 + 8;
pub const B_BEOS_ETC_DIRECTORY: c_int = 1000 + 9;
pub const B_BEOS_DOCUMENTATION_DIRECTORY: c_int = 1000 + 10;
pub const B_BEOS_PREFERENCES_DIRECTORY: c_int = 1000 + 11;
pub const B_BEOS_TRANSLATORS_DIRECTORY: c_int = 1000 + 12;
pub const B_BEOS_MEDIA_NODES_DIRECTORY: c_int = 1000 + 13;
pub const B_BEOS_SOUNDS_DIRECTORY: c_int = 1000 + 14;
//  ENUM: path_base_directory
pub const B_FIND_PATH_INSTALLATION_LOCATION_DIRECTORY: c_int = 0;
pub const B_FIND_PATH_ADD_ONS_DIRECTORY: c_int = 0 + 1;
pub const B_FIND_PATH_APPS_DIRECTORY: c_int = 0 + 2;
pub const B_FIND_PATH_BIN_DIRECTORY: c_int = 0 + 3;
pub const B_FIND_PATH_BOOT_DIRECTORY: c_int = 0 + 4;
pub const B_FIND_PATH_CACHE_DIRECTORY: c_int = 0 + 5;
pub const B_FIND_PATH_DATA_DIRECTORY: c_int = 0 + 6;
pub const B_FIND_PATH_DEVELOP_DIRECTORY: c_int = 0 + 7;
pub const B_FIND_PATH_DEVELOP_LIB_DIRECTORY: c_int = 0 + 8;
pub const B_FIND_PATH_DOCUMENTATION_DIRECTORY: c_int = 0 + 9;
pub const B_FIND_PATH_ETC_DIRECTORY: c_int = 0 + 10;
pub const B_FIND_PATH_FONTS_DIRECTORY: c_int = 0 + 11;
pub const B_FIND_PATH_HEADERS_DIRECTORY: c_int = 0 + 12;
pub const B_FIND_PATH_LIB_DIRECTORY: c_int = 0 + 13;
pub const B_FIND_PATH_LOG_DIRECTORY: c_int = 0 + 14;
pub const B_FIND_PATH_MEDIA_NODES_DIRECTORY: c_int = 0 + 15;
pub const B_FIND_PATH_PACKAGES_DIRECTORY: c_int = 0 + 16;
pub const B_FIND_PATH_PREFERENCES_DIRECTORY: c_int = 0 + 17;
pub const B_FIND_PATH_SERVERS_DIRECTORY: c_int = 0 + 18;
pub const B_FIND_PATH_SETTINGS_DIRECTORY: c_int = 0 + 19;
pub const B_FIND_PATH_SOUNDS_DIRECTORY: c_int = 0 + 20;
pub const B_FIND_PATH_SPOOL_DIRECTORY: c_int = 0 + 21;
pub const B_FIND_PATH_TRANSLATORS_DIRECTORY: c_int = 0 + 22;
pub const B_FIND_PATH_VAR_DIRECTORY: c_int = 0 + 23;
pub const B_FIND_PATH_IMAGE_PATH: c_int = 1000;
pub const B_FIND_PATH_PACKAGE_PATH: c_int = 1000 + 1;

//  ENUM: @93
pub const B_UPDATE_MIME_INFO_NO_FORCE: c_int = 0;
pub const B_UPDATE_MIME_INFO_FORCE_KEEP_TYPE: c_int = 1;
pub const B_UPDATE_MIME_INFO_FORCE_UPDATE_ALL: c_int = 2;
//  ENUM: icon_size
pub const B_LARGE_ICON: c_int = 32;
pub const B_MINI_ICON: c_int = 16;

//  ENUM: @94
pub const B_META_MIME_CHANGED: c_int = 0x4d4d4348 /* 'MMCH' */;
//  ENUM: @95
pub const B_ICON_CHANGED: c_int = 0x00000001;
pub const B_PREFERRED_APP_CHANGED: c_int = 0x00000002;
pub const B_ATTR_INFO_CHANGED: c_int = 0x00000004;
pub const B_FILE_EXTENSIONS_CHANGED: c_int = 0x00000008;
pub const B_SHORT_DESCRIPTION_CHANGED: c_int = 0x00000010;
pub const B_LONG_DESCRIPTION_CHANGED: c_int = 0x00000020;
pub const B_ICON_FOR_TYPE_CHANGED: c_int = 0x00000040;
pub const B_APP_HINT_CHANGED: c_int = 0x00000080;
pub const B_MIME_TYPE_CREATED: c_int = 0x00000100;
pub const B_MIME_TYPE_DELETED: c_int = 0x00000200;
pub const B_SNIFFER_RULE_CHANGED: c_int = 0x00000400;
pub const B_SUPPORTED_TYPES_CHANGED: c_int = 0x00000800;
pub const B_EVERYTHING_CHANGED: c_int = (int)0xFFFFFFFF;
//  ENUM: @96
pub const B_META_MIME_MODIFIED: c_int = 0x4d4d4d44 /* 'MMMD' */;
pub const B_META_MIME_DELETED: c_int = 0x4d4d444c /* 'MMDL' */;
//  ENUM: app_verb
pub const B_OPEN: c_int = 0;

pub const B_ATTR_CHANGED: c_int = 5;
pub const B_ATTR_CREATED: c_int = 1;
pub const B_ATTR_REMOVED: c_int = 2;
pub const B_DEVICE_MOUNTED: c_int = 6;
pub const B_DEVICE_UNMOUNTED: c_int = 7;
pub const B_ENTRY_CREATED: c_int = 1;
pub const B_ENTRY_MOVED: c_int = 3;
pub const B_ENTRY_REMOVED: c_int = 2;
pub const B_STAT_CHANGED: c_int = 4;
//  ENUM: @97
pub const B_STOP_WATCHING: c_int = 0x0000;
pub const B_WATCH_NAME: c_int = 0x0001;
pub const B_WATCH_STAT: c_int = 0x0002;
pub const B_WATCH_ATTR: c_int = 0x0004;
pub const B_WATCH_DIRECTORY: c_int = 0x0008;
pub const B_WATCH_ALL: c_int = 0x000f;
pub const B_WATCH_MOUNT: c_int = 0x0010;
pub const B_WATCH_INTERIM_STAT: c_int = 0x0020;
pub const B_WATCH_CHILDREN: c_int = 0x0040;
//  ENUM: @98
pub const B_STAT_MODE: c_int = 0x0001;
pub const B_STAT_UID: c_int = 0x0002;
pub const B_STAT_GID: c_int = 0x0004;
pub const B_STAT_SIZE: c_int = 0x0008;
pub const B_STAT_ACCESS_TIME: c_int = 0x0010;
pub const B_STAT_MODIFICATION_TIME: c_int = 0x0020;
pub const B_STAT_CREATION_TIME: c_int = 0x0040;
pub const B_STAT_CHANGE_TIME: c_int = 0x0080;
pub const B_STAT_INTERIM_UPDATE: c_int = 0x1000;

//  ENUM: query_op
pub const B_INVALID_OP: c_int = 0;
pub const B_EQ: c_int = 0 + 1;
pub const B_GT: c_int = 0 + 2;
pub const B_GE: c_int = 0 + 3;
pub const B_LT: c_int = 0 + 4;
pub const B_LE: c_int = 0 + 5;
pub const B_NE: c_int = 0 + 6;
pub const B_CONTAINS: c_int = 0 + 7;
pub const B_BEGINS_WITH: c_int = 0 + 8;
pub const B_ENDS_WITH: c_int = 0 + 9;
pub const B_AND: c_int = 0x101;
pub const B_OR: c_int = 0x101 + 1;
pub const B_NOT: c_int = 0x101 + 2;
pub const _B_RESERVED_OP_: c_int = 0x100000;

pub const B_ATTR_NAME_LENGTH: c_int = (B_FILE_NAME_LENGTH - 1);
pub const B_CREATE_FILE: c_int = O_CREAT		/* create the file */;
pub const B_DEV_NAME_LENGTH: c_int = 128;
pub const B_ERASE_FILE: c_int = O_TRUNC		/* erase the file's data */;
pub const B_FAIL_IF_EXISTS: c_int = O_EXCL		/* exclusive create */;
pub const B_FILE_NAME_LENGTH: c_int = NAME_MAX;
pub const B_MAX_SYMLINKS: c_int = SYMLOOP_MAX;
pub const B_MIME_TYPE_LENGTH: c_int = (B_ATTR_NAME_LENGTH - 15);
pub const B_OPEN_AT_END: c_int = O_APPEND	/* point to the end of the data */;
pub const B_PATH_NAME_LENGTH: c_int = MAXPATHLEN;
pub const B_READ_ONLY: c_int = O_RDONLY	/* read only */;
pub const B_READ_WRITE: c_int = O_RDWR		/* read and write */;
pub const B_WRITE_ONLY: c_int = O_WRONLY	/* write only */;
//  ENUM: node_flavor
pub const B_FILE_NODE: c_int = 0x01;
pub const B_SYMLINK_NODE: c_int = 0x02;
pub const B_DIRECTORY_NODE: c_int = 0x04;
pub const B_ANY_NODE: c_int = 0x07;

//  ENUM: @100
pub const B_OBJECT_CACHE: c_int = 0;
pub const B_MALLOC_CACHE: c_int = 1;

pub const B_BENDIAN_TO_HOST_DOUBLE: c_int = __swap_double(arg);
pub const B_BENDIAN_TO_HOST_FLOAT: c_int = __swap_float(arg);
pub const B_BENDIAN_TO_HOST_INT16: c_int = __swap_int16(arg);
pub const B_BENDIAN_TO_HOST_INT32: c_int = __swap_int32(arg);
pub const B_BENDIAN_TO_HOST_INT64: c_int = __swap_int64(arg);
pub const B_HOST_IS_BENDIAN: c_int = 0;
pub const B_HOST_IS_LENDIAN: c_int = 1;
pub const B_HOST_TO_BENDIAN_DOUBLE: c_int = __swap_double(arg);
pub const B_HOST_TO_BENDIAN_FLOAT: c_int = __swap_float(arg);
pub const B_HOST_TO_BENDIAN_INT16: c_int = __swap_int16(arg);
pub const B_HOST_TO_BENDIAN_INT32: c_int = __swap_int32(arg);
pub const B_HOST_TO_BENDIAN_INT64: c_int = __swap_int64(arg);
pub const B_HOST_TO_LENDIAN_DOUBLE: c_int = (double)(arg);
pub const B_HOST_TO_LENDIAN_FLOAT: c_int = (float)(arg);
pub const B_HOST_TO_LENDIAN_INT16: c_int = (uint16)(arg);
pub const B_HOST_TO_LENDIAN_INT32: c_int = (uint32)(arg);
pub const B_HOST_TO_LENDIAN_INT64: c_int = (uint64)(arg);
pub const B_LENDIAN_TO_HOST_DOUBLE: c_int = (double)(arg);
pub const B_LENDIAN_TO_HOST_FLOAT: c_int = (float)(arg);
pub const B_LENDIAN_TO_HOST_INT16: c_int = (uint16)(arg);
pub const B_LENDIAN_TO_HOST_INT32: c_int = (uint32)(arg);
pub const B_LENDIAN_TO_HOST_INT64: c_int = (uint64)(arg);
pub const B_SWAP_DOUBLE: c_int = __swap_double(arg);
pub const B_SWAP_FLOAT: c_int = __swap_float(arg);
pub const B_SWAP_INT16: c_int = __swap_int16(arg);
pub const B_SWAP_INT32: c_int = __swap_int32(arg);
pub const B_SWAP_INT64: c_int = __swap_int64(arg);
//  ENUM: swap_action
pub const B_SWAP_HOST_TO_LENDIAN: c_int = 0;
pub const B_SWAP_HOST_TO_BENDIAN: c_int = 0 + 1;
pub const B_SWAP_LENDIAN_TO_HOST: c_int = 0 + 2;
pub const B_SWAP_BENDIAN_TO_HOST: c_int = 0 + 3;
pub const B_SWAP_ALWAYS: c_int = 0 + 4;

pub const cast_as: c_int = (dynamic_cast<class*>(object));
pub const class_name: f32 = ((typeid(*(object))).name());
pub const is_instance_of: c_int = (typeid(*(object)) == typeid(class));
pub const is_kind_of: c_int = (dynamic_cast<class*>(object) != NULL);

pub const ASSERT: c_int = (void)0;
pub const ASSERT_WITH_MESSAGE: c_int = (void)0;
// NODEF: DEBUG_ONLY
pub const DEBUGGER: c_int = (void)0;
pub const IS_DEBUG_ENABLED: c_int = (void)0;
pub const PRINT: c_int = (void)0;
pub const PRINT_OBJECT: c_int = (void)0;
pub const SERIAL_PRINT: c_int = (void)0;
pub const SERIAL_TRACE: c_int = (void)0;
pub const SET_DEBUG_ENABLED: c_int = (void)0;
pub const STATIC_ASSERT: &str = static_assert(x, "static assert failed!");
pub const TRACE: c_int = (void)0;
pub const TRESPASS: c_int = (void)0;

pub const B_ALREADY_RUNNING: c_int = (B_APP_ERROR_BASE + 4);
pub const B_AMBIGUOUS_APP_LAUNCH: c_int = (B_APP_ERROR_BASE + 6);
pub const B_APP_ERROR_BASE: c_int = (B_GENERAL_ERROR_BASE + 0x2000);
pub const B_BAD_ADDRESS: c_int = (B_OS_ERROR_BASE + 0x301);
pub const B_BAD_DATA: c_int = (B_GENERAL_ERROR_BASE + 16);
pub const B_BAD_HANDLER: c_int = (B_APP_ERROR_BASE + 3);
pub const B_BAD_IMAGE_ID: c_int = (B_OS_ERROR_BASE + 0x300);
pub const B_BAD_INDEX: c_int = (B_GENERAL_ERROR_BASE + 3);
pub const B_BAD_MIME_SNIFFER_RULE: c_int = (B_APP_ERROR_BASE + 15);
pub const B_BAD_PORT_ID: c_int = (B_OS_ERROR_BASE + 0x200);
pub const B_BAD_REPLY: c_int = (B_APP_ERROR_BASE + 0);
pub const B_BAD_SCRIPT_SYNTAX: c_int = (B_APP_ERROR_BASE + 8);
pub const B_BAD_SEM_ID: c_int = (B_OS_ERROR_BASE + 0);
pub const B_BAD_SUBSCRIBER: c_int = (B_MEDIA_ERROR_BASE + 4);
pub const B_BAD_TEAM_ID: c_int = (B_OS_ERROR_BASE + 0x103);
pub const B_BAD_THREAD_ID: c_int = (B_OS_ERROR_BASE + 0x100);
pub const B_BAD_THREAD_STATE: c_int = (B_OS_ERROR_BASE + 0x102);
pub const B_BAD_TYPE: c_int = (B_GENERAL_ERROR_BASE + 4);
pub const B_BAD_VALUE: c_int = (B_GENERAL_ERROR_BASE + 5);
pub const B_BUFFER_NOT_AVAILABLE: c_int = (B_MEDIA_ERROR_BASE + 6);
pub const B_BUFFER_OVERFLOW: c_int = B_FROM_POSIX_ERROR(EOVERFLOW);
pub const B_BUSTED_PIPE: c_int = (B_STORAGE_ERROR_BASE + 13);
pub const B_BUSY: c_int = (B_GENERAL_ERROR_BASE + 14);
pub const B_CANCELED: c_int = (B_GENERAL_ERROR_BASE + 12);
pub const B_CROSS_DEVICE_LINK: c_int = (B_STORAGE_ERROR_BASE + 11);
pub const B_DEBUGGER_ALREADY_INSTALLED: c_int = (B_OS_ERROR_BASE + 0x400);
pub const B_DEV_BAD_DRIVE_NUM: c_int = (B_DEVICE_ERROR_BASE + 2);
pub const B_DEV_BAD_PID: c_int = (B_DEVICE_ERROR_BASE + 22);
pub const B_DEV_CONFIGURATION_ERROR: c_int = (B_DEVICE_ERROR_BASE + 16);
pub const B_DEV_CRC_ERROR: c_int = (B_DEVICE_ERROR_BASE + 20);
pub const B_DEV_DATA_OVERRUN: c_int = (B_DEVICE_ERROR_BASE + 24);
pub const B_DEV_DATA_UNDERRUN: c_int = (B_DEVICE_ERROR_BASE + 25);
pub const B_DEV_DISABLED_BY_USER: c_int = (B_DEVICE_ERROR_BASE + 17);
pub const B_DEV_DOOR_OPEN: c_int = (B_DEVICE_ERROR_BASE + 18);
pub const B_DEV_FIFO_OVERRUN: c_int = (B_DEVICE_ERROR_BASE + 26);
pub const B_DEV_FIFO_UNDERRUN: c_int = (B_DEVICE_ERROR_BASE + 27);
pub const B_DEV_FORMAT_ERROR: c_int = (B_DEVICE_ERROR_BASE + 5);
pub const B_DEV_ID_ERROR: c_int = (B_DEVICE_ERROR_BASE + 9);
pub const B_DEV_INVALID_IOCTL: c_int = (B_DEVICE_ERROR_BASE + 0);
pub const B_DEV_INVALID_PIPE: c_int = (B_DEVICE_ERROR_BASE + 19);
pub const B_DEV_MEDIA_CHANGE_REQUESTED: c_int = (B_DEVICE_ERROR_BASE + 14);
pub const B_DEV_MEDIA_CHANGED: c_int = (B_DEVICE_ERROR_BASE + 13);
pub const B_DEV_MULTIPLE_ERRORS: c_int = (B_DEVICE_ERROR_BASE + 29);
pub const B_DEV_NO_MEDIA: c_int = (B_DEVICE_ERROR_BASE + 3);
pub const B_DEV_NO_MEMORY: c_int = (B_DEVICE_ERROR_BASE + 1);
pub const B_DEV_NOT_READY: c_int = (B_DEVICE_ERROR_BASE + 12);
pub const B_DEV_PENDING: c_int = (B_DEVICE_ERROR_BASE + 28);
pub const B_DEV_READ_ERROR: c_int = (B_DEVICE_ERROR_BASE + 10);
pub const B_DEV_RECALIBRATE_ERROR: c_int = (B_DEVICE_ERROR_BASE + 7);
pub const B_DEV_RESOURCE_CONFLICT: c_int = (B_DEVICE_ERROR_BASE + 15);
pub const B_DEV_SEEK_ERROR: c_int = (B_DEVICE_ERROR_BASE + 8);
pub const B_DEV_STALLED: c_int = (B_DEVICE_ERROR_BASE + 21);
pub const B_DEV_TIMEOUT: c_int = (B_DEVICE_ERROR_BASE + 6);
pub const B_DEV_TOO_LATE: c_int = (B_DEVICE_ERROR_BASE + 30);
pub const B_DEV_UNEXPECTED_PID: c_int = (B_DEVICE_ERROR_BASE + 23);
pub const B_DEV_UNREADABLE: c_int = (B_DEVICE_ERROR_BASE + 4);
pub const B_DEV_WRITE_ERROR: c_int = (B_DEVICE_ERROR_BASE + 11);
pub const B_DEVICE_ERROR_BASE: c_int = (B_GENERAL_ERROR_BASE + 0xa000);
pub const B_DEVICE_FULL: c_int = (B_STORAGE_ERROR_BASE + 7);
pub const B_DEVICE_NOT_FOUND: c_int = B_FROM_POSIX_ERROR(ENODEV);
pub const B_DIRECTORY_NOT_EMPTY: c_int = (B_STORAGE_ERROR_BASE + 6);
pub const B_DONT_DO_THAT: c_int = (B_GENERAL_ERROR_BASE + 17);
pub const B_DUPLICATE_REPLY: c_int = (B_APP_ERROR_BASE + 1);
pub const B_ENTRY_NOT_FOUND: c_int = (B_STORAGE_ERROR_BASE + 3);
pub const B_ERROR: c_int = (-1);
pub const B_ERRORS_END: c_int = (B_GENERAL_ERROR_BASE + 0xffff);
pub const B_FILE_ERROR: c_int = (B_STORAGE_ERROR_BASE + 0);
pub const B_FILE_EXISTS: c_int = (B_STORAGE_ERROR_BASE + 2);
pub const B_FILE_TOO_LARGE: c_int = B_FROM_POSIX_ERROR(EFBIG);
pub const B_FROM_POSIX_ERROR: c_int = (error);
pub const B_GENERAL_ERROR_BASE: c_int = INT_MIN;
pub const B_ILLEGAL_DATA: c_int = (B_TRANSLATION_ERROR_BASE + 2);
pub const B_INTERFACE_ERROR_BASE: c_int = (B_GENERAL_ERROR_BASE + 0x3000);
pub const B_INTERRUPTED: c_int = (B_GENERAL_ERROR_BASE + 10);
pub const B_IO_ERROR: c_int = (B_GENERAL_ERROR_BASE + 1);
pub const B_IS_A_DIRECTORY: c_int = (B_STORAGE_ERROR_BASE + 9);
pub const B_LAST_BUFFER_ERROR: c_int = (B_MEDIA_ERROR_BASE + 7);
pub const B_LAUNCH_FAILED: c_int = (B_APP_ERROR_BASE + 5);
pub const B_LAUNCH_FAILED_APP_IN_TRASH: c_int = (B_APP_ERROR_BASE + 12);
pub const B_LAUNCH_FAILED_APP_NOT_FOUND: c_int = (B_APP_ERROR_BASE + 11);
pub const B_LAUNCH_FAILED_EXECUTABLE: c_int = (B_APP_ERROR_BASE + 10);
pub const B_LAUNCH_FAILED_FILES_APP_NOT_FOUND: c_int = (B_APP_ERROR_BASE + 14);
pub const B_LAUNCH_FAILED_NO_PREFERRED_APP: c_int = (B_APP_ERROR_BASE + 13);
pub const B_LAUNCH_FAILED_NO_RESOLVE_LINK: c_int = (B_APP_ERROR_BASE + 9);
pub const B_LEGACY_EXECUTABLE: c_int = (B_OS_ERROR_BASE + 0x306);
pub const B_LINK_LIMIT: c_int = (B_STORAGE_ERROR_BASE + 12);
pub const B_MAIL_ACCESS_ERROR: c_int = (B_MAIL_ERROR_BASE + 4);
pub const B_MAIL_ERROR_BASE: c_int = (B_GENERAL_ERROR_BASE + 0x8000);
pub const B_MAIL_INVALID_MAIL: c_int = (B_MAIL_ERROR_BASE + 7);
pub const B_MAIL_NO_DAEMON: c_int = (B_MAIL_ERROR_BASE + 0);
pub const B_MAIL_NO_RECIPIENT: c_int = (B_MAIL_ERROR_BASE + 6);
pub const B_MAIL_UNKNOWN_FIELD: c_int = (B_MAIL_ERROR_BASE + 5);
pub const B_MAIL_UNKNOWN_HOST: c_int = (B_MAIL_ERROR_BASE + 3);
pub const B_MAIL_UNKNOWN_USER: c_int = (B_MAIL_ERROR_BASE + 1);
pub const B_MAIL_WRONG_PASSWORD: c_int = (B_MAIL_ERROR_BASE + 2);
pub const B_MEDIA_ADDON_DISABLED: c_int = (B_MEDIA_ERROR_BASE + 123);
pub const B_MEDIA_ADDON_FAILED: c_int = (B_MEDIA_ERROR_BASE + 122);
pub const B_MEDIA_ADDON_RESTRICTED: c_int = (B_MEDIA_ERROR_BASE + 126);
pub const B_MEDIA_ALREADY_CONNECTED: c_int = (B_MEDIA_ERROR_BASE + 119);
pub const B_MEDIA_APP_ALREADY_REGISTERED: c_int = (B_MEDIA_ERROR_BASE + 111);
pub const B_MEDIA_APP_NOT_REGISTERED: c_int = (B_MEDIA_ERROR_BASE + 112);
pub const B_MEDIA_BAD_BUFFER: c_int = (B_MEDIA_ERROR_BASE + 104);
pub const B_MEDIA_BAD_CLIP_FORMAT: c_int = (B_MEDIA_ERROR_BASE + 121);
pub const B_MEDIA_BAD_DESTINATION: c_int = (B_MEDIA_ERROR_BASE + 118);
pub const B_MEDIA_BAD_FORMAT: c_int = (B_MEDIA_ERROR_BASE + 103);
pub const B_MEDIA_BAD_NODE: c_int = (B_MEDIA_ERROR_BASE + 101);
pub const B_MEDIA_BAD_SOURCE: c_int = (B_MEDIA_ERROR_BASE + 117);
pub const B_MEDIA_BUFFER_ALREADY_EXISTS: c_int = (B_MEDIA_ERROR_BASE + 108);
pub const B_MEDIA_BUFFERS_NOT_RECLAIMED: c_int = (B_MEDIA_ERROR_BASE + 114);
pub const B_MEDIA_CANNOT_CHANGE_RUN_MODE: c_int = (B_MEDIA_ERROR_BASE + 110);
pub const B_MEDIA_CANNOT_RECLAIM_BUFFERS: c_int = (B_MEDIA_ERROR_BASE + 113);
pub const B_MEDIA_CANNOT_SEEK: c_int = (B_MEDIA_ERROR_BASE + 109);
pub const B_MEDIA_CHANGE_IN_PROGRESS: c_int = (B_MEDIA_ERROR_BASE + 124);
pub const B_MEDIA_DUPLICATE_FORMAT: c_int = (B_MEDIA_ERROR_BASE + 128);
pub const B_MEDIA_ERROR_BASE: c_int = (B_GENERAL_ERROR_BASE + 0x4000);
pub const B_MEDIA_NO_HANDLER: c_int = (B_MEDIA_ERROR_BASE + 127);
pub const B_MEDIA_NODE_ALREADY_EXISTS: c_int = (B_MEDIA_ERROR_BASE + 107);
pub const B_MEDIA_NODE_BUSY: c_int = (B_MEDIA_ERROR_BASE + 102);
pub const B_MEDIA_NOT_CONNECTED: c_int = (B_MEDIA_ERROR_BASE + 120);
pub const B_MEDIA_REALTIME_DISABLED: c_int = (B_MEDIA_ERROR_BASE + 129);
pub const B_MEDIA_REALTIME_UNAVAILABLE: c_int = (B_MEDIA_ERROR_BASE + 130);
pub const B_MEDIA_STALE_CHANGE_COUNT: c_int = (B_MEDIA_ERROR_BASE + 125);
pub const B_MEDIA_SYSTEM_FAILURE: c_int = (B_MEDIA_ERROR_BASE + 100);
pub const B_MEDIA_TIME_SOURCE_BUSY: c_int = (B_MEDIA_ERROR_BASE + 116);
pub const B_MEDIA_TIME_SOURCE_STOPPED: c_int = (B_MEDIA_ERROR_BASE + 115);
pub const B_MEDIA_TOO_MANY_BUFFERS: c_int = (B_MEDIA_ERROR_BASE + 106);
pub const B_MEDIA_TOO_MANY_NODES: c_int = (B_MEDIA_ERROR_BASE + 105);
pub const B_MESSAGE_TO_SELF: c_int = (B_APP_ERROR_BASE + 2);
pub const B_MIDI_ERROR_BASE: c_int = (B_GENERAL_ERROR_BASE + 0x5000);
pub const B_MISMATCHED_VALUES: c_int = (B_GENERAL_ERROR_BASE + 6);
pub const B_MISSING_LIBRARY: c_int = (B_OS_ERROR_BASE + 0x303);
pub const B_MISSING_SYMBOL: c_int = (B_OS_ERROR_BASE + 0x304);
pub const B_NAME_IN_USE: c_int = (B_GENERAL_ERROR_BASE + 8);
pub const B_NAME_NOT_FOUND: c_int = (B_GENERAL_ERROR_BASE + 7);
pub const B_NAME_TOO_LONG: c_int = (B_STORAGE_ERROR_BASE + 4);
pub const B_NO_ERROR: c_int = ((int)0);
pub const B_NO_INIT: c_int = (B_GENERAL_ERROR_BASE + 13);
pub const B_NO_MEMORY: c_int = (B_GENERAL_ERROR_BASE + 0);
pub const B_NO_MORE_FDS: c_int = (B_STORAGE_ERROR_BASE + 10);
pub const B_NO_MORE_PORTS: c_int = (B_OS_ERROR_BASE + 0x201);
pub const B_NO_MORE_SEMS: c_int = (B_OS_ERROR_BASE + 1);
pub const B_NO_MORE_TEAMS: c_int = (B_OS_ERROR_BASE + 0x104);
pub const B_NO_MORE_THREADS: c_int = (B_OS_ERROR_BASE + 0x101);
pub const B_NO_PRINT_SERVER: c_int = (B_PRINT_ERROR_BASE + 0);
pub const B_NO_TRANSLATOR: c_int = (B_TRANSLATION_ERROR_BASE + 1);
pub const B_NOT_A_DIRECTORY: c_int = (B_STORAGE_ERROR_BASE + 5);
pub const B_NOT_A_MESSAGE: c_int = (B_APP_ERROR_BASE + 16);
pub const B_NOT_ALLOWED: c_int = (B_GENERAL_ERROR_BASE + 15);
pub const B_NOT_AN_EXECUTABLE: c_int = (B_OS_ERROR_BASE + 0x302);
pub const B_NOT_INITIALIZED: c_int = (B_GENERAL_ERROR_BASE + 13);
pub const B_NOT_SUPPORTED: c_int = B_FROM_POSIX_ERROR(EOPNOTSUPP);
pub const B_OK: c_int = ((int)0);
pub const B_OS_ERROR_BASE: c_int = (B_GENERAL_ERROR_BASE + 0x1000);
pub const B_PARTIAL_READ: c_int = (B_STORAGE_ERROR_BASE + 16);
pub const B_PARTIAL_WRITE: c_int = (B_STORAGE_ERROR_BASE + 17);
pub const B_PARTITION_TOO_SMALL: c_int = (B_STORAGE_ERROR_BASE + 15);
pub const B_PERMISSION_DENIED: c_int = (B_GENERAL_ERROR_BASE + 2);
pub const B_POSIX_ENOMEM: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 0);
pub const B_POSIX_ERROR_BASE: c_int = (B_GENERAL_ERROR_BASE + 0x7000);
pub const B_PRINT_ERROR_BASE: c_int = (B_GENERAL_ERROR_BASE + 0x9000);
pub const B_READ_ONLY_DEVICE: c_int = (B_STORAGE_ERROR_BASE + 8);
pub const B_RESOURCE_NOT_FOUND: c_int = (B_MEDIA_ERROR_BASE + 2);
pub const B_RESOURCE_UNAVAILABLE: c_int = (B_MEDIA_ERROR_BASE + 3);
pub const B_RESULT_NOT_REPRESENTABLE: c_int = B_FROM_POSIX_ERROR(ERANGE);
pub const B_SERVER_NOT_FOUND: c_int = (B_MEDIA_ERROR_BASE + 1);
pub const B_SHUTDOWN_CANCELLED: c_int = (B_APP_ERROR_BASE + 17);
pub const B_SHUTTING_DOWN: c_int = (B_APP_ERROR_BASE + 18);
pub const B_STORAGE_ERROR_BASE: c_int = (B_GENERAL_ERROR_BASE + 0x6000);
pub const B_STREAM_NOT_FOUND: c_int = (B_MEDIA_ERROR_BASE + 0);
pub const B_SUBSCRIBER_NOT_ENTERED: c_int = (B_MEDIA_ERROR_BASE + 5);
pub const B_TIMED_OUT: c_int = (B_GENERAL_ERROR_BASE + 9);
pub const B_TO_NEGATIVE_ERROR: c_int = _to_negative_error(error);
pub const B_TO_POSITIVE_ERROR: c_int = _to_positive_error(error);
pub const B_TO_POSIX_ERROR: c_int = (error);
pub const B_TOO_MANY_ARGS: c_int = B_FROM_POSIX_ERROR(E2BIG);
pub const B_TRANSLATION_BASE_ERROR: c_int = (B_TRANSLATION_ERROR_BASE + 0);
pub const B_TRANSLATION_ERROR_BASE: c_int = (B_GENERAL_ERROR_BASE + 0x4800);
pub const B_UNKNOWN_EXECUTABLE: c_int = (B_OS_ERROR_BASE + 0x305);
pub const B_UNKNOWN_MIME_TYPE: c_int = (B_APP_ERROR_BASE + 7);
pub const B_UNSUPPORTED: c_int = (B_STORAGE_ERROR_BASE + 14);
pub const B_WOULD_BLOCK: c_int = (B_GENERAL_ERROR_BASE + 11);
pub const E2BIG: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 1);
pub const EACCES: c_int = B_TO_POSIX_ERROR(B_PERMISSION_DENIED);
pub const EADDRINUSE: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 22);
pub const EADDRNOTAVAIL: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 23);
pub const EAFNOSUPPORT: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 21);
pub const EAGAIN: c_int = B_TO_POSIX_ERROR(B_WOULD_BLOCK)	/* SysV compatibility */;
pub const EALREADY: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 37);
pub const EBADF: c_int = B_TO_POSIX_ERROR(B_FILE_ERROR);
pub const EBADMSG: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 46);
pub const EBUSY: c_int = B_TO_POSIX_ERROR(B_BUSY);
pub const ECANCELED: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 47);
pub const ECHILD: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 2);
pub const ECONNABORTED: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 27);
pub const ECONNREFUSED: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 32);
pub const ECONNRESET: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 28);
pub const EDEADLK: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 3);
pub const EDESTADDRREQ: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 48);
pub const EDOM: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 16);
pub const EDQUOT: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 49);
pub const EEXIST: c_int = B_TO_POSIX_ERROR(B_FILE_EXISTS);
pub const EFAULT: c_int = B_TO_POSIX_ERROR(B_BAD_ADDRESS);
pub const EFBIG: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 4);
pub const EFPOS: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 14);
pub const EHOSTDOWN: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 45);
pub const EHOSTUNREACH: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 33);
pub const EIDRM: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 50);
pub const EILSEQ: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 38);
pub const EINPROGRESS: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 36);
pub const EINTR: c_int = B_TO_POSIX_ERROR(B_INTERRUPTED);
pub const EINVAL: c_int = B_TO_POSIX_ERROR(B_BAD_VALUE);
pub const EIO: c_int = B_TO_POSIX_ERROR(B_IO_ERROR);
pub const EISCONN: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 29);
pub const EISDIR: c_int = B_TO_POSIX_ERROR(B_IS_A_DIRECTORY);
pub const ELOOP: c_int = B_TO_POSIX_ERROR(B_LINK_LIMIT);
pub const EMFILE: c_int = B_TO_POSIX_ERROR(B_NO_MORE_FDS);
pub const EMLINK: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 5);
pub const EMSGSIZE: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 42);
pub const EMULTIHOP: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 51);
pub const ENAMETOOLONG: c_int = B_TO_POSIX_ERROR(B_NAME_TOO_LONG);
pub const ENETDOWN: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 24);
pub const ENETRESET: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 26);
pub const ENETUNREACH: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 25);
pub const ENFILE: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 6);
pub const ENOATTR: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 60);
pub const ENOBUFS: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 35);
pub const ENODATA: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 52);
pub const ENODEV: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 7);
pub const ENOENT: c_int = B_TO_POSIX_ERROR(B_ENTRY_NOT_FOUND);
pub const ENOEXEC: c_int = B_TO_POSIX_ERROR(B_NOT_AN_EXECUTABLE);
pub const ENOLCK: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 8);
pub const ENOLINK: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 53);
pub const ENOMEM: c_int = B_NO_MEMORY;
pub const ENOMSG: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 39);
pub const ENOPROTOOPT: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 34);
pub const ENOSPC: c_int = B_TO_POSIX_ERROR(B_DEVICE_FULL);
pub const ENOSR: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 54);
pub const ENOSTR: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 55);
pub const ENOSYS: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 9);
pub const ENOTCONN: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 30);
pub const ENOTDIR: c_int = B_TO_POSIX_ERROR(B_NOT_A_DIRECTORY);
pub const ENOTEMPTY: c_int = B_TO_POSIX_ERROR(B_DIRECTORY_NOT_EMPTY);
pub const ENOTRECOVERABLE: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 61);
pub const ENOTSOCK: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 44);
pub const ENOTSUP: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 56);
pub const ENOTTY: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 10);
pub const ENXIO: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 11);
pub const EOPNOTSUPP: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 43);
pub const EOVERFLOW: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 41);
pub const EOWNERDEAD: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 62);
pub const EPERM: c_int = B_TO_POSIX_ERROR(B_NOT_ALLOWED);
pub const EPFNOSUPPORT: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 20);
pub const EPIPE: c_int = B_TO_POSIX_ERROR(B_BUSTED_PIPE);
pub const EPROTO: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 57);
pub const EPROTONOSUPPORT: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 19);
pub const EPROTOTYPE: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 18);
pub const ERANGE: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 17);
pub const EROFS: c_int = B_TO_POSIX_ERROR(B_READ_ONLY_DEVICE);
pub const ESHUTDOWN: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 31);
pub const ESIGPARM: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 15);
pub const ESPIPE: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 12);
pub const ESRCH: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 13);
pub const ESTALE: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 40);
pub const ETIME: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 58);
pub const ETIMEDOUT: c_int = B_TO_POSIX_ERROR(B_TIMED_OUT);
pub const ETXTBSY: c_int = B_TO_POSIX_ERROR(B_POSIX_ERROR_BASE + 59);
pub const EWOULDBLOCK: c_int = B_TO_POSIX_ERROR(B_WOULD_BLOCK)	/* BSD compatibility */;
pub const EXDEV: c_int = B_TO_POSIX_ERROR(B_CROSS_DEVICE_LINK);

pub const PARSEDATE_DAY_RELATIVE_TIME: c_int = PARSEDATE_RELATIVE_TIME;
pub const PARSEDATE_INVALID_DATE: c_int = 0x0100;
pub const PARSEDATE_MINUTE_RELATIVE_TIME: c_int = 0x0002;
pub const PARSEDATE_RELATIVE_TIME: c_int = 0x0001;

pub const B_PRId16: &str = "d";
pub const B_PRId32: &str = __HAIKU_PRI_PREFIX_32 "d";
pub const B_PRId64: &str = __HAIKU_PRI_PREFIX_64 "d";
pub const B_PRId8: &str = "d";
pub const B_PRIi16: &str = "i";
pub const B_PRIi32: &str = __HAIKU_PRI_PREFIX_32 "i";
pub const B_PRIi64: &str = __HAIKU_PRI_PREFIX_64 "i";
pub const B_PRIi8: &str = "i";
pub const B_PRIo16: &str = "o";
pub const B_PRIo32: &str = __HAIKU_PRI_PREFIX_32 "o";
pub const B_PRIo64: &str = __HAIKU_PRI_PREFIX_64 "o";
pub const B_PRIo8: &str = "o";
pub const B_PRIu16: &str = "u";
pub const B_PRIu32: &str = __HAIKU_PRI_PREFIX_32 "u";
pub const B_PRIu64: &str = __HAIKU_PRI_PREFIX_64 "u";
pub const B_PRIu8: &str = "u";
pub const B_PRIx16: &str = "x";
pub const B_PRIX16: &str = "X";
pub const B_PRIx32: &str = __HAIKU_PRI_PREFIX_32 "x";
pub const B_PRIX32: &str = __HAIKU_PRI_PREFIX_32 "X";
pub const B_PRIx64: &str = __HAIKU_PRI_PREFIX_64 "x";
pub const B_PRIX64: &str = __HAIKU_PRI_PREFIX_64 "X";
pub const B_PRIx8: &str = "x";
pub const B_PRIX8: &str = "X";
pub const B_SCNd16: &str = "hd";
pub const B_SCNd32: &str = __HAIKU_PRI_PREFIX_32 "d";
pub const B_SCNd64: &str = __HAIKU_PRI_PREFIX_64 "d";
pub const B_SCNd8: &str = "hhd";
pub const B_SCNi16: &str = "hi";
pub const B_SCNi32: &str = __HAIKU_PRI_PREFIX_32 "i";
pub const B_SCNi64: &str = __HAIKU_PRI_PREFIX_64 "i";
pub const B_SCNi8: &str = "hhi";
pub const B_SCNo16: &str = "ho";
pub const B_SCNo32: &str = __HAIKU_PRI_PREFIX_32 "o";
pub const B_SCNo64: &str = __HAIKU_PRI_PREFIX_64 "o";
pub const B_SCNo8: &str = "hho";
pub const B_SCNu16: &str = "hu";
pub const B_SCNu32: &str = __HAIKU_PRI_PREFIX_32 "u";
pub const B_SCNu64: &str = __HAIKU_PRI_PREFIX_64 "u";
pub const B_SCNu8: &str = "hhu";
pub const B_SCNx16: &str = "hx";
pub const B_SCNx32: &str = __HAIKU_PRI_PREFIX_32 "x";
pub const B_SCNx64: &str = __HAIKU_PRI_PREFIX_64 "x";
pub const B_SCNx8: &str = "hhx";
pub const B_PRIdBIGTIME: c_int = B_PRId64;
pub const B_PRIdDEV: c_int = B_PRId32;
pub const B_PRIdINO: c_int = B_PRId64;
pub const B_PRIdOFF: c_int = B_PRId64;
pub const B_PRIdSSIZE: &str = __HAIKU_PRI_PREFIX_ADDR "d";
pub const B_PRIdTIME: c_int = B_PRId64;
pub const B_PRIiBIGTIME: c_int = B_PRIi64;
pub const B_PRIiDEV: c_int = B_PRIi32;
pub const B_PRIiINO: c_int = B_PRIi64;
pub const B_PRIiOFF: c_int = B_PRIi64;
pub const B_PRIiSSIZE: &str = __HAIKU_PRI_PREFIX_ADDR "i";
pub const B_PRIiTIME: c_int = B_PRIi64;
pub const B_PRINTF_POINTER_WIDTH: c_int = 8;
pub const B_PRIoADDR: &str = __HAIKU_PRI_PREFIX_ADDR "o";
pub const B_PRIoGENADDR: &str = __HAIKU_PRI_PREFIX_GENERIC_ADDR "o";
pub const B_PRIoPHYSADDR: &str = __HAIKU_PRI_PREFIX_PHYS_ADDR "o";
pub const B_PRIoSIZE: &str = __HAIKU_PRI_PREFIX_ADDR "o";
pub const B_PRIuADDR: &str = __HAIKU_PRI_PREFIX_ADDR "u";
pub const B_PRIuGENADDR: &str = __HAIKU_PRI_PREFIX_GENERIC_ADDR "u";
pub const B_PRIuPHYSADDR: &str = __HAIKU_PRI_PREFIX_PHYS_ADDR "u";
pub const B_PRIuSIZE: &str = __HAIKU_PRI_PREFIX_ADDR "u";
pub const B_PRIxADDR: &str = __HAIKU_PRI_PREFIX_ADDR "x";
pub const B_PRIXADDR: &str = __HAIKU_PRI_PREFIX_ADDR "X";
pub const B_PRIxGENADDR: &str = __HAIKU_PRI_PREFIX_GENERIC_ADDR "x";
pub const B_PRIXGENADDR: &str = __HAIKU_PRI_PREFIX_GENERIC_ADDR "X";
pub const B_PRIxOFF: c_int = B_PRIx64;
pub const B_PRIxPHYSADDR: &str = __HAIKU_PRI_PREFIX_PHYS_ADDR "x";
pub const B_PRIXPHYSADDR: &str = __HAIKU_PRI_PREFIX_PHYS_ADDR "X";
pub const B_PRIxSIZE: &str = __HAIKU_PRI_PREFIX_ADDR "x";
pub const B_PRIXSIZE: &str = __HAIKU_PRI_PREFIX_ADDR "X";
pub const B_SCNdOFF: c_int = B_SCNd64;
pub const B_SCNdSSIZE: &str = __HAIKU_PRI_PREFIX_ADDR "d";
pub const B_SCNiOFF: c_int = B_SCNi64;
pub const B_SCNiSSIZE: &str = __HAIKU_PRI_PREFIX_ADDR "i";
pub const B_SCNoADDR: &str = __HAIKU_PRI_PREFIX_ADDR "o";
pub const B_SCNoGENADDR: &str = __HAIKU_PRI_PREFIX_GENERIC_ADDR "o";
pub const B_SCNoPHYSADDR: &str = __HAIKU_PRI_PREFIX_PHYS_ADDR "o";
pub const B_SCNoSIZE: &str = __HAIKU_PRI_PREFIX_ADDR "o";
pub const B_SCNuADDR: &str = __HAIKU_PRI_PREFIX_ADDR "u";
pub const B_SCNuGENADDR: &str = __HAIKU_PRI_PREFIX_GENERIC_ADDR "u";
pub const B_SCNuPHYSADDR: &str = __HAIKU_PRI_PREFIX_PHYS_ADDR "u";
pub const B_SCNuSIZE: &str = __HAIKU_PRI_PREFIX_ADDR "u";
pub const B_SCNxADDR: &str = __HAIKU_PRI_PREFIX_ADDR "x";
pub const B_SCNxGENADDR: &str = __HAIKU_PRI_PREFIX_GENERIC_ADDR "x";
pub const B_SCNxOFF: c_int = B_SCNx64;
pub const B_SCNxPHYSADDR: &str = __HAIKU_PRI_PREFIX_PHYS_ADDR "x";
pub const B_SCNxSIZE: &str = __HAIKU_PRI_PREFIX_ADDR "x";
pub const FALSE: c_int = 0;
pub const TRUE: c_int = 1;
pub const B_COUNT_OF: c_int = (sizeof(a) / sizeof(a[0]));
pub const max_c: c_int = ((a)>(b)?(a):(b));
pub const min_c: c_int = ((a)>(b)?(b):(a));
pub const NULL: c_int = (0);

pub const TLS_MAX_KEYS: c_int = 64;

//  ENUM: @101
pub const B_AFFINE_TRANSFORM_TYPE: c_int = 0x414d5458 /* 'AMTX' */;
pub const B_ALIGNMENT_TYPE: c_int = 0x414c474e /* 'ALGN' */;
pub const B_ANY_TYPE: c_int = 0x414e5954 /* 'ANYT' */;
pub const B_ATOM_TYPE: c_int = 0x41544f4d /* 'ATOM' */;
pub const B_ATOMREF_TYPE: c_int = 0x41544d52 /* 'ATMR' */;
pub const B_BOOL_TYPE: c_int = 0x424f4f4c /* 'BOOL' */;
pub const B_CHAR_TYPE: c_int = 0x43484152 /* 'CHAR' */;
pub const B_COLOR_8_BIT_TYPE: c_int = 0x434c5242 /* 'CLRB' */;
pub const B_DOUBLE_TYPE: c_int = 0x44424c45 /* 'DBLE' */;
pub const B_FLOAT_TYPE: c_int = 0x464c4f54 /* 'FLOT' */;
pub const B_GRAYSCALE_8_BIT_TYPE: c_int = 0x47525942 /* 'GRYB' */;
pub const B_INT16_TYPE: c_int = 0x53485254 /* 'SHRT' */;
pub const B_INT32_TYPE: c_int = 0x4c4f4e47 /* 'LONG' */;
pub const B_INT64_TYPE: c_int = 0x4c4c4e47 /* 'LLNG' */;
pub const B_INT8_TYPE: c_int = 0x42595445 /* 'BYTE' */;
pub const B_LARGE_ICON_TYPE: c_int = 0x49434f4e /* 'ICON' */;
pub const B_MEDIA_PARAMETER_GROUP_TYPE: c_int = 0x424d4347 /* 'BMCG' */;
pub const B_MEDIA_PARAMETER_TYPE: c_int = 0x424d4354 /* 'BMCT' */;
pub const B_MEDIA_PARAMETER_WEB_TYPE: c_int = 0x424d4357 /* 'BMCW' */;
pub const B_MESSAGE_TYPE: c_int = 0x4d534747 /* 'MSGG' */;
pub const B_MESSENGER_TYPE: c_int = 0x4d534e47 /* 'MSNG' */;
pub const B_MIME_TYPE: c_int = 0x4d494d45 /* 'MIME' */;
pub const B_MINI_ICON_TYPE: c_int = 0x4d49434e /* 'MICN' */;
pub const B_MONOCHROME_1_BIT_TYPE: c_int = 0x4d4e4f42 /* 'MNOB' */;
pub const B_OBJECT_TYPE: c_int = 0x4f505452 /* 'OPTR' */;
pub const B_OFF_T_TYPE: c_int = 0x4f464654 /* 'OFFT' */;
pub const B_PATTERN_TYPE: c_int = 0x5041544e /* 'PATN' */;
pub const B_POINTER_TYPE: c_int = 0x504e5452 /* 'PNTR' */;
pub const B_POINT_TYPE: c_int = 0x42504e54 /* 'BPNT' */;
pub const B_PROPERTY_INFO_TYPE: c_int = 0x53435444 /* 'SCTD' */;
pub const B_RAW_TYPE: c_int = 0x52415754 /* 'RAWT' */;
pub const B_RECT_TYPE: c_int = 0x52454354 /* 'RECT' */;
pub const B_REF_TYPE: c_int = 0x52524546 /* 'RREF' */;
pub const B_NODE_REF_TYPE: c_int = 0x4e524546 /* 'NREF' */;
pub const B_RGB_32_BIT_TYPE: c_int = 0x52474242 /* 'RGBB' */;
pub const B_RGB_COLOR_TYPE: c_int = 0x52474243 /* 'RGBC' */;
pub const B_SIZE_TYPE: c_int = 0x53495a45 /* 'SIZE' */;
pub const B_SIZE_T_TYPE: c_int = 0x53495a54 /* 'SIZT' */;
pub const B_SSIZE_T_TYPE: c_int = 0x53535a54 /* 'SSZT' */;
pub const B_STRING_TYPE: c_int = 0x43535452 /* 'CSTR' */;
pub const B_STRING_LIST_TYPE: c_int = 0x5354524c /* 'STRL' */;
pub const B_TIME_TYPE: c_int = 0x54494d45 /* 'TIME' */;
pub const B_UINT16_TYPE: c_int = 0x55534854 /* 'USHT' */;
pub const B_UINT32_TYPE: c_int = 0x554c4e47 /* 'ULNG' */;
pub const B_UINT64_TYPE: c_int = 0x554c4c47 /* 'ULLG' */;
pub const B_UINT8_TYPE: c_int = 0x55425954 /* 'UBYT' */;
pub const B_VECTOR_ICON_TYPE: c_int = 0x5649434e /* 'VICN' */;
pub const B_XATTR_TYPE: c_int = 0x58415452 /* 'XATR' */;
pub const B_NETWORK_ADDRESS_TYPE: c_int = 0x4e574144 /* 'NWAD' */;
pub const B_MIME_STRING_TYPE: c_int = 0x4d494d53 /* 'MIMS' */;
pub const B_ASCII_TYPE: c_int = 0x54455854 /* 'TEXT' */;

//  ENUM: @102
pub const B_ISO1_CONVERSION: c_int = 0;
pub const B_ISO2_CONVERSION: c_int = 0 + 1;
pub const B_ISO3_CONVERSION: c_int = 0 + 2;
pub const B_ISO4_CONVERSION: c_int = 0 + 3;
pub const B_ISO5_CONVERSION: c_int = 0 + 4;
pub const B_ISO6_CONVERSION: c_int = 0 + 5;
pub const B_ISO7_CONVERSION: c_int = 0 + 6;
pub const B_ISO8_CONVERSION: c_int = 0 + 7;
pub const B_ISO9_CONVERSION: c_int = 0 + 8;
pub const B_ISO10_CONVERSION: c_int = 0 + 9;
pub const B_MAC_ROMAN_CONVERSION: c_int = 0 + 10;
pub const B_SJIS_CONVERSION: c_int = 0 + 11;
pub const B_EUC_CONVERSION: c_int = 0 + 12;
pub const B_JIS_CONVERSION: c_int = 0 + 13;
pub const B_MS_WINDOWS_CONVERSION: c_int = 0 + 14;
pub const B_UNICODE_CONVERSION: c_int = 0 + 15;
pub const B_KOI8R_CONVERSION: c_int = 0 + 16;
pub const B_MS_WINDOWS_1251_CONVERSION: c_int = 0 + 17;
pub const B_MS_DOS_866_CONVERSION: c_int = 0 + 18;
pub const B_MS_DOS_CONVERSION: c_int = 0 + 19;
pub const B_EUC_KR_CONVERSION: c_int = 0 + 20;
pub const B_ISO13_CONVERSION: c_int = 0 + 21;
pub const B_ISO14_CONVERSION: c_int = 0 + 22;
pub const B_ISO15_CONVERSION: c_int = 0 + 23;
pub const B_BIG5_CONVERSION: c_int = 0 + 24;
pub const B_GBK_CONVERSION: c_int = 0 + 25;
pub const B_UTF16_CONVERSION: c_int = 0 + 26;
pub const B_MS_WINDOWS_1250_CONVERSION: c_int = 0 + 27;

pub const B_TRANSLATION_CURRENT_VERSION: c_int = B_BEOS_VERSION;
pub const B_TRANSLATION_MAJOR_VERSION: c_int = (v >> 8);
pub const B_TRANSLATION_MAKE_VERSION: c_int = ((major << 8) | ((minor << 4) & 0xf0) | (revision & 0x0f));
pub const B_TRANSLATION_MIN_VERSION: c_int = 161;
pub const B_TRANSLATION_MINOR_VERSION: c_int = ((v >> 4) & 0xf);
pub const B_TRANSLATION_REVISION_VERSION: c_int = (v & 0xf);

//  ENUM: @104
pub const B_GIF_FORMAT: c_int = 0x47494620 /* 'GIF ' */;
pub const B_JPEG_FORMAT: c_int = 0x4a504547 /* 'JPEG' */;
pub const B_PNG_FORMAT: c_int = 0x504e4720 /* 'PNG ' */;
pub const B_PPM_FORMAT: c_int = 0x50504d20 /* 'PPM ' */;
pub const B_TGA_FORMAT: c_int = 0x54474120 /* 'TGA ' */;
pub const B_BMP_FORMAT: c_int = 0x424d5020 /* 'BMP ' */;
pub const B_TIFF_FORMAT: c_int = 0x54494646 /* 'TIFF' */;
pub const B_WEBP_FORMAT: c_int = 0x57656250 /* 'WebP' */;
pub const B_DXF_FORMAT: c_int = 0x44584620 /* 'DXF ' */;
pub const B_EPS_FORMAT: c_int = 0x45505320 /* 'EPS ' */;
pub const B_PICT_FORMAT: c_int = 0x50494354 /* 'PICT' */;
pub const B_WAV_FORMAT: c_int = 0x57415620 /* 'WAV ' */;
pub const B_AIFF_FORMAT: c_int = 0x41494646 /* 'AIFF' */;
pub const B_CD_FORMAT: c_int = 0x43442020 /* 'CD  ' */;
pub const B_AU_FORMAT: c_int = 0x41552020 /* 'AU  ' */;
pub const B_STYLED_TEXT_FORMAT: c_int = 0x53545854 /* 'STXT' */;
//  ENUM: TranslatorGroups
pub const B_TRANSLATOR_BITMAP: c_int = 0x62697473 /* 'bits' */;
pub const B_TRANSLATOR_PICTURE: c_int = 0x70696374 /* 'pict' */;
pub const B_TRANSLATOR_SOUND: c_int = 0x6e6f6973 /* 'nois' */;
pub const B_TRANSLATOR_TEXT: c_int = 0x54455854 /* 'TEXT' */;
pub const B_TRANSLATOR_MIDI: c_int = 0x6d696469 /* 'midi' */;
pub const B_TRANSLATOR_MEDIA: c_int = 0x6d686921 /* 'mhi!' */;
pub const B_TRANSLATOR_NONE: c_int = 0x6e6f6e65 /* 'none' */;
pub const B_TRANSLATOR_ANY_TYPE: c_int = 0;

pub const LOG_CONS: c_int = (2 << 12)	/* log to the system console on error */;
pub const LOG_NDELAY: c_int = (8 << 12)	/* connect to the syslog daemon immediately */;
pub const LOG_NOWAIT: c_int = (64 << 12)	/* do not wait for child processes */;
pub const LOG_ODELAY: c_int = (4 << 12)	/* delay open until syslog() is called */;
pub const LOG_PERROR: c_int = (32 << 12)	/* dump to stderr as well */;
pub const LOG_PID: c_int = (1 << 12)	/* log the process (thread/team) ID with each message */;
pub const LOG_SERIAL: c_int = (16 << 12)	/* dump to serial output as well (not implemented) */;
pub const LOG_AUTH: c_int = (4 << 3);
pub const LOG_AUTHPRIV: c_int = (10 << 3)	/* security/authorization messages (private) */;
pub const LOG_CRON: c_int = (9 << 3);
pub const LOG_DAEMON: c_int = (3 << 3);
pub const LOG_KERN: c_int = (0 << 3)	/* messages generated by the kernel */;
pub const LOG_LOCAL0: c_int = (16 << 3);
pub const LOG_LOCAL1: c_int = (17 << 3);
pub const LOG_LOCAL2: c_int = (18 << 3);
pub const LOG_LOCAL3: c_int = (19 << 3);
pub const LOG_LOCAL4: c_int = (20 << 3);
pub const LOG_LOCAL5: c_int = (21 << 3);
pub const LOG_LOCAL6: c_int = (22 << 3);
pub const LOG_LOCAL7: c_int = (23 << 3);
pub const LOG_LPR: c_int = (6 << 3);
pub const LOG_MAIL: c_int = (2 << 3);
pub const LOG_NEWS: c_int = (7 << 3);
pub const LOG_SYSLOG: c_int = (5 << 3)	/* messages generated internally by syslogd */;
pub const LOG_USER: c_int = (1 << 3)	/* by user processes */;
pub const LOG_UUCP: c_int = (8 << 3);
pub const LOG_ALERT: c_int = 1	/* a condition that should be corrected immediately */;
pub const LOG_CRIT: c_int = 2	/* critical conditions like hard drive errors */;
pub const LOG_DEBUG: c_int = 7;
pub const LOG_EMERG: c_int = 0	/* a panic condition */;
pub const LOG_ERR: c_int = 3;
pub const LOG_INFO: c_int = 6;
pub const LOG_NOTICE: c_int = 5;
pub const LOG_PANIC: c_int = LOG_EMERG;
pub const LOG_WARNING: c_int = 4;
pub const LOG_FAC: c_int = (((p) & LOG_FACMASK) >> 3);
pub const LOG_FACMASK: c_int = (0x03f8)	/* mask to extract facility part */;
pub const LOG_FTP: c_int = (11 << 3);
pub const LOG_MASK: c_int = (1 << (pri));
pub const LOG_NFACILITIES: c_int = (24)		/* current number of facilities */;
pub const LOG_PRI: c_int = ((p) & LOG_PRIMASK);
pub const LOG_PRIMASK: c_int = (0x7)	/* mask to extract priority part */;
pub const LOG_UPTO: c_int = ((1 << ((pri) + 1)) - 1);

//  ENUM: @7
pub const B_STICKY_EVENT: c_int = 0x01;

//  ENUM: ErrorType
pub const HostnameError: c_int = 0;
pub const NetworkError: c_int = 0 + 1;
pub const ProtocolError: c_int = 0 + 2;
pub const SystemError: c_int = 0 + 3;
pub const Canceled: c_int = 0 + 4;
//  ENUM: Verb
pub const Get: c_int = 0;
pub const Head: c_int = 0 + 1;
pub const Post: c_int = 0 + 2;
pub const Put: c_int = 0 + 3;
pub const Delete: c_int = 0 + 4;
pub const Connect: c_int = 0 + 5;
pub const Options: c_int = 0 + 6;
pub const Trace: c_int = 0 + 7;


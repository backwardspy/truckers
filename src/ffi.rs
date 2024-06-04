use std::ffi::{c_char, c_void};

const fn scs_make_version(major: u32, minor: u32) -> u32 {
    (major << 16) | minor
}

pub const SCS_TELEMETRY_VERSION_1_00: u32 = scs_make_version(1, 0);
pub const SCS_TELEMETRY_VERSION_1_01: u32 = scs_make_version(1, 1);
pub const SCS_TELEMETRY_VERSION_CURRENT: u32 = SCS_TELEMETRY_VERSION_1_01;

pub const SCS_TELEMETRY_EVENT_INVALID: u32 = 0;
pub const SCS_TELEMETRY_EVENT_FRAME_START: u32 = 1;
pub const SCS_TELEMETRY_EVENT_FRAME_END: u32 = 2;
pub const SCS_TELEMETRY_EVENT_PAUSED: u32 = 3;
pub const SCS_TELEMETRY_EVENT_STARTED: u32 = 4;
pub const SCS_TELEMETRY_EVENT_CONFIGURATION: u32 = 5;
pub const SCS_TELEMETRY_EVENT_GAMEPLAY: u32 = 6;

pub const SCS_VALUE_TYPE_INVALID: u32 = 0;
pub const SCS_VALUE_TYPE_BOOL: u32 = 1;
pub const SCS_VALUE_TYPE_S32: u32 = 2;
pub const SCS_VALUE_TYPE_U32: u32 = 3;
pub const SCS_VALUE_TYPE_U64: u32 = 4;
pub const SCS_VALUE_TYPE_FLOAT: u32 = 5;
pub const SCS_VALUE_TYPE_DOUBLE: u32 = 6;
pub const SCS_VALUE_TYPE_FVECTOR: u32 = 7;
pub const SCS_VALUE_TYPE_DVECTOR: u32 = 8;
pub const SCS_VALUE_TYPE_EULER: u32 = 9;
pub const SCS_VALUE_TYPE_FPLACEMENT: u32 = 10;
pub const SCS_VALUE_TYPE_DPLACEMENT: u32 = 11;
pub const SCS_VALUE_TYPE_STRING: u32 = 12;
pub const SCS_VALUE_TYPE_S64: u32 = 13;

pub const SCS_LOG_TYPE_MESSAGE: i32 = 0;
pub const SCS_LOG_TYPE_WARNING: i32 = 1;
pub const SCS_LOG_TYPE_ERROR: i32 = 2;

pub type SCSTelemetryEventCallbackFn =
    extern "stdcall" fn(event: u32, event_info: *const c_void, context: *const c_void);

pub type SCSTelemetryRegisterForEventFn = extern "stdcall" fn(
    event: u32,
    callback: SCSTelemetryEventCallbackFn,
    context: *const c_void,
) -> i32;

pub type SCSTelemetryUnregisterFromEventFn = extern "stdcall" fn(event: u32) -> i32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SCSValueFVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SCSValueDVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SCSValueEuler {
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SCSValueFPlacement {
    pub position: SCSValueFVector,
    pub orientation: SCSValueEuler,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SCSValueDPlacement {
    pub position: SCSValueDVector,
    pub orientation: SCSValueEuler,
    _padding: u32,
}

#[repr(C)]
pub union SCSValueUnion {
    pub Bool: bool,
    pub S32: i32,
    pub U32: u32,
    pub S64: i64,
    pub U64: u64,
    pub Float: f32,
    pub Double: f64,
    pub FVector: SCSValueFVector,
    pub DVector: SCSValueDVector,
    pub Euler: SCSValueEuler,
    pub FPlacement: SCSValueFPlacement,
    pub DPlacement: SCSValueDPlacement,
    pub String: *const c_char,
}

#[repr(C)]
pub struct SCSValue {
    pub value_type: u32,
    _padding: u32,
    pub value: SCSValueUnion,
}

#[repr(C)]
pub struct SCSNamedValue {
    pub name: *const c_char,
    pub index: u32,
    _padding: u32,
    pub value: SCSValue,
}

pub type SCSTelemetryChannelCallbackFn = extern "stdcall" fn(
    name: *const c_char,
    index: u32,
    value: *const SCSValue,
    context: *const c_void,
);

pub type SCSTelemetryRegisterForChannelFn = extern "stdcall" fn(
    name: *const c_char,
    index: u32,
    value_type: u32,
    flags: u32,
    callback: SCSTelemetryChannelCallbackFn,
    context: *const c_void,
) -> i32;

pub type SCSTelemetryUnregisterFromChannelFn =
    extern "stdcall" fn(name: *const c_char, index: u32, value_type: u32) -> i32;

pub type SCSLogFn = extern "stdcall" fn(log_type: i32, message: *const c_char);

#[repr(C)]
#[derive(Debug)]
pub struct SCSSDKInitParams {
    pub game_name: *const c_char,
    pub game_id: *const c_char,
    pub game_version: u32,
    _padding: u32,
    pub log: SCSLogFn,
}

#[repr(C)]
#[derive(Debug)]
pub struct SCSTelemetryInitParams {
    pub common: SCSSDKInitParams,

    pub register_for_event: SCSTelemetryRegisterForEventFn,
    pub unregister_from_event: SCSTelemetryUnregisterFromEventFn,

    pub register_for_channel: SCSTelemetryRegisterForChannelFn,
    pub unregister_from_channel: SCSTelemetryUnregisterFromChannelFn,
}

#[repr(C)]
#[derive(Debug)]
pub struct SCSTelemetryConfiguration {
    /// Set of logically grouped configuration parameters this
    /// event describes (e.g. truck configuration, trailer configuration).
    ///
    /// See SCS_TELEMETRY_CONFIGURATION_ID_* constants for the game in question.
    ///
    /// This pointer will be never NULL.
    pub id: *const c_char,

    /// Array of individual attributes.
    ///
    /// The array is terminated by entry whose name pointer is set to NULL.
    ///
    /// Names of the attributes are the SCS_TELEMETRY_CONFIG_ATTRIBUTE_* constants
    /// for the game in question.
    ///
    /// This pointer will be never NULL.
    pub attributes: *const SCSNamedValue,
}

#[repr(C)]
#[derive(Debug)]
pub struct SCSTelemetryGameplayEvent {
    /// The event id.
    ///
    ///The event ID name - check SCS_TELEMETRY_GAMEPLAY_EVENT_* for possible names.
    pub id: *const c_char,

    /// Array of individual attributes.
    ///
    /// The array is terminated by entry whose name pointer is set to NULL.
    ///
    /// Names of the attributes are the SCS_TELEMETRY_GAMEPLAY_EVENT_ATTRIBUTE_* constants
    /// for the game in question.
    ///
    /// This pointer will be never NULL.
    pub attributes: *const SCSNamedValue,
}

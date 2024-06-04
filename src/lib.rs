use std::ffi::{c_char, c_void, CStr};

use ffi::*;
use tracing::{debug, error, info};
use tracing_subscriber::ETS2TracingSubscriber;

mod ffi;
mod tracing_subscriber;

/// # Safety
///
/// This function is called by the game and assumes that the provided parameters are valid.
#[no_mangle]
pub unsafe extern "stdcall" fn scs_telemetry_init(
    version: u32,
    params: *const SCSTelemetryInitParams,
) -> i32 {
    tracing::subscriber::set_global_default(ETS2TracingSubscriber::new(unsafe {
        (*params).common.log
    }))
    .unwrap();

    info!("hello from trucke.rs!");

    match version {
        SCS_TELEMETRY_VERSION_1_00 => info!("SDK version 1.00"),
        SCS_TELEMETRY_VERSION_1_01 => info!("SDK version 1.01"),
        _ => {
            error!("unknown SDK version");
            return -1;
        }
    }

    let register =
        |event, callback| ((*params).register_for_event)(event, callback, std::ptr::null());

    register(SCS_TELEMETRY_EVENT_FRAME_START, telemetry_frame_start);
    register(SCS_TELEMETRY_EVENT_FRAME_END, telemetry_frame_end);
    register(SCS_TELEMETRY_EVENT_PAUSED, telemetry_paused);
    register(SCS_TELEMETRY_EVENT_STARTED, telemetry_started);
    register(SCS_TELEMETRY_EVENT_CONFIGURATION, telemetry_configuration);
    register(SCS_TELEMETRY_EVENT_GAMEPLAY, telemetry_gameplay_event);

    0
}

#[no_mangle]
pub extern "stdcall" fn scs_telemetry_shutdown() {
    debug!("shutting down");
}

#[no_mangle]
pub extern "stdcall" fn telemetry_frame_start(
    _event: u32,
    _event_info: *const c_void,
    _context: *const c_void,
) {
    // info!("telemetry frame start");
}

#[no_mangle]
pub extern "stdcall" fn telemetry_frame_end(
    _event: u32,
    _event_info: *const c_void,
    _context: *const c_void,
) {
    // info!("telemetry frame end");
}

#[no_mangle]
pub extern "stdcall" fn telemetry_paused(
    _event: u32,
    _event_info: *const c_void,
    _context: *const c_void,
) {
    info!("telemetry paused");
}

#[no_mangle]
pub extern "stdcall" fn telemetry_started(
    _event: u32,
    _event_info: *const c_void,
    _context: *const c_void,
) {
    info!("telemetry started");
}

#[no_mangle]
pub extern "stdcall" fn telemetry_configuration(
    _event: u32,
    event_info: *const c_void,
    _context: *const c_void,
) {
    let event_info = event_info as *const SCSTelemetryConfiguration;
    let id = unsafe { CStr::from_ptr((*event_info).id) };
    debug!(?id, "telemetry configuration");
    unsafe { log_attributes((*event_info).attributes) };
}

#[no_mangle]
pub extern "stdcall" fn telemetry_gameplay_event(
    _event: u32,
    event_info: *const c_void,
    _context: *const c_void,
) {
    let event_info = event_info as *const SCSTelemetryGameplayEvent;
    let id = unsafe { CStr::from_ptr((*event_info).id) };
    debug!(?id, "telemetry gameplay event");
    unsafe { log_attributes((*event_info).attributes) };
}

#[no_mangle]
pub extern "stdcall" fn telemetry_store_orientation(
    _name: *const c_char,
    _index: u32,
    _value: *const SCSValue,
    _context: *const c_void,
) {
    debug!("telemetry gameplay value");
}

#[no_mangle]
pub extern "stdcall" fn telemetry_store_float(
    _name: *const c_char,
    _index: u32,
    _value: *const SCSValue,
    _context: *const c_void,
) {
    debug!("telemetry store float");
}

#[no_mangle]
pub extern "stdcall" fn telemetry_store_s32(
    _name: *const c_char,
    _index: u32,
    _value: *const SCSValue,
    _context: *const c_void,
) {
    debug!("telemetry store s32");
}

#[cfg(windows)]
#[no_mangle]
extern "system" fn DllMain(_: *const u8, _: u32, _: *const u8) -> u32 {
    1
}

unsafe fn log_attributes(attributes: *const SCSNamedValue) {
    let mut idx = 0;
    loop {
        let attribute = attributes.offset(idx);
        if (*attribute).name.is_null() {
            break;
        }

        let name = CStr::from_ptr((*attribute).name);
        debug!(?name);
        match (*attribute).value.value_type {
            SCS_VALUE_TYPE_BOOL => {
                let v: bool = (*attribute).value.value.Bool;
                debug!(?v, "bool");
            }
            SCS_VALUE_TYPE_S32 => {
                let v: i32 = (*attribute).value.value.S32;
                debug!(?v, "s32");
            }
            SCS_VALUE_TYPE_U32 => {
                let v: u32 = (*attribute).value.value.U32;
                debug!(?v, "u32");
            }
            SCS_VALUE_TYPE_U64 => {
                let v: u64 = (*attribute).value.value.U64;
                debug!(?v, "u64");
            }
            SCS_VALUE_TYPE_FLOAT => {
                let v: f32 = (*attribute).value.value.Float;
                debug!(?v, "float");
            }
            SCS_VALUE_TYPE_DOUBLE => {
                let v: f64 = (*attribute).value.value.Double;
                debug!(?v, "double");
            }
            SCS_VALUE_TYPE_FVECTOR => {
                let v: SCSValueFVector = (*attribute).value.value.FVector;
                debug!(?v, "fvector");
            }
            SCS_VALUE_TYPE_DVECTOR => {
                let v: SCSValueDVector = (*attribute).value.value.DVector;
                debug!(?v, "dvector");
            }
            SCS_VALUE_TYPE_EULER => {
                let v: SCSValueEuler = (*attribute).value.value.Euler;
                debug!(?v, "euler");
            }
            SCS_VALUE_TYPE_FPLACEMENT => {
                let v: SCSValueFPlacement = (*attribute).value.value.FPlacement;
                debug!(?v, "fplacement");
            }
            SCS_VALUE_TYPE_DPLACEMENT => {
                let v: SCSValueDPlacement = (*attribute).value.value.DPlacement;
                debug!(?v, "dplacement");
            }
            SCS_VALUE_TYPE_STRING => {
                let v = CStr::from_ptr((*attribute).value.value.String);
                debug!(?v, "string");
            }
            SCS_VALUE_TYPE_S64 => {
                let v: i64 = (*attribute).value.value.S64;
                debug!(?v, "s64");
            }
            _ => {
                debug!("unknown");
            }
        }

        idx += 1;
    }
}

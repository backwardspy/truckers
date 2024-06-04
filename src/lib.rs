#![allow(non_upper_case_globals)]
use std::{
    backtrace::Backtrace,
    collections::HashMap,
    ffi::{c_char, c_void, CStr, CString},
    fs::File,
    io::Write as _,
};

use telemetry_events::{TelemetryConfigurationEvent, TelemetryGameplayEvent};
use tracing::{debug, error, info};
use tracing_subscriber::ETS2TracingSubscriber;

use truckers_scssdk_sys::*;
use value::Value;

mod telemetry_events;
mod tracing_subscriber;
mod value;

#[cfg(windows)]
mod win {
    use windows::core::s;
    use windows::Win32::Foundation::FreeLibrary;
    use windows::Win32::{
        Foundation::HMODULE,
        System::LibraryLoader::{GetModuleHandleExA, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS},
    };
    pub(crate) unsafe fn get_module_handle() -> HMODULE {
        let mut module = HMODULE(0);
        GetModuleHandleExA(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            s!("truckers"),
            (&mut module) as _,
        )
        .unwrap();
        module
    }

    pub(crate) unsafe fn free_library(module: HMODULE) {
        FreeLibrary(module).unwrap();
    }
}

/// # Safety
///
/// where is your god now
#[no_mangle]
pub unsafe extern "C" fn scs_telemetry_init(
    version: u32,
    params: *const scs_telemetry_init_params_v101_t,
) -> i32 {
    std::panic::set_hook(Box::new(|info| {
        let mut f = File::create("pigeon-panic.txt").unwrap();
        let _ = writeln!(&mut f, "{}", chrono::Local::now());
        let _ = writeln!(&mut f, "{}", info);
        let _ = writeln!(&mut f, "\nStack:\n");
        let bt = Backtrace::force_capture();
        let _ = writeln!(&mut f, "{}", bt);

        // this is here because nyx wanted it
        #[cfg(windows)]
        win::free_library(win::get_module_handle());
    }));

    tracing::subscriber::set_global_default(ETS2TracingSubscriber::new(unsafe {
        (*params).common.log.unwrap()
    }))
    .unwrap();

    info!("hello from trucke.rs!");

    match version {
        SCS_TELEMETRY_version_1_00 => info!("SDK version 1.00"),
        SCS_TELEMETRY_version_1_01 => info!("SDK version 1.01"),
        _ => {
            error!("unknown SDK version");
            return -1;
        }
    }

    let register = |event, callback| {
        ((*params).register_for_event.unwrap())(event, callback, std::ptr::null_mut())
    };

    register(SCS_TELEMETRY_EVENT_frame_start, Some(telemetry_frame_start));
    register(SCS_TELEMETRY_EVENT_frame_end, Some(telemetry_frame_end));
    register(SCS_TELEMETRY_EVENT_paused, Some(telemetry_paused));
    register(SCS_TELEMETRY_EVENT_started, Some(telemetry_started));
    register(
        SCS_TELEMETRY_EVENT_configuration,
        Some(telemetry_configuration),
    );
    register(SCS_TELEMETRY_EVENT_gameplay, Some(telemetry_gameplay_event));

    ((*params).register_for_channel.unwrap())(
        CStr::from_bytes_with_nul(SCS_TELEMETRY_TRUCK_CHANNEL_speed)
            .unwrap()
            .as_ptr(),
        SCS_U32_NIL,
        SCS_VALUE_TYPE_float,
        SCS_TELEMETRY_CHANNEL_FLAG_none,
        Some(telemetry_log_float),
        std::ptr::null_mut(),
    );

    0
}

/// # Safety
///
/// where is your god now
#[no_mangle]
pub unsafe extern "C" fn telemetry_log_float(
    name: *const c_char,
    index: u32,
    value: *const scs_value_t,
    context: *mut c_void,
) {
    let name = CStr::from_ptr(name).to_string_lossy();
    let value = Value::from(*value);
    debug!(?name, index, ?value, ?context);
}

/// # Safety
///
/// where is your god now
#[no_mangle]
pub unsafe extern "C" fn scs_telemetry_shutdown() {
    debug!("shutting down");
}

/// # Safety
///
/// where is your god now
#[no_mangle]
pub unsafe extern "C" fn telemetry_frame_start(
    _event: u32,
    _event_info: *const c_void,
    _context: *mut c_void,
) {
    // info!("telemetry frame start");
}

/// # Safety
///
/// where is your god now
#[no_mangle]
pub unsafe extern "C" fn telemetry_frame_end(
    _event: u32,
    _event_info: *const c_void,
    _context: *mut c_void,
) {
    // info!("telemetry frame end");
}

/// # Safety
///
/// where is your god now
#[no_mangle]
pub unsafe extern "C" fn telemetry_paused(
    _event: u32,
    _event_info: *const c_void,
    _context: *mut c_void,
) {
    info!("telemetry paused");
}

/// # Safety
///
/// where is your god now
#[no_mangle]
pub unsafe extern "C" fn telemetry_started(
    _event: u32,
    _event_info: *const c_void,
    _context: *mut c_void,
) {
    info!("telemetry started");
}

/// # Safety
///
/// where is your god now
#[no_mangle]
pub unsafe extern "C" fn telemetry_configuration(
    _event: u32,
    event_info: *const c_void,
    _context: *mut c_void,
) {
    let configuration_event = event_info as *const scs_telemetry_configuration_t;
    let configuration_event = TelemetryConfigurationEvent::from(*configuration_event);
    let _attributes: HashMap<String, Value> = HashMap::from_iter(
        configuration_event
            .attributes
            .into_iter()
            .map(|attr| (attr.name.clone(), attr.value)),
    );
    // debug!("{:#?}", attributes);
}

/// # Safety
///
/// where is your god now
#[no_mangle]
pub unsafe extern "C" fn telemetry_gameplay_event(
    _event: u32,
    event_info: *const c_void,
    _context: *mut c_void,
) {
    let gameplay_event = event_info as *const scs_telemetry_gameplay_event_t;
    let gameplay_event = TelemetryGameplayEvent::from(*gameplay_event);
    debug!("{:#?}", gameplay_event);
}

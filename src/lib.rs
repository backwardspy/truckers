#![allow(non_upper_case_globals)]
use std::{
    backtrace::Backtrace,
    ffi::{c_char, c_void, CStr},
    sync::{Mutex, OnceLock},
};

use nice::{
    callbacks::register_for_event,
    telemetry::events::{ConfigurationEvent, GameplayEvent},
};
use tracing::{debug, error, info};
use tracing_subscriber::ETS2TracingSubscriber;

use truckers_scssdk_sys::*;

mod nice;
mod notnice;
mod tracing_subscriber;

pub(crate) static init_params: OnceLock<Mutex<scs_telemetry_init_params_v101_t>> = OnceLock::new();

/// # Safety
///
/// where is your god now
#[no_mangle]
pub unsafe extern "C" fn scs_telemetry_init(
    version: u32,
    params: *const scs_telemetry_init_params_v101_t,
) -> i32 {
    set_panic_hook();

    init_params
        .set(Mutex::new(*params))
        .expect("init was already called");

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

    register_for_event(&mut |event: ConfigurationEvent| {
        debug!(?event);
    });

    register_for_event(&mut |event: GameplayEvent| {
        debug!(?event);
    });

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
    // register_for_channel(Channel::Wheel(2), ChannelFlag::Always, &mut |speed: f32| {});

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
    // let name = CStr::from_ptr(name).to_string_lossy();
    // let value = Value::from(*value);
    // debug!(?name, index, ?value, ?context);
}

/// # Safety
///
/// where is your god now
#[no_mangle]
pub unsafe extern "C" fn scs_telemetry_shutdown() {
    debug!("shutting down");
}

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

fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let _ = std::fs::write(
            "truckers-panic.txt",
            format!(
                "{} {}\nStack:\n{}",
                chrono::Local::now(),
                info,
                Backtrace::force_capture()
            ),
        );

        // this is here because nyx wanted it
        #[cfg(windows)]
        unsafe {
            win::free_library(win::get_module_handle())
        };
    }));
}

use std::ffi::c_void;

use truckers_scssdk_sys::{
    scs_telemetry_configuration_t, scs_telemetry_frame_start_t, scs_telemetry_gameplay_event_t,
    SCS_TELEMETRY_EVENT_configuration, SCS_TELEMETRY_EVENT_frame_end,
    SCS_TELEMETRY_EVENT_frame_start, SCS_TELEMETRY_EVENT_gameplay, SCS_TELEMETRY_EVENT_paused,
    SCS_TELEMETRY_EVENT_started,
};

use crate::{
    init_params,
    nice::telemetry::events::{
        ConfigurationEvent, FrameEndEvent, GameplayEvent, PausedEvent, StartedEvent,
    },
};

use super::telemetry::events::{Event, FrameStartEvent};

unsafe extern "C" fn invoke_event(event: u32, event_info: *const c_void, context: *mut c_void) {
    unsafe fn callback<E: Event, C: Into<E> + Copy>(
        event_info: *const c_void,
        context: *mut c_void,
    ) {
        // see `register_for_event`
        let callback = context as *mut &mut dyn Fn(E);
        (*callback)((*(event_info as *const C)).into());
    }

    match event {
        SCS_TELEMETRY_EVENT_frame_start => {
            callback::<FrameStartEvent, scs_telemetry_frame_start_t>(event_info, context)
        }
        SCS_TELEMETRY_EVENT_frame_end => callback::<FrameEndEvent, ()>(event_info, context),
        SCS_TELEMETRY_EVENT_paused => callback::<PausedEvent, ()>(event_info, context),
        SCS_TELEMETRY_EVENT_started => callback::<StartedEvent, ()>(event_info, context),
        SCS_TELEMETRY_EVENT_configuration => {
            callback::<ConfigurationEvent, scs_telemetry_configuration_t>(event_info, context)
        }
        SCS_TELEMETRY_EVENT_gameplay => {
            callback::<GameplayEvent, scs_telemetry_gameplay_event_t>(event_info, context)
        }
        _ => todo!(),
    }
}

pub fn register_for_event<T: Event>(callback: &mut dyn Fn(T)) {
    let callback = Box::new(callback);
    unsafe {
        (init_params
            .get()
            .expect("init was not called")
            .lock()
            .expect("init was poisoned")
            .register_for_event
            .unwrap())(
            T::id(),
            Some(invoke_event),
            // don't ask, you have no idea how long this took to get right
            // please don't try and improve it
            Box::leak(callback) as *mut &mut dyn Fn(T) as *mut c_void,
        );
    }
}

pub fn unregister_from_event<T: Event>() {
    unsafe {
        (init_params
            .get()
            .expect("init was not called")
            .lock()
            .expect("init was poisoned")
            .unregister_from_event
            .unwrap())(T::id());
    }
}

pub fn register_for_channel() {}

pub fn unregister_from_channel() {}

use std::ffi::{CStr};

use truckers_scssdk_sys::{
    scs_named_value_t, scs_telemetry_configuration_t, scs_telemetry_frame_start_t,
    scs_telemetry_gameplay_event_t, SCS_TELEMETRY_EVENT_configuration,
    SCS_TELEMETRY_EVENT_frame_end, SCS_TELEMETRY_EVENT_frame_start, SCS_TELEMETRY_EVENT_gameplay,
    SCS_TELEMETRY_EVENT_paused, SCS_TELEMETRY_EVENT_started,
};

use crate::nice::value::NamedValue;

pub trait Event {
    fn id() -> u32;
}

#[derive(Debug, Clone)]
pub struct FrameStartEvent {
    pub flags: u32,
    pub render_time: u64,
    pub simulation_time: u64,
    pub paused_simulation_time: u64,
}

impl Event for FrameStartEvent {
    fn id() -> u32 {
        SCS_TELEMETRY_EVENT_frame_start
    }
}

impl From<scs_telemetry_frame_start_t> for FrameStartEvent {
    fn from(value: scs_telemetry_frame_start_t) -> Self {
        Self {
            flags: value.flags,
            render_time: value.render_time,
            simulation_time: value.simulation_time,
            paused_simulation_time: value.paused_simulation_time,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FrameEndEvent;

impl Event for FrameEndEvent {
    fn id() -> u32 {
        SCS_TELEMETRY_EVENT_frame_end
    }
}

impl From<()> for FrameEndEvent {
    fn from(_: ()) -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PausedEvent;

impl Event for PausedEvent {
    fn id() -> u32 {
        SCS_TELEMETRY_EVENT_paused
    }
}

impl From<()> for PausedEvent {
    fn from(_: ()) -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct StartedEvent;

impl Event for StartedEvent {
    fn id() -> u32 {
        SCS_TELEMETRY_EVENT_started
    }
}

impl From<()> for StartedEvent {
    fn from(_: ()) -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct ConfigurationEvent {
    id: String,
    attributes: Vec<NamedValue>,
}

impl Event for ConfigurationEvent {
    fn id() -> u32 {
        SCS_TELEMETRY_EVENT_configuration
    }
}

impl From<scs_telemetry_configuration_t> for ConfigurationEvent {
    fn from(value: scs_telemetry_configuration_t) -> Self {
        Self {
            id: unsafe { CStr::from_ptr(value.id) }
                .to_string_lossy()
                .into_owned(),
            attributes: attributes_iter(value.attributes).collect(),
        }
    }
}

#[derive(Debug)]
pub struct GameplayEvent {
    id: String,
    attributes: Vec<NamedValue>,
}

impl Event for GameplayEvent {
    fn id() -> u32 {
        SCS_TELEMETRY_EVENT_gameplay
    }
}

impl From<scs_telemetry_gameplay_event_t> for GameplayEvent {
    fn from(value: scs_telemetry_gameplay_event_t) -> Self {
        Self {
            id: unsafe { CStr::from_ptr(value.id) }
                .to_string_lossy()
                .into_owned(),
            attributes: attributes_iter(value.attributes).collect(),
        }
    }
}

fn attributes_iter(attributes: *const scs_named_value_t) -> impl Iterator<Item = NamedValue> {
    let mut idx = 0;
    std::iter::from_fn(move || {
        let attribute = unsafe { attributes.offset(idx) };
        if unsafe { (*attribute).name.is_null() } {
            None
        } else {
            idx += 1;
            Some(NamedValue::from(unsafe { *attribute }))
        }
    })
}

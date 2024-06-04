use std::ffi::CStr;

use truckers_scssdk_sys::{
    scs_named_value_t, scs_telemetry_configuration_t, scs_telemetry_gameplay_event_t,
};

use crate::value::NamedValue;

#[derive(Debug)]
pub struct TelemetryConfigurationEvent {
    pub id: String,
    pub attributes: Vec<NamedValue>,
}

#[derive(Debug)]
pub struct TelemetryGameplayEvent {
    pub id: String,
    pub attributes: Vec<NamedValue>,
}

impl From<scs_telemetry_configuration_t> for TelemetryConfigurationEvent {
    fn from(t: scs_telemetry_configuration_t) -> Self {
        unsafe {
            let id = CStr::from_ptr(t.id).to_string_lossy().into_owned();
            let attributes = attributes_iter(t.attributes).collect();
            Self { id, attributes }
        }
    }
}

impl From<scs_telemetry_gameplay_event_t> for TelemetryGameplayEvent {
    fn from(t: scs_telemetry_gameplay_event_t) -> Self {
        unsafe {
            let id = CStr::from_ptr(t.id).to_string_lossy().into_owned();
            let attributes = attributes_iter(t.attributes).collect();
            Self { id, attributes }
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

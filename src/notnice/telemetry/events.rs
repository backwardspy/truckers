use crate::nice::value::NamedValue;

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

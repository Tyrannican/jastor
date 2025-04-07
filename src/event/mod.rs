use crate::error::JastorError;
pub mod flags;
pub mod types;
pub use types::*;

pub trait Event {
    fn event_type(&self) -> EventType;
    fn timestamp<'a>(&'a self) -> &'a str;
}

// TODO:
// Base event (first X params)
// Subevents (DamageEvent, MissedEvent, HealEvent, HealAbsorbEvent, etc.)

pub fn parse_event(line: String) -> Result<(), JastorError> {
    let Some((ts, event)) = line.split_once("  ") else {
        return Err(JastorError::ParseError(format!(
            "expected timestamp with 2 spaces - got {line}"
        )));
    };

    let Some((event_type, args)) = event.split_once(',') else {
        return Err(JastorError::ParseError(format!(
            "expected event type to be present - got {event}"
        )));
    };

    let event = EventType::from_str(&event_type);

    println!("received event: {event:?}");

    match event {
        EventType::UnknownEvent(ref e) => {
            return Err(JastorError::ParseError(format!(
                "unknown event type encountered: {e}"
            )));
        }
        _ => {}
    };

    Ok(())
}

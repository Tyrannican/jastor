pub mod error;
pub mod event;

use error::JastorError;
use event::*;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Default)]
pub struct JastorParser {
    size: usize,
}

impl JastorParser {
    pub fn parse(&mut self, infile: impl AsRef<Path>) -> Result<(), JastorError> {
        let fh =
            File::open(infile.as_ref()).map_err(|e| JastorError::FileReadError(e.to_string()))?;
        let reader = BufReader::new(fh);

        for line in reader.lines() {
            let line = line.map_err(|e| JastorError::ParseError(e.to_string()))?;
            self.parse_event(&line)?;
            self.size += 1;
        }

        Ok(())
    }

    pub fn total_events(&self) -> usize {
        self.size
    }

    fn parse_event(&mut self, line: &str) -> Result<(), JastorError> {
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
        // Special Parsing logic for shorter events
        if !event.is_general_combat_event() {
            println!("{ts} {event_type} {args:?}\n");
        } else {
            // Parse General events
            // (Base Params, Prefix Params, Advanced Params, Suffix Params)
        }

        match event {
            // EventType::EnvironmentalDamage => {
            //     println!("{ts} {event_type} {args:?}\n");
            // }
            // EventType::DamageSplit => {
            //     println!("{ts} {event_type} {args:?}\n");
            // }
            EventType::UnknownEvent(ref e) => {
                return Err(JastorError::ParseError(format!(
                    "unknown event type encountered: {e}"
                )));
            }
            _ => {}
        };

        Ok(())
    }
}

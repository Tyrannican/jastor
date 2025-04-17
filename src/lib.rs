pub mod error;
pub mod event;
pub(crate) mod util;

use error::JastorError;
use event::{Event, parser::EventParser};

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Default)]
pub struct CombatLogParser {
    size: usize,
    events: Vec<Event>,
}

impl CombatLogParser {
    pub fn parse(&mut self, infile: impl AsRef<Path>) -> Result<(), JastorError> {
        let fh =
            File::open(infile.as_ref()).map_err(|e| JastorError::FileReadError(e.to_string()))?;
        let reader = BufReader::new(fh);

        for line in reader.lines() {
            let line = line.map_err(|e| JastorError::ParseError(e.to_string()))?;
            let parser = EventParser::new(&line)?;
            if parser.skip() {
                continue;
            }

            self.events.push(parser.parse()?);
            self.size += 1;
        }

        Ok(())
    }

    pub fn total_events(&self) -> usize {
        self.size
    }
}

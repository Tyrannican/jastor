use eyre::{Result, eyre};
use jastor::event::{CombatEvent, EventType};
use jiff::{civil::DateTime, fmt::strtime};
use std::io::{BufRead, BufReader};

struct EventLogParser<R: BufRead> {
    reader: R,
}

impl<R: BufRead> EventLogParser<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn parse_event(&self, line: String) -> Result<CombatEvent> {
        let Some((ts, rest)) = line.split_once("  ") else {
            panic!("invalid event found (expected timestamp) - {line}");
        };

        // These are auto-generated strings from the WoW client
        // Any errors here means the actual log file is corrupted
        // We expect here as we assume they're untampered with
        let timestamp: DateTime = strtime::parse("%m/%d/%Y %H:%M:%S%.f", ts)
            .expect("a valid timestamp string")
            .to_datetime()
            .expect("a valid timestamp conversion");

        let Some((event, args)) = rest.split_once(',') else {
            return Err(eyre!("invalid event - expected argument list: {line}"));
        };

        let event_type = EventType::try_from(event)?;

        todo!()
    }
}

impl<R: BufRead> Iterator for EventLogParser<R> {
    type Item = Result<CombatEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => return None,
            Ok(_) => {}
            Err(e) => {
                eprintln!("error occurred parsing line - {}", e.to_string());
                return None;
            }
        }

        Some(self.parse_event(line))
    }
}

fn main() -> eyre::Result<()> {
    let paths = std::fs::read_dir("./logs")?;
    for path in paths {
        let entry = path?;
        let f = std::fs::File::open(entry.path())?;
        let reader = BufReader::new(f);
        let parser = EventLogParser::new(reader);

        for event in parser.into_iter() {
            todo!()
        }
    }

    Ok(())
}

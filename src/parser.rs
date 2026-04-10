use std::io::BufRead;

use crate::event::{
    Difficulty, Event, EventType, Guid, LogVersionEvent, MapChangeEvent, StaggerEvent,
    ZoneChangeEvent,
};
use eyre::{Context, Result, eyre};
use jiff::{civil::DateTime, fmt::strtime};

#[derive(Debug, Clone)]
pub struct ParsedEvent {
    timestamp: DateTime,
    event_type: EventType,
    event: Event,
}

pub struct EventLogParser<R: BufRead> {
    reader: R,
}

impl<R: BufRead> EventLogParser<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    fn parse_event(&self, line: String) -> Result<ParsedEvent> {
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

        let args = args.trim();
        let event_type = EventType::try_from(event)?;
        let event = match event_type {
            EventType::CombatLogVersion => Event::LogVersion(self.parse_header(args)?),
            EventType::ZoneChange => Event::ZoneChange(self.parse_zone_change(args)?),
            EventType::MapChange => Event::MapChange(self.parse_map_change(args)?),
            EventType::StaggerPrevented | EventType::StaggerClear => {
                Event::Stagger(self.parse_stagger_event(event_type, args)?)
            }
            _ => todo!("not implemented yet - {event_type}"),
        };

        Ok(ParsedEvent {
            timestamp,
            event_type,
            event,
        })
    }

    fn parse_header(&self, args: &str) -> Result<LogVersionEvent> {
        let args = args
            .split(',')
            .enumerate()
            .filter_map(|(i, value)| if i % 2 == 0 { Some(value) } else { None })
            .collect::<Vec<&str>>();

        assert!(args.len() >= 3);
        let version = args[0]
            .parse::<u32>()
            .wrap_err_with(|| format!("unable to extract combat log version: {}", args[0]))?;

        let advanced_log = args[1] == "1";
        let build = args[2].to_string();

        Ok(LogVersionEvent {
            version,
            advanced_log,
            build,
        })
    }

    fn parse_zone_change(&self, args: &str) -> Result<ZoneChangeEvent> {
        let args = args.split(',').collect::<Vec<&str>>();
        assert_eq!(args.len(), 3);

        let instance_id = args[0]
            .parse::<u32>()
            .wrap_err_with(|| format!("unable to parse instance id - {}", args[0]))?;
        let zone_name = args[1].trim_matches('"').to_string();
        let difficulty = args[2]
            .parse::<u16>()
            .wrap_err_with(|| format!("unable to parse difficulty ID - {}", args[2]))?;

        Ok(ZoneChangeEvent {
            instance_id,
            zone_name,
            difficulty: Difficulty::from(difficulty),
        })
    }

    fn parse_map_change(&self, args: &str) -> Result<MapChangeEvent> {
        let args = args.split(',').collect::<Vec<&str>>();
        assert_eq!(args.len(), 6);

        let map_id = args[0]
            .parse::<u32>()
            .wrap_err_with(|| format!("unable to parse map id: {}", args[0]))?;

        let map_name = args[1].trim_matches('"').to_string();

        let x0 = args[2]
            .parse::<f32>()
            .wrap_err_with(|| format!("unable to parse x0 value - {}", args[2]))?;
        let x1 = args[3]
            .parse::<f32>()
            .wrap_err_with(|| format!("unable to parse x1 value - {}", args[3]))?;
        let y0 = args[4]
            .parse::<f32>()
            .wrap_err_with(|| format!("unable to parse y0 value - {}", args[4]))?;
        let y1 = args[5]
            .parse::<f32>()
            .wrap_err_with(|| format!("unable to parse y1 value - {}", args[5]))?;

        Ok(MapChangeEvent {
            map_id,
            map_name,
            x0,
            x1,
            y0,
            y1,
        })
    }

    fn parse_stagger_event(&self, event_type: EventType, args: &str) -> Result<StaggerEvent> {
        let args = args.split(',').collect::<Vec<&str>>();
        let target_len = match event_type {
            EventType::StaggerPrevented => 3,
            EventType::StaggerClear => 2,
            _ => unreachable!("impossible as only those two are passed in"),
        };

        assert_eq!(args.len(), target_len);

        let guid = Guid(args[0].to_string());
        let (spell_id, amount): (Option<u32>, f32) = if event_type == EventType::StaggerPrevented {
            let spell_id =
                Some(args[1].parse::<u32>().wrap_err_with(|| {
                    format!("unable to parse spell id for stagger - {}", args[1])
                })?);
            let amount = args[2]
                .parse::<f32>()
                .wrap_err_with(|| format!("unable to parse stagger amount - {}", args[2]))?;

            (spell_id, amount)
        } else {
            (
                None,
                args[1]
                    .parse::<f32>()
                    .wrap_err_with(|| format!("unable to parse stagger amount - {}", args[2]))?,
            )
        };

        Ok(StaggerEvent {
            guid,
            spell_id,
            amount,
        })
    }
}

impl<R: BufRead> Iterator for EventLogParser<R> {
    type Item = Result<ParsedEvent>;

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

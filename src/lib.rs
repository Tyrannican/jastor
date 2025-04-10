pub mod error;
pub mod event;
pub(crate) mod util;

use error::JastorError;
use event::*;
use flags::Difficulty;
use util::param_handler::ParamHandler;

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

        let event = EventType::from_str(&event_type)?;
        if event.skip() {
            return Ok(());
        }

        if event.has_short_parameters() {
            let event = self.parse_short_event(event, args)?;
            self.events.push(event);
            return Ok(());
        }
        // Advanced parameter fields:
        // 1. GUID
        // 2. Owner GUID (00000000000000000)
        // 3. Current HP
        // 4. Max HP
        // 5. Attack Power
        // 6. Spell Power
        // 7 ? - Armor apparently but no dice
        // 8. ? - Absorb shield
        // 9. ?
        // 10. ?
        // 11. Power Type
        // 12. Current Power
        // 13. Max Power
        // 14. Power Cost
        // 15. X coord
        // 16. Y Coord
        // 17. Map ID
        // 18. Facing Direction
        // 19. Level (NPC) or iLvl (Player)
        //
        // Only Need GUID -> Max HP and Current Power -> Level

        Ok(())
    }

    fn parse_short_event(
        &mut self,
        event_type: EventType,
        args: &str,
    ) -> Result<Event, JastorError> {
        let handler = ParamHandler::new(args);

        println!("Handling event: {event_type}");
        match event_type {
            EventType::CombatLogVersion => {
                let version = handler.as_number::<u8>(0)?;
                let build = handler.as_string(4)?;

                return Ok(Event::CombatLogVersion { version, build });
            }
            EventType::ZoneChange => {
                let instance = handler.as_number::<usize>(0)?;
                let name = handler.as_string(1)?;
                let difficulty = Difficulty::from(handler.as_number::<u16>(2)?);

                return Ok(Event::ZoneChange {
                    id: instance,
                    name,
                    difficulty,
                });
            }
            EventType::MapChange => {
                let id = handler.as_number::<usize>(0)?;
                let name = handler.as_string(1)?;
                let x0 = handler.as_number::<f32>(2)?;
                let x1 = handler.as_number::<f32>(2)?;
                let y0 = handler.as_number::<f32>(2)?;
                let y1 = handler.as_number::<f32>(2)?;

                return Ok(Event::MapChange {
                    id,
                    name,
                    x0,
                    x1,
                    y0,
                    y1,
                });
            }
            EventType::StaggerClear => {
                let guid = handler.as_string(0)?;
                let value = handler.as_number::<f32>(1)?;
                return Ok(Event::StaggerClear { guid, value });
            }
            EventType::EncounterStart => {
                let id = handler.as_number::<usize>(0)?;
                let name = handler.as_string(1)?;
                let difficulty = Difficulty::from(handler.as_number::<u16>(2)?);
                let size = handler.as_number::<usize>(3)?;
                let instance = handler.as_number::<usize>(4)?;

                return Ok(Event::EncounterStart {
                    id,
                    name,
                    difficulty,
                    size,
                    instance,
                });
            }
            EventType::EncounterEnd => {
                let id = handler.as_number::<usize>(0)?;
                let name = handler.as_string(1)?;
                let difficulty = Difficulty::from(handler.as_number::<u16>(2)?);
                let size = handler.as_number::<usize>(3)?;
                let success = match handler.as_number::<u8>(4)? {
                    1 => true,
                    _ => false,
                };
                let length = handler.as_number::<u64>(5)?;
                return Ok(Event::EncounterEnd {
                    id,
                    name,
                    difficulty,
                    size,
                    success,
                    length,
                });
            }
            EventType::ArenaMatchStart => {
                let id = handler.as_number::<usize>(0)?;
                let unk = handler.as_number::<usize>(1)?;
                let match_type = handler.as_string(2)?;
                let team = handler.as_number::<usize>(3)?;

                return Ok(Event::ArenaMatchStart {
                    id,
                    unk,
                    match_type,
                    team,
                });
            }
            EventType::ArenaMatchEnd => {
                let winner = handler.as_number::<usize>(0)?;
                let duration = handler.as_number::<u64>(1)?;
                let team_one_rating = handler.as_number::<usize>(2)?;
                let team_two_rating = handler.as_number::<usize>(3)?;

                return Ok(Event::ArenaMatchEnd {
                    winner,
                    duration,
                    team_one_rating,
                    team_two_rating,
                });
            }
            // Self::CombatLogVersion
            // | Self::StaggerClear
            // | Self::ArenaMatchStart
            // | Self::ArenaMatchEnd
            // | Self::EncounterStart
            // | Self::EncounterEnd
            // | Self::ChallengeModeStart
            // | Self::ChallengeModeEnd
            // | Self::WorldMarkerPlaced
            // | Self::WorldMarkerRemoved
            // | Self::MapChange
            // | Self::ZoneChange => true,
            _ => println!("{event_type} {args}"),
        }

        todo!("implement event type {event_type}")
    }
}

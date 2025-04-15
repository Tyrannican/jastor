pub mod error;
pub mod event;
pub(crate) mod util;

use error::JastorError;
use event::*;
use flags::*;
use util::param_handler::{ArgumentHandler, ParameterHandler};

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
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
            self.parse_single_event(&line)?;
            self.size += 1;
        }

        Ok(())
    }

    pub fn total_events(&self) -> usize {
        self.size
    }

    fn parse_single_event(&mut self, line: &str) -> Result<(), JastorError> {
        let Some((_, event)) = line.split_once("  ") else {
            return Err(JastorError::ParseError(format!(
                "expected timestamp with 2 spaces - got {line}"
            )));
        };

        let Some((event_type, args)) = event.split_once(',') else {
            return Err(JastorError::ParseError(format!(
                "expected event type to be present - got {event}"
            )));
        };

        let event = EventType::from_str(event_type)?;

        // TODO: Handle Combatant Info
        if event.skip() {
            return Ok(());
        }

        if event.has_short_parameters() {
            return self.parse_short_event(event, args);
        }

        self.parse_combat_event(event, args)
    }

    fn parse_combat_event(&mut self, event_type: EventType, args: &str) -> Result<(), JastorError> {
        let handler = ArgumentHandler::new(args);

        let base_params = handler.base_params()?;
        let prefix_params = handler.prefix_parameters(event_type)?;
        let advanced_params = handler.advanced_parameters(event_type)?;
        let suffix_params = handler.additional_parameters(event_type)?;

        let advanced_event = AdvancedParameters::parse(advanced_params)?;

        match event_type {
            EventType::SpellDamage
            | EventType::RangeDamage
            | EventType::SpellBuildingDamage
            | EventType::SpellPeriodicDamage
            | EventType::SwingDamage
            | EventType::SwingDamageLanded => {
                self.events.push(Event::Damage(DamageEvent::new(
                    event_type,
                    base_params,
                    advanced_event,
                    prefix_params,
                    suffix_params,
                )?));
            }
            // EventType::SpellMissed | EventType::RangeMissed | EventType::SwingMissed => {
            //     println!("{event_type} {base_params:?} {prefix_params:?} {suffix_params:?}");
            //     println!();
            // }
            EventType::SpellCastStart => {
                let src = Unit::parse(&base_params[..4])?;
                let target = Unit::parse(&base_params[4..]).ok();
                let spell = Spell::parse(prefix_params)?;
                let failure_reason = if !suffix_params.is_empty() {
                    Some(suffix_params[0].to_string())
                } else {
                    None
                };

                self.events.push(Event::SpellCast {
                    event_type,
                    src,
                    target,
                    spell,
                    failure_reason,
                });
            }
            _ => {}
        }

        Ok(())
    }

    fn parse_short_event(&mut self, event_type: EventType, args: &str) -> Result<(), JastorError> {
        let handler = ArgumentHandler::new(args);

        match event_type {
            EventType::CombatLogVersion => {
                let version = handler.as_number::<u8>(0)?;
                let build = handler.as_string(4)?;
                self.events.push(Event::CombatLogVersion { version, build });
            }
            EventType::ZoneChange => {
                let instance = handler.as_number::<usize>(0)?;
                let name = handler.as_string(1)?;
                let difficulty = Difficulty::from(handler.as_number::<u16>(2)?);

                self.events.push(Event::ZoneChange {
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

                self.events.push(Event::MapChange {
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
                self.events.push(Event::StaggerClear { guid, value });
            }
            EventType::EncounterStart => {
                let id = handler.as_number::<usize>(0)?;
                let name = handler.as_string(1)?;
                let difficulty = Difficulty::from(handler.as_number::<u16>(2)?);
                let size = handler.as_number::<usize>(3)?;
                let instance = handler.as_number::<usize>(4)?;

                self.events.push(Event::EncounterStart {
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
                let success = handler.boolean_flag(4)?;
                let length = handler.as_number::<u64>(5)?;
                self.events.push(Event::EncounterEnd {
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

                self.events.push(Event::ArenaMatchStart {
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

                self.events.push(Event::ArenaMatchEnd {
                    winner,
                    duration,
                    team_one_rating,
                    team_two_rating,
                });
            }
            EventType::ChallengeModeStart => {
                let name = handler.as_string(0)?;
                let id = handler.as_number::<usize>(1)?;
                let challenge_id = handler.as_number::<usize>(2)?;
                let keystone_level = handler.as_number::<usize>(3)?;
                let affix_list = serde_json::from_str::<Vec<u16>>(&handler.as_string(4)?)
                    .map_err(|e| JastorError::ParseError(e.to_string()))?;
                let affixes = affix_list
                    .into_iter()
                    .map(Affix::from)
                    .collect::<Vec<Affix>>();

                self.events.push(Event::ChallengeModeStart {
                    name,
                    id,
                    challenge_id,
                    keystone_level,
                    affixes,
                });
            }
            EventType::ChallengeModeEnd => {
                let id = handler.as_number::<usize>(0)?;
                let success = handler.boolean_flag(1)?;
                let keystone_level = handler.as_number::<usize>(2)?;
                let duration = handler.as_number::<u64>(3)?;

                self.events.push(Event::ChallengeModeEnd {
                    id,
                    success,
                    keystone_level,
                    duration,
                });
            }
            EventType::WorldMarkerPlaced => {
                let instance = handler.as_number::<usize>(0)?;
                let marker = RaidMarker::from(handler.as_number::<u8>(1)?);
                let x = handler.as_number::<f32>(2)?;
                let y = handler.as_number::<f32>(3)?;

                self.events.push(Event::WorldMarkerPlaced {
                    instance,
                    marker,
                    x,
                    y,
                });
            }
            EventType::WorldMarkerRemoved => {
                let marker = RaidMarker::from(handler.as_number::<u8>(0)?);
                self.events.push(Event::WorldMarkerRemoved { marker });
            }
            _ => println!("{event_type} {args}"),
        }

        Ok(())
    }
}

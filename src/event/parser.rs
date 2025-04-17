use crate::{
    error::JastorError,
    event::*,
    util::param_handler::{ArgumentHandler, ParameterHandler, SliceHander},
};

use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct EventParser {
    timestamp: i64,
    event_type: EventType,
    handler: ArgumentHandler,
}

impl EventParser {
    pub fn new(args: &str) -> Result<Self, JastorError> {
        let Some((timestamp, event)) = args.split_once("  ") else {
            return Err(JastorError::ParseError(format!(
                "expected timestamp with 2 spaces - got {args}"
            )));
        };

        let Some((event_type, args)) = event.split_once(',') else {
            return Err(JastorError::ParseError(format!(
                "expected event type to be present - got {event}"
            )));
        };

        let timestamp = chrono::NaiveDateTime::parse_from_str(timestamp, "%m/%d/%Y %H:%M:%S%.f")
            .map_err(|_| {
                JastorError::ParseError(format!("unable to parse timestamp string: {timestamp}"))
            })?;

        let event_type = EventType::from_str(event_type)?;

        Ok(Self {
            timestamp: timestamp.and_utc().timestamp(),
            event_type,
            handler: ArgumentHandler::new(args),
        })
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn skip(&self) -> bool {
        self.event_type.skip()
    }

    pub fn units(&self) -> Result<(Option<Unit>, Option<Unit>), JastorError> {
        let base = self.handler.base_params()?;
        Ok((Unit::parse(&base[..4]).ok(), Unit::parse(&base[4..]).ok()))
    }

    pub fn spell_prefix(&self) -> Result<Option<SpellInfo>, JastorError> {
        let prefix = self.handler.prefix_parameters(self.event_type)?;
        if prefix.is_empty() {
            return Ok(None);
        }

        Ok(Some(SpellInfo::parse(prefix)?))
    }

    pub fn advanced(&self) -> Result<Option<AdvancedParameters>, JastorError> {
        AdvancedParameters::parse(self.handler.advanced_parameters(self.event_type)?)
    }

    pub fn suffix(&self) -> Result<&[String], JastorError> {
        self.handler.suffix_parameters(self.event_type)
    }

    pub fn parse(&self) -> Result<Event, JastorError> {
        if self.event_type.is_special_event() {
            return self.special_event();
        }

        self.combat_event()
    }

    fn combat_event(&self) -> Result<Event, JastorError> {
        let advanced = self.advanced()?;
        let (source, target) = self.units()?;

        match self.event_type {
            EventType::SpellDamage
            | EventType::SpellPeriodicDamage
            | EventType::SpellBuildingDamage
            | EventType::RangeDamage
            | EventType::SwingDamage
            | EventType::DamageSplit
            | EventType::DamageShield
            | EventType::SwingDamageLanded => {
                let spell_info = self.spell_prefix()?;
                let handler = SliceHander::new(self.suffix()?);

                let amount = handler.as_number::<isize>(0)?;
                let base_amount = handler.as_number::<isize>(1)?;
                let overkill = handler.as_number::<isize>(2)?;
                let school = SpellSchool::from(handler.as_number::<u8>(3)?);
                let resisted = handler.as_number::<isize>(4)?;
                let blocked = handler.as_number::<isize>(5)?;
                let absorbed = handler.as_number::<isize>(6)?;
                let critical = handler.as_boolean(7)?;
                let glancing = handler.as_boolean(8)?;
                let is_offhand = handler.as_boolean(9)?;
                let damage_type = match self.event_type {
                    EventType::SpellDamage
                    | EventType::SpellPeriodicDamage
                    | EventType::RangeDamage => Some(DamageType::from_str(handler.raw(10)?)?),
                    _ => None,
                };

                return Ok(Event::Damage {
                    source,
                    target,
                    advanced,
                    spell_info,
                    amount,
                    base_amount,
                    overkill,
                    school,
                    resisted,
                    blocked,
                    absorbed,
                    critical,
                    glancing,
                    is_offhand,
                    damage_type,
                });
            }
            EventType::SpellMissed
            | EventType::SpellPeriodicMissed
            | EventType::SpellBuildingMissed
            | EventType::DamageShieldMissed
            | EventType::RangeMissed
            | EventType::SwingMissed => {
                let spell_info = self.spell_prefix()?;
                let handler = SliceHander::new(self.suffix()?);
                let miss_type = MissType::from_str(handler.raw(0)?)?;
                let is_offhand = handler.as_boolean(1)?;
                let (amount, total_amount, critical) = match miss_type {
                    MissType::Absorb => {
                        let amount = handler.as_number::<isize>(2)?;
                        let total_amount = handler.as_number::<isize>(3)?;
                        let critical = handler.as_boolean(4)?;

                        (amount, total_amount, critical)
                    }
                    _ => (0, 0, false),
                };

                return Ok(Event::Miss {
                    source,
                    target,
                    spell_info,
                    advanced,
                    miss_type,
                    is_offhand,
                    amount,
                    total_amount,
                    critical,
                });
            }
            _ => {}
        }
        Ok(Event::Placeholder)
    }

    fn special_event(&self) -> Result<Event, JastorError> {
        match self.event_type {
            EventType::CombatLogVersion => {
                let version = self.handler.as_number::<u8>(0)?;
                let build = self.handler.as_string(4)?;
                Ok(Event::CombatLogVersion { version, build })
            }
            EventType::ZoneChange => {
                let instance = self.handler.as_number::<usize>(0)?;
                let name = self.handler.as_string(1)?;
                let difficulty = Difficulty::from(self.handler.as_number::<u16>(2)?);

                Ok(Event::ZoneChange {
                    id: instance,
                    name,
                    difficulty,
                })
            }
            EventType::MapChange => {
                let id = self.handler.as_number::<usize>(0)?;
                let name = self.handler.as_string(1)?;
                let x0 = self.handler.as_number::<f32>(2)?;
                let x1 = self.handler.as_number::<f32>(2)?;
                let y0 = self.handler.as_number::<f32>(2)?;
                let y1 = self.handler.as_number::<f32>(2)?;

                Ok(Event::MapChange {
                    id,
                    name,
                    x0,
                    x1,
                    y0,
                    y1,
                })
            }
            EventType::StaggerClear => {
                let guid = self.handler.as_string(0)?;
                let value = self.handler.as_number::<f32>(1)?;
                Ok(Event::StaggerClear { guid, value })
            }
            EventType::EncounterStart => {
                let id = self.handler.as_number::<usize>(0)?;
                let name = self.handler.as_string(1)?;
                let difficulty = Difficulty::from(self.handler.as_number::<u16>(2)?);
                let size = self.handler.as_number::<usize>(3)?;
                let instance = self.handler.as_number::<usize>(4)?;

                Ok(Event::EncounterStart {
                    id,
                    name,
                    difficulty,
                    size,
                    instance,
                })
            }
            EventType::EncounterEnd => {
                let id = self.handler.as_number::<usize>(0)?;
                let name = self.handler.as_string(1)?;
                let difficulty = Difficulty::from(self.handler.as_number::<u16>(2)?);
                let size = self.handler.as_number::<usize>(3)?;
                let success = self.handler.as_boolean(4)?;
                let length = self.handler.as_number::<u64>(5)?;
                Ok(Event::EncounterEnd {
                    id,
                    name,
                    difficulty,
                    size,
                    success,
                    length,
                })
            }
            EventType::ArenaMatchStart => {
                let id = self.handler.as_number::<usize>(0)?;
                let unk = self.handler.as_number::<usize>(1)?;
                let match_type = self.handler.as_string(2)?;
                let team = self.handler.as_number::<usize>(3)?;

                Ok(Event::ArenaMatchStart {
                    id,
                    unk,
                    match_type,
                    team,
                })
            }
            EventType::ArenaMatchEnd => {
                let winner = self.handler.as_number::<usize>(0)?;
                let duration = self.handler.as_number::<u64>(1)?;
                let team_one_rating = self.handler.as_number::<usize>(2)?;
                let team_two_rating = self.handler.as_number::<usize>(3)?;

                Ok(Event::ArenaMatchEnd {
                    winner,
                    duration,
                    team_one_rating,
                    team_two_rating,
                })
            }
            EventType::ChallengeModeStart => {
                let name = self.handler.as_string(0)?;
                let id = self.handler.as_number::<usize>(1)?;
                let challenge_id = self.handler.as_number::<usize>(2)?;
                let keystone_level = self.handler.as_number::<usize>(3)?;
                let affix_list = serde_json::from_str::<Vec<u16>>(&self.handler.as_string(4)?)
                    .map_err(|e| JastorError::ParseError(e.to_string()))?;
                let affixes = affix_list
                    .into_iter()
                    .map(Affix::from)
                    .collect::<Vec<Affix>>();

                Ok(Event::ChallengeModeStart {
                    name,
                    id,
                    challenge_id,
                    keystone_level,
                    affixes,
                })
            }
            EventType::ChallengeModeEnd => {
                let id = self.handler.as_number::<usize>(0)?;
                let success = self.handler.as_boolean(1)?;
                let keystone_level = self.handler.as_number::<usize>(2)?;
                let duration = self.handler.as_number::<u64>(3)?;

                Ok(Event::ChallengeModeEnd {
                    id,
                    success,
                    keystone_level,
                    duration,
                })
            }
            EventType::WorldMarkerPlaced => {
                let instance = self.handler.as_number::<usize>(0)?;
                let marker = RaidMarker::from(self.handler.as_number::<u8>(1)?);
                let x = self.handler.as_number::<f32>(2)?;
                let y = self.handler.as_number::<f32>(3)?;

                Ok(Event::WorldMarkerPlaced {
                    instance,
                    marker,
                    x,
                    y,
                })
            }
            EventType::WorldMarkerRemoved => {
                let marker = RaidMarker::from(self.handler.as_number::<u8>(0)?);
                Ok(Event::WorldMarkerRemoved { marker })
            }
            EventType::CombatantInfo => {
                println!("{:?}", self.handler.params);
                todo!()
            }
            _ => Err(JastorError::InvalidSpecialEvent(
                self.event_type.to_string(),
            )),
        }
    }
}

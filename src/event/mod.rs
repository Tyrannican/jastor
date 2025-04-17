pub mod flags;
pub mod raw;

use flags::*;
pub use raw::*;

use crate::{
    error::JastorError,
    util::param_handler::{ArgumentHandler, ParameterHandler, SliceHander},
};

use std::str::FromStr;

// Damage Event Parameters
// amount, base_amount, overkill, school, resisted, blocked, absorbed, critical, glancing,
// is_offhand

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

    pub fn skip(&self) -> bool {
        self.event_type.skip()
    }

    pub fn base(&self) -> Result<(Option<Unit>, Option<Unit>), JastorError> {
        let base = self.handler.base_params()?;
        Ok((Unit::parse(&base[..4]).ok(), Unit::parse(&base[4..]).ok()))
    }

    pub fn prefix(&self) -> Result<Option<Spell>, JastorError> {
        let prefix = self.handler.prefix_parameters(self.event_type)?;
        if prefix.is_empty() {
            return Ok(None);
        }

        Ok(Some(Spell::parse(prefix)?))
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
                let success = self.handler.boolean_flag(4)?;
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
                let success = self.handler.boolean_flag(1)?;
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

#[derive(Debug)]
pub enum Event {
    // Special Events
    CombatLogVersion {
        version: u8,
        build: String,
    },
    ZoneChange {
        id: usize,
        name: String,
        difficulty: Difficulty,
    },
    MapChange {
        id: usize,
        name: String,
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
    },
    StaggerClear {
        guid: String,
        value: f32,
    },
    EncounterStart {
        id: usize,
        name: String,
        difficulty: Difficulty,
        size: usize,
        instance: usize,
    },
    EncounterEnd {
        id: usize,
        name: String,
        difficulty: Difficulty,
        size: usize,
        success: bool,
        length: u64,
    },
    ArenaMatchStart {
        id: usize,
        unk: usize,
        match_type: String, // TODO: <- Figure out these (e.g. Skirmish)
        team: usize,
    },
    ArenaMatchEnd {
        winner: usize,
        duration: u64,
        team_one_rating: usize,
        team_two_rating: usize,
    },
    ChallengeModeStart {
        name: String,
        id: usize,
        challenge_id: usize,
        keystone_level: usize,
        affixes: Vec<Affix>,
    },
    ChallengeModeEnd {
        id: usize,
        success: bool,
        keystone_level: usize,
        duration: u64,
    },
    WorldMarkerPlaced {
        instance: usize,
        marker: RaidMarker,
        x: f32,
        y: f32,
    },
    WorldMarkerRemoved {
        marker: RaidMarker,
    },
    Placeholder,
}

#[derive(Debug, Clone)]
pub struct AdvancedParameters {
    guid: String,
    owner: String,
    current_hp: usize,
    max_hp: usize,
    current_power: usize,
    max_power: usize,
    power_cost: usize,
    x: f32,
    y: f32,
    map_id: usize,
    direction: f32,
    level: usize,
}

impl AdvancedParameters {
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
    pub fn parse(params: Option<&[String]>) -> Result<Option<Self>, JastorError> {
        let Some(params) = params else {
            return Ok(None);
        };

        let handler = SliceHander::new(params);
        let (_, current_power) = handler.as_multi_value_number::<usize>(11)?;
        let (_, max_power) = handler.as_multi_value_number::<usize>(11)?;
        let (_, power_cost) = handler.as_multi_value_number::<usize>(11)?;

        Ok(Some(Self {
            guid: handler.as_string(0)?,
            owner: handler.as_string(1)?,
            current_hp: handler.as_number::<usize>(2)?,
            max_hp: handler.as_number::<usize>(3)?,
            current_power,
            max_power,
            power_cost,
            x: handler.as_number::<f32>(14)?,
            y: handler.as_number::<f32>(15)?,
            map_id: handler.as_number::<usize>(16)?,
            direction: handler.as_number::<f32>(17)?,
            level: handler.as_number::<usize>(18)?,
        }))
    }
}

#[derive(Debug, Clone)]
pub struct Unit {
    guid: String,
    name: String,
    flags: UnitFlag,
    raid_flags: RaidMarker,
}

impl Unit {
    pub fn parse(params: &[String]) -> Result<Self, JastorError> {
        let handler = SliceHander::new(params);
        let guid = handler.as_string(0)?;
        let name = handler.as_string(1)?;
        let flags = UnitFlag::parse(handler.as_number::<u32>(2)?)?;
        let raid_flags = RaidMarker::parse_flag(handler.as_number::<u32>(3)?);

        Ok(Self {
            guid,
            name,
            flags,
            raid_flags,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Spell {
    id: usize,
    name: String,
    school: SpellSchool,
}

impl Spell {
    pub fn parse(input: &[String]) -> Result<Self, JastorError> {
        let handler = SliceHander::new(input);
        let id = handler.as_number::<usize>(0)?;
        let name = handler.as_string(1)?;
        let school = SpellSchool::from(handler.as_number::<u8>(2)?);

        Ok(Self { id, name, school })
    }
}

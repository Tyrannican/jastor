pub mod flags;
pub mod raw;

use flags::*;
pub use raw::*;

use crate::{
    ArgumentHandler,
    error::JastorError,
    util::param_handler::{ParameterHandler, SliceHander},
};

#[derive(Debug, Clone)]
pub struct EventParser {
    event_type: EventType,
    handler: ArgumentHandler,
}

impl EventParser {
    pub fn new(event_type: EventType, args: &str) -> Self {
        Self {
            event_type,
            handler: ArgumentHandler::new(args),
        }
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

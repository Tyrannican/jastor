pub mod flags;
pub mod raw;

use std::str::FromStr;

use flags::*;
pub use raw::*;

use crate::{
    error::JastorError,
    util::param_handler::{ParameterHandler, SliceHander},
};

#[derive(Debug)]
pub enum Event {
    Damage(DamageEvent),
    SpellCast {
        event_type: EventType,
        src: Unit,
        target: Option<Unit>,
        spell: Spell,
        failure_reason: Option<String>,
    },

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
pub struct DamageEvent {
    event_type: EventType,
    src: Unit,
    target: Unit,
    spell: Option<Spell>,
    advanced_parameters: Option<AdvancedParameters>,
    amount: isize,
    base_amount: isize,
    overkill: isize,
    school: SpellSchool,
    resisted: isize,
    blocked: isize,
    absorbed: isize,
    critical: bool,
    is_offhand: bool,
    damage_type: Option<DamageType>,
}

impl DamageEvent {
    pub fn new(
        event_type: EventType,
        base: &[String],
        advanced_parameters: Option<AdvancedParameters>,
        prefix: &[String],
        suffix: &[String],
    ) -> Result<Self, JastorError> {
        let src = Unit::parse(&base[..4])?;
        let target = Unit::parse(&base[4..])?;
        let spell = if !prefix.is_empty() {
            Some(Spell::parse(prefix)?)
        } else {
            None
        };

        let handler = SliceHander::new(suffix);
        let amount = handler.as_number::<isize>(0)?;
        let base_amount = handler.as_number::<isize>(1)?;
        let overkill = handler.as_number::<isize>(2)?;
        let school = SpellSchool::from(handler.as_number::<u8>(3)?);
        let resisted = handler.as_number::<isize>(4)?;
        let blocked = handler.as_number::<isize>(5)?;
        let absorbed = handler.as_number::<isize>(6)?;
        let critical = handler.boolean_flag(7)?;

        let (is_offhand, damage_type) = if event_type == EventType::SwingDamage {
            (handler.boolean_flag(handler.len() - 1)?, None)
        } else {
            let last_item = handler.as_string(handler.len() - 1)?;
            DamageType::from_str(&last_item)?;
            (
                false,
                Some(DamageType::from_str(
                    &handler.as_string(handler.len() - 1)?,
                )?),
            )
        };

        Ok(Self {
            event_type,
            src,
            target,
            spell,
            advanced_parameters,
            amount,
            base_amount,
            overkill,
            school,
            resisted,
            blocked,
            absorbed,
            critical,
            is_offhand,
            damage_type,
        })
    }
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

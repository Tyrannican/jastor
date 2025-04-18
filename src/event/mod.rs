pub mod flags;
pub mod parser;
pub mod raw;

use flags::*;
pub use raw::*;

use crate::{
    error::JastorError,
    util::param_handler::{ParameterHandler, SliceHander},
};

#[derive(Debug)]
pub enum Event {
    // amount, base_amount, overkill, school, resisted, blocked, absorbed, critical, glancing,
    // is_offhand
    Damage {
        source: Option<Unit>,
        target: Option<Unit>,
        advanced: Option<AdvancedParameters>,
        spell_info: Option<SpellInfo>,
        amount: isize,
        base_amount: isize,
        overkill: isize,
        school: SpellSchool,
        resisted: isize,
        blocked: isize,
        absorbed: isize,
        critical: bool,
        glancing: bool,
        is_offhand: bool,
        damage_type: Option<DamageType>,
        support_guid: Option<String>,
    },
    Miss {
        source: Option<Unit>,
        target: Option<Unit>,
        spell_info: Option<SpellInfo>,
        advanced: Option<AdvancedParameters>,
        miss_type: MissType,
        is_offhand: bool,
        amount: isize,
        total_amount: isize,
        critical: bool,
    },
    Heal {
        source: Option<Unit>,
        target: Option<Unit>,
        spell_info: Option<SpellInfo>,
        advanced: Option<AdvancedParameters>,
        amount: isize,
        base_amount: isize,
        overhealing: isize,
        absorbed: isize,
        critical: bool,
        support_guid: Option<String>,
    },
    Absorb {
        source: Option<Unit>,
        target: Option<Unit>,
        spell_info: Option<SpellInfo>,
        advanced: Option<AdvancedParameters>,
        caster: Option<Unit>,
        absorbed_spell: Option<SpellInfo>,
        amount: isize,
        total_amount: isize,
        critical: bool,
    },

    HealAbsorb {
        source: Option<Unit>,
        target: Option<Unit>,
        spell_info: Option<SpellInfo>,
        advanced: Option<AdvancedParameters>,
        extra_unit: Option<Unit>,
        extra_spell_info: Option<SpellInfo>,
        absorbed_amount: isize,
        total_amount: isize,
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
pub struct SpellInfo {
    id: usize,
    name: String,
    school: SpellSchool,
}

impl SpellInfo {
    pub fn parse(input: &[String]) -> Result<Self, JastorError> {
        let handler = SliceHander::new(input);
        let id = handler.as_number::<usize>(0)?;
        let name = handler.as_string(1)?;
        let school = SpellSchool::from(handler.as_number::<u8>(2)?);

        Ok(Self { id, name, school })
    }
}

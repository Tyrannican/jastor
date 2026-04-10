use crate::types::{Faction, Guid, Specialization};

pub type Gem = (u32, u32);
pub type Enchantment = (u32, u32, u32);
pub type PvpTalents = (u32, u32, u32, u32);

#[derive(Debug, Clone)]
pub struct Combatant {
    guid: Guid,
    faction: Faction,
    stats: Stats,
    spec: Specialization,
    talents: Vec<Talent>,
    pvp_talents: PvpTalents,
    equipment: Vec<Equipment>,
    auras: Vec<TrackedAura>,
    pvp_stats: PvpStats,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Stats {
    strength: u32,
    agility: u32,
    stamina: u32,
    intelligence: u32,
    dodge: u32,
    parry: u32,
    block: u32,
    crit_melee: u32,
    crit_ranged: u32,
    crit_spell: u32,
    speed: u32,
    lifesteal: u32,
    haste_melee: u32,
    haste_ranged: u32,
    haste_spell: u32,
    avoidance: u32,
    mastery: u32,
    versatility_damage: u32,
    versatility_healing: u32,
    versatility_damage_taken: u32,
    armor: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PvpStats {
    honor_level: u32,
    season: u32,
    rating: u32,
    tier: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Talent {
    node_id: u32,
    entry_id: u32,
    rank: u32,
}

#[derive(Debug, Clone)]
pub struct Equipment {
    item_id: u32,
    item_level: u32,
    enchantment: Enchantment,
    bonus_id: Vec<u32>,
    gems: Vec<Gem>,
}

#[derive(Debug, Clone)]
pub struct TrackedAura {
    caster: Guid,
    spell_id: u32,
    stacks: u32,
}

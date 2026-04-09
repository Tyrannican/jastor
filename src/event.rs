use jiff::civil::DateTime;

pub use crate::types::{
    AuraType, EnvironmentalType, EventType, MissType, PowerType, RaidFlag, SpellSchool, UnitFlags,
};

#[derive(Debug, Clone)]
pub struct CombatEvent {
    timestamp: DateTime,
    event_type: EventType,
    src: Option<Target>,
    dst: Option<Target>,
    spell: Option<SpellParameters>,
    environmental: Option<EnvironmentalType>,
    adv: Option<AdvancedParameters>,
    suffix: Option<Suffix>,
}

#[derive(Debug, Clone)]
pub struct Guid(String);

#[derive(Debug, Clone)]
pub struct Target {
    guid: Guid,
    name: String,
    unit_flags: UnitFlags,
    raid_flags: RaidFlag,
}

#[derive(Debug, Clone)]
pub struct BaseParameters {
    src: Option<Target>,
    dst: Option<Target>,
}

#[derive(Debug, Clone)]
pub struct SpellParameters {
    spell_id: u32,
    spell_name: String,
    school: SpellSchool,
}

#[derive(Debug, Clone)]
pub struct AdvancedParameters {
    info: Guid,
    owner: Guid,
    current_hp: u32,
    max_hp: u32,
    attack_power: u32,
    spell_power: u32,
    armor: u32,
    absorb: u32,
    power_type: PowerType,
    current_power: u32,
    max_power: u32,
    x: f32,
    y: f32,
    map_id: u32,
    facing: f32,
    level: u32,
}

#[derive(Debug, Clone)]
pub enum Suffix {
    Damage(DamageEvent),
    Missed(MissEvent),
    Heal(HealEvent),
    HealAbsorbed,
    Absorbed(AbsorbEvent),
    Energize(EnergizeEvent),
    Drain(DrainEvent),
    Leech(DrainEvent),
    Interrupt(StealEvent),
    Dispel(StealWithAuraEvent),
    DispelFailed(StealEvent),
    Stolen(StealWithAuraEvent),
    ExtraAttacks(u32),
    Aura(AuraEvent),
    AuraBroken(AuraType),
    AuraBrokenSpell(AuraWithSpellEvent),
    Empower(u32),
    Enchant(EnchantEvent),
}

#[derive(Debug, Clone)]
pub struct DamageEvent {
    amount: u32,
    base_amount: u32,
    overkill: u32,
    school: SpellSchool,
    resisted: u32,
    blocked: u32,
    absorbed: u32,
    critical: bool,
    glancing: bool,
    crushing: bool,
}

#[derive(Debug, Clone)]
pub struct MissEvent {
    miss_type: MissType,
    offhand: bool,
    amount: u32,
    critical: bool,
}

#[derive(Debug, Clone)]
pub struct HealEvent {
    amount: u32,
    base_amount: u32,
    overhealing: u32,
    absorbed: u32,
    critical: bool,
}

#[derive(Debug, Clone)]
pub struct AbsorbEvent {
    params: SpellParameters,
    amount: u32,
    critical: bool,
}

#[derive(Debug, Clone)]
pub struct EnergizeEvent {
    amount: u32,
    over_energize: u32,
    power: PowerType,
    max: u32,
}

#[derive(Debug, Clone)]
pub struct DrainEvent {
    amount: u32,
    power: PowerType,
    extra_amount: u32,
    max: u32,
}

#[derive(Debug, Clone)]
pub struct StealEvent {
    spell: u32,
    name: String,
    school: SpellSchool,
}

#[derive(Debug, Clone)]
pub struct StealWithAuraEvent {
    spell: u32,
    name: String,
    school: SpellSchool,
    aura: AuraType,
}

#[derive(Debug, Clone)]
pub struct AuraEvent {
    aura: AuraType,
    amount: u32,
}

#[derive(Debug, Clone)]
pub struct AuraWithSpellEvent {
    spell: SpellParameters,
    aura: AuraType,
}

#[derive(Debug, Clone)]
pub struct EnchantEvent {
    name: String,
    item_id: u32,
    item_name: String,
}

#[derive(Debug, Clone)]
pub struct EncounterStartEvent {
    encounter_id: u32,
    encounter_name: String,
    // TODO: Difficulty ID mapping
    difficulty: u32,
    group_size: u32,
    instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct EncounterEndEvent {
    encounter_id: u32,
    encounter_name: String,
    // TODO: Difficulty ID mapping
    difficulty: u32,
    group_size: u32,
    success: bool,
    fight_time: u32,
}

#[derive(Debug, Clone)]
pub struct WorldMarkerPlacedEvent {
    instance_id: u32,
    marker: RaidFlag,
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
pub struct WorldMarkerRemovedEvent(RaidFlag);

pub use crate::{
    player::Combatant,
    types::{
        AuraType, Difficulty, EnvironmentalType, EventType, Guid, MissType, MultiValue, PowerType,
        RaidFlag, SpellSchool, Target, UnitFlags,
    },
};

#[derive(Debug, Clone)]
pub enum Event {
    LogVersion(LogVersionEvent),
    Combat(CombatEvent),
    Stagger(StaggerEvent),
    Combatant(Combatant),
    EncounterStart(EncounterStartEvent),
    EncounterEnd(EncounterEndEvent),
    ArenaStart(ArenaStartEvent),
    ArenaEnd(ArenaEndEvent),
    WorldMarkerPlaced(WorldMarkerPlacedEvent),
    WorldMarkerRemoved(RaidFlag),
    ZoneChange(ZoneChangeEvent),
    MapChange(MapChangeEvent),
    Emote(EmoteEvent),
    Placeholder,
}

#[derive(Debug, Clone)]
pub struct LogVersionEvent {
    pub version: u32,
    pub advanced_log: bool,
    pub build: String,
}

#[derive(Debug, Clone)]
pub struct CombatEvent {
    pub src: Option<Target>,
    pub dst: Option<Target>,
    pub spell: Option<SpellParameters>,
    pub adv: Option<AdvancedParameters>,
    pub environmental: Option<EnvironmentalType>,
    pub suffix: Option<Suffix>,
}

#[derive(Debug, Clone)]
pub struct BaseParameters {
    src: Option<Target>,
    dst: Option<Target>,
}

#[derive(Debug, Clone)]
pub struct SpellParameters {
    pub spell_id: u32,
    pub spell_name: String,
    pub school: SpellSchool,
}

#[derive(Debug, Clone)]
pub struct AdvancedParameters {
    pub info: Guid,
    pub owner: Guid,
    pub current_hp: i32,
    pub max_hp: u32,
    pub attack_power: u32,
    pub spell_power: u32,
    // TODO: Determine what the missing two events are
    pub armor: i32,
    pub absorb: u32,

    pub power_type: MultiValue<PowerType>,
    pub current_power: MultiValue<u32>,
    pub max_power: MultiValue<u32>,
    pub power_cost: MultiValue<u32>,

    pub x: f32,
    pub y: f32,
    pub map_id: u32,
    pub facing: f32,
    pub level: u32,
}

#[derive(Debug, Clone)]
pub enum Suffix {
    Damage(DamageEvent),
    Missed(MissEvent),
    Heal(HealEvent),
    HealAbsorbed(HealAbsorbEvent),
    Fail(FailEvent),
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
    UnitDied(u32),
    UnitDestroyed(u32),
    UnitDissipates(u32),
}

#[derive(Debug, Clone)]
pub struct DamageEvent {
    pub amount: u32,
    pub base_amount: u32,
    pub overkill: u32,
    pub school: SpellSchool,
    pub resisted: u32,
    pub blocked: u32,
    pub absorbed: i32,
    pub critical: bool,
    pub glancing: bool,
    pub crushing: bool,
}

#[derive(Debug, Clone)]
pub struct FailEvent {
    pub msg: String,
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
    pub amount: u32,
    pub base_amount: u32,
    pub overhealing: u32,
    pub absorbed: u32,
    pub critical: bool,
}

#[derive(Debug, Clone)]
pub struct HealAbsorbEvent {
    pub extra: Target,
    pub spell: SpellParameters,
    pub absorbed: u32,
    pub total_absorbed: u32,
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
    pub aura: AuraType,
    pub amount: u32,
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
    pub encounter_id: u32,
    pub encounter_name: String,
    pub difficulty: Difficulty,
    pub group_size: u32,
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct EncounterEndEvent {
    pub encounter_id: u32,
    pub encounter_name: String,
    pub difficulty: Difficulty,
    pub group_size: u32,
    pub success: bool,
    pub fight_time: u32,
}

#[derive(Debug, Clone)]
pub struct ArenaStartEvent {
    instance_id: u32,
    unk: u32,
    match_type: String,
    team_id: u32,
}

#[derive(Debug, Clone)]
pub struct ArenaEndEvent {
    winning_team: bool,
    match_duration: u32,
    new_rating_team_one: u32,
    new_rating_team_two: u32,
}

#[derive(Debug, Clone)]
pub struct WorldMarkerPlacedEvent {
    instance_id: u32,
    marker: RaidFlag,
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
pub struct MapChangeEvent {
    pub map_id: u32,
    pub map_name: String,
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
}

#[derive(Debug, Clone)]
pub struct ZoneChangeEvent {
    pub instance_id: u32,
    pub zone_name: String,
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone)]
pub struct EmoteEvent;

#[derive(Debug, Clone)]
pub struct StaggerEvent {
    pub guid: Guid,
    pub spell_id: Option<u32>,
    pub amount: f32,
}

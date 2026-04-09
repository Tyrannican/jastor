use eyre::{Report, eyre};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EventType {
    SpellAuraAppliedDose,
    SpellInterrupt,
    PartyKill,
    StaggerPrevented,
    SpellAuraBroken,
    SpellAuraApplied,
    SwingDamage,
    SpellCastFailed,
    SpellDamageSupport,
    EncounterEnd,
    RangeDamage,
    MapChange,
    SpellAbsorbed,
    RangeDamageSupport,
    CombatLogVersion,
    SpellAuraRemoved,
    SpellPeriodicDamage,
    SpellResurrect,
    EnvironmentalDamage,
    SpellStolen,
    SpellAuraRefresh,
    SwingDamageLandedSupport,
    EnchantApplied,
    SpellDamage,
    SpellHealSupport,
    SpellAuraBrokenSpell,
    SpellDispel,
    SpellEmpowerInterrupt,
    SpellAbsorbedSupport,
    SpellEnergize,
    UnitDied,
    SpellPeriodicHeal,
    ZoneChange,
    SpellPeriodicHealSupport,
    SpellCreate,
    SpellEmpowerStart,
    SpellCastSuccess,
    DamageSplit,
    SpellSummon,
    CombatantInfo,
    SpellMissed,
    SpellHealAbsorbed,
    SpellCastStart,
    SpellExtraAttacks,
    SpellHeal,
    StaggerClear,
    SpellAuraRemovedDose,
    Emote,
    SpellDrain,
    EncounterStart,
    SpellPeriodicDamageSupport,
    SpellPeriodicMissed,
    SwingMissed,
    SpellPeriodicEnergize,
    SpellEmpowerEnd,
    RangeMissed,
    SwingDamageLanded,
    SpellInstakill,
    ArenaMatchStart,
    ArenaMatchEnd,
    ChallengeModeStart,
    ChallengeModeEnd,
    WorldMarkerPlaced,
    WorldMarkerRemoved,
}
impl TryFrom<&str> for EventType {
    type Error = Report;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SPELL_AURA_APPLIED_DOSE" => Ok(Self::SpellAuraAppliedDose),
            "SPELL_INTERRUPT" => Ok(Self::SpellInterrupt),
            "PARTY_KILL" => Ok(Self::PartyKill),
            "STAGGER_PREVENTED" => Ok(Self::StaggerPrevented),
            "SPELL_AURA_BROKEN" => Ok(Self::SpellAuraBroken),
            "SPELL_AURA_APPLIED" => Ok(Self::SpellAuraApplied),
            "SWING_DAMAGE" => Ok(Self::SwingDamage),
            "SPELL_CAST_FAILED" => Ok(Self::SpellCastFailed),
            "SPELL_DAMAGE_SUPPORT" => Ok(Self::SpellDamageSupport),
            "ENCOUNTER_END" => Ok(Self::EncounterEnd),
            "RANGE_DAMAGE" => Ok(Self::RangeDamage),
            "MAP_CHANGE" => Ok(Self::MapChange),
            "SPELL_ABSORBED" => Ok(Self::SpellAbsorbed),
            "RANGE_DAMAGE_SUPPORT" => Ok(Self::RangeDamageSupport),
            "COMBAT_LOG_VERSION" => Ok(Self::CombatLogVersion),
            "SPELL_AURA_REMOVED" => Ok(Self::SpellAuraRemoved),
            "SPELL_PERIODIC_DAMAGE" => Ok(Self::SpellPeriodicDamage),
            "SPELL_RESURRECT" => Ok(Self::SpellResurrect),
            "ENVIRONMENTAL_DAMAGE" => Ok(Self::EnvironmentalDamage),
            "SPELL_STOLEN" => Ok(Self::SpellStolen),
            "SPELL_AURA_REFRESH" => Ok(Self::SpellAuraRefresh),
            "SWING_DAMAGE_LANDED_SUPPORT" => Ok(Self::SwingDamageLandedSupport),
            "ENCHANT_APPLIED" => Ok(Self::EnchantApplied),
            "SPELL_DAMAGE" => Ok(Self::SpellDamage),
            "SPELL_HEAL_SUPPORT" => Ok(Self::SpellHealSupport),
            "SPELL_AURA_BROKEN_SPELL" => Ok(Self::SpellAuraBrokenSpell),
            "SPELL_DISPEL" => Ok(Self::SpellDispel),
            "SPELL_EMPOWER_INTERRUPT" => Ok(Self::SpellEmpowerInterrupt),
            "SPELL_ABSORBED_SUPPORT" => Ok(Self::SpellAbsorbedSupport),
            "SPELL_ENERGIZE" => Ok(Self::SpellEnergize),
            "UNIT_DIED" => Ok(Self::UnitDied),
            "SPELL_PERIODIC_HEAL" => Ok(Self::SpellPeriodicHeal),
            "ZONE_CHANGE" => Ok(Self::ZoneChange),
            "SPELL_PERIODIC_HEAL_SUPPORT" => Ok(Self::SpellPeriodicHealSupport),
            "SPELL_CREATE" => Ok(Self::SpellCreate),
            "SPELL_EMPOWER_START" => Ok(Self::SpellEmpowerStart),
            "SPELL_CAST_SUCCESS" => Ok(Self::SpellCastSuccess),
            "DAMAGE_SPLIT" => Ok(Self::DamageSplit),
            "SPELL_SUMMON" => Ok(Self::SpellSummon),
            "COMBATANT_INFO" => Ok(Self::CombatantInfo),
            "SPELL_MISSED" => Ok(Self::SpellMissed),
            "SPELL_HEAL_ABSORBED" => Ok(Self::SpellHealAbsorbed),
            "SPELL_CAST_START" => Ok(Self::SpellCastStart),
            "SPELL_EXTRA_ATTACKS" => Ok(Self::SpellExtraAttacks),
            "SPELL_HEAL" => Ok(Self::SpellHeal),
            "STAGGER_CLEAR" => Ok(Self::StaggerClear),
            "SPELL_AURA_REMOVED_DOSE" => Ok(Self::SpellAuraRemovedDose),
            "EMOTE" => Ok(Self::Emote),
            "SPELL_DRAIN" => Ok(Self::SpellDrain),
            "ENCOUNTER_START" => Ok(Self::EncounterStart),
            "SPELL_PERIODIC_DAMAGE_SUPPORT" => Ok(Self::SpellPeriodicDamageSupport),
            "SPELL_PERIODIC_MISSED" => Ok(Self::SpellPeriodicMissed),
            "SWING_MISSED" => Ok(Self::SwingMissed),
            "SPELL_PERIODIC_ENERGIZE" => Ok(Self::SpellPeriodicEnergize),
            "SPELL_EMPOWER_END" => Ok(Self::SpellEmpowerEnd),
            "RANGE_MISSED" => Ok(Self::RangeMissed),
            "SWING_DAMAGE_LANDED" => Ok(Self::SwingDamageLanded),
            "SPELL_INSTAKILL" => Ok(Self::SpellInstakill),
            "ARENA_MATCH_START" => Ok(Self::ArenaMatchStart),
            "ARENA_MATCH_END" => Ok(Self::ArenaMatchEnd),
            "CHALLENGE_MODE_START" => Ok(Self::ChallengeModeStart),
            "CHALLENGE_MODE_END" => Ok(Self::ChallengeModeEnd),
            "WORLD_MARKER_PLACED" => Ok(Self::WorldMarkerPlaced),
            "WORLD_MARKER_REMOVED" => Ok(Self::WorldMarkerRemoved),
            _ => Err(eyre!("invalid event - {value}")),
        }
    }
}
impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpellAuraAppliedDose => write!(f, "SPELL_AURA_APPLIED_DOSE"),
            Self::SpellInterrupt => write!(f, "SPELL_INTERRUPT"),
            Self::PartyKill => write!(f, "PARTY_KILL"),
            Self::StaggerPrevented => write!(f, "STAGGER_PREVENTED"),
            Self::SpellAuraBroken => write!(f, "SPELL_AURA_BROKEN"),
            Self::SpellAuraApplied => write!(f, "SPELL_AURA_APPLIED"),
            Self::SwingDamage => write!(f, "SWING_DAMAGE"),
            Self::SpellCastFailed => write!(f, "SPELL_CAST_FAILED"),
            Self::SpellDamageSupport => write!(f, "SPELL_DAMAGE_SUPPORT"),
            Self::EncounterEnd => write!(f, "ENCOUNTER_END"),
            Self::RangeDamage => write!(f, "RANGE_DAMAGE"),
            Self::MapChange => write!(f, "MAP_CHANGE"),
            Self::SpellAbsorbed => write!(f, "SPELL_ABSORBED"),
            Self::RangeDamageSupport => write!(f, "RANGE_DAMAGE_SUPPORT"),
            Self::CombatLogVersion => write!(f, "COMBAT_LOG_VERSION"),
            Self::SpellAuraRemoved => write!(f, "SPELL_AURA_REMOVED"),
            Self::SpellPeriodicDamage => write!(f, "SPELL_PERIODIC_DAMAGE"),
            Self::SpellResurrect => write!(f, "SPELL_RESURRECT"),
            Self::EnvironmentalDamage => write!(f, "ENVIRONMENTAL_DAMAGE"),
            Self::SpellStolen => write!(f, "SPELL_STOLEN"),
            Self::SpellAuraRefresh => write!(f, "SPELL_AURA_REFRESH"),
            Self::SwingDamageLandedSupport => write!(f, "SWING_DAMAGE_LANDED_SUPPORT"),
            Self::EnchantApplied => write!(f, "ENCHANT_APPLIED"),
            Self::SpellDamage => write!(f, "SPELL_DAMAGE"),
            Self::SpellHealSupport => write!(f, "SPELL_HEAL_SUPPORT"),
            Self::SpellAuraBrokenSpell => write!(f, "SPELL_AURA_BROKEN_SPELL"),
            Self::SpellDispel => write!(f, "SPELL_DISPEL"),
            Self::SpellEmpowerInterrupt => write!(f, "SPELL_EMPOWER_INTERRUPT"),
            Self::SpellAbsorbedSupport => write!(f, "SPELL_ABSORBED_SUPPORT"),
            Self::SpellEnergize => write!(f, "SPELL_ENERGIZE"),
            Self::UnitDied => write!(f, "UNIT_DIED"),
            Self::SpellPeriodicHeal => write!(f, "SPELL_PERIODIC_HEAL"),
            Self::ZoneChange => write!(f, "ZONE_CHANGE"),
            Self::SpellPeriodicHealSupport => write!(f, "SPELL_PERIODIC_HEAL_SUPPORT"),
            Self::SpellCreate => write!(f, "SPELL_CREATE"),
            Self::SpellEmpowerStart => write!(f, "SPELL_EMPOWER_START"),
            Self::SpellCastSuccess => write!(f, "SPELL_CAST_SUCCESS"),
            Self::DamageSplit => write!(f, "DAMAGE_SPLIT"),
            Self::SpellSummon => write!(f, "SPELL_SUMMON"),
            Self::CombatantInfo => write!(f, "COMBATANT_INFO"),
            Self::SpellMissed => write!(f, "SPELL_MISSED"),
            Self::SpellHealAbsorbed => write!(f, "SPELL_HEAL_ABSORBED"),
            Self::SpellCastStart => write!(f, "SPELL_CAST_START"),
            Self::SpellExtraAttacks => write!(f, "SPELL_EXTRA_ATTACKS"),
            Self::SpellHeal => write!(f, "SPELL_HEAL"),
            Self::StaggerClear => write!(f, "STAGGER_CLEAR"),
            Self::SpellAuraRemovedDose => write!(f, "SPELL_AURA_REMOVED_DOSE"),
            Self::Emote => write!(f, "EMOTE"),
            Self::SpellDrain => write!(f, "SPELL_DRAIN"),
            Self::EncounterStart => write!(f, "ENCOUNTER_START"),
            Self::SpellPeriodicDamageSupport => write!(f, "SPELL_PERIODIC_DAMAGE_SUPPORT"),
            Self::SpellPeriodicMissed => write!(f, "SPELL_PERIODIC_MISSED"),
            Self::SwingMissed => write!(f, "SWING_MISSED"),
            Self::SpellPeriodicEnergize => write!(f, "SPELL_PERIODIC_ENERGIZE"),
            Self::SpellEmpowerEnd => write!(f, "SPELL_EMPOWER_END"),
            Self::RangeMissed => write!(f, "RANGE_MISSED"),
            Self::SwingDamageLanded => write!(f, "SWING_DAMAGE_LANDED"),
            Self::SpellInstakill => write!(f, "SPELL_INSTAKILL"),
            Self::ArenaMatchStart => write!(f, "ARENA_MATCH_START"),
            Self::ArenaMatchEnd => write!(f, "ARENA_MATCH_END"),
            Self::ChallengeModeStart => write!(f, "CHALLENGE_MODE_START"),
            Self::ChallengeModeEnd => write!(f, "CHALLENGE_MODE_END"),
            Self::WorldMarkerPlaced => write!(f, "WORLD_MARKER_PLACED"),
            Self::WorldMarkerRemoved => write!(f, "WORLD_MARKER_REMOVED"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MissType {
    Absorb,
    Block,
    Deflect,
    Dodge,
    Evade,
    Immune,
    Miss,
    Parry,
    Reflect,
    Resist,
}

impl TryFrom<&str> for MissType {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ABSORB" => Ok(Self::Absorb),
            "BLOCK" => Ok(Self::Block),
            "DEFLECT" => Ok(Self::Deflect),
            "DODGE" => Ok(Self::Dodge),
            "EVADE" => Ok(Self::Evade),
            "IMMUNE" => Ok(Self::Immune),
            "MISS" => Ok(Self::Miss),
            "PARRY" => Ok(Self::Parry),
            "REFLECT" => Ok(Self::Reflect),
            "RESIST" => Ok(Self::Resist),
            _ => Err(eyre!("invalid miss type - {value}")),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AuraType {
    Buff,
    Debuff,
}

impl TryFrom<&str> for AuraType {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "BUFF" => Ok(Self::Buff),
            "DEBUFF" => Ok(Self::Debuff),
            _ => Err(eyre!("invalid aura type - {value}")),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnvironmentalType {
    Drowning,
    Falling,
    Fatigue,
    Fire,
    Lava,
    Slime,
}

impl TryFrom<&str> for EnvironmentalType {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Drowning" => Ok(Self::Drowning),
            "Falling" => Ok(Self::Falling),
            "Fatigue" => Ok(Self::Fatigue),
            "Fire" => Ok(Self::Fire),
            "Lava" => Ok(Self::Lava),
            "Slime" => Ok(Self::Slime),
            _ => Err(eyre!("invalid environmental type - {value}")),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PowerType {
    Mana,
    Rage,
    Focus,
    Energy,
    ComboPoints,
    Runes,
    RunicPower,
    SoulShards,
    LunarPower,
    HolyPower,
    Alternate,
    Maelstrom,
    Chi,
    Insanity,
    BurningEmbers,
    DemonicFury,
    ArcaneCharges,
    Fury,
    Pain,
    Essence,
    RuneBlood,
    RuneFrost,
    RuneUnholy,
    AlternateQuest,
    AlternateEncounte,
    AlternateMount,
    Balance,
    Happiness,
    ShadowOrbs,
    RuneChromatic,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SpellSchool {
    Physical = 1,
    Holy = 2,
    HolyStrike = 3,
    Fire = 4,
    Flamestrike = 5,
    Radiant = 6,
    Nature = 8,
    Stormstrike = 9,
    Holystorm = 10,
    Volcanic = 12,
    Frost = 16,
    Froststrike = 17,
    Holyfrost = 18,
    Frostfire = 20,
    Froststorm = 24,
    Elemental = 28,
    Shadow = 32,
    Shadowstrike = 33,
    Twilight = 34,
    Shadowflame = 36,
    Plague = 40,
    Shadowfrost = 48,
    Chromatic = 62,
    Arcane = 64,
    Spellstrike = 65,
    Divine = 66,
    Spellfire = 68,
    Astral = 72,
    Spellfrost = 80,
    Spellshadow = 96,
    Cosmic = 106,
    Magic = 126,
    Chaos = 127,
}

impl std::fmt::Display for SpellSchool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Physical => write!(f, "Physical"),
            Self::Holy => write!(f, "Holy"),
            Self::HolyStrike => write!(f, "HolyStrike"),
            Self::Fire => write!(f, "Fire"),
            Self::Flamestrike => write!(f, "Flamestrike"),
            Self::Radiant => write!(f, "Radiant"),
            Self::Nature => write!(f, "Nature"),
            Self::Stormstrike => write!(f, "Stormstrike"),
            Self::Holystorm => write!(f, "Holystorm"),
            Self::Volcanic => write!(f, "Volcanic"),
            Self::Frost => write!(f, "Frost"),
            Self::Froststrike => write!(f, "Froststrike"),
            Self::Holyfrost => write!(f, "Holyfrost"),
            Self::Frostfire => write!(f, "Frostfire"),
            Self::Froststorm => write!(f, "Froststorm"),
            Self::Elemental => write!(f, "Elemental"),
            Self::Shadow => write!(f, "Shadow"),
            Self::Shadowstrike => write!(f, "Shadowstrike"),
            Self::Twilight => write!(f, "Twilight"),
            Self::Shadowflame => write!(f, "Shadowflame"),
            Self::Plague => write!(f, "Plague"),
            Self::Shadowfrost => write!(f, "Shadowfrost"),
            Self::Chromatic => write!(f, "Chromatic"),
            Self::Arcane => write!(f, "Arcane"),
            Self::Spellstrike => write!(f, "Spellstrike"),
            Self::Divine => write!(f, "Divine"),
            Self::Spellfire => write!(f, "Spellfire"),
            Self::Astral => write!(f, "Astral"),
            Self::Spellfrost => write!(f, "Spellfrost"),
            Self::Spellshadow => write!(f, "Spellshadow"),
            Self::Cosmic => write!(f, "Cosmic"),
            Self::Magic => write!(f, "Magic"),
            Self::Chaos => write!(f, "Chaos"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RaidFlag {
    None = 0,
    Star = 1,
    Circle = 2,
    Diamond = 4,
    Triangle = 8,
    Moon = 16,
    Square = 32,
    Cross = 64,
    Skull = 128,
}

impl std::fmt::Display for RaidFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Star => write!(f, "Star"),
            Self::Circle => write!(f, "Circle"),
            Self::Diamond => write!(f, "Diamond"),
            Self::Triangle => write!(f, "Triangle"),
            Self::Moon => write!(f, "Moon"),
            Self::Square => write!(f, "Square"),
            Self::Cross => write!(f, "Cross"),
            Self::Skull => write!(f, "Skull"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnitFlags {
    affiliation: Affiliation,
    reaction: Reaction,
    controller: Controller,
    classification: Classification,
    special: Special,
}

impl UnitFlags {
    pub fn new(flag: u32) -> Result<Self, &'static str> {
        Ok(Self {
            affiliation: Affiliation::try_from(flag & 0xF)?,
            reaction: Reaction::try_from(flag & 0xF0)?,
            controller: Controller::try_from(flag & 0x300)?,
            classification: Classification::try_from(flag & 0xFC00)?,
            special: Special::try_from(flag & 0xFFFF0000)?,
        })
    }
}

impl std::fmt::Display for UnitFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UnitFlags({}, {}, {}, {}, {})",
            self.affiliation, self.reaction, self.controller, self.classification, self.special
        )
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Affiliation {
    Mine = 0x1,
    Party = 0x2,
    Raid = 0x4,
    Outsider = 0x8,
}

impl TryFrom<u32> for Affiliation {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Self::Mine),
            0x2 => Ok(Self::Party),
            0x4 => Ok(Self::Raid),
            0x8 => Ok(Self::Outsider),
            _ => Err("invalid unit affiliation"),
        }
    }
}

impl std::fmt::Display for Affiliation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mine => write!(f, "Mine"),
            Self::Party => write!(f, "Party"),
            Self::Raid => write!(f, "Raid"),
            Self::Outsider => write!(f, "Outsider"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Reaction {
    Friendly = 0x10,
    Hostile = 0x40,
    Neutral = 0x20,
}

impl TryFrom<u32> for Reaction {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x10 => Ok(Self::Friendly),
            0x20 => Ok(Self::Neutral),
            0x40 => Ok(Self::Hostile),
            _ => Err("invalid unit reaction"),
        }
    }
}

impl std::fmt::Display for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Friendly => write!(f, "Friendly"),
            Self::Hostile => write!(f, "Hostile"),
            Self::Neutral => write!(f, "Neutral"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Controller {
    Player = 0x100,
    Npc = 0x200,
}

impl TryFrom<u32> for Controller {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x100 => Ok(Self::Player),
            0x200 => Ok(Self::Npc),
            _ => Err("invalid unit controller"),
        }
    }
}

impl std::fmt::Display for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player => write!(f, "Player"),
            Self::Npc => write!(f, "Npc"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Classification {
    Player = 0x400,
    Npc = 0x800,
    Pet = 0x1000,
    Guardian = 0x2000,
    Other = 0x4000,
}

impl TryFrom<u32> for Classification {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x400 => Ok(Self::Player),
            0x800 => Ok(Self::Npc),
            0x1000 => Ok(Self::Pet),
            0x2000 => Ok(Self::Guardian),
            0x4000 => Ok(Self::Other),
            _ => Err("invalid unit classification"),
        }
    }
}

impl std::fmt::Display for Classification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player => write!(f, "Player"),
            Self::Npc => write!(f, "Npc"),
            Self::Pet => write!(f, "Pet"),
            Self::Guardian => write!(f, "Guardian"),
            Self::Other => write!(f, "Other"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Special {
    Target = 0x10000,
    Focus = 0x20000,
    MainTank = 0x40000,
    MainAssist = 0x80000,
    None = 0x80000000,
}

impl TryFrom<u32> for Special {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x10000 => Ok(Self::Target),
            0x20000 => Ok(Self::Focus),
            0x40000 => Ok(Self::MainTank),
            0x80000 => Ok(Self::MainAssist),
            0x80000000 => Ok(Self::None),
            _ => Err("invalid unit special flag"),
        }
    }
}

impl std::fmt::Display for Special {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Target => write!(f, "Target"),
            Self::Focus => write!(f, "Focus"),
            Self::MainTank => write!(f, "MainTank"),
            Self::MainAssist => write!(f, "MainAssist"),
            Self::None => write!(f, "None"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Faction {
    Horde,
    Alliance,
}

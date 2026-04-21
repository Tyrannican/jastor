use eyre::{Report, Result, eyre};

#[derive(Debug, Clone)]
pub struct Guid(pub String);

impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for Guid {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Target {
    pub guid: Guid,
    pub name: String,
    pub unit_flags: UnitFlags,
    pub raid_flags: RaidFlag,
}

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
    EnchantRemoved,
    SpellDamage,
    SpellHealSupport,
    SpellAuraBrokenSpell,
    SpellDispel,
    SpellEmpowerInterrupt,
    SpellAbsorbedSupport,
    SpellEnergize,
    UnitDied,
    UnitDestroyed,
    UnitDissipates,
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

impl EventType {
    pub fn has_spell_parameters(&self) -> bool {
        match self {
            Self::ArenaMatchStart
            | Self::ArenaMatchEnd
            | Self::EncounterStart
            | Self::EncounterEnd
            | Self::ChallengeModeStart
            | Self::ChallengeModeEnd
            | Self::WorldMarkerPlaced
            | Self::WorldMarkerRemoved
            | Self::ZoneChange
            | Self::MapChange
            | Self::CombatLogVersion
            | Self::CombatantInfo
            | Self::Emote
            | Self::StaggerPrevented
            | Self::StaggerClear
            | Self::UnitDied
            | Self::UnitDestroyed
            | Self::UnitDissipates
            | Self::PartyKill
            | Self::SwingDamageLanded
            | Self::SwingMissed
            | Self::EnvironmentalDamage
            | Self::EnchantApplied
            | Self::EnchantRemoved
            | Self::SpellAbsorbedSupport
            | Self::SwingDamage => false,
            _ => true,
        }
    }

    pub fn has_advanced_parameters(&self) -> bool {
        match self {
            Self::SpellAuraAppliedDose
            | Self::SpellAuraRemoved
            | Self::SpellAuraApplied
            | Self::SpellAuraRemovedDose
            | Self::SpellAuraRefresh
            | Self::SpellPeriodicMissed
            | Self::SpellCastStart
            | Self::SpellMissed
            | Self::SpellHealAbsorbed
            | Self::SwingMissed
            | Self::SpellExtraAttacks
            | Self::SpellSummon
            | Self::UnitDied
            | Self::PartyKill
            | Self::SpellCastFailed
            | Self::SpellInterrupt
            | Self::SpellDispel
            | Self::SpellCreate
            | Self::SpellAuraBrokenSpell
            | Self::RangeMissed
            | Self::SpellResurrect
            | Self::SpellInstakill
            | Self::EnchantApplied
            | Self::EnchantRemoved
            | Self::SpellEmpowerStart
            | Self::SpellEmpowerEnd
            | Self::SpellAbsorbedSupport
            | Self::SpellStolen
            | Self::SpellAuraBroken
            | Self::SpellEmpowerInterrupt
            | Self::SpellAbsorbed => false,
            _ => true,
        }
    }
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
            "ENCHANT_REMOVED" => Ok(Self::EnchantRemoved),
            "SPELL_DAMAGE" => Ok(Self::SpellDamage),
            "SPELL_HEAL_SUPPORT" => Ok(Self::SpellHealSupport),
            "SPELL_AURA_BROKEN_SPELL" => Ok(Self::SpellAuraBrokenSpell),
            "SPELL_DISPEL" => Ok(Self::SpellDispel),
            "SPELL_EMPOWER_INTERRUPT" => Ok(Self::SpellEmpowerInterrupt),
            "SPELL_ABSORBED_SUPPORT" => Ok(Self::SpellAbsorbedSupport),
            "SPELL_ENERGIZE" => Ok(Self::SpellEnergize),
            "UNIT_DIED" => Ok(Self::UnitDied),
            "UNIT_DESTROYED" => Ok(Self::UnitDestroyed),
            "UNIT_DISSIPATES" => Ok(Self::UnitDissipates),
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
            Self::EnchantRemoved => write!(f, "ENCHANT_REMOVED"),
            Self::SpellDamage => write!(f, "SPELL_DAMAGE"),
            Self::SpellHealSupport => write!(f, "SPELL_HEAL_SUPPORT"),
            Self::SpellAuraBrokenSpell => write!(f, "SPELL_AURA_BROKEN_SPELL"),
            Self::SpellDispel => write!(f, "SPELL_DISPEL"),
            Self::SpellEmpowerInterrupt => write!(f, "SPELL_EMPOWER_INTERRUPT"),
            Self::SpellAbsorbedSupport => write!(f, "SPELL_ABSORBED_SUPPORT"),
            Self::SpellEnergize => write!(f, "SPELL_ENERGIZE"),
            Self::UnitDied => write!(f, "UNIT_DIED"),
            Self::UnitDestroyed => write!(f, "UNIT_DESTROYED"),
            Self::UnitDissipates => write!(f, "UNIT_DISSIPATES"),
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

impl TryFrom<u8> for PowerType {
    type Error = Report;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Mana),
            1 => Ok(Self::Rage),
            2 => Ok(Self::Focus),
            3 => Ok(Self::Energy),
            4 => Ok(Self::ComboPoints),
            5 => Ok(Self::Runes),
            6 => Ok(Self::RunicPower),
            7 => Ok(Self::SoulShards),
            8 => Ok(Self::LunarPower),
            9 => Ok(Self::HolyPower),
            10 => Ok(Self::Alternate),
            11 => Ok(Self::Maelstrom),
            12 => Ok(Self::Chi),
            13 => Ok(Self::Insanity),
            14 => Ok(Self::BurningEmbers),
            15 => Ok(Self::DemonicFury),
            16 => Ok(Self::ArcaneCharges),
            17 => Ok(Self::Fury),
            18 => Ok(Self::Pain),
            19 => Ok(Self::Essence),
            20 => Ok(Self::RuneBlood),
            21 => Ok(Self::RuneFrost),
            22 => Ok(Self::RuneUnholy),
            23 => Ok(Self::AlternateQuest),
            24 => Ok(Self::AlternateEncounte),
            25 => Ok(Self::AlternateMount),
            26 => Ok(Self::Balance),
            27 => Ok(Self::Happiness),
            28 => Ok(Self::ShadowOrbs),
            29 => Ok(Self::RuneChromatic),
            _ => Err(eyre!("invalid value for power type - {value}")),
        }
    }
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

impl TryFrom<u8> for SpellSchool {
    type Error = Report;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Physical),
            2 => Ok(Self::Holy),
            3 => Ok(Self::HolyStrike),
            4 => Ok(Self::Fire),
            5 => Ok(Self::Flamestrike),
            6 => Ok(Self::Radiant),
            8 => Ok(Self::Nature),
            9 => Ok(Self::Stormstrike),
            10 => Ok(Self::Holystorm),
            12 => Ok(Self::Volcanic),
            16 => Ok(Self::Frost),
            17 => Ok(Self::Froststrike),
            18 => Ok(Self::Holyfrost),
            20 => Ok(Self::Frostfire),
            24 => Ok(Self::Froststorm),
            28 => Ok(Self::Elemental),
            32 => Ok(Self::Shadow),
            33 => Ok(Self::Shadowstrike),
            34 => Ok(Self::Twilight),
            36 => Ok(Self::Shadowflame),
            40 => Ok(Self::Plague),
            48 => Ok(Self::Shadowfrost),
            62 => Ok(Self::Chromatic),
            64 => Ok(Self::Arcane),
            65 => Ok(Self::Spellstrike),
            66 => Ok(Self::Divine),
            68 => Ok(Self::Spellfire),
            72 => Ok(Self::Astral),
            80 => Ok(Self::Spellfrost),
            96 => Ok(Self::Spellshadow),
            106 | 110 => Ok(Self::Cosmic),
            126 => Ok(Self::Magic),
            124 | 127 => Ok(Self::Chaos),
            _ => Ok(Self::Physical),
            // _ => Err(eyre!("invalid spell school id - {value}")),
        }
    }
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

#[repr(u32)]
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

impl TryFrom<u32> for RaidFlag {
    type Error = Report;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Star),
            2 => Ok(Self::Circle),
            4 => Ok(Self::Diamond),
            8 => Ok(Self::Triangle),
            16 => Ok(Self::Moon),
            32 => Ok(Self::Square),
            64 => Ok(Self::Cross),
            128 => Ok(Self::Skull),
            0x80000000 => Ok(Self::None),
            _ => Err(eyre!("invalid raid flag value: {value}")),
        }
    }
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
    pub fn new(flag: u32) -> Result<Self> {
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
    None,
}

impl TryFrom<u32> for Affiliation {
    type Error = Report;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Self::Mine),
            0x2 => Ok(Self::Party),
            0x4 => Ok(Self::Raid),
            0x8 => Ok(Self::Outsider),
            _ => Ok(Self::None),
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
            Self::None => write!(f, "None"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Reaction {
    Friendly = 0x10,
    Hostile = 0x40,
    Neutral = 0x20,
    None,
}

impl TryFrom<u32> for Reaction {
    type Error = Report;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x10 => Ok(Self::Friendly),
            0x20 => Ok(Self::Neutral),
            0x40 => Ok(Self::Hostile),
            _ => Ok(Self::None),
        }
    }
}

impl std::fmt::Display for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Friendly => write!(f, "Friendly"),
            Self::Hostile => write!(f, "Hostile"),
            Self::Neutral => write!(f, "Neutral"),
            Self::None => write!(f, "None"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Controller {
    Player = 0x100,
    Npc = 0x200,
    None,
}

impl TryFrom<u32> for Controller {
    type Error = Report;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x100 => Ok(Self::Player),
            0x200 => Ok(Self::Npc),
            _ => Ok(Self::None),
            // _ => Err(eyre!("invalid unit controller - {value}")),
        }
    }
}

impl std::fmt::Display for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player => write!(f, "Player"),
            Self::Npc => write!(f, "Npc"),
            Self::None => write!(f, "None"),
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
    None,
}

impl TryFrom<u32> for Classification {
    type Error = Report;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x400 => Ok(Self::Player),
            0x800 => Ok(Self::Npc),
            0x1000 => Ok(Self::Pet),
            0x2000 => Ok(Self::Guardian),
            0x4000 => Ok(Self::Other),
            _ => Ok(Self::None),
            // _ => Err(eyre!("invalid unit classification")),
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
            Self::None => write!(f, "None"),
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
    Other(u32),
}

impl TryFrom<u32> for Special {
    type Error = Report;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x10000 => Ok(Self::Target),
            0x20000 => Ok(Self::Focus),
            0x40000 => Ok(Self::MainTank),
            0x80000 => Ok(Self::MainAssist),
            0x0 | 0x80000000 => Ok(Self::None),
            other => Ok(Self::Other(other)),
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
            Self::Other(value) => write!(f, "Other({value})"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Faction {
    Horde,
    Alliance,
}

impl std::fmt::Display for Faction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horde => write!(f, "Horde"),
            Self::Alliance => write!(f, "Alliance"),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Difficulty {
    NormalParty = 1,
    HeroicParty = 2,
    MythicKeystone = 8,
    NormalRaid = 14,
    HeroicRaid = 15,
    MythicRaid = 16,
    LookingForRaid = 17,
    MythicParty = 23,
    TimewalkingParty = 24,
    TimewalkingRaid = 33,
    Pvp = 34,
    FollowerParty = 205,
    Delve = 208,
    StoryRaid = 220,
    Other(u16),
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NormalParty => write!(f, "Party (Normal)"),
            Self::HeroicParty => write!(f, "Party (Heroic)"),
            Self::MythicKeystone => write!(f, "Mythic Keystone"),
            Self::NormalRaid => write!(f, "Raid (Normal)"),
            Self::HeroicRaid => write!(f, "Raid (Heroic)"),
            Self::MythicRaid => write!(f, "Raid (Mythic)"),
            Self::LookingForRaid => write!(f, "Looking For Raid"),
            Self::MythicParty => write!(f, "Party (Mythic)"),
            Self::TimewalkingParty => write!(f, "Party (Timewalking)"),
            Self::TimewalkingRaid => write!(f, "Raid (Timewalking)"),
            Self::Pvp => write!(f, "PVP"),
            Self::FollowerParty => write!(f, "Party (Follower)"),
            Self::Delve => write!(f, "Delve"),
            Self::StoryRaid => write!(f, "Raid (Story)"),
            Self::Other(value) => write!(f, "Other (ID: {})", value),
        }
    }
}

impl From<u16> for Difficulty {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::NormalParty,
            2 => Self::HeroicParty,
            8 => Self::MythicKeystone,
            14 => Self::NormalRaid,
            15 => Self::HeroicRaid,
            16 => Self::MythicRaid,
            17 => Self::LookingForRaid,
            23 => Self::MythicParty,
            24 => Self::TimewalkingParty,
            33 => Self::TimewalkingRaid,
            34 => Self::Pvp,
            205 => Self::FollowerParty,
            208 => Self::Delve,
            220 => Self::StoryRaid,
            other => Self::Other(other),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Specialization {
    BloodDeathKnight = 250,
    FrostDeathKnight = 251,
    UnholyDeathKnight = 252,
    DeathKnightInitial = 1455,

    HavocDemonHunter = 577,
    VengeanceDemonHunter = 581,
    DevourerDemonHunter = 1480,
    DemonHunterInitial = 1456,

    BalanceDruid = 102,
    FeralDruid = 103,
    GuardianDruid = 104,
    RestorationDruid = 105,
    DruidInitial = 1447,

    DevastationEvoker = 1467,
    PreservationEvoker = 1468,
    AugmentationEvoker = 1473,
    EvokerInitial = 1465,

    BeastMasteryHunter = 253,
    MarksmanshipHunter = 254,
    SurvivalHunter = 255,
    HunterInitial = 1448,

    ArcaneMage = 62,
    FireMage = 63,
    FrostMage = 64,
    MageInitial = 1449,

    BrewmasterMonk = 268,
    WindwalkerMonk = 269,
    MistweaverMonk = 270,
    MonkInitial = 1450,

    HolyPaladin = 65,
    ProtectionPaladin = 66,
    RetributionPaladin = 70,
    PaladinInitial = 1451,

    DisciplinePriest = 256,
    HolyPriest = 257,
    ShadowPriest = 258,
    PriestInitial = 1452,

    AssassinationRogue = 259,
    OutlawRogue = 260,
    SubtletyRogue = 261,
    RogueInitial = 1453,

    ElementalShaman = 262,
    EnhancementShaman = 263,
    RestorationShaman = 264,
    ShamanInitial = 1444,

    AfflictionWarlock = 265,
    DemonologyWarlock = 266,
    DestructionWarlock = 267,
    WarlockInitial = 1454,

    ArmsWarrior = 71,
    FuryWarrior = 72,
    ProtectionWarrior = 73,
    WarriorInitial = 1446,
}

impl TryFrom<u16> for Specialization {
    type Error = Report;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            250 => Ok(Self::BloodDeathKnight),
            251 => Ok(Self::FrostDeathKnight),
            252 => Ok(Self::UnholyDeathKnight),
            1455 => Ok(Self::DeathKnightInitial),
            577 => Ok(Self::HavocDemonHunter),
            581 => Ok(Self::VengeanceDemonHunter),
            1480 => Ok(Self::DevourerDemonHunter),
            1456 => Ok(Self::DemonHunterInitial),
            102 => Ok(Self::BalanceDruid),
            103 => Ok(Self::FeralDruid),
            104 => Ok(Self::GuardianDruid),
            105 => Ok(Self::RestorationDruid),
            1447 => Ok(Self::DruidInitial),
            1467 => Ok(Self::DevastationEvoker),
            1468 => Ok(Self::PreservationEvoker),
            1473 => Ok(Self::AugmentationEvoker),
            1465 => Ok(Self::EvokerInitial),
            253 => Ok(Self::BeastMasteryHunter),
            254 => Ok(Self::MarksmanshipHunter),
            255 => Ok(Self::SurvivalHunter),
            1448 => Ok(Self::HunterInitial),
            62 => Ok(Self::ArcaneMage),
            63 => Ok(Self::FireMage),
            64 => Ok(Self::FrostMage),
            1449 => Ok(Self::MageInitial),
            268 => Ok(Self::BrewmasterMonk),
            269 => Ok(Self::WindwalkerMonk),
            270 => Ok(Self::MistweaverMonk),
            1450 => Ok(Self::MonkInitial),
            65 => Ok(Self::HolyPaladin),
            66 => Ok(Self::ProtectionPaladin),
            70 => Ok(Self::RetributionPaladin),
            1451 => Ok(Self::PaladinInitial),
            256 => Ok(Self::DisciplinePriest),
            257 => Ok(Self::HolyPriest),
            258 => Ok(Self::ShadowPriest),
            1452 => Ok(Self::PriestInitial),
            259 => Ok(Self::AssassinationRogue),
            260 => Ok(Self::OutlawRogue),
            261 => Ok(Self::SubtletyRogue),
            1453 => Ok(Self::RogueInitial),
            262 => Ok(Self::ElementalShaman),
            263 => Ok(Self::EnhancementShaman),
            264 => Ok(Self::RestorationShaman),
            1444 => Ok(Self::ShamanInitial),
            265 => Ok(Self::AfflictionWarlock),
            266 => Ok(Self::DemonologyWarlock),
            267 => Ok(Self::DestructionWarlock),
            1454 => Ok(Self::WarlockInitial),
            71 => Ok(Self::ArmsWarrior),
            72 => Ok(Self::FuryWarrior),
            73 => Ok(Self::ProtectionWarrior),
            1446 => Ok(Self::WarriorInitial),
            other => Err(eyre!("unknown specialization id: {other}")),
        }
    }
}

impl std::fmt::Display for Specialization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BloodDeathKnight => write!(f, "Blood Death Knight"),
            Self::FrostDeathKnight => write!(f, "Frost Death Knight"),
            Self::UnholyDeathKnight => write!(f, "Unholy Death Knight"),
            Self::DeathKnightInitial => write!(f, "Death Knight (Initial)"),

            Self::HavocDemonHunter => write!(f, "Havoc Demon Hunter"),
            Self::VengeanceDemonHunter => write!(f, "Vengeance Demon Hunter"),
            Self::DevourerDemonHunter => write!(f, "Devourer Demon Hunter"),
            Self::DemonHunterInitial => write!(f, "Demon Hunter (Initial)"),

            Self::BalanceDruid => write!(f, "Balance Druid"),
            Self::FeralDruid => write!(f, "Feral Druid"),
            Self::GuardianDruid => write!(f, "Guardian Druid"),
            Self::RestorationDruid => write!(f, "Restoration Druid"),
            Self::DruidInitial => write!(f, "Druid (Initial)"),

            Self::DevastationEvoker => write!(f, "Devastation Evoker"),
            Self::PreservationEvoker => write!(f, "Preservation Evoker"),
            Self::AugmentationEvoker => write!(f, "Augmentation Evoker"),
            Self::EvokerInitial => write!(f, "Evoker (Initial)"),

            Self::BeastMasteryHunter => write!(f, "Beast Mastery Hunter"),
            Self::MarksmanshipHunter => write!(f, "Marksmanship Hunter"),
            Self::SurvivalHunter => write!(f, "Survival Hunter"),
            Self::HunterInitial => write!(f, "Hunter (Initial)"),

            Self::ArcaneMage => write!(f, "Arcane Mage"),
            Self::FireMage => write!(f, "Fire Mage"),
            Self::FrostMage => write!(f, "Frost Mage"),
            Self::MageInitial => write!(f, "Mage (Initial)"),

            Self::BrewmasterMonk => write!(f, "Brewmaster Monk"),
            Self::WindwalkerMonk => write!(f, "Windwalker Monk"),
            Self::MistweaverMonk => write!(f, "Mistweaver Monk"),
            Self::MonkInitial => write!(f, "Monk Initial"),

            Self::HolyPaladin => write!(f, "Holy Paladin"),
            Self::ProtectionPaladin => write!(f, "Protection Paladin"),
            Self::RetributionPaladin => write!(f, "Retribution Paladin"),
            Self::PaladinInitial => write!(f, "Paladin (Initial)"),

            Self::DisciplinePriest => write!(f, "Discipline Priest"),
            Self::HolyPriest => write!(f, "Holy Priest"),
            Self::ShadowPriest => write!(f, "Shadow Priest"),
            Self::PriestInitial => write!(f, "Priest (Initial)"),

            Self::AssassinationRogue => write!(f, "Assassination Rogue"),
            Self::OutlawRogue => write!(f, "Outlaw Rogue"),
            Self::SubtletyRogue => write!(f, "Subtlety Rogue"),
            Self::RogueInitial => write!(f, "Rogue (Initial)"),

            Self::ElementalShaman => write!(f, "Elemental Shaman"),
            Self::EnhancementShaman => write!(f, "Enhancement Shaman"),
            Self::RestorationShaman => write!(f, "Restoration Shaman"),
            Self::ShamanInitial => write!(f, "Shaman (Initial)"),

            Self::AfflictionWarlock => write!(f, "Affliction Warlock"),
            Self::DemonologyWarlock => write!(f, "Demonology Warlock"),
            Self::DestructionWarlock => write!(f, "Destruction Warlock"),
            Self::WarlockInitial => write!(f, "Warlock (Initial)"),

            Self::ArmsWarrior => write!(f, "Arms Warrior"),
            Self::FuryWarrior => write!(f, "Fury Warrior"),
            Self::ProtectionWarrior => write!(f, "Protection Warrior"),
            Self::WarriorInitial => write!(f, "Warrior (Initial)"),
        }
    }
}

#[derive(Clone)]
pub struct MultiValue<T>(pub Vec<T>);

impl<T> std::ops::Deref for MultiValue<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::fmt::Debug for MultiValue<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.len() {
            0 => return Ok(()),
            1 => write!(f, "{:?}", self.0[0]),
            len => {
                let last = len - 1;
                write!(f, "(")?;
                for (i, entry) in self.0.iter().enumerate() {
                    write!(f, "{:?}", entry)?;
                    if i != last {
                        write!(f, ",")?;
                    }
                }

                write!(f, ")")
            }
        }
    }
}

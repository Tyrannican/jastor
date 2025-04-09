use crate::error::JastorError;

pub mod flags;

// TODO: Use for type coerscion?
pub trait Event {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    SwingDamage,
    SwingDamageLanded,
    SwingMissed,
    RangeDamage,
    RangeMissed,
    SpellDamage,
    SpellDamageSupport,
    SpellMissed,
    SpellHeal,
    SpellHealAbsorbed,
    SpellAbsorbed,
    SpellEnergize,
    SpellDrain,
    SpellLeech,
    SpellInterrupt,
    SpellDispel,
    SpellDispelFailed,
    SpellStolen,
    SpellExtraAttacks,
    SpellAuraApplied,
    SpellAuraRemoved,
    SpellAuraAppliedDose,
    SpellAuraRemovedDose,
    SpellAuraRefresh,
    SpellAuraBroken,
    SpellAuraBrokenSpell,
    SpellCastStart,
    SpellCastSuccess,
    SpellCastFailed,
    SpellInstaKill,
    SpellDurabilityDamage,
    SpellDurabilityDamageAll,
    SpellCreate,
    SpellSummon,
    SpellResurrect,
    SpellEmpowerStart,
    SpellEmpowerEnd,
    SpellEmpowerInterrupt,

    // The `SpellPeriodic` and `SpellBuilding` fields have some suffixes
    // added that might not even be present but they're included for completion
    // i guess...

    // Periodic
    SpellPeriodicDamage,
    SpellPeriodicDamageSupport,
    SpellPeriodicMissed,
    SpellPeriodicHeal,
    SpellPeriodicHealAbsorbed,
    SpellPeriodicAbsorbed,
    SpellPeriodicEnergize,
    SpellPeriodicDrain,
    SpellPeriodicLeech,
    SpellPeriodicInterrupt,
    SpellPeriodicDispel,
    SpellPeriodicDispelFailed,
    SpellPeriodicStolen,
    SpellPeriodicExtraAttacks,
    SpellPeriodicAuraApplied,
    SpellPeriodicAuraRemoved,
    SpellPeriodicAuraAppliedDose,
    SpellPeriodicAuraRemovedDose,
    SpellPeriodicAuraRefresh,
    SpellPeriodicAuraBroken,
    SpellPeriodicAuraBrokenSpell,
    SpellPeriodicCastStart,
    SpellPeriodicCastSuccess,
    SpellPeriodicCastFailed,
    SpellPeriodicInstaKill,
    SpellPeriodicDurabilityDamage,
    SpellPeriodicDurabilityDamageAll,
    SpellPeriodicCreate,
    SpellPeriodicSummon,
    SpellPeriodicResurrect,
    SpellPeriodicEmpowerStart,
    SpellPeriodicEmpowerEnd,
    SpellPeriodicEmpowerInterrupt,

    // Building
    SpellBuildingDamage,
    SpellBuildingMissed,
    SpellBuildingHeal,
    SpellBuildingHealAbsorbed,
    SpellBuildingAbsorbed,
    SpellBuildingEnergize,
    SpellBuildingDrain,
    SpellBuildingLeech,
    SpellBuildingInterrupt,
    SpellBuildingDispel,
    SpellBuildingDispelFailed,
    SpellBuildingStolen,
    SpellBuildingExtraAttacks,
    SpellBuildingAuraApplied,
    SpellBuildingAuraRemoved,
    SpellBuildingAuraAppliedDose,
    SpellBuildingAuraRemovedDose,
    SpellBuildingAuraRefresh,
    SpellBuildingAuraBroken,
    SpellBuildingAuraBrokenSpell,
    SpellBuildingCastStart,
    SpellBuildingCastSuccess,
    SpellBuildingCastFailed,
    SpellBuildingInstaKill,
    SpellBuildingDurabilityDamage,
    SpellBuildingDurabilityDamageAll,
    SpellBuildingCreate,
    SpellBuildingSummon,
    SpellBuildingResurrect,
    SpellBuildingEmpowerStart,
    SpellBuildingEmpowerEnd,
    SpellBuildingEmpowerInterrupt,

    // Environmental
    EnvironmentalDamage,

    // Special Events
    DamageSplit,
    DamageShield,
    DamageShieldMissed,
    EnchantApplied,
    EnchantRemoved,
    PartyKill,
    UnitDied,
    UnitDestroyed,
    UnitDissapates,

    // General Events
    CombatLogVersion,
    StaggerClear,
    ZoneChange,
    MapChange,
    EncounterStart,
    EncounterEnd,
    CombatantInfo,
    ArenaMatchStart,
    ArenaMatchEnd,
    ChallengeModeStart,
    ChallengeModeEnd,
    Emote,
    WorldMarkerPlaced,
    WorldMarkerRemoved,

    // Fallback
    UnknownEvent(String),
}

impl EventType {
    pub fn from_str(value: &str) -> Self {
        match value {
            "SWING_DAMAGE" => Self::SwingDamage,
            "SWING_DAMAGE_LANDED" => Self::SwingDamageLanded,
            "SWING_MISSED" => Self::SwingMissed,
            "RANGE_DAMAGE" => Self::RangeDamage,
            "RANGE_MISSED" => Self::RangeMissed,
            "STAGGER_CLEAR" => Self::StaggerClear,
            "SPELL_DAMAGE" => Self::SpellDamage,
            "SPELL_DAMAGE_SUPPORT" => Self::SpellDamageSupport,
            "SPELL_MISSED" => Self::SpellMissed,
            "SPELL_HEAL" => Self::SpellHeal,
            "SPELL_HEAL_ABSORBED" => Self::SpellHealAbsorbed,
            "SPELL_ABSORBED" => Self::SpellAbsorbed,
            "SPELL_ENERGIZE" => Self::SpellEnergize,
            "SPELL_DRAIN" => Self::SpellDrain,
            "SPELL_LEECH" => Self::SpellLeech,
            "SPELL_INTERRUPT" => Self::SpellInterrupt,
            "SPELL_DISPEL" => Self::SpellDispel,
            "SPELL_DISPEL_FAILED" => Self::SpellDispelFailed,
            "SPELL_STOLEN" => Self::SpellStolen,
            "SPELL_EXTRA_ATTACKS" => Self::SpellExtraAttacks,
            "SPELL_AURA_APPLIED" => Self::SpellAuraApplied,
            "SPELL_AURA_REMOVED" => Self::SpellAuraRemoved,
            "SPELL_AURA_APPLIED_DOSE" => Self::SpellAuraAppliedDose,
            "SPELL_AURA_REMOVED_DOSE" => Self::SpellAuraRemovedDose,
            "SPELL_AURA_REFRESH" => Self::SpellAuraRefresh,
            "SPELL_AURA_BROKEN" => Self::SpellAuraBroken,
            "SPELL_AURA_BROKEN_SPELL" => Self::SpellAuraBrokenSpell,
            "SPELL_CAST_START" => Self::SpellCastStart,
            "SPELL_CAST_SUCCESS" => Self::SpellCastSuccess,
            "SPELL_CAST_FAILED" => Self::SpellCastFailed,
            "SPELL_INSTAKILL" => Self::SpellInstaKill,
            "SPELL_DURABILITY_DAMAGE" => Self::SpellDurabilityDamage,
            "SPELL_DURABILITY_DAMAGE_ALL" => Self::SpellDurabilityDamageAll,
            "SPELL_CREATE" => Self::SpellCreate,
            "SPELL_SUMMON" => Self::SpellSummon,
            "SPELL_RESURRECT" => Self::SpellResurrect,
            "SPELL_EMPOWER_START" => Self::SpellEmpowerStart,
            "SPELL_EMPOWER_END" => Self::SpellEmpowerEnd,
            "SPELL_EMPOWER_INTERRUPT" => Self::SpellEmpowerInterrupt,
            "SPELL_PERIODIC_DAMAGE" => Self::SpellPeriodicDamage,
            "SPELL_PERIODIC_DAMAGE_SUPPORT" => Self::SpellPeriodicDamageSupport,
            "SPELL_PERIODIC_MISSED" => Self::SpellPeriodicMissed,
            "SPELL_PERIODIC_HEAL" => Self::SpellPeriodicHeal,
            "SPELL_PERIODIC_HEAL_ABSORBED" => Self::SpellPeriodicHealAbsorbed,
            "SPELL_PERIODIC_ABSORBED" => Self::SpellPeriodicAbsorbed,
            "SPELL_PERIODIC_ENERGIZE" => Self::SpellPeriodicEnergize,
            "SPELL_PERIODIC_DRAIN" => Self::SpellPeriodicDrain,
            "SPELL_PERIODIC_LEECH" => Self::SpellPeriodicLeech,
            "SPELL_PERIODIC_INTERRUPT" => Self::SpellPeriodicInterrupt,
            "SPELL_PERIODIC_DISPEL" => Self::SpellPeriodicDispel,
            "SPELL_PERIODIC_DISPEL_FAILED" => Self::SpellPeriodicDispelFailed,
            "SPELL_PERIODIC_STOLEN" => Self::SpellPeriodicStolen,
            "SPELL_PERIODIC_EXTRA_ATTACKS" => Self::SpellPeriodicExtraAttacks,
            "SPELL_PERIODIC_AURA_APPLIED" => Self::SpellPeriodicAuraApplied,
            "SPELL_PERIODIC_AURA_REMOVED" => Self::SpellPeriodicAuraRemoved,
            "SPELL_PERIODIC_AURA_APPLIED_DOSE" => Self::SpellPeriodicAuraAppliedDose,
            "SPELL_PERIODIC_AURA_REMOVED_DOSE" => Self::SpellPeriodicAuraRemovedDose,
            "SPELL_PERIODIC_AURA_REFRESH" => Self::SpellPeriodicAuraRefresh,
            "SPELL_PERIODIC_AURA_BROKEN" => Self::SpellPeriodicAuraBroken,
            "SPELL_PERIODIC_AURA_BROKEN_SPELL" => Self::SpellPeriodicAuraBrokenSpell,
            "SPELL_PERIODIC_CAST_START" => Self::SpellPeriodicCastStart,
            "SPELL_PERIODIC_CAST_SUCCESS" => Self::SpellPeriodicCastSuccess,
            "SPELL_PERIODIC_CAST_FAILED" => Self::SpellPeriodicCastFailed,
            "SPELL_PERIODIC_INSTA_KILL" => Self::SpellPeriodicInstaKill,
            "SPELL_PERIODIC_DURABILITY_DAMAGE" => Self::SpellPeriodicDurabilityDamage,
            "SPELL_PERIODIC_DURABILITY_DAMAGE_ALL" => Self::SpellPeriodicDurabilityDamageAll,
            "SPELL_PERIODIC_CREATE" => Self::SpellPeriodicCreate,
            "SPELL_PERIODIC_SUMMON" => Self::SpellPeriodicSummon,
            "SPELL_PERIODIC_RESURRECT" => Self::SpellPeriodicResurrect,
            "SPELL_PERIODIC_EMPOWER_START" => Self::SpellPeriodicEmpowerStart,
            "SPELL_PERIODIC_EMPOWER_END" => Self::SpellPeriodicEmpowerEnd,
            "SPELL_PERIODIC_EMPOWER_INTERRUPT" => Self::SpellPeriodicEmpowerInterrupt,
            "SPELL_BUILDING_DAMAGE" => Self::SpellBuildingDamage,
            "SPELL_BUILDING_MISSED" => Self::SpellBuildingMissed,
            "SPELL_BUILDING_HEAL" => Self::SpellBuildingHeal,
            "SPELL_BUILDING_HEAL_ABSORBED" => Self::SpellBuildingHealAbsorbed,
            "SPELL_BUILDING_ABSORBED" => Self::SpellBuildingAbsorbed,
            "SPELL_BUILDING_ENERGIZE" => Self::SpellBuildingEnergize,
            "SPELL_BUILDING_DRAIN" => Self::SpellBuildingDrain,
            "SPELL_BUILDING_LEECH" => Self::SpellBuildingLeech,
            "SPELL_BUILDING_INTERRUPT" => Self::SpellBuildingInterrupt,
            "SPELL_BUILDING_DISPEL" => Self::SpellBuildingDispel,
            "SPELL_BUILDING_DISPEL_FAILED" => Self::SpellBuildingDispelFailed,
            "SPELL_BUILDING_STOLEN" => Self::SpellBuildingStolen,
            "SPELL_BUILDING_EXTRA_ATTACKS" => Self::SpellBuildingExtraAttacks,
            "SPELL_BUILDING_AURA_APPLIED" => Self::SpellBuildingAuraApplied,
            "SPELL_BUILDING_AURA_REMOVED" => Self::SpellBuildingAuraRemoved,
            "SPELL_BUILDING_AURA_APPLIED_DOSE" => Self::SpellBuildingAuraAppliedDose,
            "SPELL_BUILDING_AURA_REMOVED_DOSE" => Self::SpellBuildingAuraRemovedDose,
            "SPELL_BUILDING_AURA_REFRESH" => Self::SpellBuildingAuraRefresh,
            "SPELL_BUILDING_AURA_BROKEN" => Self::SpellBuildingAuraBroken,
            "SPELL_BUILDING_AURA_BROKEN_SPELL" => Self::SpellBuildingAuraBrokenSpell,
            "SPELL_BUILDING_CAST_START" => Self::SpellBuildingCastStart,
            "SPELL_BUILDING_CAST_SUCCESS" => Self::SpellBuildingCastSuccess,
            "SPELL_BUILDING_CAST_FAILED" => Self::SpellBuildingCastFailed,
            "SPELL_BUILDING_INSTA_KILL" => Self::SpellBuildingInstaKill,
            "SPELL_BUILDING_DURABILITY_DAMAGE" => Self::SpellBuildingDurabilityDamage,
            "SPELL_BUILDING_DURABILITY_DAMAGE_ALL" => Self::SpellBuildingDurabilityDamageAll,
            "SPELL_BUILDING_CREATE" => Self::SpellBuildingCreate,
            "SPELL_BUILDING_SUMMON" => Self::SpellBuildingSummon,
            "SPELL_BUILDING_RESURRECT" => Self::SpellBuildingResurrect,
            "SPELL_BUILDING_EMPOWER_START" => Self::SpellBuildingEmpowerStart,
            "SPELL_BUILDING_EMPOWER_END" => Self::SpellBuildingEmpowerEnd,
            "SPELL_BUILDING_EMPOWER_INTERRUPT" => Self::SpellBuildingEmpowerInterrupt,
            "ENVIRONMENTAL_DAMAGE" => Self::EnvironmentalDamage,
            "DAMAGE_SPLIT" => Self::DamageSplit,
            "DAMAGE_SHIELD" => Self::DamageShield,
            "DAMAGE_SHIELD_MISSED" => Self::DamageShieldMissed,
            "ENCHANT_APPLIED" => Self::EnchantApplied,
            "ENCHANT_REMOVED" => Self::EnchantRemoved,
            "PARTY_KILL" => Self::PartyKill,
            "UNIT_DIED" => Self::UnitDied,
            "UNIT_DESTROYED" => Self::UnitDestroyed,
            "UNIT_DISSAPATES" => Self::UnitDissapates,
            "COMBAT_LOG_VERSION" => Self::CombatLogVersion,
            "ZONE_CHANGE" => Self::ZoneChange,
            "MAP_CHANGE" => Self::MapChange,
            "ENCOUNTER_START" => Self::EncounterStart,
            "ENCOUNTER_END" => Self::EncounterEnd,
            "COMBATANT_INFO" => Self::CombatantInfo,
            "ARENA_MATCH_START" => Self::ArenaMatchStart,
            "ARENA_MATCH_END" => Self::ArenaMatchEnd,
            "CHALLENGE_MODE_START" => Self::ChallengeModeStart,
            "CHALLENGE_MODE_END" => Self::ChallengeModeEnd,
            "EMOTE" => Self::Emote,
            "WORLD_MARKER_PLACED" => Self::WorldMarkerPlaced,
            "WORLD_MARKER_REMOVED" => Self::WorldMarkerRemoved,
            _ => Self::UnknownEvent(value.to_string()),
        }
    }

    pub fn skip(&self) -> bool {
        match *self {
            Self::Emote | Self::CombatantInfo => true,
            _ => false,
        }
    }

    pub fn is_valid(&self) -> Result<(), JastorError> {
        match *self {
            Self::UnknownEvent(ref e) => {
                return Err(JastorError::ParseError(format!(
                    "unknown event type encountered: {e}"
                )));
            }
            _ => Ok(()),
        }
    }

    pub fn prefix_parameters(&self) -> usize {
        match *self {
            Self::SwingDamage | Self::SwingDamageLanded | Self::SwingMissed => 0,
            Self::SpellDamage
            | Self::DamageSplit
            | Self::DamageShield // <- This needs checked
            | Self::DamageShieldMissed // <- This needs checked
            | Self::SpellDamageSupport
            | Self::SpellMissed
            | Self::SpellHeal
            | Self::SpellHealAbsorbed
            | Self::SpellAbsorbed
            | Self::SpellEnergize
            | Self::SpellDrain
            | Self::SpellLeech
            | Self::SpellInterrupt
            | Self::SpellDispel
            | Self::SpellDispelFailed
            | Self::SpellStolen
            | Self::SpellExtraAttacks
            | Self::SpellAuraApplied
            | Self::SpellAuraRemoved
            | Self::SpellAuraAppliedDose
            | Self::SpellAuraRemovedDose
            | Self::SpellAuraRefresh
            | Self::SpellAuraBroken
            | Self::SpellAuraBrokenSpell
            | Self::SpellCastStart
            | Self::SpellCastSuccess
            | Self::SpellCastFailed
            | Self::SpellInstaKill
            | Self::SpellDurabilityDamage
            | Self::SpellDurabilityDamageAll
            | Self::SpellCreate
            | Self::SpellSummon
            | Self::SpellResurrect
            | Self::SpellEmpowerStart
            | Self::SpellEmpowerEnd
            | Self::SpellEmpowerInterrupt
            | Self::SpellPeriodicDamage
            | Self::SpellPeriodicDamageSupport
            | Self::SpellPeriodicMissed
            | Self::SpellPeriodicHeal
            | Self::SpellPeriodicHealAbsorbed
            | Self::SpellPeriodicAbsorbed
            | Self::SpellPeriodicEnergize
            | Self::SpellPeriodicDrain
            | Self::SpellPeriodicLeech
            | Self::SpellPeriodicInterrupt
            | Self::SpellPeriodicDispel
            | Self::SpellPeriodicDispelFailed
            | Self::SpellPeriodicStolen
            | Self::SpellPeriodicExtraAttacks
            | Self::SpellPeriodicAuraApplied
            | Self::SpellPeriodicAuraRemoved
            | Self::SpellPeriodicAuraAppliedDose
            | Self::SpellPeriodicAuraRemovedDose
            | Self::SpellPeriodicAuraRefresh
            | Self::SpellPeriodicAuraBroken
            | Self::SpellPeriodicAuraBrokenSpell
            | Self::SpellPeriodicCastStart
            | Self::SpellPeriodicCastSuccess
            | Self::SpellPeriodicCastFailed
            | Self::SpellPeriodicInstaKill
            | Self::SpellPeriodicDurabilityDamage
            | Self::SpellPeriodicDurabilityDamageAll
            | Self::SpellPeriodicCreate
            | Self::SpellPeriodicSummon
            | Self::SpellPeriodicResurrect
            | Self::SpellPeriodicEmpowerStart
            | Self::SpellPeriodicEmpowerEnd
            | Self::SpellPeriodicEmpowerInterrupt
            | Self::SpellBuildingDamage
            | Self::SpellBuildingMissed
            | Self::SpellBuildingHeal
            | Self::SpellBuildingHealAbsorbed
            | Self::SpellBuildingAbsorbed
            | Self::SpellBuildingEnergize
            | Self::SpellBuildingDrain
            | Self::SpellBuildingLeech
            | Self::SpellBuildingInterrupt
            | Self::SpellBuildingDispel
            | Self::SpellBuildingDispelFailed
            | Self::SpellBuildingStolen
            | Self::SpellBuildingExtraAttacks
            | Self::SpellBuildingAuraApplied
            | Self::SpellBuildingAuraRemoved
            | Self::SpellBuildingAuraAppliedDose
            | Self::SpellBuildingAuraRemovedDose
            | Self::SpellBuildingAuraRefresh
            | Self::SpellBuildingAuraBroken
            | Self::SpellBuildingAuraBrokenSpell
            | Self::SpellBuildingCastStart
            | Self::SpellBuildingCastSuccess
            | Self::SpellBuildingCastFailed
            | Self::SpellBuildingInstaKill
            | Self::SpellBuildingDurabilityDamage
            | Self::SpellBuildingDurabilityDamageAll
            | Self::SpellBuildingCreate
            | Self::SpellBuildingSummon
            | Self::SpellBuildingResurrect
            | Self::SpellBuildingEmpowerStart
            | Self::SpellBuildingEmpowerEnd
            | Self::SpellBuildingEmpowerInterrupt => 3,
            Self::EnvironmentalDamage => 1,
            _ => 0,
        }
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::SwingDamage => write!(f, "SWING_DAMAGE"),
            Self::SwingDamageLanded => write!(f, "SWING_DAMAGE_LANDED"),
            Self::SwingMissed => write!(f, "SWING_MISSED"),
            Self::RangeDamage => write!(f, "RANGE_DAMAGE"),
            Self::RangeMissed => write!(f, "RANGE_MISSED"),
            Self::StaggerClear => write!(f, "STAGGER_CLEAR"),
            Self::SpellDamage => write!(f, "SPELL_DAMAGE"),
            Self::SpellDamageSupport => write!(f, "SPELL_DAMAGE_SUPPORT"),
            Self::SpellMissed => write!(f, "SPELL_MISSED"),
            Self::SpellHeal => write!(f, "SPELL_HEAL"),
            Self::SpellHealAbsorbed => write!(f, "SPELL_HEAL_ABSORBED"),
            Self::SpellAbsorbed => write!(f, "SPELL_ABSORBED"),
            Self::SpellEnergize => write!(f, "SPELL_ENERGIZE"),
            Self::SpellDrain => write!(f, "SPELL_DRAIN"),
            Self::SpellLeech => write!(f, "SPELL_LEECH"),
            Self::SpellInterrupt => write!(f, "SPELL_INTERRUPT"),
            Self::SpellDispel => write!(f, "SPELL_DISPEL"),
            Self::SpellDispelFailed => write!(f, "SPELL_DISPEL_FAILED"),
            Self::SpellStolen => write!(f, "SPELL_STOLEN"),
            Self::SpellExtraAttacks => write!(f, "SPELL_EXTRA_ATTACKS"),
            Self::SpellAuraApplied => write!(f, "SPELL_AURA_APPLIED"),
            Self::SpellAuraRemoved => write!(f, "SPELL_AURA_REMOVED"),
            Self::SpellAuraAppliedDose => write!(f, "SPELL_AURA_APPLIED_DOSE"),
            Self::SpellAuraRemovedDose => write!(f, "SPELL_AURA_REMOVED_DOSE"),
            Self::SpellAuraRefresh => write!(f, "SPELL_AURA_REFRESH"),
            Self::SpellAuraBroken => write!(f, "SPELL_AURA_BROKEN"),
            Self::SpellAuraBrokenSpell => write!(f, "SPELL_AURA_BROKEN_SPELL"),
            Self::SpellCastStart => write!(f, "SPELL_CAST_START"),
            Self::SpellCastSuccess => write!(f, "SPELL_CAST_SUCCESS"),
            Self::SpellCastFailed => write!(f, "SPELL_CAST_FAILED"),
            Self::SpellInstaKill => write!(f, "SPELL_INSTAKILL"),
            Self::SpellDurabilityDamage => write!(f, "SPELL_DURABILITY_DAMAGE"),
            Self::SpellDurabilityDamageAll => write!(f, "SPELL_DURABILITY_DAMAGE_ALL"),
            Self::SpellCreate => write!(f, "SPELL_CREATE"),
            Self::SpellSummon => write!(f, "SPELL_SUMMON"),
            Self::SpellResurrect => write!(f, "SPELL_RESURRECT"),
            Self::SpellEmpowerStart => write!(f, "SPELL_EMPOWER_START"),
            Self::SpellEmpowerEnd => write!(f, "SPELL_EMPOWER_END"),
            Self::SpellEmpowerInterrupt => write!(f, "SPELL_EMPOWER_INTERRUPT"),
            Self::SpellPeriodicDamage => write!(f, "SPELL_PERIODIC_DAMAGE"),
            Self::SpellPeriodicDamageSupport => write!(f, "SPELL_PERIODIC_DAMAGE_SUPPORT"),
            Self::SpellPeriodicMissed => write!(f, "SPELL_PERIODIC_MISSED"),
            Self::SpellPeriodicHeal => write!(f, "SPELL_PERIODIC_HEAL"),
            Self::SpellPeriodicHealAbsorbed => write!(f, "SPELL_PERIODIC_HEAL_ABSORBED"),
            Self::SpellPeriodicAbsorbed => write!(f, "SPELL_PERIODIC_ABSORBED"),
            Self::SpellPeriodicEnergize => write!(f, "SPELL_PERIODIC_ENERGIZE"),
            Self::SpellPeriodicDrain => write!(f, "SPELL_PERIODIC_DRAIN"),
            Self::SpellPeriodicLeech => write!(f, "SPELL_PERIODIC_LEECH"),
            Self::SpellPeriodicInterrupt => write!(f, "SPELL_PERIODIC_INTERRUPT"),
            Self::SpellPeriodicDispel => write!(f, "SPELL_PERIODIC_DISPEL"),
            Self::SpellPeriodicDispelFailed => write!(f, "SPELL_PERIODIC_DISPEL_FAILED"),
            Self::SpellPeriodicStolen => write!(f, "SPELL_PERIODIC_STOLEN"),
            Self::SpellPeriodicExtraAttacks => write!(f, "SPELL_PERIODIC_EXTRA_ATTACKS"),
            Self::SpellPeriodicAuraApplied => write!(f, "SPELL_PERIODIC_AURA_APPLIED"),
            Self::SpellPeriodicAuraRemoved => write!(f, "SPELL_PERIODIC_AURA_REMOVED"),
            Self::SpellPeriodicAuraAppliedDose => write!(f, "SPELL_PERIODIC_AURA_APPLIED_DOSE"),
            Self::SpellPeriodicAuraRemovedDose => write!(f, "SPELL_PERIODIC_AURA_REMOVED_DOSE"),
            Self::SpellPeriodicAuraRefresh => write!(f, "SPELL_PERIODIC_AURA_REFRESH"),
            Self::SpellPeriodicAuraBroken => write!(f, "SPELL_PERIODIC_AURA_BROKEN"),
            Self::SpellPeriodicAuraBrokenSpell => write!(f, "SPELL_PERIODIC_AURA_BROKEN_SPELL"),
            Self::SpellPeriodicCastStart => write!(f, "SPELL_PERIODIC_CAST_START"),
            Self::SpellPeriodicCastSuccess => write!(f, "SPELL_PERIODIC_CAST_SUCCESS"),
            Self::SpellPeriodicCastFailed => write!(f, "SPELL_PERIODIC_CAST_FAILED"),
            Self::SpellPeriodicInstaKill => write!(f, "SPELL_PERIODIC_INSTA_KILL"),
            Self::SpellPeriodicDurabilityDamage => write!(f, "SPELL_PERIODIC_DURABILITY_DAMAGE"),
            Self::SpellPeriodicDurabilityDamageAll => {
                write!(f, "SPELL_PERIODIC_DURABILITY_DAMAGE_ALL")
            }
            Self::SpellPeriodicCreate => write!(f, "SPELL_PERIODIC_CREATE"),
            Self::SpellPeriodicSummon => write!(f, "SPELL_PERIODIC_SUMMON"),
            Self::SpellPeriodicResurrect => write!(f, "SPELL_PERIODIC_RESURRECT"),
            Self::SpellPeriodicEmpowerStart => write!(f, "SPELL_PERIODIC_EMPOWER_START"),
            Self::SpellPeriodicEmpowerEnd => write!(f, "SPELL_PERIODIC_EMPOWER_END"),
            Self::SpellPeriodicEmpowerInterrupt => write!(f, "SPELL_PERIODIC_EMPOWER_INTERRUPT"),
            Self::SpellBuildingDamage => write!(f, "SPELL_BUILDING_DAMAGE"),
            Self::SpellBuildingMissed => write!(f, "SPELL_BUILDING_MISSED"),
            Self::SpellBuildingHeal => write!(f, "SPELL_BUILDING_HEAL"),
            Self::SpellBuildingHealAbsorbed => write!(f, "SPELL_BUILDING_HEAL_ABSORBED"),
            Self::SpellBuildingAbsorbed => write!(f, "SPELL_BUILDING_ABSORBED"),
            Self::SpellBuildingEnergize => write!(f, "SPELL_BUILDING_ENERGIZE"),
            Self::SpellBuildingDrain => write!(f, "SPELL_BUILDING_DRAIN"),
            Self::SpellBuildingLeech => write!(f, "SPELL_BUILDING_LEECH"),
            Self::SpellBuildingInterrupt => write!(f, "SPELL_BUILDING_INTERRUPT"),
            Self::SpellBuildingDispel => write!(f, "SPELL_BUILDING_DISPEL"),
            Self::SpellBuildingDispelFailed => write!(f, "SPELL_BUILDING_DISPEL_FAILED"),
            Self::SpellBuildingStolen => write!(f, "SPELL_BUILDING_STOLEN"),
            Self::SpellBuildingExtraAttacks => write!(f, "SPELL_BUILDING_EXTRA_ATTACKS"),
            Self::SpellBuildingAuraApplied => write!(f, "SPELL_BUILDING_AURA_APPLIED"),
            Self::SpellBuildingAuraRemoved => write!(f, "SPELL_BUILDING_AURA_REMOVED"),
            Self::SpellBuildingAuraAppliedDose => write!(f, "SPELL_BUILDING_AURA_APPLIED_DOSE"),
            Self::SpellBuildingAuraRemovedDose => write!(f, "SPELL_BUILDING_AURA_REMOVED_DOSE"),
            Self::SpellBuildingAuraRefresh => write!(f, "SPELL_BUILDING_AURA_REFRESH"),
            Self::SpellBuildingAuraBroken => write!(f, "SPELL_BUILDING_AURA_BROKEN"),
            Self::SpellBuildingAuraBrokenSpell => write!(f, "SPELL_BUILDING_AURA_BROKEN_SPELL"),
            Self::SpellBuildingCastStart => write!(f, "SPELL_BUILDING_CAST_START"),
            Self::SpellBuildingCastSuccess => write!(f, "SPELL_BUILDING_CAST_SUCCESS"),
            Self::SpellBuildingCastFailed => write!(f, "SPELL_BUILDING_CAST_FAILED"),
            Self::SpellBuildingInstaKill => write!(f, "SPELL_BUILDING_INSTA_KILL"),
            Self::SpellBuildingDurabilityDamage => write!(f, "SPELL_BUILDING_DURABILITY_DAMAGE"),
            Self::SpellBuildingDurabilityDamageAll => {
                write!(f, "SPELL_BUILDING_DURABILITY_DAMAGE_ALL")
            }
            Self::SpellBuildingCreate => write!(f, "SPELL_BUILDING_CREATE"),
            Self::SpellBuildingSummon => write!(f, "SPELL_BUILDING_SUMMON"),
            Self::SpellBuildingResurrect => write!(f, "SPELL_BUILDING_RESURRECT"),
            Self::SpellBuildingEmpowerStart => write!(f, "SPELL_BUILDING_EMPOWER_START"),
            Self::SpellBuildingEmpowerEnd => write!(f, "SPELL_BUILDING_EMPOWER_END"),
            Self::SpellBuildingEmpowerInterrupt => write!(f, "SPELL_BUILDING_EMPOWER_INTERRUPT"),
            Self::EnvironmentalDamage => write!(f, "ENVIRONMENTAL_DAMAGE"),
            Self::DamageSplit => write!(f, "DAMAGE_SPLIT"),
            Self::DamageShield => write!(f, "DAMAGE_SHIELD"),
            Self::DamageShieldMissed => write!(f, "DAMAGE_SHIELD_MISSED"),
            Self::EnchantApplied => write!(f, "ENCHANT_APPLIED"),
            Self::EnchantRemoved => write!(f, "ENCHANT_REMOVED"),
            Self::PartyKill => write!(f, "PARTY_KILL"),
            Self::UnitDied => write!(f, "UNIT_DIED"),
            Self::UnitDestroyed => write!(f, "UNIT_DESTROYED"),
            Self::UnitDissapates => write!(f, "UNIT_DISSAPATES"),
            Self::CombatLogVersion => write!(f, "COMBAT_LOG_VERSION"),
            Self::ZoneChange => write!(f, "ZONE_CHANGE"),
            Self::MapChange => write!(f, "MAP_CHANGE"),
            Self::EncounterStart => write!(f, "ENCOUNTER_START"),
            Self::EncounterEnd => write!(f, "ENCOUNTER_END"),
            Self::CombatantInfo => write!(f, "COMBATANT_INFO"),
            Self::ArenaMatchStart => write!(f, "ARENA_MATCH_START"),
            Self::ArenaMatchEnd => write!(f, "ARENA_MATCH_END"),
            Self::ChallengeModeStart => write!(f, "CHALLENGE_MODE_START"),
            Self::ChallengeModeEnd => write!(f, "CHALLENGE_MODE_END"),
            Self::Emote => write!(f, "EMOTE"),
            Self::WorldMarkerPlaced => write!(f, "WORLD_MARKER_PLACED"),
            Self::WorldMarkerRemoved => write!(f, "WORLD_MARKER_REMOVED"),
            Self::UnknownEvent(ref event) => write!(f, "{event}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SpellSchool {
    Physical,
    Holy,
    Fire,
    Nature,
    Frost,
    Shadow,
    Arcane,
    Holystrike,
    Flamestrike,
    Radiant,
    Stormstrike,
    Holystorm,
    Volcanic,
    Froststrike,
    Holyfrost,
    Frostfire,
    Froststorm,
    Shadowstrike,
    Twilight,
    Shadowflame,
    Plague,
    Shadowfrost,
    Spellstrike,
    Divine,
    Spellfire,
    Astral,
    Spellfrost,
    Spellshadow,
    Elemental,
    Chromatic,
    Cosmic,
    LimitedChaos,
    Magic,
    Chaos,
    Unknown(u8),
}

impl SpellSchool {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Physical,
            2 => Self::Holy,
            4 => Self::Fire,
            8 => Self::Nature,
            16 => Self::Frost,
            32 => Self::Shadow,
            64 => Self::Arcane,
            3 => Self::Holystrike,
            5 => Self::Flamestrike,
            6 => Self::Radiant,
            9 => Self::Stormstrike,
            10 => Self::Holystorm,
            12 => Self::Volcanic,
            17 => Self::Froststrike,
            18 => Self::Holyfrost,
            20 => Self::Frostfire,
            24 => Self::Froststorm,
            33 => Self::Shadowstrike,
            34 => Self::Twilight,
            36 => Self::Shadowflame,
            40 => Self::Plague,
            48 => Self::Shadowfrost,
            65 => Self::Spellstrike,
            66 => Self::Divine,
            68 => Self::Spellfire,
            72 => Self::Astral,
            80 => Self::Spellfrost,
            96 => Self::Spellshadow,
            28 => Self::Elemental,
            62 => Self::Chromatic,
            106 => Self::Cosmic,
            124 => Self::LimitedChaos,
            126 => Self::Magic,
            127 => Self::Chaos,
            _ => Self::Unknown(value),
        }
    }

    pub fn as_u8(&self) -> u8 {
        match *self {
            Self::Physical => 1,
            Self::Holy => 2,
            Self::Fire => 4,
            Self::Nature => 8,
            Self::Frost => 16,
            Self::Shadow => 32,
            Self::Arcane => 64,
            Self::Holystrike => 3,
            Self::Flamestrike => 5,
            Self::Radiant => 6,
            Self::Stormstrike => 9,
            Self::Holystorm => 10,
            Self::Volcanic => 12,
            Self::Froststrike => 17,
            Self::Holyfrost => 18,
            Self::Frostfire => 20,
            Self::Froststorm => 24,
            Self::Shadowstrike => 33,
            Self::Twilight => 34,
            Self::Shadowflame => 36,
            Self::Plague => 40,
            Self::Shadowfrost => 48,
            Self::Spellstrike => 65,
            Self::Divine => 66,
            Self::Spellfire => 68,
            Self::Astral => 72,
            Self::Spellfrost => 80,
            Self::Spellshadow => 96,
            Self::Elemental => 28,
            Self::Chromatic => 62,
            Self::Cosmic => 106,
            Self::LimitedChaos => 124,
            Self::Magic => 126,
            Self::Chaos => 127,
            Self::Unknown(val) => val,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum PowerType {
    Mana, // If unsure, it's probably Mana
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

    // Unsure what these are used for?
    RuneBlood,
    RuneFrost,
    RuneUnholy,
    AlternateQuest,
    AlternateEncounter,
    AlternateMount, // Vigor for Sky Riding
    Balance,
    Happiness,
    ShadowOrbs,
    RuneChromatic,
}

impl From<u8> for PowerType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Mana,
            1 => Self::Rage,
            2 => Self::Focus,
            3 => Self::Energy,
            4 => Self::ComboPoints,
            5 => Self::Runes,
            6 => Self::RunicPower,
            7 => Self::SoulShards,
            8 => Self::LunarPower,
            9 => Self::HolyPower,
            10 => Self::Alternate,
            11 => Self::Maelstrom,
            12 => Self::Chi,
            13 => Self::Insanity,
            14 => Self::BurningEmbers,
            15 => Self::DemonicFury,
            16 => Self::ArcaneCharges,
            17 => Self::Fury,
            18 => Self::Pain,
            19 => Self::Essence,
            20 => Self::RuneBlood,
            21 => Self::RuneFrost,
            22 => Self::RuneUnholy,
            23 => Self::AlternateQuest,
            24 => Self::AlternateEncounter,
            25 => Self::AlternateMount,
            26 => Self::Balance,
            27 => Self::Happiness,
            28 => Self::ShadowOrbs,
            29 => Self::RuneChromatic,
            _ => Self::Mana,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
    Unknown(String),
}

impl MissType {
    pub fn from_str(value: &str) -> Self {
        match value {
            "ABSORB" => Self::Absorb,
            "BLOCK" => Self::Block,
            "DEFLECT" => Self::Deflect,
            "DODGE" => Self::Dodge,
            "EVADE" => Self::Evade,
            "IMMUNE" => Self::Immune,
            "MISS" => Self::Miss,
            "PARRY" => Self::Parry,
            "REFLECT" => Self::Reflect,
            "RESIST" => Self::Resist,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

impl std::fmt::Display for MissType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Absorb => write!(f, "ABSORB"),
            Self::Block => write!(f, "BLOCK"),
            Self::Deflect => write!(f, "DEFLECT"),
            Self::Dodge => write!(f, "DODGE"),
            Self::Evade => write!(f, "EVADE"),
            Self::Immune => write!(f, "IMMUNE"),
            Self::Miss => write!(f, "MISS"),
            Self::Parry => write!(f, "PARRY"),
            Self::Reflect => write!(f, "REFLECT"),
            Self::Resist => write!(f, "RESIST"),
            Self::Unknown(ref event) => write!(f, "{event}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AuraType {
    Buff,
    Debuff,
    Unknown(String),
}

impl AuraType {
    pub fn from_str(value: &str) -> Self {
        match value {
            "BUFF" => Self::Buff,
            "DEBUFF" => Self::Debuff,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

impl std::fmt::Display for AuraType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Buff => write!(f, "BUFF"),
            Self::Debuff => write!(f, "DEBUFF"),
            Self::Unknown(ref aura) => write!(f, "{aura}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnvironmentalType {
    Drowning,
    Falling,
    Fatigue,
    Fire,
    Lava,
    Slime,
    Unknown(String),
}

impl EnvironmentalType {
    pub fn from_str(value: &str) -> Self {
        match value {
            "Drowning" => Self::Drowning,
            "Falling" => Self::Falling,
            "Fatigue" => Self::Fatigue,
            "Fire" => Self::Fire,
            "Lava" => Self::Lava,
            "Slime" => Self::Slime,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

impl std::fmt::Display for EnvironmentalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Drowning => write!(f, "Drowning"),
            Self::Falling => write!(f, "Falling"),
            Self::Fatigue => write!(f, "Fatigue"),
            Self::Fire => write!(f, "Fire"),
            Self::Lava => write!(f, "Lava"),
            Self::Slime => write!(f, "Slime"),
            Self::Unknown(ref env) => write!(f, "{env}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Affix {
    Overflowing,
    Skittish,
    Volcanic,
    Necrotic,
    Teeming,
    Raging,
    Bolstering,
    Sanguine,
    Tyrannical,
    Fortified,
    Bursting,
    Grievous,
    Explosive,
    Quaking,
    Infested,
    Reaping,
    Beguiling,
    Awakened,
    Prideful,
    Inspiring,
    Spiteful,
    Storming,
    Tormented,
    Infernal,
    Encrypted,
    Shrouded,
    Thundering,
    Focused,
    Entangling,
    Afflicted,
    Incorporeal,
    Shielding,
    Thorned,
    Reckless,
    Attuned,
    XalGuile,
    XalBargainAscendant,
    ChallengerPeril,
    XalBargainFrenzied,
    XalBargainVoidbound,
    XalBargainOblivion,
    XalBargainDevour,
    XalBargainPulsar,
    Unknown(u16),
}

impl From<u16> for Affix {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::Overflowing,
            2 => Self::Skittish,
            3 => Self::Volcanic,
            4 => Self::Necrotic,
            5 => Self::Teeming,
            6 => Self::Raging,
            7 => Self::Bolstering,
            8 => Self::Sanguine,
            9 => Self::Tyrannical,
            10 => Self::Fortified,
            11 => Self::Bursting,
            12 => Self::Grievous,
            13 => Self::Explosive,
            14 => Self::Quaking,
            16 => Self::Infested,
            117 => Self::Reaping,
            119 => Self::Beguiling,
            120 => Self::Awakened,
            121 => Self::Prideful,
            122 => Self::Inspiring,
            123 => Self::Spiteful,
            124 => Self::Storming,
            128 => Self::Tormented,
            129 => Self::Infernal,
            130 => Self::Encrypted,
            131 => Self::Shrouded,
            132 => Self::Thundering,
            133 => Self::Focused,
            134 => Self::Entangling,
            135 => Self::Afflicted,
            136 => Self::Incorporeal,
            137 => Self::Shielding,
            144 => Self::Thorned,
            145 => Self::Reckless,
            146 => Self::Attuned,
            147 => Self::XalGuile,
            148 => Self::XalBargainAscendant,
            152 => Self::ChallengerPeril,
            153 => Self::XalBargainFrenzied,
            158 => Self::XalBargainVoidbound,
            159 => Self::XalBargainOblivion,
            160 => Self::XalBargainDevour,
            162 => Self::XalBargainPulsar,
            _ => Self::Unknown(value),
        }
    }
}

impl std::fmt::Display for Affix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Overflowing => write!(f, "Affix: \"Overflowing\""),
            Self::Skittish => write!(f, "Affix: \"Skittish\""),
            Self::Volcanic => write!(f, "Affix: \"Volcanic\""),
            Self::Necrotic => write!(f, "Affix: \"Necrotic\""),
            Self::Teeming => write!(f, "Affix: \"Teeming\""),
            Self::Raging => write!(f, "Affix: \"Raging\""),
            Self::Bolstering => write!(f, "Affix: \"Bolstering\""),
            Self::Sanguine => write!(f, "Affix: \"Sanguine\""),
            Self::Tyrannical => write!(f, "Affix: \"Tyrannical\""),
            Self::Fortified => write!(f, "Affix: \"Fortified\""),
            Self::Bursting => write!(f, "Affix: \"Bursting\""),
            Self::Grievous => write!(f, "Affix: \"Grievous\""),
            Self::Explosive => write!(f, "Affix: \"Explosive\""),
            Self::Quaking => write!(f, "Affix: \"Quaking\""),
            Self::Infested => write!(f, "Affix: \"Infested\""),
            Self::Reaping => write!(f, "Affix: \"Reaping\""),
            Self::Beguiling => write!(f, "Affix: \"Beguiling\""),
            Self::Awakened => write!(f, "Affix: \"Awakened\""),
            Self::Prideful => write!(f, "Affix: \"Prideful\""),
            Self::Inspiring => write!(f, "Affix: \"Inspiring\""),
            Self::Spiteful => write!(f, "Affix: \"Spiteful\""),
            Self::Storming => write!(f, "Affix: \"Storming\""),
            Self::Tormented => write!(f, "Affix: \"Tormented\""),
            Self::Infernal => write!(f, "Affix: \"Infernal\""),
            Self::Encrypted => write!(f, "Affix: \"Encrypted\""),
            Self::Shrouded => write!(f, "Affix: \"Shrouded\""),
            Self::Thundering => write!(f, "Affix: \"Thundering\""),
            Self::Focused => write!(f, "Affix: \"Focused\""),
            Self::Entangling => write!(f, "Affix: \"Entangling\""),
            Self::Afflicted => write!(f, "Affix: \"Afflicted\""),
            Self::Incorporeal => write!(f, "Affix: \"Incorporeal\""),
            Self::Shielding => write!(f, "Affix: \"Shielding\""),
            Self::Thorned => write!(f, "Affix: \"Thorned\""),
            Self::Reckless => write!(f, "Affix: \"Reckless\""),
            Self::Attuned => write!(f, "Affix: \"Attuned\""),
            Self::XalGuile => write!(f, "Affix: \"Xal'atath's Guile\""),
            Self::XalBargainAscendant => write!(f, "Affix: \"Xal'atath's Bargain: Ascendant\""),
            Self::ChallengerPeril => write!(f, "Affix: \"Challenger's Peril\""),
            Self::XalBargainFrenzied => write!(f, "Affix: \"Xal'atath's Bargain: Frenzied\""),
            Self::XalBargainVoidbound => write!(f, "Affix: \"Xal'atath's Bargain: Voidbound\""),
            Self::XalBargainOblivion => write!(f, "Affix: \"Xal'atath's Bargain: Oblivion\""),
            Self::XalBargainDevour => write!(f, "Affix: \"Xal'atath's Bargain: Devour\""),
            Self::XalBargainPulsar => write!(f, "Affix: \"Xal'atath's Bargain: Pulser\""),
            Self::Unknown(value) => write!(f, "Unknown affix id: {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContentDifficulty {
    Unknown(u8),
}

impl From<u8> for ContentDifficulty {
    fn from(value: u8) -> Self {
        match value {
            _ => Self::Unknown(value),
        }
    }
}

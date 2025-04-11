use std::str::FromStr;

use crate::error::JastorError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    SpellHealSupport,
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
}

impl FromStr for EventType {
    type Err = JastorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SWING_DAMAGE" => Ok(Self::SwingDamage),
            "SWING_DAMAGE_LANDED" => Ok(Self::SwingDamageLanded),
            "SWING_MISSED" => Ok(Self::SwingMissed),
            "RANGE_DAMAGE" => Ok(Self::RangeDamage),
            "RANGE_MISSED" => Ok(Self::RangeMissed),
            "STAGGER_CLEAR" => Ok(Self::StaggerClear),
            "SPELL_DAMAGE" => Ok(Self::SpellDamage),
            "SPELL_DAMAGE_SUPPORT" => Ok(Self::SpellDamageSupport),
            "SPELL_MISSED" => Ok(Self::SpellMissed),
            "SPELL_HEAL" => Ok(Self::SpellHeal),
            "SPELL_HEAL_SUPPORT" => Ok(Self::SpellHealSupport),
            "SPELL_HEAL_ABSORBED" => Ok(Self::SpellHealAbsorbed),
            "SPELL_ABSORBED" => Ok(Self::SpellAbsorbed),
            "SPELL_ENERGIZE" => Ok(Self::SpellEnergize),
            "SPELL_DRAIN" => Ok(Self::SpellDrain),
            "SPELL_LEECH" => Ok(Self::SpellLeech),
            "SPELL_INTERRUPT" => Ok(Self::SpellInterrupt),
            "SPELL_DISPEL" => Ok(Self::SpellDispel),
            "SPELL_DISPEL_FAILED" => Ok(Self::SpellDispelFailed),
            "SPELL_STOLEN" => Ok(Self::SpellStolen),
            "SPELL_EXTRA_ATTACKS" => Ok(Self::SpellExtraAttacks),
            "SPELL_AURA_APPLIED" => Ok(Self::SpellAuraApplied),
            "SPELL_AURA_REMOVED" => Ok(Self::SpellAuraRemoved),
            "SPELL_AURA_APPLIED_DOSE" => Ok(Self::SpellAuraAppliedDose),
            "SPELL_AURA_REMOVED_DOSE" => Ok(Self::SpellAuraRemovedDose),
            "SPELL_AURA_REFRESH" => Ok(Self::SpellAuraRefresh),
            "SPELL_AURA_BROKEN" => Ok(Self::SpellAuraBroken),
            "SPELL_AURA_BROKEN_SPELL" => Ok(Self::SpellAuraBrokenSpell),
            "SPELL_CAST_START" => Ok(Self::SpellCastStart),
            "SPELL_CAST_SUCCESS" => Ok(Self::SpellCastSuccess),
            "SPELL_CAST_FAILED" => Ok(Self::SpellCastFailed),
            "SPELL_INSTAKILL" => Ok(Self::SpellInstaKill),
            "SPELL_DURABILITY_DAMAGE" => Ok(Self::SpellDurabilityDamage),
            "SPELL_DURABILITY_DAMAGE_ALL" => Ok(Self::SpellDurabilityDamageAll),
            "SPELL_CREATE" => Ok(Self::SpellCreate),
            "SPELL_SUMMON" => Ok(Self::SpellSummon),
            "SPELL_RESURRECT" => Ok(Self::SpellResurrect),
            "SPELL_EMPOWER_START" => Ok(Self::SpellEmpowerStart),
            "SPELL_EMPOWER_END" => Ok(Self::SpellEmpowerEnd),
            "SPELL_EMPOWER_INTERRUPT" => Ok(Self::SpellEmpowerInterrupt),
            "SPELL_PERIODIC_DAMAGE" => Ok(Self::SpellPeriodicDamage),
            "SPELL_PERIODIC_DAMAGE_SUPPORT" => Ok(Self::SpellPeriodicDamageSupport),
            "SPELL_PERIODIC_MISSED" => Ok(Self::SpellPeriodicMissed),
            "SPELL_PERIODIC_HEAL" => Ok(Self::SpellPeriodicHeal),
            "SPELL_PERIODIC_HEAL_ABSORBED" => Ok(Self::SpellPeriodicHealAbsorbed),
            "SPELL_PERIODIC_ABSORBED" => Ok(Self::SpellPeriodicAbsorbed),
            "SPELL_PERIODIC_ENERGIZE" => Ok(Self::SpellPeriodicEnergize),
            "SPELL_PERIODIC_DRAIN" => Ok(Self::SpellPeriodicDrain),
            "SPELL_PERIODIC_LEECH" => Ok(Self::SpellPeriodicLeech),
            "SPELL_PERIODIC_INTERRUPT" => Ok(Self::SpellPeriodicInterrupt),
            "SPELL_PERIODIC_DISPEL" => Ok(Self::SpellPeriodicDispel),
            "SPELL_PERIODIC_DISPEL_FAILED" => Ok(Self::SpellPeriodicDispelFailed),
            "SPELL_PERIODIC_STOLEN" => Ok(Self::SpellPeriodicStolen),
            "SPELL_PERIODIC_EXTRA_ATTACKS" => Ok(Self::SpellPeriodicExtraAttacks),
            "SPELL_PERIODIC_AURA_APPLIED" => Ok(Self::SpellPeriodicAuraApplied),
            "SPELL_PERIODIC_AURA_REMOVED" => Ok(Self::SpellPeriodicAuraRemoved),
            "SPELL_PERIODIC_AURA_APPLIED_DOSE" => Ok(Self::SpellPeriodicAuraAppliedDose),
            "SPELL_PERIODIC_AURA_REMOVED_DOSE" => Ok(Self::SpellPeriodicAuraRemovedDose),
            "SPELL_PERIODIC_AURA_REFRESH" => Ok(Self::SpellPeriodicAuraRefresh),
            "SPELL_PERIODIC_AURA_BROKEN" => Ok(Self::SpellPeriodicAuraBroken),
            "SPELL_PERIODIC_AURA_BROKEN_SPELL" => Ok(Self::SpellPeriodicAuraBrokenSpell),
            "SPELL_PERIODIC_CAST_START" => Ok(Self::SpellPeriodicCastStart),
            "SPELL_PERIODIC_CAST_SUCCESS" => Ok(Self::SpellPeriodicCastSuccess),
            "SPELL_PERIODIC_CAST_FAILED" => Ok(Self::SpellPeriodicCastFailed),
            "SPELL_PERIODIC_INSTA_KILL" => Ok(Self::SpellPeriodicInstaKill),
            "SPELL_PERIODIC_DURABILITY_DAMAGE" => Ok(Self::SpellPeriodicDurabilityDamage),
            "SPELL_PERIODIC_DURABILITY_DAMAGE_ALL" => Ok(Self::SpellPeriodicDurabilityDamageAll),
            "SPELL_PERIODIC_CREATE" => Ok(Self::SpellPeriodicCreate),
            "SPELL_PERIODIC_SUMMON" => Ok(Self::SpellPeriodicSummon),
            "SPELL_PERIODIC_RESURRECT" => Ok(Self::SpellPeriodicResurrect),
            "SPELL_PERIODIC_EMPOWER_START" => Ok(Self::SpellPeriodicEmpowerStart),
            "SPELL_PERIODIC_EMPOWER_END" => Ok(Self::SpellPeriodicEmpowerEnd),
            "SPELL_PERIODIC_EMPOWER_INTERRUPT" => Ok(Self::SpellPeriodicEmpowerInterrupt),
            "SPELL_BUILDING_DAMAGE" => Ok(Self::SpellBuildingDamage),
            "SPELL_BUILDING_MISSED" => Ok(Self::SpellBuildingMissed),
            "SPELL_BUILDING_HEAL" => Ok(Self::SpellBuildingHeal),
            "SPELL_BUILDING_HEAL_ABSORBED" => Ok(Self::SpellBuildingHealAbsorbed),
            "SPELL_BUILDING_ABSORBED" => Ok(Self::SpellBuildingAbsorbed),
            "SPELL_BUILDING_ENERGIZE" => Ok(Self::SpellBuildingEnergize),
            "SPELL_BUILDING_DRAIN" => Ok(Self::SpellBuildingDrain),
            "SPELL_BUILDING_LEECH" => Ok(Self::SpellBuildingLeech),
            "SPELL_BUILDING_INTERRUPT" => Ok(Self::SpellBuildingInterrupt),
            "SPELL_BUILDING_DISPEL" => Ok(Self::SpellBuildingDispel),
            "SPELL_BUILDING_DISPEL_FAILED" => Ok(Self::SpellBuildingDispelFailed),
            "SPELL_BUILDING_STOLEN" => Ok(Self::SpellBuildingStolen),
            "SPELL_BUILDING_EXTRA_ATTACKS" => Ok(Self::SpellBuildingExtraAttacks),
            "SPELL_BUILDING_AURA_APPLIED" => Ok(Self::SpellBuildingAuraApplied),
            "SPELL_BUILDING_AURA_REMOVED" => Ok(Self::SpellBuildingAuraRemoved),
            "SPELL_BUILDING_AURA_APPLIED_DOSE" => Ok(Self::SpellBuildingAuraAppliedDose),
            "SPELL_BUILDING_AURA_REMOVED_DOSE" => Ok(Self::SpellBuildingAuraRemovedDose),
            "SPELL_BUILDING_AURA_REFRESH" => Ok(Self::SpellBuildingAuraRefresh),
            "SPELL_BUILDING_AURA_BROKEN" => Ok(Self::SpellBuildingAuraBroken),
            "SPELL_BUILDING_AURA_BROKEN_SPELL" => Ok(Self::SpellBuildingAuraBrokenSpell),
            "SPELL_BUILDING_CAST_START" => Ok(Self::SpellBuildingCastStart),
            "SPELL_BUILDING_CAST_SUCCESS" => Ok(Self::SpellBuildingCastSuccess),
            "SPELL_BUILDING_CAST_FAILED" => Ok(Self::SpellBuildingCastFailed),
            "SPELL_BUILDING_INSTA_KILL" => Ok(Self::SpellBuildingInstaKill),
            "SPELL_BUILDING_DURABILITY_DAMAGE" => Ok(Self::SpellBuildingDurabilityDamage),
            "SPELL_BUILDING_DURABILITY_DAMAGE_ALL" => Ok(Self::SpellBuildingDurabilityDamageAll),
            "SPELL_BUILDING_CREATE" => Ok(Self::SpellBuildingCreate),
            "SPELL_BUILDING_SUMMON" => Ok(Self::SpellBuildingSummon),
            "SPELL_BUILDING_RESURRECT" => Ok(Self::SpellBuildingResurrect),
            "SPELL_BUILDING_EMPOWER_START" => Ok(Self::SpellBuildingEmpowerStart),
            "SPELL_BUILDING_EMPOWER_END" => Ok(Self::SpellBuildingEmpowerEnd),
            "SPELL_BUILDING_EMPOWER_INTERRUPT" => Ok(Self::SpellBuildingEmpowerInterrupt),
            "ENVIRONMENTAL_DAMAGE" => Ok(Self::EnvironmentalDamage),
            "DAMAGE_SPLIT" => Ok(Self::DamageSplit),
            "DAMAGE_SHIELD" => Ok(Self::DamageShield),
            "DAMAGE_SHIELD_MISSED" => Ok(Self::DamageShieldMissed),
            "ENCHANT_APPLIED" => Ok(Self::EnchantApplied),
            "ENCHANT_REMOVED" => Ok(Self::EnchantRemoved),
            "PARTY_KILL" => Ok(Self::PartyKill),
            "UNIT_DIED" => Ok(Self::UnitDied),
            "UNIT_DESTROYED" => Ok(Self::UnitDestroyed),
            "UNIT_DISSAPATES" => Ok(Self::UnitDissapates),
            "COMBAT_LOG_VERSION" => Ok(Self::CombatLogVersion),
            "ZONE_CHANGE" => Ok(Self::ZoneChange),
            "MAP_CHANGE" => Ok(Self::MapChange),
            "ENCOUNTER_START" => Ok(Self::EncounterStart),
            "ENCOUNTER_END" => Ok(Self::EncounterEnd),
            "COMBATANT_INFO" => Ok(Self::CombatantInfo),
            "ARENA_MATCH_START" => Ok(Self::ArenaMatchStart),
            "ARENA_MATCH_END" => Ok(Self::ArenaMatchEnd),
            "CHALLENGE_MODE_START" => Ok(Self::ChallengeModeStart),
            "CHALLENGE_MODE_END" => Ok(Self::ChallengeModeEnd),
            "EMOTE" => Ok(Self::Emote),
            "WORLD_MARKER_PLACED" => Ok(Self::WorldMarkerPlaced),
            "WORLD_MARKER_REMOVED" => Ok(Self::WorldMarkerRemoved),
            _ => Err(JastorError::UnknownEvent(s.to_string())),
        }
    }
}

impl EventType {
    pub fn skip(&self) -> bool {
        matches!(*self, Self::Emote | Self::CombatantInfo)
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
            | Self::SpellHealSupport
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

    pub fn has_short_parameters(&self) -> bool {
        matches!(
            *self,
            Self::CombatLogVersion
                | Self::StaggerClear
                | Self::ArenaMatchStart
                | Self::ArenaMatchEnd
                | Self::EncounterStart
                | Self::EncounterEnd
                | Self::ChallengeModeStart
                | Self::ChallengeModeEnd
                | Self::WorldMarkerPlaced
                | Self::WorldMarkerRemoved
                | Self::MapChange
                | Self::ZoneChange
        )
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
            Self::SpellHealSupport => write!(f, "SPELL_HEAL_SUPPORT"),
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
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl FromStr for MissType {
    type Err = JastorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
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
            _ => Err(JastorError::UnknownValue(format!("MissType: {s}"))),
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
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AuraType {
    Buff,
    Debuff,
}

impl FromStr for AuraType {
    type Err = JastorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUFF" => Ok(Self::Buff),
            "DEBUFF" => Ok(Self::Debuff),
            _ => Err(JastorError::UnknownValue(format!("AuraType: {s}"))),
        }
    }
}

impl std::fmt::Display for AuraType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Buff => write!(f, "BUFF"),
            Self::Debuff => write!(f, "DEBUFF"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EnvironmentalType {
    Drowning,
    Falling,
    Fatigue,
    Fire,
    Lava,
    Slime,
}

impl FromStr for EnvironmentalType {
    type Err = JastorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Drowning" => Ok(Self::Drowning),
            "Falling" => Ok(Self::Falling),
            "Fatigue" => Ok(Self::Fatigue),
            "Fire" => Ok(Self::Fire),
            "Lava" => Ok(Self::Lava),
            "Slime" => Ok(Self::Slime),
            _ => Err(JastorError::UnknownValue(format!("EnvironmentalType: {s}"))),
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
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

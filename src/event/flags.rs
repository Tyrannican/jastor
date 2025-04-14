// Unit Affiliation Flags
const UNIT_AFFILIATION_MINE: u32 = 0x00000001;
const UNIT_AFFILIATION_PARTY: u32 = 0x00000002;
const UNIT_AFFILIATION_RAID: u32 = 0x00000004;
const UNIT_AFFILIATION_OUTSIDER: u32 = 0x00000008;
const UNIT_AFFILIATION_MASK: u32 = 0x0000000F;

// Unit Reaction Flags
const UNIT_REACTION_FRIENDLY: u32 = 0x00000010;
const UNIT_REACTION_NEUTRAL: u32 = 0x00000020;
const UNIT_REACTION_HOSTILE: u32 = 0x00000040;
const UNIT_REACTION_MASK: u32 = 0x000000F0;

// Unit Control Flags
const UNIT_CONTROL_PLAYER: u32 = 0x00000100;
const UNIT_CONTROL_NPC: u32 = 0x00000200;
const UNIT_CONTROL_MASK: u32 = 0x00000300;

// Unit Type Flags
const UNIT_TYPE_PLAYER: u32 = 0x00000400;
const UNIT_TYPE_NPC: u32 = 0x00000800;
const UNIT_TYPE_PET: u32 = 0x00001000;
const UNIT_TYPE_GUARDIAN: u32 = 0x00002000;
const UNIT_TYPE_OBJECT: u32 = 0x00004000;
const UNIT_TYPE_MASK: u32 = 0x0000FC00;

// Unit Object Flags
const UNIT_OBJECT_TARGET: u32 = 0x00010000;
const UNIT_OBJECT_FOCUS: u32 = 0x00020000;
const UNIT_OBJECT_MAINTANK: u32 = 0x00040000;
const UNIT_OBJECT_MAINASSIST: u32 = 0x00080000;
// const UNIT_OBJECT_NONE: u32 = 0x80000000;
const UNIT_OBJECT_SPECIAL_MASK: u32 = 0xFFFF0000;

// Raid Target Flags
const RAID_TARGET_STAR: u32 = 0x00000001;
const RAID_TARGET_CIRCLE: u32 = 0x00000002;
const RAID_TARGET_DIAMOND: u32 = 0x00000004;
const RAID_TARGET_TRIANGLE: u32 = 0x00000008;
const RAID_TARGET_MOON: u32 = 0x00000010;
const RAID_TARGET_SQUARE: u32 = 0x00000020;
const RAID_TARGET_CROSS: u32 = 0x00000040;
const RAID_TARGET_SKULL: u32 = 0x00000080;
const RAID_TARGET_MASK: u32 = 0x000000FF;

#[derive(Debug, Clone)]
pub struct UnitFlag {
    pub affiliation: UnitAffiliation,
    pub reaction: UnitReaction,
    pub controller: UnitController,
    pub unit_type: UnitType,
    pub special: UnitSpecial,
}

impl UnitFlag {
    pub fn parse(flag: u32) -> Self {
        let affiliation = match flag & UNIT_AFFILIATION_MASK {
            UNIT_AFFILIATION_MINE => UnitAffiliation::Mine,
            UNIT_AFFILIATION_PARTY => UnitAffiliation::Party,
            UNIT_AFFILIATION_RAID => UnitAffiliation::Raid,
            UNIT_AFFILIATION_OUTSIDER => UnitAffiliation::Outsider,
            _ => unreachable!("WoW ensures the flag is always valid"),
        };

        let reaction = match flag & UNIT_REACTION_MASK {
            UNIT_REACTION_FRIENDLY => UnitReaction::Friendly,
            UNIT_REACTION_NEUTRAL => UnitReaction::Neutral,
            UNIT_REACTION_HOSTILE => UnitReaction::Hostile,
            _ => unreachable!("WoW ensures the flag is always valid"),
        };

        let controller = match flag & UNIT_CONTROL_MASK {
            UNIT_CONTROL_PLAYER => UnitController::Player,
            UNIT_CONTROL_NPC => UnitController::Npc,
            _ => unreachable!("WoW ensures the flag is always valid"),
        };

        let unit_type = match flag & UNIT_TYPE_MASK {
            UNIT_TYPE_PLAYER => UnitType::Player,
            UNIT_TYPE_NPC => UnitType::Npc,
            UNIT_TYPE_PET => UnitType::Pet,
            UNIT_TYPE_GUARDIAN => UnitType::Guardian,
            UNIT_TYPE_OBJECT => UnitType::Object,
            _ => unreachable!("WoW ensures the flag is always valid"),
        };

        let special = match flag & UNIT_OBJECT_SPECIAL_MASK {
            UNIT_OBJECT_TARGET => UnitSpecial::Target,
            UNIT_OBJECT_FOCUS => UnitSpecial::Focus,
            UNIT_OBJECT_MAINTANK => UnitSpecial::MainTank,
            UNIT_OBJECT_MAINASSIST => UnitSpecial::MainAssist,
            _ => UnitSpecial::None,
        };

        Self {
            affiliation,
            reaction,
            controller,
            unit_type,
            special,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitAffiliation {
    Mine,
    Party,
    Raid,
    Outsider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitReaction {
    Friendly,
    Neutral,
    Hostile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitController {
    Player,
    Npc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Player,
    Npc,
    Pet,
    Guardian,
    Object,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitSpecial {
    Target,
    Focus,
    MainTank,
    MainAssist,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaidMarker {
    Star,
    Circle,
    Diamond,
    Triangle,
    Moon,
    Square,
    Cross,
    Skull,
    None,
}

impl RaidMarker {
    pub fn parse_flag(flag: u32) -> Self {
        match flag & RAID_TARGET_MASK {
            RAID_TARGET_STAR => Self::Star,
            RAID_TARGET_CIRCLE => Self::Circle,
            RAID_TARGET_DIAMOND => Self::Diamond,
            RAID_TARGET_TRIANGLE => Self::Triangle,
            RAID_TARGET_MOON => Self::Moon,
            RAID_TARGET_SQUARE => Self::Square,
            RAID_TARGET_CROSS => Self::Cross,
            RAID_TARGET_SKULL => Self::Skull,
            _ => Self::None,
        }
    }
}

impl From<u8> for RaidMarker {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Star,
            2 => Self::Circle,
            3 => Self::Diamond,
            4 => Self::Triangle,
            5 => Self::Moon,
            6 => Self::Square,
            7 => Self::Cross,
            8 => Self::Skull,
            _ => Self::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Difficulty {
    NormalParty,
    HeroicParty,
    NormalRaid10,
    NormalRaid25,
    HeroicRaid10,
    HeroicRaid25,
    LegacyLFR,
    MythicKeystone,
    LegacyRaid40,
    HeroicScenario,
    NormalScenario,
    NormalRaid,
    HeroicRaid,
    MythicRaid,
    LFR,
    EventRaid,
    EventParty,
    EventScenario,
    MythicParty,
    TimewalkingParty,
    WorldPvpScenario,
    PvevpScenario,
    EventScenario2,
    WorldPvpScenario2,
    TimewalkingRaid,
    Pvp,
    NormalScenario2,
    HeroicScenario2,
    MythicScenario,
    PvpScenario,
    NormalWarfront,
    HeroicWarfront,
    NormalParty2,
    TimewalkingLFR,
    NZothVision,
    TeemingIsland,
    Torghast,
    PathOfAscensionCourage,
    PathOfAscensionLoyalty,
    PathOfAscensionWisdom,
    PathOfAscensionHumility,
    WorldBoss,
    ChallengeLevel1,
    FollowerParty,
    Delve,
    Quest,
    StoryRaid,
    HeroicNone,
    Unknown(u16),
}

impl From<Difficulty> for u16 {
    fn from(val: Difficulty) -> Self {
        match val {
            Difficulty::NormalParty => 1,
            Difficulty::HeroicParty => 2,
            Difficulty::NormalRaid10 => 3,
            Difficulty::NormalRaid25 => 4,
            Difficulty::HeroicRaid10 => 5,
            Difficulty::HeroicRaid25 => 6,
            Difficulty::LegacyLFR => 7,
            Difficulty::MythicKeystone => 8,
            Difficulty::LegacyRaid40 => 9,
            Difficulty::HeroicScenario => 11,
            Difficulty::NormalScenario => 12,
            Difficulty::NormalRaid => 14,
            Difficulty::HeroicRaid => 15,
            Difficulty::MythicRaid => 16,
            Difficulty::LFR => 17,
            Difficulty::EventRaid => 18,
            Difficulty::EventParty => 19,
            Difficulty::EventScenario => 20,
            Difficulty::MythicParty => 23,
            Difficulty::TimewalkingParty => 24,
            Difficulty::WorldPvpScenario => 25,
            Difficulty::PvevpScenario => 29,
            Difficulty::EventScenario2 => 30,
            Difficulty::WorldPvpScenario2 => 32,
            Difficulty::TimewalkingRaid => 33,
            Difficulty::Pvp => 34,
            Difficulty::NormalScenario2 => 38,
            Difficulty::HeroicScenario2 => 39,
            Difficulty::MythicScenario => 40,
            Difficulty::PvpScenario => 45,
            Difficulty::NormalWarfront => 147,
            Difficulty::HeroicWarfront => 149,
            Difficulty::NormalParty2 => 150,
            Difficulty::TimewalkingLFR => 151,
            Difficulty::NZothVision => 152,
            Difficulty::TeemingIsland => 153,
            Difficulty::Torghast => 167,
            Difficulty::PathOfAscensionCourage => 168,
            Difficulty::PathOfAscensionLoyalty => 169,
            Difficulty::PathOfAscensionWisdom => 170,
            Difficulty::PathOfAscensionHumility => 171,
            Difficulty::WorldBoss => 172,
            Difficulty::ChallengeLevel1 => 192,
            Difficulty::FollowerParty => 205,
            Difficulty::Delve => 208,
            Difficulty::Quest => 216,
            Difficulty::StoryRaid => 220,
            Difficulty::HeroicNone => 230,
            Difficulty::Unknown(value) => value,
        }
    }
}

impl From<u16> for Difficulty {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::NormalParty,
            2 => Self::HeroicParty,
            3 => Self::NormalRaid10,
            4 => Self::NormalRaid25,
            5 => Self::HeroicRaid10,
            6 => Self::HeroicRaid25,
            7 => Self::LegacyLFR,
            8 => Self::MythicKeystone,
            9 => Self::LegacyRaid40,
            11 => Self::HeroicScenario,
            12 => Self::NormalScenario,
            14 => Self::NormalRaid,
            15 => Self::HeroicRaid,
            16 => Self::MythicRaid,
            17 => Self::LFR,
            18 => Self::EventRaid,
            19 => Self::EventParty,
            20 => Self::EventScenario,
            23 => Self::MythicParty,
            24 => Self::TimewalkingParty,
            25 => Self::WorldPvpScenario,
            29 => Self::PvevpScenario,
            30 => Self::EventScenario2,
            32 => Self::WorldPvpScenario2,
            33 => Self::TimewalkingRaid,
            34 => Self::Pvp,
            38 => Self::NormalScenario2,
            39 => Self::HeroicScenario2,
            40 => Self::MythicScenario,
            45 => Self::PvpScenario,
            147 => Self::NormalWarfront,
            149 => Self::HeroicWarfront,
            150 => Self::NormalParty2,
            151 => Self::TimewalkingLFR,
            152 => Self::NZothVision,
            153 => Self::TeemingIsland,
            167 => Self::Torghast,
            168 => Self::PathOfAscensionCourage,
            169 => Self::PathOfAscensionLoyalty,
            170 => Self::PathOfAscensionWisdom,
            171 => Self::PathOfAscensionHumility,
            172 => Self::WorldBoss,
            192 => Self::ChallengeLevel1,
            205 => Self::FollowerParty,
            208 => Self::Delve,
            216 => Self::Quest,
            220 => Self::StoryRaid,
            230 => Self::HeroicNone,
            _ => Self::Unknown(value),
        }
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NormalParty => write!(f, "Normal (5)"),
            Self::HeroicParty => write!(f, "Heroic (5)"),
            Self::NormalRaid10 => write!(f, "Normal Raid (10)"),
            Self::NormalRaid25 => write!(f, "Normal Raid (25)"),
            Self::HeroicRaid10 => write!(f, "Heroic Raid (10)"),
            Self::HeroicRaid25 => write!(f, "Heroic Raid (25)"),
            Self::LegacyLFR => write!(f, "Legacy Looking For Raid"),
            Self::MythicKeystone => write!(f, "Mythic Keystone"),
            Self::LegacyRaid40 => write!(f, "Legacy Raid (40)"),
            Self::HeroicScenario => write!(f, "Heroic Scenario"),
            Self::NormalScenario => write!(f, "Normal Scenario"),
            Self::NormalRaid => write!(f, "Normal Raid"),
            Self::HeroicRaid => write!(f, "Heroic Raid"),
            Self::MythicRaid => write!(f, "Mythic Raid"),
            Self::LFR => write!(f, "Looking For Raid"),
            Self::EventRaid => write!(f, "Event (Raid)"),
            Self::EventParty => write!(f, "Event (5)"),
            Self::EventScenario => write!(f, "Event (Scenario)"),
            Self::MythicParty => write!(f, "Mythic (5)"),
            Self::TimewalkingParty => write!(f, "Timewalking (5)"),
            Self::WorldPvpScenario => write!(f, "World PvP (Scenario)"),
            Self::PvevpScenario => write!(f, "PvEvP (Scenario)"),
            Self::EventScenario2 => write!(f, "Event (Scenario)"),
            Self::WorldPvpScenario2 => write!(f, "World PvP (Scenario)"),
            Self::TimewalkingRaid => write!(f, "Timewalking (Raid)"),
            Self::Pvp => write!(f, "PvP"),
            Self::NormalScenario2 => write!(f, "Normal(Scenario)"),
            Self::HeroicScenario2 => write!(f, "Heroic (Scenario)"),
            Self::MythicScenario => write!(f, "Mythic (Scenario)"),
            Self::PvpScenario => write!(f, "PvP (Scenario)"),
            Self::NormalWarfront => write!(f, "Normal Warfront"),
            Self::HeroicWarfront => write!(f, "Heroic Warfront"),
            Self::NormalParty2 => write!(f, "Normal (5)"),
            Self::TimewalkingLFR => write!(f, "Timewalking (Looking For Raid)"),
            Self::NZothVision => write!(f, "Vision of N'Zoth"),
            Self::TeemingIsland => write!(f, "Island (Teeming)"),
            Self::Torghast => write!(f, "Torghast"),
            Self::PathOfAscensionCourage => write!(f, "Path of Ascension: Courage"),
            Self::PathOfAscensionLoyalty => write!(f, "Path of Ascension: Loyalty"),
            Self::PathOfAscensionWisdom => write!(f, "Path of Ascension: Wisdom"),
            Self::PathOfAscensionHumility => write!(f, "Path of Ascension: Humility"),
            Self::WorldBoss => write!(f, "World Boss"),
            Self::ChallengeLevel1 => write!(f, "Challenge Level 1"),
            Self::FollowerParty => write!(f, "Follower (5)"),
            Self::Delve => write!(f, "Delve"),
            Self::Quest => write!(f, "Quest"),
            Self::StoryRaid => write!(f, "Raid (Story Mode)"),
            Self::HeroicNone => write!(f, "Heroic (?)"),
            Self::Unknown(value) => write!(f, "unknown difficulty id: {value}"),
        }
    }
}

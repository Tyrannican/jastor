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
const UNIT_OBJECT_NONE: u32 = 0x80000000;
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

pub struct UnitFlag;

impl UnitFlag {
    pub fn affiliation(flag: u32) -> UnitAffiliation {
        match flag & UNIT_AFFILIATION_MASK {
            UNIT_AFFILIATION_MINE => UnitAffiliation::Mine,
            UNIT_AFFILIATION_PARTY => UnitAffiliation::Party,
            UNIT_AFFILIATION_RAID => UnitAffiliation::Raid,
            UNIT_AFFILIATION_OUTSIDER => UnitAffiliation::Outsider,
            _ => unreachable!("WoW ensures the flag is always valid"),
        }
    }

    pub fn reaction(flag: u32) -> UnitReaction {
        match flag & UNIT_REACTION_MASK {
            UNIT_REACTION_FRIENDLY => UnitReaction::Friendly,
            UNIT_REACTION_NEUTRAL => UnitReaction::Friendly,
            UNIT_REACTION_HOSTILE => UnitReaction::Friendly,
            _ => unreachable!("WoW ensures the flag is always valid"),
        }
    }

    pub fn controller(flag: u32) -> UnitController {
        match flag & UNIT_CONTROL_MASK {
            UNIT_CONTROL_PLAYER => UnitController::Player,
            UNIT_CONTROL_NPC => UnitController::Npc,
            _ => unreachable!("WoW ensures the flag is always valid"),
        }
    }

    pub fn unit_type(flag: u32) -> UnitType {
        match flag & UNIT_TYPE_MASK {
            UNIT_TYPE_PLAYER => UnitType::Player,
            UNIT_TYPE_NPC => UnitType::Npc,
            UNIT_TYPE_PET => UnitType::Pet,
            UNIT_TYPE_GUARDIAN => UnitType::Guardian,
            UNIT_TYPE_OBJECT => UnitType::Object,
            _ => unreachable!("WoW ensures the flag is always valid"),
        }
    }

    pub fn special(flag: u32) -> UnitSpecial {
        match flag & UNIT_OBJECT_SPECIAL_MASK {
            UNIT_OBJECT_TARGET => UnitSpecial::Target,
            UNIT_OBJECT_FOCUS => UnitSpecial::Focus,
            UNIT_OBJECT_MAINTANK => UnitSpecial::MainTank,
            UNIT_OBJECT_MAINASSIST => UnitSpecial::MainAssist,
            UNIT_OBJECT_NONE => UnitSpecial::None,
            _ => unreachable!("WoW ensures the flag is always valid"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnitAffiliation {
    Mine,
    Party,
    Raid,
    Outsider,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnitReaction {
    Friendly,
    Neutral,
    Hostile,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnitController {
    Player,
    Npc,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnitType {
    Player,
    Npc,
    Pet,
    Guardian,
    Object,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnitSpecial {
    Target,
    Focus,
    MainTank,
    MainAssist,
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RaidFlag {
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

impl RaidFlag {
    pub fn parse(flag: u32) -> Self {
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

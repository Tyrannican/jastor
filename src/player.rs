use std::str::FromStr;

use crate::{
    parser::EventParser,
    types::{Faction, Guid, Specialization},
};
use eyre::{Context, Result};
use num::Num;

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

impl Combatant {
    pub fn new(args: &str) -> Result<Self> {
        let mut root_parser = CombatantParser::new(args);
        let guid = Guid(root_parser.next_string()?.to_string());
        let faction = if root_parser.next_numeric::<u8>()? == 0 {
            Faction::Horde
        } else {
            Faction::Alliance
        };

        let stats = Stats::new(&mut root_parser).context("parsing combatant stats")?;
        let spec = Specialization::try_from(
            root_parser
                .next_numeric::<u16>()
                .context("parsing spec id")?,
        )
        .context("parsing specialisation")?;

        let talent_str = root_parser.next();
        let talents = parse_talents(&talent_str)?;

        let pvp_talent_str = root_parser.next();
        let pvp_talents = parse_pvp_talents(&pvp_talent_str)?;

        let equipment_str = root_parser.next();
        let equipment = parse_equipment(equipment_str)?;

        let auras = parse_tracked_auras(root_parser.next())?;
        let pvp_stats = PvpStats::new(&mut root_parser)?;

        Ok(Combatant {
            guid,
            faction,
            stats,
            spec,
            talents,
            pvp_talents,
            equipment,
            auras,
            pvp_stats,
        })
    }
}

fn parse_talents(talent_str: &str) -> Result<Vec<Talent>> {
    let mut talents = Vec::new();
    let mut talent_parser = CombatantParser::new(&talent_str);

    let mut talent = talent_parser.next();
    while !talent.is_empty() {
        let talent_ids = talent_parser.parse_array(&talent)?;
        assert!(talent_ids.len() == 3);
        talents.push(Talent {
            node_id: talent_ids[0],
            entry_id: talent_ids[1],
            rank: talent_ids[2],
        });
        talent = talent_parser.next();
    }

    Ok(talents)
}

fn parse_equipment(equipment_str: &str) -> Result<Vec<Equipment>> {
    let mut parser = CombatantParser::new(equipment_str);
    let mut equip_str = parser.next();

    let mut equipment = Vec::new();
    while !equip_str.is_empty() {
        equipment.push(Equipment::new(equip_str)?);
        equip_str = parser.next();
    }

    Ok(equipment)
}

fn parse_tracked_auras(aura_str: &str) -> Result<Vec<TrackedAura>> {
    let auras = aura_str.split(',').collect::<Vec<&str>>();
    auras
        .chunks(3)
        .map(|aura| {
            Ok(TrackedAura {
                caster: Guid(aura[0].to_string()),
                spell_id: aura[1]
                    .parse::<u32>()
                    .wrap_err_with(|| format!("expected spell id for aura - {}", aura[1]))?,
                stacks: aura[2]
                    .parse::<u32>()
                    .wrap_err_with(|| format!("expected stacks for aura - {}", aura[2]))?,
            })
        })
        .collect::<Result<Vec<TrackedAura>>>()
}

fn parse_pvp_talents(pvp_str: &str) -> Result<PvpTalents> {
    let mut parser = CombatantParser::new(pvp_str);
    Ok((
        parser.next_numeric::<u32>()?,
        parser.next_numeric::<u32>()?,
        parser.next_numeric::<u32>()?,
        parser.next_numeric::<u32>()?,
    ))
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
    critical_block: u32, // Deduced from this value matching Crit rating
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

impl Stats {
    pub fn new(parser: &mut CombatantParser) -> Result<Self> {
        Ok(Self {
            strength: parser.next_numeric::<u32>()?,
            agility: parser.next_numeric::<u32>()?,
            stamina: parser.next_numeric::<u32>()?,
            intelligence: parser.next_numeric::<u32>()?,
            dodge: parser.next_numeric::<u32>()?,
            parry: parser.next_numeric::<u32>()?,
            critical_block: parser.next_numeric::<u32>()?,
            block: parser.next_numeric::<u32>()?,
            crit_melee: parser.next_numeric::<u32>()?,
            crit_ranged: parser.next_numeric::<u32>()?,
            crit_spell: parser.next_numeric::<u32>()?,
            speed: parser.next_numeric::<u32>()?,
            lifesteal: parser.next_numeric::<u32>()?,
            haste_melee: parser.next_numeric::<u32>()?,
            haste_ranged: parser.next_numeric::<u32>()?,
            haste_spell: parser.next_numeric::<u32>()?,
            avoidance: parser.next_numeric::<u32>()?,
            mastery: parser.next_numeric::<u32>()?,
            versatility_damage: parser.next_numeric::<u32>()?,
            versatility_healing: parser.next_numeric::<u32>()?,
            versatility_damage_taken: parser.next_numeric::<u32>()?,
            armor: parser.next_numeric::<u32>()?,
        })
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PvpStats {
    honor_level: u32,
    season: u32,
    rating: u32,
    tier: u32,
}

impl PvpStats {
    pub fn new(parser: &mut CombatantParser) -> Result<Self> {
        Ok(Self {
            honor_level: parser.next_numeric::<u32>()?,
            season: parser.next_numeric::<u32>()?,
            rating: parser.next_numeric::<u32>()?,
            tier: parser.next_numeric::<u32>()?,
        })
    }
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
    enchantment: Option<Enchantment>,
    bonuses: Vec<u32>,
    gems: Vec<Gem>,
}

impl Equipment {
    pub fn new(gear_str: &str) -> Result<Self> {
        let mut parser = CombatantParser::new(gear_str);
        let item_id = parser.next_numeric::<u32>()?;
        let item_level = parser.next_numeric::<u32>()?;
        let enchantment_str = parser.next();
        let enchantment = if enchantment_str.is_empty() {
            None
        } else {
            let values = enchantment_str
                .split(',')
                .map(|v| {
                    v.parse::<u32>()
                        .wrap_err_with(|| format!("expected numeric value for {v}"))
                })
                .collect::<Result<Vec<u32>>>()?;

            assert!(values.len() == 3);
            Some((values[0], values[1], values[2]))
        };
        let bonuses = parser
            .next()
            .split(',')
            .filter_map(|v| {
                if v.is_empty() {
                    None
                } else {
                    Some(
                        v.parse::<u32>()
                            .wrap_err_with(|| format!("expected bonus id value for {v}")),
                    )
                }
            })
            .collect::<Result<Vec<u32>>>()?;

        let gem_strs = parser.next();
        let gems = if gem_strs.is_empty() {
            Vec::new()
        } else {
            let gem_parse = gem_strs
                .split(',')
                .map(|g| {
                    g.parse::<u32>()
                        .wrap_err_with(|| format!("expected value for gem - {g}"))
                })
                .collect::<Result<Vec<u32>>>()?;

            assert_eq!(gem_parse.len() % 2, 0);
            gem_parse
                .chunks(2)
                .map(|gem| (gem[0], gem[1]))
                .collect::<Vec<Gem>>()
        };

        Ok(Self {
            item_id,
            item_level,
            enchantment,
            bonuses,
            gems,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TrackedAura {
    caster: Guid,
    spell_id: u32,
    stacks: u32,
}

#[derive(Debug, Clone)]
pub struct CombatantParser<'a> {
    line: &'a str,
    rest: &'a str,
    delimiter: char,
}

impl<'a> CombatantParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            line: input,
            rest: input,
            delimiter: ',',
        }
    }

    pub fn parse_array<T: Num + FromStr>(&self, input: &str) -> Result<Vec<T>>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
        <T as Num>::FromStrRadixErr: std::fmt::Debug + std::fmt::Display + Send + Sync + 'static,
    {
        input
            .split(',')
            .map(|v| {
                v.parse::<T>()
                    .wrap_err_with(|| format!("unable to convert value to numeric - {v}"))
            })
            .collect::<Result<Vec<T>>>()
    }
}

impl<'a> EventParser<'a> for CombatantParser<'a> {
    fn next(&mut self) -> &'a str {
        let mut end = self.rest.len();
        let mut new_start = self.rest.len();
        let mut stack = Vec::with_capacity(4);
        let mut iter = self.rest.char_indices();

        while let Some((i, ch)) = iter.next() {
            match ch {
                '"' => {
                    for (_, inner) in iter.by_ref() {
                        if inner == '"' {
                            break;
                        }
                    }
                }
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                ')' | ']' => {
                    if stack.last() == Some(&ch) {
                        stack.pop();
                    }
                }
                ch if ch == self.delimiter && stack.is_empty() => {
                    end = i;
                    new_start = i + ch.len_utf8();
                    break;
                }
                _ => {}
            }
        }

        let value = &self.rest[..end];
        self.rest = &self.rest[new_start..];

        if value.starts_with('"') || value.starts_with('(') || value.starts_with('[') {
            &value[1..value.len() - 1]
        } else {
            value
        }
    }
}

#[cfg(test)]
mod combatant_tests {
    use super::*;

    #[test]
    fn parsing_array() {
        let test = "[(101035,124805,2),(101036,124806,1),(101037,124807,1),(101038,124809,1),(101039,124810,1),(101045,124817,1),(101048,124820,1),(101052,124825,1),(101053,124826,1),(101054,124827,1),(101055,124828,1),(101056,124829,1),(101059,124833,1),(101060,124834,1),(101136,124926,1),(101137,124927,1),(101140,124930,1),(101146,124936,1),(101147,124937,1),(101149,124939,1),(101150,124941,1),(101153,124944,1),(101159,124953,1),(101160,124954,1),(101162,124956,1),(101165,124960,1),(101166,124961,2),(101167,124962,1),(101168,124963,1),(101169,124964,2),(101170,124965,1),(101173,124968,1),(101174,124970,1),(101175,124971,1),(101178,124975,1),(101179,124976,2),(101182,124980,1),(101183,124982,1),(101184,124983,2),(101185,124984,1),(101203,125009,1),(101205,125012,1),(101206,125013,1),(101208,125015,1),(101209,125016,1),(101210,125017,1),(101213,125020,1),(101215,125022,1),(101216,125023,1),(101217,125025,1),(101218,125026,1),(101232,125046,1),(101244,125063,1),(101245,125065,1),(101246,125066,1),(101247,125068,1),(101249,125070,1),(101250,125071,1),(101251,125072,1),(101252,125073,1),(101253,125074,1),(101254,125076,1),(101044,126026,1),(101207,134452,1),(101057,134453,1),(108946,134543,1),(109697,135955,1),(109698,135956,1),(109699,135957,1),(110022,136514,1),(110023,136515,1),(110025,136518,1),(110098,136599,1),(110436,137076,1),(110436,137077,2),(110436,137078,1),(101142,124932,1),(101186,124985,1),(101248,125069,1)]";

        let expected = "(101035,124805,2),(101036,124806,1),(101037,124807,1),(101038,124809,1),(101039,124810,1),(101045,124817,1),(101048,124820,1),(101052,124825,1),(101053,124826,1),(101054,124827,1),(101055,124828,1),(101056,124829,1),(101059,124833,1),(101060,124834,1),(101136,124926,1),(101137,124927,1),(101140,124930,1),(101146,124936,1),(101147,124937,1),(101149,124939,1),(101150,124941,1),(101153,124944,1),(101159,124953,1),(101160,124954,1),(101162,124956,1),(101165,124960,1),(101166,124961,2),(101167,124962,1),(101168,124963,1),(101169,124964,2),(101170,124965,1),(101173,124968,1),(101174,124970,1),(101175,124971,1),(101178,124975,1),(101179,124976,2),(101182,124980,1),(101183,124982,1),(101184,124983,2),(101185,124984,1),(101203,125009,1),(101205,125012,1),(101206,125013,1),(101208,125015,1),(101209,125016,1),(101210,125017,1),(101213,125020,1),(101215,125022,1),(101216,125023,1),(101217,125025,1),(101218,125026,1),(101232,125046,1),(101244,125063,1),(101245,125065,1),(101246,125066,1),(101247,125068,1),(101249,125070,1),(101250,125071,1),(101251,125072,1),(101252,125073,1),(101253,125074,1),(101254,125076,1),(101044,126026,1),(101207,134452,1),(101057,134453,1),(108946,134543,1),(109697,135955,1),(109698,135956,1),(109699,135957,1),(110022,136514,1),(110023,136515,1),(110025,136518,1),(110098,136599,1),(110436,137076,1),(110436,137077,2),(110436,137078,1),(101142,124932,1),(101186,124985,1),(101248,125069,1)";

        let mut parser = CombatantParser::new(test);
        let result = parser.next();
        assert_eq!(result, expected);
    }

    #[test]
    fn parsing_tuples() {
        let test = "[(101035,124805,2),(101036,124806,1),(101037,124807,1),(101038,124809,1),(101039,124810,1),(101045,124817,1),(101048,124820,1),(101052,124825,1),(101053,124826,1),(101054,124827,1),(101055,124828,1),(101056,124829,1),(101059,124833,1),(101060,124834,1),(101136,124926,1),(101137,124927,1),(101140,124930,1),(101146,124936,1),(101147,124937,1),(101149,124939,1),(101150,124941,1),(101153,124944,1),(101159,124953,1),(101160,124954,1),(101162,124956,1),(101165,124960,1),(101166,124961,2),(101167,124962,1),(101168,124963,1),(101169,124964,2),(101170,124965,1),(101173,124968,1),(101174,124970,1),(101175,124971,1),(101178,124975,1),(101179,124976,2),(101182,124980,1),(101183,124982,1),(101184,124983,2),(101185,124984,1),(101203,125009,1),(101205,125012,1),(101206,125013,1),(101208,125015,1),(101209,125016,1),(101210,125017,1),(101213,125020,1),(101215,125022,1),(101216,125023,1),(101217,125025,1),(101218,125026,1),(101232,125046,1),(101244,125063,1),(101245,125065,1),(101246,125066,1),(101247,125068,1),(101249,125070,1),(101250,125071,1),(101251,125072,1),(101252,125073,1),(101253,125074,1),(101254,125076,1),(101044,126026,1),(101207,134452,1),(101057,134453,1),(108946,134543,1),(109697,135955,1),(109698,135956,1),(109699,135957,1),(110022,136514,1),(110023,136515,1),(110025,136518,1),(110098,136599,1),(110436,137076,1),(110436,137077,2),(110436,137078,1),(101142,124932,1),(101186,124985,1),(101248,125069,1)]";

        let mut parser = CombatantParser::new(test);
        let result = parser.next();
        let mut parser = CombatantParser::new(&result);
        let result = parser.next();
        eprintln!("{result:?}");
    }

    #[test]
    fn temp_test() -> Result<()> {
        let test = "Player-1309-0CEDFD7A,0,7762,12001,587813,113923,0,0,0,14116,14116,14116,0,0,23681,23681,23681,0,8156,0,0,0,23121,62,[(62085,80141,1),(62087,80143,1),(62091,80147,1),(62093,80149,1),(62098,80155,1),(62099,80156,2),(62101,80158,2),(62102,80159,1),(62104,80161,1),(62105,80163,1),(62107,80165,1),(62111,80169,2),(62113,80171,1),(62114,80173,2),(62115,80174,1),(62118,80177,1),(62120,80179,1),(62122,80181,1),(62123,80182,2),(62124,80183,1),(62127,80187,1),(62128,80188,2),(93524,115877,1),(94654,117257,1),(94655,117258,1),(94656,117259,1),(94658,117261,1),(94659,117262,1),(94660,117263,1),(94661,117264,1),(94662,117265,1),(94663,117266,1),(99830,123344,1),(62092,125817,1),(62094,126060,1),(102435,126505,1),(102437,126507,1),(102439,126509,1),(102443,126513,1),(102444,126514,1),(102445,126515,1),(102446,126516,1),(102447,126517,1),(102448,126518,1),(102449,126519,1),(102450,126520,2),(102452,126522,1),(102454,126524,1),(102456,126526,1),(102457,126527,1),(102458,126528,1),(102459,126529,1),(102465,126535,1),(102467,126537,1),(102468,126538,1),(102469,126539,1),(102470,126540,1),(102471,126541,1),(102473,126543,1),(102474,126544,1),(102475,126545,1),(102477,126547,1),(102480,126550,1),(94657,128267,1),(104113,128689,1),(62121,80180,1),(94664,117267,1)],(0,1220739,235711,352278),[(237718,694,(),(12350,12365,12231,12676,1504),()),(237569,701,(),(6652,10394,10393,10355,12352,1511,10255),()),(237716,681,(),(12233,6652,12675,12286,1491,10255),()),(4335,1,(),(),()),(237721,691,(),(12293,12229,12676,1501),()),(237715,701,(),(12921,12352,1511),()),(237717,691,(),(12232,12676,12293,1501,10255),()),(243305,675,(),(43,12239,10353,12284,13501,1485,10255),()),(237714,684,(),(12921,12291,1494),()),(127599,678,(),(7756,12090,12281,11383,10255),()),(242405,704,(),(6652,10395,10393,10355,12353,1514,10255),()),(237567,691,(),(6652,10394,10392,10354,12293,1501,10255),()),(242395,678,(),(6652,10353,12285,1488,10255),()),(230027,658,(),(6652,10354,11984,1507,10255),()),(235499,730,(),(12401,9893,12262),(238039,571)),(171576,694,(),(12350,7756,10384,11374,10255),()),(0,0,(),(),()),(69209,1,(),(),())],[Player-3702-0B437AD9,6673,1,Player-1309-0CEDFD7A,1459,1,Player-1309-0CEDFD7A,1242340,1,Player-1396-0B3D67F5,465,1,Player-1309-0CEDFD7A,321526,1,Player-1309-0CEDFD7A,384858,1,Player-1309-0CEDFD7A,384452,1,Player-1309-0CEDFD7A,384612,1,Player-1309-0CEDFD7A,461457,1,Player-1309-0CEDFD7A,384581,1,Player-1309-0CEDFD7A,384651,1,Player-1403-06EE977F,21562,1,Player-1309-0CEDFD7A,1237913,1,Player-1084-0AB6626B,1126,1,Player-1402-0B152F91,465,1,Player-1379-05529E0E,465,1],7,0,0,0";

        let _c = Combatant::new(test)?;
        Ok(())
    }
}

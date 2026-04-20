use std::str::FromStr;

use crate::{
    parser::EventParser,
    types::{Faction, Guid, Specialization},
};
use eyre::{Context, Result, eyre};
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
        let guid = Guid(root_parser.next_string()?);
        let faction = if root_parser.next_numeric::<u8>()? == 0 {
            Faction::Horde
        } else {
            Faction::Alliance
        };

        let stats = Stats::new(&mut root_parser)?;
        let spec = Specialization::try_from(root_parser.next_numeric::<u16>()?)?;

        let talent_str = root_parser.next();
        let talents = parse_talents(&talent_str)?;

        let pvp_talent_str = root_parser.next();
        let pvp_talents = parse_pvp_talents(&pvp_talent_str)?;

        let gear_str = root_parser.next();
        eprintln!("GEAR: {gear_str}");
        let auras = root_parser.next();
        let pvp_stats = PvpStats::new(&mut root_parser)?;

        Ok(Combatant {
            guid,
            faction,
            stats,
            spec,
            talents,
            pvp_talents,
            equipment: todo!(),
            auras: todo!(),
            pvp_stats,
        })
    }
}

fn parse_talents(talent_str: &str) -> Result<Vec<Talent>> {
    let mut talents = Vec::new();
    let mut talent_parser = CombatantParser::new(&talent_str);

    loop {
        let talent = talent_parser.next();
        if talent.is_empty() {
            break;
        }

        let talent_ids = talent_parser.parse_array(&talent)?;
        assert!(talent_ids.len() == 3);
        talents.push(Talent {
            node_id: talent_ids[0],
            entry_id: talent_ids[1],
            rank: talent_ids[2],
        });
    }

    Ok(talents)
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

impl<'a> EventParser for CombatantParser<'a> {
    fn next(&mut self) -> String {
        let mut output = String::new();
        let mut chars = self.rest.chars();

        while let Some(next) = chars.next() {
            match next {
                '"' => {
                    while let Some(inner) = chars.next() {
                        match inner {
                            '"' => break,
                            ch => output.push(ch),
                        }
                    }
                }
                ch if (ch == '(' || ch == '[') => {
                    let mut stack = vec![ch];
                    let delim = if ch == '(' { ')' } else { ']' };
                    while let Some(inner) = chars.next() {
                        match inner {
                            c if c == ch => {
                                stack.push(c);
                                output.push(c);
                            }
                            c if c == delim => {
                                stack.pop();
                                if stack.is_empty() {
                                    break;
                                }

                                output.push(c);
                            }
                            _ => output.push(inner),
                        }
                    }
                }
                ch if ch == self.delimiter => break,
                _ => output.push(next),
            }
        }

        self.rest = chars.as_str();
        output
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
}

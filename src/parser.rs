use std::{io::BufRead, str::FromStr};

use crate::event::{
    AdvancedParameters, AuraEvent, AuraType, AuraWithSpellEvent, CombatEvent, Combatant,
    DamageEvent, Difficulty, DrainEvent, EmoteEvent, EncounterEndEvent, EncounterStartEvent,
    EnergizeEvent, EnvironmentalType, Event, EventType, FailEvent, Guid, HealAbsorbEvent,
    HealEvent, LogVersionEvent, MapChangeEvent, MissEvent, MissType, MultiValue, PowerType,
    RaidFlag, SpellParameters, SpellSchool, StaggerEvent, StealEvent, StealWithAuraEvent, Suffix,
    Target, UnitFlags, ZoneChangeEvent,
};

use eyre::{Context, Result, eyre};
use jiff::{civil::DateTime, fmt::strtime};
use num::Num;

#[derive(Debug, Clone)]
pub struct ParsedEvent {
    pub timestamp: DateTime,
    pub event_type: EventType,
    pub event: Event,
}

pub struct EventLogParser<R: BufRead> {
    reader: R,
}

impl<R: BufRead> EventLogParser<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    fn parse_event(&self, line: impl AsRef<str>) -> Result<ParsedEvent> {
        let line = line.as_ref();
        let Some((ts, rest)) = line.split_once("  ") else {
            panic!("invalid event found (expected timestamp) - {line}");
        };

        // These are auto-generated strings from the WoW client
        // Any errors here means the actual log file is corrupted
        // We expect here as we assume they're untampered with
        let timestamp: DateTime = strtime::parse("%m/%d/%Y %H:%M:%S%.f", ts)
            .expect("a valid timestamp string")
            .to_datetime()
            .expect("a valid timestamp conversion");

        let Some((event, args)) = rest.split_once(',') else {
            return Err(eyre!("invalid event - expected argument list: {line}"));
        };

        let args = args.trim();
        let event_type = EventType::try_from(event)?;
        let event = match event_type {
            EventType::CombatLogVersion => Event::LogVersion(
                self.parse_header(args)
                    .context("parsing combat log version")?,
            ),
            EventType::ZoneChange => Event::ZoneChange(
                self.parse_zone_change(args)
                    .context("parsing zone change")?,
            ),
            EventType::MapChange => {
                Event::MapChange(self.parse_map_change(args).context("parsing map change")?)
            }
            EventType::EncounterStart | EventType::EncounterEnd => self
                .parse_encounter_start_end(event_type, args)
                .context("parsing encounter start / end")?,
            EventType::StaggerPrevented | EventType::StaggerClear => Event::Stagger(
                self.parse_stagger_event(event_type, args)
                    .context("parsing stagger")?,
            ),
            EventType::CombatantInfo => {
                Event::Combatant(Combatant::new(args).context("parsing combatant info")?)
            }
            EventType::Emote => Event::Emote(EmoteEvent),
            _ => Event::Combat(
                self.parse_combat_event(event_type, args)
                    .context(format!("processing {} event", event_type))
                    .with_context(|| format!("parsing combat event - {event_type}"))?,
            ),
        };

        Ok(ParsedEvent {
            timestamp,
            event_type,
            event,
        })
    }

    fn parse_header(&self, args: &str) -> Result<LogVersionEvent> {
        let mut arg_parser = EventArgParser::new(args, ',');
        let version = arg_parser.next_numeric::<u32>()?;
        arg_parser.next_string()?;
        let advanced_log = arg_parser.next_string()? == "1";
        arg_parser.next_string()?;
        let build = arg_parser.next_string()?.to_string();

        Ok(LogVersionEvent {
            version,
            advanced_log,
            build,
        })
    }

    fn parse_zone_change(&self, args: &str) -> Result<ZoneChangeEvent> {
        let mut parser = EventArgParser::new(args, ',');
        let instance_id = parser.next_numeric::<u32>()?;
        let zone_name = parser.next_string()?.trim_matches('"').to_string();
        let difficulty = parser.next_numeric::<u16>()?;

        Ok(ZoneChangeEvent {
            instance_id,
            zone_name,
            difficulty: Difficulty::from(difficulty),
        })
    }

    fn parse_map_change(&self, args: &str) -> Result<MapChangeEvent> {
        let mut parser = EventArgParser::new(args, ',');
        let map_id = parser.next_numeric::<u32>()?;
        let map_name = parser.next_string()?.trim_matches('"').to_string();
        let x0 = parser.next_numeric::<f32>()?;
        let x1 = parser.next_numeric::<f32>()?;
        let y0 = parser.next_numeric::<f32>()?;
        let y1 = parser.next_numeric::<f32>()?;

        Ok(MapChangeEvent {
            map_id,
            map_name,
            x0,
            x1,
            y0,
            y1,
        })
    }

    fn parse_encounter_start_end(&self, event_type: EventType, args: &str) -> Result<Event> {
        let mut parser = EventArgParser::new(args, ',');

        match event_type {
            EventType::EncounterStart => {
                let encounter_id = parser.next_numeric::<u32>()?;
                let encounter_name = parser.next_string()?.to_string();
                let difficulty = Difficulty::from(parser.next_numeric::<u16>()?);
                let group_size = parser.next_numeric::<u32>()?;
                let instance_id = parser.next_numeric::<u32>()?;

                Ok(Event::EncounterStart(EncounterStartEvent {
                    encounter_id,
                    encounter_name,
                    difficulty,
                    group_size,
                    instance_id,
                }))
            }
            EventType::EncounterEnd => {
                let encounter_id = parser.next_numeric::<u32>()?;
                let encounter_name = parser.next_string()?.to_string();
                let difficulty = Difficulty::from(parser.next_numeric::<u16>()?);
                let group_size = parser.next_numeric::<u32>()?;
                let success = parser.next_numeric::<u8>()? == 1;
                let fight_time = (parser.next_numeric::<u64>()? / 1000) as u32;

                Ok(Event::EncounterEnd(EncounterEndEvent {
                    encounter_id,
                    encounter_name,
                    difficulty,
                    group_size,
                    success,
                    fight_time,
                }))
            }
            _ => unreachable!("checked in outer match"),
        }
    }

    fn parse_stagger_event(&self, event_type: EventType, args: &str) -> Result<StaggerEvent> {
        let mut parser = EventArgParser::new(args, ',');
        let guid = Guid(parser.next_string()?.to_string());
        let (spell_id, amount): (Option<u32>, f32) = if event_type == EventType::StaggerPrevented {
            let spell_id = parser.next_numeric::<u32>()?;
            let amount = parser.next_numeric::<f32>()?;
            (Some(spell_id), amount)
        } else {
            (None, parser.next_numeric::<f32>()?)
        };

        Ok(StaggerEvent {
            guid,
            spell_id,
            amount,
        })
    }

    fn parse_combat_event(&self, event_type: EventType, args: &str) -> Result<CombatEvent> {
        let mut parser = EventArgParser::new(args, ',');
        let src = parser.target()?;
        let dst = parser.target()?;

        let spell_parameters = if event_type == EventType::SpellAbsorbed {
            parser.spell_parameters().ok()
        } else if event_type.has_spell_parameters() {
            Some(parser.spell_parameters()?)
        } else {
            None
        };

        let adv = if event_type.has_advanced_parameters() {
            Some(parser.advanced_parameters()?)
        } else {
            None
        };

        let environmental = if event_type == EventType::EnvironmentalDamage {
            parser.environmental().ok()
        } else {
            None
        };

        let suffix = self.parse_combat_suffix(event_type, &mut parser)?;

        Ok(CombatEvent {
            src: Some(src),
            dst: Some(dst),
            spell: spell_parameters,
            adv,
            environmental,
            suffix,
        })
    }

    fn parse_combat_suffix(
        &self,
        event_type: EventType,
        parser: &mut EventArgParser,
    ) -> Result<Option<Suffix>> {
        let suffix = match event_type {
            EventType::SwingDamage
            | EventType::SpellDamage
            | EventType::RangeDamage
            | EventType::SpellPeriodicDamage
            | EventType::SpellPeriodicDamageSupport
            | EventType::DamageSplit
            | EventType::DamageShield
            | EventType::EnvironmentalDamage => Some(Suffix::Damage(parser.damage()?)),
            // TMP
            EventType::SwingMissed
            | EventType::SpellMissed
            | EventType::RangeMissed
            | EventType::DamageShieldMissed
            | EventType::SpellPeriodicMissed => {
                // eprintln!("{} -- {}", event_type, parser.rest);
                None
            }
            EventType::SpellHeal
            | EventType::SpellPeriodicHeal
            | EventType::SpellPeriodicHealSupport => Some(Suffix::Heal(parser.heal()?)),
            EventType::SpellHealAbsorbed => Some(Suffix::HealAbsorbed(parser.heal_absorb()?)),
            EventType::SpellCastFailed => Some(Suffix::Fail(parser.fail()?)),
            EventType::SpellEnergize | EventType::SpellPeriodicEnergize => {
                Some(Suffix::Energize(parser.energise()?))
            }
            EventType::SpellDrain => Some(Suffix::Drain(parser.drain(false)?)),
            EventType::SpellInterrupt | EventType::SpellDispelFailed => {
                Some(Suffix::Interrupt(parser.steal()?))
            }
            EventType::SpellStolen | EventType::SpellDispel => match event_type {
                EventType::SpellStolen => Some(Suffix::Stolen(parser.steal_with_aura()?)),
                EventType::SpellDispel => Some(Suffix::Dispel(parser.steal_with_aura()?)),
                _ => unreachable!("caught by outer match arm"),
            },
            EventType::SpellExtraAttacks => {
                Some(Suffix::ExtraAttacks(parser.next_numeric::<u32>()?))
            }
            EventType::SpellAuraApplied
            | EventType::SpellAuraRemoved
            | EventType::SpellAuraAppliedDose
            | EventType::SpellAuraRemovedDose
            | EventType::SpellAuraRefresh => Some(Suffix::Aura(parser.aura()?)),
            EventType::SpellAuraBroken => Some(Suffix::AuraBroken(AuraType::try_from(
                parser.next_string()?,
            )?)),
            EventType::SpellAuraBrokenSpell => Some(Suffix::AuraBrokenSpell(parser.aura_spell()?)),
            EventType::SpellEmpowerInterrupt | EventType::SpellEmpowerEnd => {
                Some(Suffix::Empower(parser.next_numeric::<u32>()?))
            }
            _ => None,
        };

        Ok(suffix)
    }
}

impl<R: BufRead> Iterator for EventLogParser<R> {
    type Item = Result<ParsedEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => return None,
            Ok(_) => {}
            Err(e) => {
                eprintln!("error occurred parsing line - {}", e.to_string());
                return None;
            }
        }

        let event = match self.parse_event(&line) {
            Ok(e) => Ok(e),
            Err(e) => {
                eprintln!("error occurred on line {line}\n{}", e.to_string());
                Err(e)
            }
        };

        Some(event)
    }
}

pub trait EventParser<'a> {
    fn next(&mut self) -> &'a str;

    fn next_string(&mut self) -> Result<&'a str> {
        let value = self.next();
        if value.is_empty() {
            return Err(eyre!("expected a value, received empty string",));
        }

        Ok(value)
    }

    fn next_boolean(&mut self) -> bool {
        self.next() == "1"
    }

    fn next_numeric<T: Num + FromStr>(&mut self) -> Result<T>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
        <T as Num>::FromStrRadixErr: std::fmt::Debug + std::fmt::Display + Send + Sync + 'static,
    {
        let value = self.next();
        if value.is_empty() {
            return Err(eyre!("expected a value, received empty string",));
        }

        if let Some(hex) = value.strip_prefix("0x") {
            T::from_str_radix(hex, 16)
                .map_err(|e| eyre!("unable to convert hex value: '{}': {}", value, e))
        } else {
            value
                .parse::<T>()
                .wrap_err_with(|| format!("unable to convert value to numeric - {value}",))
        }
    }
}

pub struct EventArgParser<'a> {
    rest: &'a str,
    delimiter: char,
}

impl<'a> EventArgParser<'a> {
    pub fn new(input: &'a str, delim: char) -> Self {
        Self {
            rest: input,
            delimiter: delim,
        }
    }

    pub fn target(&mut self) -> Result<Target> {
        let guid = Guid(self.next_string()?.to_string());
        let name = self.next_string()?.trim_matches('"').to_string();
        let unit_flags = self.next_numeric::<u32>()?;
        let raid_flags = self.next_numeric::<u32>()?;

        Ok(Target {
            guid,
            name,
            unit_flags: UnitFlags::new(unit_flags)?,
            raid_flags: RaidFlag::try_from(raid_flags & 0xFF)?,
        })
    }

    pub fn spell_parameters(&mut self) -> Result<SpellParameters> {
        let spell_id = self.next_numeric::<u32>()?;
        let spell_name = self.next_string()?.trim_matches('"').to_string();
        let spell_school = SpellSchool::try_from(self.next_numeric::<u8>()?)?;

        Ok(SpellParameters {
            spell_id,
            spell_name,
            school: spell_school,
        })
    }

    pub fn advanced_parameters(&mut self) -> Result<AdvancedParameters> {
        let info = Guid(self.next_string()?.to_string());
        let owner = Guid(self.next_string()?.to_string());
        let current_hp = self.next_numeric::<i32>()?;
        let max_hp = self.next_numeric::<u32>()?;
        let attack_power = self.next_numeric::<u32>()?;
        let spell_power = self.next_numeric::<u32>()?;
        let armor = self.next_numeric::<i32>()?;

        // No idea what these are -- clarify
        let _ = self.next_numeric::<u32>()?;
        let _ = self.next_numeric::<u32>()?;

        let absorb = self.next_numeric::<u32>()?;

        let power_type = MultiValue(
            self.multi_value()?
                .into_iter()
                .map(|v| PowerType::try_from(v as u8).expect("power_type value in range of u8"))
                .collect(),
        );
        let current_power = MultiValue(self.multi_value()?);
        let max_power = MultiValue(self.multi_value()?);
        let power_cost = MultiValue(self.multi_value()?);

        let x = self.next_numeric::<f32>()?;
        let y = self.next_numeric::<f32>()?;
        let map_id = self.next_numeric::<u32>()?;
        let facing = self.next_numeric::<f32>()?;
        let level = self.next_numeric::<u32>()?;

        Ok(AdvancedParameters {
            info,
            owner,
            current_hp,
            max_hp,
            attack_power,
            spell_power,
            armor,
            absorb,
            power_type,
            current_power,
            max_power,
            power_cost,
            x,
            y,
            map_id,
            facing,
            level,
        })
    }

    pub fn aura(&mut self) -> Result<AuraEvent> {
        let aura = AuraType::try_from(self.next_string()?)?;
        let amount = if !self.is_empty() {
            Some(self.next_numeric::<u32>()?)
        } else {
            None
        };

        Ok(AuraEvent { aura, amount })
    }

    pub fn aura_spell(&mut self) -> Result<AuraWithSpellEvent> {
        let spell = self.spell_parameters()?;
        let aura = AuraType::try_from(self.next_string()?)?;

        Ok(AuraWithSpellEvent { spell, aura })
    }

    pub fn environmental(&mut self) -> Result<EnvironmentalType> {
        EnvironmentalType::try_from(self.next_string()?)
    }

    pub fn multi_value(&mut self) -> Result<Vec<u32>> {
        let value = self.next_string()?;
        Ok(value
            .split('|')
            .map(|s| {
                let v = s
                    .parse::<i32>()
                    .expect(&format!("a valid numeric value for multi-value - {s}"));
                if v < 0 { 0 } else { v as u32 }
            })
            .collect())
    }

    pub fn damage(&mut self) -> Result<DamageEvent> {
        let amount = self.next_numeric::<u32>()?;
        let base_amount = self.next_numeric::<u32>()?;
        let overkill = self.next_numeric::<i32>()?;
        let overkill = if overkill < 0 { 0 } else { overkill as u32 };
        let school = SpellSchool::try_from(self.next_numeric::<u8>()?)?;
        let resisted = self.next_numeric::<u32>()?;
        let blocked = self.next_numeric::<u32>()?;
        let absorbed = self.next_numeric::<i32>()?;
        let critical = self.next_boolean();
        let glancing = self.next_boolean();
        let crushing = self.next_boolean();

        Ok(DamageEvent {
            amount,
            base_amount,
            overkill,
            school,
            resisted,
            blocked,
            absorbed,
            critical,
            glancing,
            crushing,
        })
    }

    // TODO: Deal with this - has differnet values dependent on
    // Miss Type
    // Need to extend `MissEvent`
    pub fn missed(&mut self) -> Result<MissEvent> {
        // let miss_type = MissType::try_from(self.next_string()?)?;
        // eprintln!("MISS: {miss_type:?}");
        // let is_offhand = self.next_boolean();
        // eprintln!("OFFHAND: {is_offhand}");
        // let amount = self.next_numeric::<u32>()?;
        // eprintln!("AMOUNT: {amount}");
        // let critical = self.next_string()?;
        // eprintln!("CRITICAL: {critical}");
        todo!()
    }

    pub fn heal(&mut self) -> Result<HealEvent> {
        let amount = self.next_numeric::<u32>()?;
        let base_amount = self.next_numeric::<u32>()?;
        let overhealing = self.next_numeric::<u32>()?;
        let absorbed = self.next_numeric::<u32>()?;
        let critical = self.next_boolean();

        Ok(HealEvent {
            amount,
            base_amount,
            overhealing,
            absorbed,
            critical,
        })
    }

    pub fn heal_absorb(&mut self) -> Result<HealAbsorbEvent> {
        let extra = self.target()?;
        let params = self.spell_parameters()?;
        let absorbed = self.next_numeric::<u32>()?;
        let total_absorbed = self.next_numeric::<u32>()?;

        Ok(HealAbsorbEvent {
            extra,
            spell: params,
            absorbed,
            total_absorbed,
        })
    }

    pub fn fail(&mut self) -> Result<FailEvent> {
        Ok(FailEvent {
            msg: self.next_string()?.to_string(),
        })
    }

    pub fn energise(&mut self) -> Result<EnergizeEvent> {
        let amount = self.next_numeric::<f32>()?;
        let over_energize = self.next_numeric::<f32>()?;
        let power = PowerType::try_from(self.next_numeric::<u8>()?)?;
        let max = self.next_numeric::<u32>()?;

        Ok(EnergizeEvent {
            amount,
            over_energize,
            power,
            max,
        })
    }

    pub fn drain(&mut self, is_leech: bool) -> Result<DrainEvent> {
        let amount = self.next_numeric::<u32>()?;
        let power = PowerType::try_from(self.next_numeric::<u8>()?)?;
        let extra_amount = self.next_numeric::<u32>()?;

        let max = if is_leech {
            0
        } else {
            self.next_numeric::<u32>()?
        };

        Ok(DrainEvent {
            amount,
            power,
            extra_amount,
            max,
        })
    }

    pub fn steal(&mut self) -> Result<StealEvent> {
        let spell = self.spell_parameters()?;
        Ok(StealEvent(spell))
    }

    pub fn steal_with_aura(&mut self) -> Result<StealWithAuraEvent> {
        let spell = self.spell_parameters()?;
        let aura = AuraType::try_from(self.next_string()?)?;

        Ok(StealWithAuraEvent { spell, aura })
    }

    fn is_empty(&self) -> bool {
        self.rest.is_empty()
    }
}

impl<'a> EventParser<'a> for EventArgParser<'a> {
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

use std::{io::BufRead, str::FromStr};

use crate::event::{
    AdvancedParameters, AuraEvent, AuraType, CombatEvent, Combatant, Difficulty,
    EncounterStartEvent, EnvironmentalType, Event, EventType, Guid, LogVersionEvent,
    MapChangeEvent, MultiValue, PowerType, RaidFlag, SpellParameters, SpellSchool, StaggerEvent,
    Suffix, Target, UnitFlags, ZoneChangeEvent,
};

use eyre::{Context, Result, eyre};
use jiff::{civil::DateTime, fmt::strtime};
use num::Num;

#[derive(Debug, Clone)]
pub struct ParsedEvent {
    timestamp: DateTime,
    event_type: EventType,
    event: Event,
}

pub struct EventLogParser<R: BufRead> {
    reader: R,
}

impl<R: BufRead> EventLogParser<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    fn parse_event(&self, line: String) -> Result<ParsedEvent> {
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
            EventType::CombatLogVersion => Event::LogVersion(self.parse_header(args)?),
            EventType::ZoneChange => Event::ZoneChange(self.parse_zone_change(args)?),
            EventType::MapChange => Event::MapChange(self.parse_map_change(args)?),
            EventType::EncounterStart => Event::EncounterStart(self.parse_encounter_start(args)?),
            EventType::StaggerPrevented | EventType::StaggerClear => {
                Event::Stagger(self.parse_stagger_event(event_type, args)?)
            }
            EventType::CombatantInfo => todo!("combatant info needs done"),
            EventType::SwingDamage => Event::Combat(self.parse_combat_event(event_type, args)?),
            _ => Event::Combat(
                self.parse_combat_event(event_type, args)
                    .context(format!("processing {} event", event_type))?,
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
        let build = arg_parser.next_string()?;

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

    fn parse_encounter_start(&self, args: &str) -> Result<EncounterStartEvent> {
        let mut parser = EventArgParser::new(args, ',');
        let encounter_id = parser.next_numeric::<u32>()?;
        let encounter_name = parser.next_string()?;
        let difficulty = Difficulty::from(parser.next_numeric::<u16>()?);
        let group_size = parser.next_numeric::<u32>()?;
        let instance_id = parser.next_numeric::<u32>()?;

        Ok(EncounterStartEvent {
            encounter_id,
            encounter_name,
            difficulty,
            group_size,
            instance_id,
        })
    }

    fn parse_stagger_event(&self, event_type: EventType, args: &str) -> Result<StaggerEvent> {
        let mut parser = EventArgParser::new(args, ',');
        let guid = Guid(parser.next_string()?);
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
        eprintln!("EVENT: {event_type} ARGS: {args}");
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

        let environmental = None;
        let suffix = None;

        Ok(CombatEvent {
            src: Some(src),
            dst: Some(dst),
            spell: spell_parameters,
            adv,
            environmental,
            suffix,
        })
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

        Some(self.parse_event(line))
    }
}

pub struct EventArgParser<'a> {
    line: &'a str,
    rest: &'a str,
    delimiter: char,
}

impl<'a> EventArgParser<'a> {
    pub fn new(input: &'a str, delim: char) -> Self {
        Self {
            line: input,
            rest: input,
            delimiter: delim,
        }
    }

    pub fn target(&mut self) -> Result<Target> {
        let guid = Guid(self.next_string()?);
        let name = self.next_string()?.trim_matches('"').to_string();
        let unit_flags = self.next_numeric::<u32>()?;
        let raid_flags = self.next_numeric::<u32>()?;

        Ok(Target {
            guid,
            name,
            unit_flags: UnitFlags::new(unit_flags)?,
            raid_flags: RaidFlag::try_from(raid_flags)?,
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
        let info = Guid(self.next_string()?);
        let owner = Guid(self.next_string()?);
        let current_hp = self.next_numeric::<u32>()?;
        let max_hp = self.next_numeric::<u32>()?;
        let attack_power = self.next_numeric::<u32>()?;
        let spell_power = self.next_numeric::<u32>()?;
        let armor = self.next_numeric::<u32>()?;

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

    pub fn aura(&mut self, ignore_amount: bool) -> Result<(AuraType, u32)> {
        let aura = AuraType::try_from(self.next_string()?.as_str())?;
        let amount = if !ignore_amount {
            self.next_numeric::<u32>()?
        } else {
            0
        };

        Ok((aura, amount))
    }

    pub fn environmental(&mut self) -> Result<EnvironmentalType> {
        EnvironmentalType::try_from(self.next_string()?.as_str())
    }

    pub fn multi_value(&mut self) -> Result<Vec<u32>> {
        let value = self.next_string()?;
        Ok(value
            .split('|')
            .map(|s| {
                s.parse::<u32>()
                    .expect("a valid numeric value for multi-value")
            })
            .collect())
    }

    pub fn next_numeric<T: Num + FromStr>(&mut self) -> Result<T>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
        <T as Num>::FromStrRadixErr: std::fmt::Debug + std::fmt::Display + Send + Sync + 'static,
    {
        let value = self.next();
        if value.is_empty() {
            return Err(eyre!(
                "expected a value, received empty string - (Source: {}, Remainder: {})",
                self.line,
                self.rest
            ));
        }

        if let Some(hex) = value.strip_prefix("0x") {
            T::from_str_radix(hex, 16)
                .map_err(|e| eyre!("unable to convert hex value: '{}': {}", value, e))
        } else {
            value.parse::<T>().wrap_err_with(|| {
                format!(
                    "unable to convert value to numeric - {value} ({})",
                    self.line
                )
            })
        }
    }

    pub fn next_string(&mut self) -> Result<String> {
        let value = self.next();
        if value.is_empty() {
            return Err(eyre!(
                "expected a value, received empty string - (Source: {}, Remainder: {})",
                self.line,
                self.rest
            ));
        }

        Ok(value)
    }

    fn next(&mut self) -> String {
        let mut item = String::new();
        let mut chars = self.rest.chars();

        while let Some(next) = chars.next() {
            match next {
                '"' => {
                    while let Some(inner) = chars.next() {
                        match inner {
                            '"' => break,
                            ch => item.push(ch),
                        }
                    }
                }
                ch if ch == self.delimiter => {
                    break;
                }
                ch => item.push(ch),
            }
        }

        self.rest = chars.as_str();
        item
    }
}

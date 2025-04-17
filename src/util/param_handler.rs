use num_traits::{Num, NumCast};
use std::{fmt::Display, str::FromStr};

use crate::{error::JastorError, event::EventType};

const BASE_PARAMETERS_IDX: usize = 8;
const ADVANCED_PARAM_LEN: usize = 19;

pub trait ParameterHandler {
    fn len(&self) -> usize;
    fn raw(&self, idx: usize) -> Result<&String, JastorError>;
    fn valid_idx(&self, idx: usize) -> Result<(), JastorError>;
    fn as_string(&self, idx: usize) -> Result<String, JastorError>;
    fn as_boolean(&self, idx: usize) -> Result<bool, JastorError>;
    fn as_number<T>(&self, idx: usize) -> Result<T, JastorError>
    where
        T: FromStr + Num + NumCast,
        T::Err: Display;
}

#[derive(Debug, Clone)]
pub(crate) struct ArgumentHandler {
    pub(crate) params: Vec<String>,
}

impl ArgumentHandler {
    pub fn new(args: &str) -> Self {
        let mut params = Vec::new();
        let mut param = String::new();
        let mut inside_string = false;
        let mut stack = Vec::new();
        let mut iter = args.chars().peekable();

        while let Some(c) = iter.next() {
            match c {
                ',' => {
                    if inside_string {
                        param.push(c)
                    } else {
                        params.push(param.clone());
                        param.clear();
                    }
                }
                '[' | '(' => {
                    param.push(c);
                    stack.push(c);
                    while !stack.is_empty() {
                        let Some(next) = iter.next() else {
                            break;
                        };

                        match next {
                            ']' | ')' => {
                                stack.pop();
                            }
                            '[' | '(' => {
                                stack.push(next);
                            }
                            _ => {}
                        }

                        param.push(next)
                    }
                }
                '\"' => {
                    inside_string = !inside_string;
                }
                _ => param.push(c),
            }
        }

        params.push(param);

        Self { params }
    }

    pub fn base_params(&self) -> Result<&[String], JastorError> {
        self.valid_idx(BASE_PARAMETERS_IDX)?;
        Ok(&self.params[..BASE_PARAMETERS_IDX])
    }

    pub fn prefix_parameters(&self, event_type: EventType) -> Result<&[String], JastorError> {
        let (n_prefix, idx) = match event_type.prefix_parameters() {
            0 => (0, BASE_PARAMETERS_IDX),
            n => (n, BASE_PARAMETERS_IDX + (n - 1)),
        };
        self.valid_idx(idx)?;
        Ok(&self.params[BASE_PARAMETERS_IDX..BASE_PARAMETERS_IDX + n_prefix])
    }

    pub fn advanced_parameters(
        &self,
        event_type: EventType,
    ) -> Result<Option<&[String]>, JastorError> {
        if event_type.has_no_advanced_parameters() {
            return Ok(None);
        }
        let idx = match event_type.prefix_parameters() {
            0 => BASE_PARAMETERS_IDX,
            n => BASE_PARAMETERS_IDX + n,
        };
        self.valid_idx(idx)?;
        Ok(Some(&self.params[idx..idx + ADVANCED_PARAM_LEN]))
    }

    pub fn suffix_parameters(&self, event_type: EventType) -> Result<&[String], JastorError> {
        let base_idx = match event_type.prefix_parameters() {
            0 => BASE_PARAMETERS_IDX,
            n => BASE_PARAMETERS_IDX + n,
        };

        let offset = if event_type.has_no_advanced_parameters() {
            base_idx
        } else {
            base_idx + ADVANCED_PARAM_LEN
        };

        match self.valid_idx(offset) {
            Ok(_) => Ok(&self.params[offset..]),
            Err(_) => Ok(&[]),
        }
    }
}

impl ParameterHandler for ArgumentHandler {
    fn len(&self) -> usize {
        self.params.len()
    }

    fn raw(&self, idx: usize) -> Result<&String, JastorError> {
        self.valid_idx(idx)?;
        Ok(&self.params[idx])
    }

    fn valid_idx(&self, idx: usize) -> Result<(), JastorError> {
        if idx >= self.params.len() {
            return Err(JastorError::GenericError(format!(
                "invalid index for param check - received {idx}, param length {}, params {:?}",
                self.params.len(),
                self.params
            )));
        }

        Ok(())
    }
    fn as_string(&self, idx: usize) -> Result<String, JastorError> {
        self.valid_idx(idx)?;
        Ok(self.params[idx]
            .to_string()
            .replace("nil", "")
            .replace("\"", ""))
    }

    fn as_number<T>(&self, idx: usize) -> Result<T, JastorError>
    where
        T: FromStr + Num + NumCast,
        T::Err: Display,
    {
        self.valid_idx(idx)?;
        let value = &self.params[idx];
        if value == "nil" {
            return Ok(T::zero());
        }

        if let Some(stripped) = value.strip_prefix("0x") {
            T::from_str_radix(stripped, 16).map_err(|_| {
                JastorError::ParseError(format!("cannot convert value from hex: {value}"))
            })
        } else {
            T::from_str_radix(value, 10).map_err(|_| {
                JastorError::ParseError(format!("cannot convert string to number: {value}"))
            })
        }
    }

    fn as_boolean(&self, idx: usize) -> Result<bool, JastorError> {
        self.valid_idx(idx)?;
        if self.params[idx] == "nil" {
            return Ok(false);
        }

        let flag = self.params[idx]
            .parse::<i8>()
            .map_err(|e| JastorError::ParseError(e.to_string()))?;

        match flag {
            1 => Ok(true),
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SliceHander<'a> {
    params: &'a [String],
}

impl<'a> SliceHander<'a> {
    pub fn new(params: &'a [String]) -> Self {
        Self { params }
    }

    pub fn as_multi_value_number<T>(&self, idx: usize) -> Result<(usize, T), JastorError>
    where
        T: FromStr + Num + NumCast,
        T::Err: Display,
    {
        self.valid_idx(idx)?;
        let value = &self.params[idx];
        if value.contains("|") {
            let Some((power_type, val)) = value.split_once('|') else {
                unreachable!("covered by conditional");
            };

            let power_type = power_type
                .parse::<usize>()
                .map_err(|e| JastorError::ParseError(e.to_string()))?;

            let val = T::from_str_radix(val, 10).map_err(|_| {
                JastorError::ParseError(format!("cannot convert string to number: {value}"))
            })?;

            Ok((power_type, val))
        } else {
            Ok((
                0,
                T::from_str_radix(value, 10).map_err(|_| {
                    JastorError::ParseError(format!("cannot convert string to number: {value}"))
                })?,
            ))
        }
    }
}

impl ParameterHandler for SliceHander<'_> {
    fn len(&self) -> usize {
        self.params.len()
    }

    fn raw(&self, idx: usize) -> Result<&String, JastorError> {
        self.valid_idx(idx)?;
        Ok(&self.params[idx])
    }

    fn valid_idx(&self, idx: usize) -> Result<(), JastorError> {
        if idx >= self.params.len() {
            return Err(JastorError::GenericError(format!(
                "invalid index for param check - received {idx}, param length {}, params {:?}",
                self.params.len(),
                self.params
            )));
        }

        Ok(())
    }
    fn as_string(&self, idx: usize) -> Result<String, JastorError> {
        self.valid_idx(idx)?;
        Ok(self.params[idx]
            .to_string()
            .replace("nil", "")
            .replace("\"", ""))
    }

    fn as_number<T>(&self, idx: usize) -> Result<T, JastorError>
    where
        T: FromStr + Num + NumCast,
        T::Err: Display,
    {
        self.valid_idx(idx)?;
        let value = &self.params[idx];
        if let Some(stripped) = value.strip_prefix("0x") {
            T::from_str_radix(stripped, 16).map_err(|_| {
                JastorError::ParseError(format!("cannot convert value from hex: {value}"))
            })
        } else {
            T::from_str_radix(value, 10).map_err(|_| {
                JastorError::ParseError(format!("cannot convert string to number: {value}"))
            })
        }
    }

    fn as_boolean(&self, idx: usize) -> Result<bool, JastorError> {
        self.valid_idx(idx)?;
        if self.params[idx] == "nil" {
            return Ok(false);
        }

        let flag = self.params[idx]
            .parse::<i8>()
            .map_err(|e| JastorError::ParseError(e.to_string()))?;

        match flag {
            1 => Ok(true),
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
mod param_handler_test {
    use super::*;

    #[test]
    fn strings() {
        let p = ArgumentHandler::new("3015,\"Mug'Zee, Heads of Security\",17,25,2769");
        assert_eq!(
            p.params,
            vec!["3015", "Mug'Zee, Heads of Security", "17", "25", "2769"]
        );
    }

    #[test]
    fn combat_info() {
        let p = ArgumentHandler::new(
            "Player-1309-0D24DBE3,0,14647,53163,319844,12360,0,0,0,11559,11559,11559,0,0,6698,6698,6698,0,12141,4096,4096,4096,24884,259,[(90624,112509,2),(90625,112510,1),(90628,112513,1),(90633,112518,1),(90635,112520,1),(90636,112521,1),(90684,112572,1),(90686,117143,1),(90692,112580,1),(90695,112583,1),(90697,112585,1),(90742,114737,1),(90743,112633,1),(90744,112634,1),(90745,112635,1),(90746,112636,1),(90750,112642,1),(90751,112643,2),(90752,112644,1),(90754,112646,1),(90756,112648,1),(90757,112649,1),(90758,112650,1),(90759,112651,2),(90760,112652,1),(90762,112654,1),(90763,112655,1),(90764,112657,1),(90769,112662,1),(90771,112664,1),(90783,112676,1),(90785,112678,2),(90786,112679,1),(94536,117106,1),(94554,117132,1),(94556,117136,1),(94557,117139,1),(94562,117145,1),(94563,117146,1),(95108,117705,1),(95109,117706,1),(95110,117707,1),(95117,117714,1),(95123,117720,1),(95131,117728,1),(95132,117729,1),(95135,117732,1),(95142,117739,1),(90741,117740,1),(99844,123375,1),(95106,126029,1),(95136,117733,1),(90740,112630,1),(90621,112505,1),(90626,112511,1),(90638,112523,1),(90639,112525,2),(90640,112526,1),(90766,112659,1),(90767,112660,2),(90768,112661,1),(90772,112665,2),(94552,117130,1),(94553,117131,1),(90622,112507,1),(90770,112663,1),(90777,112670,1)],(0,0,0,0),[(229289,645,(),(6652,11980,12176,10389,12178,11960,1494),()),(221060,619,(),(11954,10388,6652,10394,10393,12045,1654,10254),()),(229287,649,(),(6652,10355,11985,8094,12179,11962,1498),()),(0,0,(),(),()),(235417,636,(),(11977,10389,6652,11964,10383,1671,10255),()),(232727,642,(),(6652,12176,11966,10354,11979,1491,10255),()),(221042,636,(),(11977,10389,6652,11964,10383,1671,10255),()),(232400,606,(),(6652,11968,11945,1478,10254),()),(221142,535,(),(10385,10387,6652,11968),()),(221036,636,(),(11977,10389,6652,11964,10383,1671,10255),()),(221197,636,(),(11977,10389,6652,10394,10392,10383,1671,10255),()),(178872,636,(),(11977,10389,6652,10395,10878,10383,9945,10255),()),(230198,632,(),(6652,10353,11972,1481,10255),()),(232541,645,(),(10389,6652,10383,11980,1481,10255),()),(234507,636,(),(11977,10389,6652,11964,10383,1485,10255),()),(234493,645,(),(43,11215,11976,1494,10255),()),(228897,626,(),(6652,10353,11970,1475,10255),()),(202197,1,(),(),())],[Player-2073-0A627916,6673,1,Player-1403-0ABE9799,465,1,Player-1587-0E860787,21562,1,Player-1085-0AAE12D0,1459,1,Player-1084-0A81ADAB,381754,1,Player-1092-0A8E2252,462854,1,Player-1598-06DC0828,1126,1],1,0,0,0",
        );
        for param in p.params {
            let new_params = param.replace("(", "[").replace(")", "]");
            println!("{new_params}");
        }
    }

    #[test]
    fn challenge_mode() {
        let p = ArgumentHandler::new("\"Mists of Tirna Scithe\",2290,375,11,[9,122,4,121]");
        assert_eq!(
            p.params,
            vec![
                "Mists of Tirna Scithe",
                "2290",
                "375",
                "11",
                "[9,122,4,121]"
            ]
        );
    }
}

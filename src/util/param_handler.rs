use num_traits::{Num, NumCast};
use std::{fmt::Display, str::FromStr};

use crate::{error::JastorError, event::EventType};

const BASE_PARAMETERS_IDX: usize = 8;
const ADVANCED_PARAM_LEN: usize = 19;

pub trait ParameterHandler {
    fn valid_idx(&self, idx: usize) -> Result<(), JastorError>;
    fn as_string(&self, idx: usize) -> Result<String, JastorError>;
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

    pub fn success_flag(&self, idx: usize) -> Result<bool, JastorError> {
        self.valid_idx(idx)?;
        let flag = self.params[idx]
            .parse::<u8>()
            .map_err(|e| JastorError::ParseError(e.to_string()))?;

        match flag {
            1 => Ok(true),
            _ => Ok(false),
        }
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

    pub fn additional_parameters(&self, event_type: EventType) -> Result<&[String], JastorError> {
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
        Ok(self.params[idx].to_string().replace("\"", ""))
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
}

#[derive(Debug, Clone)]
pub struct SliceHander<'a> {
    params: &'a [String],
}

impl<'a> SliceHander<'a> {
    pub fn new(params: &'a [String]) -> Self {
        Self { params }
    }

    pub fn as_multi_value_number<T, U>(&self, idx: usize) -> Result<(T, U), JastorError>
    where
        T: FromStr + Num + NumCast,
        T::Err: Display,
        U: FromStr + Num + NumCast,
        U::Err: Display,
    {
        self.valid_idx(idx)?;
        let value = &self.params[idx];
        if value.contains("|") {
            todo!()
        } else {
            // Handle zero case
            todo!()
        }
    }
}

impl ParameterHandler for SliceHander<'_> {
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
        Ok(self.params[idx].to_string().replace("\"", ""))
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
            "Player-3299-004E8630,1,132,184,906,653,0,0,0,257,257,257,11,0,188,188,188,0,118,90,90,90,120,257,(193155,64129,238136,200199,321377,193157,265202),(0,235587,215982,328530),[0,0,[],[],[]],[(173845,90,(),(1479,4786,6502),()),(158075,140,(),(4932,4933,6316),()),(157971,105,(),(1514,4786,6506),()),(3427,1,(),(),()),(157994,105,(),(1514,4786,6504),()),(173341,90,(0,0,4223),(6707),()),(174237,100,(),(4822,6516,6513,1487,4786),()),(173322,90,(),(6707),()),(158037,99,(),(4803,4802,42,6516,6515,1513,4786),()),(183675,110,(),(1482,4786),()),(173344,98,(),(6706),()),(174469,100,(),(4822,6516,6513,1487,4786),()),(173349,98,(),(6706),()),(175719,104,(),(6707,6901),()),(169223,133,(),(6276,1472),()),(165628,115,(),(5844,1527,4786),()),(0,0,(),(),()),(0,0,(),(),())],[Player-3299-004E8630,295365,Player-3299-004E8630,298268,Player-3299-004E8630,296320],1,0,0,0",
        );
        println!("Arguments: {:?}", p.params);
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

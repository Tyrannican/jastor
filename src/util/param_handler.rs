use std::{fmt::Display, str::FromStr};

use crate::error::JastorError;

const BASE_PARAMETERS_IDX: usize = 8;

#[derive(Debug, Clone)]
pub(crate) struct ParamHandler {
    pub(crate) params: Vec<String>,
}

impl ParamHandler {
    pub fn new(args: &str) -> Self {
        // Strategy:
        // We want to break on ','
        // We want to NOT break on inner strings (e.g. \"Mug'Zee, Heads of Security\")
        // - This should be turned into "Mug'zee, Heads of Security"
        // We want to NOT break on arrays + tuples (e.g. [(123, 1232), (4875, 9873), ...])
        // - Should become "[(123, 1232), (4875, 9873), ...]"
        // There's probably other cases here

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

    pub fn as_string(&self, idx: usize) -> Result<String, JastorError> {
        self.valid_idx(idx)?;
        Ok(self.params[idx].to_string().replace("\"", ""))
    }

    pub fn as_number<T>(&self, idx: usize) -> Result<T, JastorError>
    where
        T: FromStr,
        T::Err: Display,
    {
        self.valid_idx(idx)?;
        self.params[idx]
            .parse::<T>()
            .map_err(|e| JastorError::ParseError(e.to_string()))
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

    pub fn prefix_parameters(&self, number: usize) -> Result<&[String], JastorError> {
        if number == 0 {
            self.valid_idx(BASE_PARAMETERS_IDX)?
        } else {
            self.valid_idx(BASE_PARAMETERS_IDX + (number - 1))?;
        }

        Ok(&self.params[BASE_PARAMETERS_IDX..BASE_PARAMETERS_IDX + number])
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
}

#[cfg(test)]
mod param_handler_test {
    use super::*;

    #[test]
    fn strings() {
        let p = ParamHandler::new("3015,\"Mug'Zee, Heads of Security\",17,25,2769");
        assert_eq!(
            p.params,
            vec!["3015", "Mug'Zee, Heads of Security", "17", "25", "2769"]
        );
    }

    #[test]
    fn combat_info() {
        let p = ParamHandler::new(
            "Player-3299-004E8630,1,132,184,906,653,0,0,0,257,257,257,11,0,188,188,188,0,118,90,90,90,120,257,(193155,64129,238136,200199,321377,193157,265202),(0,235587,215982,328530),[0,0,[],[],[]],[(173845,90,(),(1479,4786,6502),()),(158075,140,(),(4932,4933,6316),()),(157971,105,(),(1514,4786,6506),()),(3427,1,(),(),()),(157994,105,(),(1514,4786,6504),()),(173341,90,(0,0,4223),(6707),()),(174237,100,(),(4822,6516,6513,1487,4786),()),(173322,90,(),(6707),()),(158037,99,(),(4803,4802,42,6516,6515,1513,4786),()),(183675,110,(),(1482,4786),()),(173344,98,(),(6706),()),(174469,100,(),(4822,6516,6513,1487,4786),()),(173349,98,(),(6706),()),(175719,104,(),(6707,6901),()),(169223,133,(),(6276,1472),()),(165628,115,(),(5844,1527,4786),()),(0,0,(),(),()),(0,0,(),(),())],[Player-3299-004E8630,295365,Player-3299-004E8630,298268,Player-3299-004E8630,296320],1,0,0,0",
        );
        println!("Params: {:?}", p.params);
    }

    #[test]
    fn challenge_mode() {
        let p = ParamHandler::new("\"Mists of Tirna Scithe\",2290,375,11,[9,122,4,121]");
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

use std::{fmt::Display, str::FromStr};

use crate::error::JastorError;

#[derive(Debug, Clone)]
pub(crate) struct ParamHandler<'a> {
    params: Vec<&'a str>,
}

impl<'a> ParamHandler<'a> {
    pub fn simple(args: &'a str) -> Self {
        let params = args.split(',').collect::<Vec<&'a str>>();
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

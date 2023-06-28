use std::{
    error,
    fmt::{Display, Formatter, Result},
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new(msg: &str) -> Self {
        Self(String::from(msg))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.0)
    }
}

impl error::Error for Error {}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Self::new(&value.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::new(&value.to_string())
    }
}

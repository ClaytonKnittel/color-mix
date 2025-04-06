use core::fmt;
use std::{
  error::Error,
  fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum ColorMixError {
  InvalidAction(String),
  ParseError(String),
}

impl Display for ColorMixError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      ColorMixError::InvalidAction(msg) => write!(f, "Invalid action: {msg}"),
      ColorMixError::ParseError(msg) => write!(f, "Parse error: {msg}"),
    }
  }
}

impl Error for ColorMixError {}

pub type ColorMixResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

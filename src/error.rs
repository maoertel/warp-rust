use std::num::ParseIntError;

use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
  ParseError(std::num::ParseIntError),
  MissingParameters,
}

impl Reject for Error {}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::ParseError(error) => write!(f, "{}", error),
      Error::MissingParameters => write!(f, "Missing parameter"),
    }
  }
}

impl From<ParseIntError> for Error {
  fn from(error: ParseIntError) -> Self {
    Error::ParseError(error)
  }
}

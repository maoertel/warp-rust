use std::num::ParseIntError;

use reqwest::StatusCode;
use warp::{cors::CorsForbidden, reject::Reject, Rejection, Reply};

use crate::model::InvalidId;

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

pub(crate) async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
  if let Some(error) = r.find::<CorsForbidden>() {
    Ok(warp::reply::with_status(error.to_string(), StatusCode::FORBIDDEN))
  } else if let Some(error) = r.find::<Error>() {
    Ok(warp::reply::with_status(
      error.to_string(),
      StatusCode::RANGE_NOT_SATISFIABLE,
    ))
  } else if let Some(InvalidId) = r.find::<InvalidId>() {
    Ok(warp::reply::with_status(
      "No valid ID presented".to_string(),
      StatusCode::UNPROCESSABLE_ENTITY,
    ))
  } else {
    Ok(warp::reply::with_status(
      "Route not found".to_string(),
      StatusCode::NOT_FOUND,
    ))
  }
}

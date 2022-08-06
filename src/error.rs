use reqwest::StatusCode;
use std::num::ParseIntError;
use uuid::Uuid;
use warp::{body::BodyDeserializeError, cors::CorsForbidden, reject::Reject, Rejection, Reply};

#[derive(Debug)]
pub enum Error {
  Parse(std::num::ParseIntError),
  MissingParameters,
  QuestionNotFound(Uuid),
  InitLogConfig,
}

impl Reject for Error {}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::Parse(error) => write!(f, "{}", error),
      Error::MissingParameters => write!(f, "Missing parameter"),
      Error::QuestionNotFound(id) => write!(f, "Question with id {id} not found."),
      Error::InitLogConfig => write!(f, "Error initialising the log4rs config."),
    }
  }
}

impl From<ParseIntError> for Error {
  fn from(error: ParseIntError) -> Self {
    Error::Parse(error)
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
  } else if let Some(error) = r.find::<BodyDeserializeError>() {
    Ok(warp::reply::with_status(
      error.to_string(),
      StatusCode::UNPROCESSABLE_ENTITY,
    ))
  } else {
    Ok(warp::reply::with_status(
      "Route not found".to_string(),
      StatusCode::NOT_FOUND,
    ))
  }
}

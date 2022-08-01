use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp::reject::Reject;

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Question {
  pub(crate) id: QuestionId,
  pub(crate) title: String,
  pub(crate) content: String,
  pub(crate) tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq, Hash)]
pub(crate) struct QuestionId(pub String);

impl Question {
  pub fn new(id: QuestionId, title: &str, content: &str, tags: Option<&[String]>) -> Question {
    Question {
      id,
      title: title.to_string(),
      content: content.to_string(),
      tags: tags.map(|t| t.to_vec()),
    }
  }
}

#[derive(Debug)]
pub(crate) struct InvalidId;
impl Reject for InvalidId {}

#[derive(Debug)]
pub(crate) struct Pagination {
  pub(crate) start: usize,
  pub(crate) end: usize,
}

impl Pagination {
  pub(crate) fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    match (params.get("start"), params.get("end")) {
      (Some(start), Some(end)) => Ok(Pagination {
        start: start.parse::<usize>()?,
        end: end.parse::<usize>()?,
      }),
      _ => Err(Error::MissingParameters),
    }
  }
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Question {
  pub(crate) id: Uuid,
  pub(crate) title: String,
  pub(crate) content: String,
  pub(crate) tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub(crate) struct QuestionInput {
  pub(crate) title: String,
  pub(crate) content: String,
  pub(crate) tags: Option<Vec<String>>,
}

impl From<QuestionInput> for Question {
  fn from(QuestionInput { title, content, tags }: QuestionInput) -> Self {
    let id = Uuid::new_v4();
    Question {
      id,
      title,
      content,
      tags,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Answer {
  pub(crate) id: Uuid,
  pub(crate) content: String,
  pub(crate) question_id: Uuid,
}

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

use crate::{error::Error, model::Answer, persistence::Store};
use reqwest::StatusCode;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;
use warp::{Rejection, Reply};

pub(crate) async fn add(repo: Arc<Store>, params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  match (params.get("content"), params.get("questionId")) {
    (Some(content), Some(question_id)) => {
      let id = Uuid::new_v4();
      let answer = Answer {
        id,
        content: String::from(content),
        question_id: Uuid::parse_str(question_id).unwrap(),
      };
      repo.answers.write().insert(id, answer);
      Ok(warp::reply::with_status("Answer added", StatusCode::OK))
    }
    _ => Err(warp::reject::custom(Error::MissingParameters)),
  }
}

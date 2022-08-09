use crate::{
  error::Error,
  model::{Pagination, Question, QuestionInput},
  persistence::Store,
};
use reqwest::StatusCode;
use std::{collections::HashMap, sync::Arc};
use tracing::{info, instrument};
use uuid::Uuid;
use warp::{Rejection, Reply};

#[instrument]
pub(crate) async fn get(params: HashMap<String, String>, repo: Arc<Store>) -> Result<impl Reply, Rejection> {
  info!("Querying questions");
  let question_pairs = repo.questions.read().await;
  let questions: &[&Question] = &question_pairs.values().collect::<Vec<_>>();

  let questions = if !params.is_empty() {
    let Pagination { start, end } = Pagination::extract_pagination(params)?;
    info!(pagination = true);
    &questions[start..end]
  } else {
    info!(pagination = false);
    questions
  };

  Ok(warp::reply::json(&questions))
}

pub(crate) async fn add(repo: Arc<Store>, question_input: QuestionInput) -> Result<impl Reply, Rejection> {
  let question = Question::from(question_input);
  repo.questions.write().await.insert(question.id, question);
  Ok(warp::reply::with_status("Question added".to_string(), StatusCode::OK))
}

pub(crate) async fn update(
  repo: Arc<Store>,
  id: Uuid,
  QuestionInput { title, content, tags }: QuestionInput,
) -> Result<impl Reply, Rejection> {
  let question = Question {
    id,
    title,
    content,
    tags,
  };
  match repo.questions.write().await.get_mut(&id) {
    Some(q) => {
      *q = question;
      Ok(warp::reply::with_status(
        format!("Question with id {id} updated."),
        StatusCode::OK,
      ))
    }
    None => Err(warp::reject::custom(Error::QuestionNotFound(id))),
  }
}

pub(crate) async fn delete(repo: Arc<Store>, id: Uuid) -> Result<impl Reply, Rejection> {
  match repo.questions.write().await.remove(&id) {
    Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
    None => Err(warp::reject::custom(Error::QuestionNotFound(id))),
  }
}

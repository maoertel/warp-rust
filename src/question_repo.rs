use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use reqwest::StatusCode;
use uuid::Uuid;
use warp::{Rejection, Reply};

use crate::{
  error::Error,
  model::{Answer, Pagination, Question, QuestionInput},
};

pub(crate) struct Store {
  pub(crate) questions: RwLock<HashMap<Uuid, Question>>,
  pub(crate) answers: RwLock<HashMap<Uuid, Answer>>,
}

impl Store {
  fn init() -> RwLock<HashMap<Uuid, Question>> {
    let file = include_str!("../questions.json");
    RwLock::new(serde_json::from_str(file).expect("Not able to read question.json."))
  }

  pub fn new() -> Self {
    Store {
      questions: Self::init(),
      answers: RwLock::new(HashMap::new()),
    }
  }

  pub(crate) async fn get_questions(
    params: HashMap<String, String>,
    repo: Arc<Store>,
  ) -> Result<impl Reply, Rejection> {
    let question_pairs = repo.questions.read();
    let mut questions: &[&Question] = &question_pairs.values().collect::<Vec<_>>();

    if !params.is_empty() {
      let Pagination { start, end } = Pagination::extract_pagination(params)?;
      questions = &questions[start..end];
    }

    Ok(warp::reply::json(&questions))
  }

  pub(crate) async fn add_question(repo: Arc<Store>, question_input: QuestionInput) -> Result<impl Reply, Rejection> {
    let question = Question::from(question_input);
    repo.questions.write().insert(question.id, question);
    Ok(warp::reply::with_status("Question added".to_string(), StatusCode::OK))
  }

  pub(crate) async fn update_question(
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
    match repo.questions.write().get_mut(&id) {
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

  pub(crate) async fn delete_question(repo: Arc<Store>, id: Uuid) -> Result<impl Reply, Rejection> {
    match repo.questions.write().remove(&id) {
      Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
      None => Err(warp::reject::custom(Error::QuestionNotFound(id))),
    }
  }

  pub(crate) async fn add_answer(repo: Arc<Store>, params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
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
}

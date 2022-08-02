use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use reqwest::StatusCode;
use uuid::Uuid;
use warp::{Rejection, Reply};

use crate::model::{Pagination, Question, QuestionInput};

pub(crate) struct Store {
  pub(crate) questions: RwLock<HashMap<Uuid, Question>>,
}

impl Store {
  fn init() -> RwLock<HashMap<Uuid, Question>> {
    let file = include_str!("../questions.json");
    RwLock::new(serde_json::from_str(file).expect("Not able to read question.json."))
  }

  pub fn new() -> Self {
    Store {
      questions: Self::init(),
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
}

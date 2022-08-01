use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use warp::{Rejection, Reply};

use crate::model::{Pagination, Question, QuestionId};

pub(crate) struct Store {
  pub(crate) questions: RwLock<HashMap<QuestionId, Question>>,
}

impl Store {
  fn init() -> RwLock<HashMap<QuestionId, Question>> {
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
}

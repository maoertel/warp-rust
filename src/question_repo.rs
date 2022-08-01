use std::collections::HashMap;

use warp::{Rejection, Reply};

use crate::model::{Pagination, Question, QuestionId};

#[derive(Clone)]
pub(crate) struct Store {
  pub(crate) questions: HashMap<QuestionId, Question>,
}

impl Store {
  fn init() -> HashMap<QuestionId, Question> {
    let file = include_str!("../questions.json");
    serde_json::from_str(file).expect("Not able to read question.json.")
  }

  pub fn new() -> Self {
    Store {
      questions: Self::init(),
    }
  }

  pub fn add_question(mut self, question: Question) -> Self {
    self.questions.insert(question.id.clone(), question);
    self
  }

  pub(crate) async fn get_questions(params: HashMap<String, String>, repo: Store) -> Result<impl Reply, Rejection> {
    let mut res: &[&Question] = &repo.questions.values().collect::<Vec<_>>();

    if !params.is_empty() {
      let Pagination { start, end } = Pagination::extract_pagination(params)?;
      res = &res[start..end];
    }

    Ok(warp::reply::json(&res))
  }
}

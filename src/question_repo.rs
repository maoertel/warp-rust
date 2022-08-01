use std::collections::HashMap;

use crate::model::{Question, QuestionId};

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
}

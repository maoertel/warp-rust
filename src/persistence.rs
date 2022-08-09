use crate::model::{Answer, Question};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
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
}

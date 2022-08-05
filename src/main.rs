pub mod error;
mod model;
mod question_repo;
mod question_routes;

use std::{convert::Infallible, sync::Arc};
use uuid::Uuid;
use warp::Filter;

use error::return_error;

use question_repo::Store;
use reqwest::Method;

lazy_static::lazy_static! {
    static ref STORE: Arc<Store> = Arc::new(Store::new());
}

#[tokio::main]
async fn main() {
  let question_path = "questions";
  let answer_path = "answers";

  fn inject(store_ref: &Arc<Store>) -> impl Filter<Extract = (Arc<Store>,), Error = Infallible> + Copy + '_ {
    warp::any().map(move || Arc::clone(store_ref))
  }

  let cors = warp::cors()
    .allow_any_origin()
    .allow_header("content-type")
    .allow_methods(&[Method::PUT, Method::POST, Method::DELETE, Method::GET]);

  let get_questions = warp::get()
    .and(warp::path(question_path))
    .and(warp::path::end())
    .and(warp::query())
    .and(inject(&STORE))
    .and_then(Store::get_questions);

  let add_question = warp::post()
    .and(warp::path(question_path))
    .and(warp::path::end())
    .and(inject(&STORE))
    .and(warp::body::json())
    .and_then(Store::add_question);

  let update_question = warp::put()
    .and(warp::path(question_path))
    .and(inject(&STORE))
    .and(warp::path::param::<Uuid>())
    .and(warp::path::end())
    .and(warp::body::json())
    .and_then(Store::update_question);

  let delete_question = warp::delete()
    .and(warp::path(question_path))
    .and(inject(&STORE))
    .and(warp::path::param::<Uuid>())
    .and(warp::path::end())
    .and_then(Store::delete_question);

  let add_answer = warp::post()
    .and(warp::path(answer_path))
    .and(warp::path::end())
    .and(inject(&STORE))
    .and(warp::body::form())
    .and_then(Store::add_answer);

  let routes = get_questions
    .or(add_question)
    .or(update_question)
    .or(delete_question)
    .or(add_answer)
    .with(cors)
    .recover(return_error);

  warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

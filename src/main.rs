use uuid::Uuid;
use warp::Filter;
pub mod error;
mod model;
mod question_repo;
mod question_routes;

use std::sync::Arc;

use error::return_error;

use question_repo::Store;
use reqwest::Method;

#[tokio::main]
async fn main() {
  let store = Arc::new(Store::new());
  let store_filter = warp::any().map(move || store.clone());
  let path = "questions";

  let cors = warp::cors()
    .allow_any_origin()
    .allow_header("content-type")
    .allow_methods(&[Method::PUT, Method::POST, Method::DELETE, Method::GET]);

  let get_questions = warp::get()
    .and(warp::path(path))
    .and(warp::path::end())
    .and(warp::query())
    .and(store_filter.clone())
    .and_then(Store::get_questions);

  let add_question = warp::post()
    .and(warp::path(path))
    .and(warp::path::end())
    .and(store_filter.clone())
    .and(warp::body::json())
    .and_then(Store::add_question);

  let update_question = warp::put()
    .and(warp::path(path))
    .and(store_filter.clone())
    .and(warp::path::param::<Uuid>())
    .and(warp::path::end())
    .and(warp::body::json())
    .and_then(Store::update_question);

  let delete_question = warp::delete()
    .and(warp::path(path))
    .and(store_filter)
    .and(warp::path::param::<Uuid>())
    .and(warp::path::end())
    .and_then(Store::delete_question);

  let routes = get_questions
    .or(add_question)
    .or(update_question)
    .or(delete_question)
    .with(cors)
    .recover(return_error);

  warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

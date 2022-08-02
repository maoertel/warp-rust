pub mod error;
mod model;
mod question_repo;

use error::return_error;
use std::sync::Arc;

use question_repo::Store;
use reqwest::Method;
use warp::Filter;

#[tokio::main]
async fn main() {
  let store = Arc::new(Store::new());
  let store_filter = warp::any().map(move || store.clone());

  let cors = warp::cors()
    .allow_any_origin()
    .allow_header("content-type")
    .allow_methods(&[Method::PUT, Method::POST, Method::DELETE, Method::GET]);

  let get_items = warp::get()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and(warp::query())
    .and(store_filter.clone())
    .and_then(Store::get_questions);

  let add_question = warp::post()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and(store_filter.clone())
    .and(warp::body::json())
    .and_then(Store::add_question);

  let routes = get_items.or(add_question).with(cors).recover(return_error);

  warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

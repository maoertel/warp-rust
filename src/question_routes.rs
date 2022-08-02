use std::sync::Arc;
use uuid::Uuid;
use warp::Filter;

use crate::question_repo::Store;

pub(crate) fn create(store: Arc<Store>) -> impl Filter {
  let store_filter = warp::any().map(move || store.clone());

  let path = "questions";

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

  get_questions.or(add_question).or(update_question).or(delete_question)
}

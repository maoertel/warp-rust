use std::{convert::Infallible, sync::Arc};

use reqwest::Method;
use uuid::Uuid;
use warp::Filter;

use crate::{error::return_error, question_repo::Store};

static QUESTION_PATH: &str = "questions";
static ANSWER_PATH: &str = "answers";

pub(crate) async fn start<'a>(store: &'static Arc<Store>) {
  let cors = warp::cors()
    .allow_any_origin()
    .allow_header("content-type")
    .allow_methods(&[Method::PUT, Method::POST, Method::DELETE, Method::GET]);

  let get_questions = warp::get()
    .and(warp::path(QUESTION_PATH))
    .and(warp::path::end())
    .and(warp::query())
    .and(inject(store))
    .and_then(Store::get_questions);

  let add_question = warp::post()
    .and(warp::path(QUESTION_PATH))
    .and(warp::path::end())
    .and(inject(store))
    .and(warp::body::json())
    .and_then(Store::add_question);

  let update_question = warp::put()
    .and(warp::path(QUESTION_PATH))
    .and(inject(store))
    .and(warp::path::param::<Uuid>())
    .and(warp::path::end())
    .and(warp::body::json())
    .and_then(Store::update_question);

  let delete_question = warp::delete()
    .and(warp::path(QUESTION_PATH))
    .and(inject(store))
    .and(warp::path::param::<Uuid>())
    .and(warp::path::end())
    .and_then(Store::delete_question);

  let add_answer = warp::post()
    .and(warp::path(ANSWER_PATH))
    .and(warp::path::end())
    .and(inject(store))
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

fn inject(store_ref: &Arc<Store>) -> impl Filter<Extract = (Arc<Store>,), Error = Infallible> + Copy + '_ {
  warp::any().map(move || Arc::clone(store_ref))
}

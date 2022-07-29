mod model;

use std::i32;

use model::{InvalidId, Question, QuestionId};
use reqwest::Method;
use warp::{cors::CorsForbidden, http::StatusCode, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
  let cors = warp::cors()
    .allow_any_origin()
    .allow_header("content-type")
    .allow_methods(&[Method::PUT, Method::POST, Method::DELETE, Method::GET]);

  let get_items = warp::get()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and_then(get_questions)
    .recover(return_error);

  let routes = get_items.with(cors);

  warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
  if let Some(error) = r.find::<CorsForbidden>() {
    Ok(warp::reply::with_status(error.to_string(), StatusCode::FORBIDDEN))
  } else if let Some(InvalidId) = r.find() {
    Ok(warp::reply::with_status(
      "No valid ID presented".to_string(),
      StatusCode::UNPROCESSABLE_ENTITY,
    ))
  } else {
    Ok(warp::reply::with_status(
      "Route not found".to_string(),
      StatusCode::NOT_FOUND,
    ))
  }
}

async fn get_questions() -> Result<impl Reply, Rejection> {
  let question = Question::new(
    QuestionId(String::from("3")),
    "What\'s the fuzz about",
    "Tell me more!",
    Some(&["faq".to_string()]),
  );

  match question.id.0.parse::<i32>() {
    Ok(_) => Ok(warp::reply::json(&question)),
    Err(_) => Err(warp::reject::custom(InvalidId)),
  }
}

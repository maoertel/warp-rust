pub mod error;
mod model;
mod question_repo;

use std::collections::HashMap;

use error::Error;
use model::{InvalidId, Pagination, Question};
use question_repo::Store;
use reqwest::Method;
use warp::{cors::CorsForbidden, http::StatusCode, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
  let store = Store::new();
  let store_filter = warp::any().map(move || store.clone());
  let cors = warp::cors()
    .allow_any_origin()
    .allow_header("content-type")
    .allow_methods(&[Method::PUT, Method::POST, Method::DELETE, Method::GET]);

  let get_items = warp::get()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and(warp::query())
    .and(store_filter)
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

async fn get_questions(params: HashMap<String, String>, repo: Store) -> Result<impl Reply, Rejection> {
  let mut res: &[&Question] = &repo.questions.values().collect::<Vec<_>>();

  if !params.is_empty() {
    let Pagination { start, end } = extract_pagination(params)?;
    res = &res[start..end];
  }

  Ok(warp::reply::json(&res))
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
  match (params.get("start"), params.get("end")) {
    (Some(start), Some(end)) => Ok(Pagination {
      start: start.parse::<usize>()?,
      end: end.parse::<usize>()?,
    }),
    _ => Err(Error::MissingParameters),
  }
}

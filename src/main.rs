use std::i32;

use serde::Serialize;
use warp::{http::StatusCode, reject::Reject, Filter, Rejection, Reply};

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: &str, content: &str, tags: Option<&[String]>) -> Question {
        Question {
            id,
            title: title.to_string(),
            content: content.to_string(),
            tags: tags.map(|t| t.to_vec()),
        }
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

#[tokio::main]
async fn main() {
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(InvalidId) = r.find() {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
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

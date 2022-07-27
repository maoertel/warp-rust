use warp::Filter;

#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
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

#[tokio::main]
async fn main() {
    let question = Question::new(
        QuestionId(String::from("me")),
        "What\'s the fuzz about",
        "Tell me more!",
        Some(&["faq".to_string()]),
    );

    println!("{:#?}", question);

    let hello = warp::get().map(|| format!("Hello world!"));
    warp::serve(hello).run(([127, 0, 0, 1], 1337)).await
}

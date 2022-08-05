pub mod error;
mod model;
mod question_repo;
mod question_routes;
mod server;

use std::sync::Arc;

use question_repo::Store;

lazy_static::lazy_static! {
    static ref STORE: Arc<Store> = Arc::new(Store::new());
}

#[tokio::main]
async fn main() {
  server::start(&STORE).await
}

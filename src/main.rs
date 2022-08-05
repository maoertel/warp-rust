mod answers;
mod error;
mod model;
mod persistence;
mod questions;
mod server;

use persistence::Store;
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref STORE: Arc<Store> = Arc::new(Store::new());
}

#[tokio::main]
async fn main() {
  server::start(&STORE).await
}

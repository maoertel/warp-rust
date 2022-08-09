mod answers;
mod error;
mod model;
mod persistence;
mod questions;
mod server;
mod tracing;

use error::Error;
use persistence::Store;
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref STORE: Arc<Store> = Arc::new(Store::new());
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  tracing::init();
  server::start(&STORE).await;

  Ok(())
}

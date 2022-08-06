mod answers;
mod error;
mod logging;
mod model;
mod persistence;
mod questions;
mod server;

use error::Error;
use persistence::Store;
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref STORE: Arc<Store> = Arc::new(Store::new());
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  logging::init("log4rs.yaml")?;

  let logger = warp::log::custom(|info| eprintln!("{} {} {}", info.method(), info.path(), info.status()));
  server::start(&STORE, logger).await;

  Ok(())
}

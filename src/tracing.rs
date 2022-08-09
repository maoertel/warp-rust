use tracing_subscriber::fmt::format::FmtSpan;

pub(crate) fn init() {
  let log_filter = std::env::var("RUST_LOG");
  let log_filter = log_filter.iter().fold("practical_rust_book=info,warp=error", |_, v| v);

  tracing_subscriber::fmt()
    .with_env_filter(log_filter)
    .with_span_events(FmtSpan::CLOSE)
    .init();
}

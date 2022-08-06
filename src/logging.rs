use crate::error::Error;

pub fn init(file_path: &str) -> Result<(), Error> {
  log4rs::init_file(file_path, Default::default()).map_err(|_| Error::InitLogConfig)
}

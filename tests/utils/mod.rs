#![allow(dead_code)]

pub mod assets;

pub fn start_logger(default: &str) -> Result<(), flexi_logger::FlexiLoggerError> {
  flexi_logger::Logger::try_with_env_or_str(default)?.start()?;
  Ok(())
}

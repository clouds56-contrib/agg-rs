use std::env;
use std::path::{Path, PathBuf};
// https://www.reddit.com/r/rust/comments/ahsz9q/psa_if_the_examples_for_your_crate_rely_on_media/

pub fn is_base_dir(cwd: &Path) -> Option<(PathBuf, PathBuf)> {
  if !cwd.join("Cargo.toml").is_file() {
    return None;
  }
  let mut images = cwd.to_path_buf();
  let mut test_tmp = cwd.to_path_buf();
  images.push("images");
  test_tmp.push("tests");
  test_tmp.push("tmp");
  if images.is_dir() && test_tmp.is_dir() {
    Some((images, test_tmp))
  } else {
    None
  }
}

pub fn start_logger(default: &str) -> Result<(), flexi_logger::FlexiLoggerError> {
  flexi_logger::Logger::try_with_env_or_str(default)?.start()?;
  Ok(())
}

pub fn find_assets() -> Option<(PathBuf, PathBuf)> {
  start_logger("debug").ok();

  // First check currnet directory
  let cwd = env::current_dir().ok()?;
  if let Some(v) = is_base_dir(&cwd) {
    return Some(v);
  }
  // Search backwards from current executable path
  let mut exec = env::current_exe().ok()?;
  while let Some(dir) = exec.parent() {
    if let Some(v) = is_base_dir(dir) {
      return Some(v);
    }
    exec = dir.to_path_buf();
  }
  // Could not find base directory
  None
}

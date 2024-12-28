use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use crate::log;

use super::data;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize, Serialize)]
struct Patch {
  name: String,
  author: String,
  repository: String,
  version: String,
  archive_path: String,
  sha256: String,
  unpack_on: String,
}

#[derive(Deserialize, Serialize)]
struct Body {
  pathes: HashMap<String, Patch>,
  version: String,
}

/**
 * Receives data with current patches
 */
pub(crate) async fn check() -> anyhow::Result<()> {
  let version = data::get()
    .await?
    .version;

  version.eq(VERSION)
    .then(|| ())
    .ok_or_else(|| anyhow::anyhow!("You have to update the tool! New version {version}, current version {VERSION}."))?;

  log::success(&format!("Tool is up to date (v{VERSION})"));

  Ok(())
}
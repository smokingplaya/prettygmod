use anyhow::Context;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

lazy_static::lazy_static! {
  static ref DATA: Mutex<Option<Body>> = Mutex::new(None);
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Url(String);

impl Url {
  pub fn get_file_name(
    &self
  ) -> anyhow::Result<String> {
    self.0.rsplit('/').next()
      .filter(|filename| !filename.is_empty())
      .map(|filename| filename.to_string())
      .ok_or_else(|| anyhow::anyhow!("Invalid URL format: no filename found"))
  }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum EntityTypes {
  #[serde(rename = "zip")]
  Zip,
  #[serde(rename = "rar")]
  Rar,
  #[serde(rename = "exe")]
  Exe,
  #[serde(rename = "bat")]
  Bat,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Entity {
  pub r#type: EntityTypes,
  pub url: Url,
  pub sha256: String,
  pub unpack: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Patch {
  pub name: String,
  pub author: String,
  pub repository: String,
  pub version: String,
  pub entity: Entity
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Body {
  pub patches: Vec<Patch>,
  pub version: String,
}

/**
 * Receives data with current patches
 */
pub(crate) async fn request() -> anyhow::Result<()> {
  let json = reqwest::get("https://raw.githubusercontent.com/smokingplaya/prettygmod/refs/heads/master/app.json")
    .await?
    .error_for_status()?
    .text()
    .await?;

  *DATA.lock().await = Some(serde_json::from_str::<Body>(&json)?);

  Ok(())
}

#[allow(unused)]
pub(crate) async fn get() -> anyhow::Result<Body> {
  Ok(DATA.lock().await.clone().context("Data isn't loaded")?)
}
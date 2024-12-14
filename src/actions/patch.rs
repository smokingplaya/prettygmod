use crate::log;

use self::data::Patch;
use super::data;

impl Patch {
  async fn run(
    &self
  ) -> anyhow::Result<()> {
    Ok(())
  }
}

pub(crate) async fn all() -> anyhow::Result<()> {
  let patch_list = data::get()
    .await?;

  for patch in patch_list.pathes {
    match patch.run().await {
      Ok(_) => continue,
      Err(e) => log::error(&format!("Unable to install patch {}: {e}", patch.name))
    }
  }

  Ok(())
}
use colored::Colorize;
use crate::log;
use self::data::Patch;
use super::data;

impl Patch {
  async fn download(
    &self
  ) -> anyhow::Result<()> {
    log::action(&format!("Downloading patch \"{}\" {}", self.name.clone().blue(), self.version.clone().blue()));
    println!("\t\tby {}", self.author.clone().blue());

    Ok(())
  }

  async fn install(
    &self
  ) -> anyhow::Result<()> {
    log::action(&format!("Installing patch \"{}\"", self.name.clone().blue()));
    log::subaction(&format!("Moving {} to GMod path", self.name.clone()));

    let archive_filename = self.entity.url
      .clone()
      .get_file_name()?;

    log::subaction(&format!("Unpacking {}", archive_filename.blue()));
    log::success(&format!("Patch {} successfuly installed.", self.name.clone().blue()));

    Ok(())
  }

  pub async fn run(
    &self
  ) -> anyhow::Result<()> {
    self.download()
      .await?;

    log::space();

    self.install()
      .await?;

    Ok(())
  }
}

pub(crate) async fn all() -> anyhow::Result<()> {
  let patch_list = data::get()
    .await?;

  for patch in patch_list.patches {
    match patch.run().await {
      Ok(_) => continue,
      Err(e) => log::error(&format!("Unable to install patch {}: {e}", patch.name))
    }
  }

  Ok(())
}
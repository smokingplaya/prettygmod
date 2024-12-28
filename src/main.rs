mod actions;
mod log;

const NAME: &str = "PrettyGModPatcher";

async fn runtime() -> anyhow::Result<()> {
  log::title()?;
  log::action("Check tool version");

  actions::data::request()
    .await?;

  actions::version::check()
    .await?;

  log::space();

  let _game_path = actions::gmod::get_path()
    .await?;

  log::space();

  actions::patch::all()
    .await?;

  Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Yeeep
  // Перехватываем ошибку и принтим её по красоте
  let _ = runtime().await
    .map_err(|e| log::error(&e.to_string()));

  Ok(())
}
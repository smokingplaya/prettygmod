mod actions;
mod log;

const NAME: &str = "PrettyGModPatcher";

async fn runtime() -> anyhow::Result<()> {
  log::title()?;
  log::action("Check tool version");

  actions::data::request()
    .await?;

  println!();

  actions::version::check()
    .await?;

  println!();

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
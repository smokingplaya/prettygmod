use crate::log;

/// Todo
pub(crate) async fn get_path() -> anyhow::Result<String> {
  log::action("Try to get installed GMod path");
  log::success("Successfull");

  Ok(String::new())
}
use std::{env, path::PathBuf};
use anyhow::Context;
use colored::Colorize;

use crate::NAME;

const LOG_FILE: &str = "prettygmod.log";
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static::lazy_static! {
  static ref LOG_FILE_PATH: PathBuf = {
    // yeppi yep
    let home = env::var("HOME")
      .or_else(|_| env::var("USERPROFILE"))
      .unwrap();
    let path = PathBuf::from(home);

    path.join(LOG_FILE)
  };
}

fn log(log: &str) -> anyhow::Result<()> {
  println!("{log}");

  // Todo file log

  Ok(())
}

pub(crate) fn title() -> anyhow::Result<()> {
  let log_path = LOG_FILE_PATH.to_owned();
  let log_path_nice = log_path
    .to_str()
    .context("Unable to convert PathBuf to &str")?;

  println!("{} {VERSION} by {}", NAME.cyan(), AUTHORS.blue());
  println!("\t\t\t(Serenity Team)"); // yep
  println!("Logs will be saved in {}", log_path_nice.blue());
  println!();

  Ok(())
}

pub(crate) fn action(msg: &str) {
  let _ = log(&format!("{} {msg}", "$".yellow()));
}

pub(crate) fn error(msg: &str) {
  let _ = log(&format!("{} {msg}", "?".red()));
}
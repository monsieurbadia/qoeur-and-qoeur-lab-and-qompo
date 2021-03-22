extern crate qute;

use crate::arg::Arg;
use crate::cmd::{Cmd, CmdResult};
use crate::kind::input::InputKind;
use crate::reader::Reader;

use chrono::{DateTime, FixedOffset, Utc};
use qute::prelude::*;

impl Cmd {
  pub fn run_repl(&mut self) -> CmdResult<()> {
    let reader = Reader::new();

    match &self.arg {
      Arg {
        cmd: Ok(_),
        input: Ok(_),
        mode: Ok(_),
        optional: None,
        path: None,
      } => reader.add_arg(&self.arg).readline(),
      Arg {
        cmd: Ok(_),
        input: Ok(i),
        mode: Ok(_),
        optional: Some(_),
        path: Some(_),
      }
      | Arg {
        cmd: Ok(_),
        input: Ok(i),
        mode: Ok(_),
        optional: None,
        path: Some(_),
      } => match i {
        InputKind::Line => reader.add_arg(&self.arg).readline(),
        InputKind::File => reader.add_arg(&self.arg).readfile(),
      },
      _ => self.run_help(),
    }
  }

  pub fn banner() {
    let help_fmt = format!("{}", "\"help\"");
    let help_styled = qute!(&help_fmt).cyan().italic();

    let copyright_fmt = format!("{}", "\"copyright\"");
    let copyright_styled = qute!(&copyright_fmt).cyan().italic();

    let license_fmt = format!("{}", "\"license\"");
    let license_styled = qute!(&license_fmt).cyan().italic();

    let heart_fmt = " \u{2665} ";
    let heart_styled = qute!(&heart_fmt).red();

    let mode_fmt = format!("{}", heart_styled);
    let mode_styled = qute!(&mode_fmt).black().background_white();

    let langname_fmt = format!(" v{} ", Cmd::version());
    let langname_styled = qute!(&langname_fmt).black().background_cyan();

    let username_styled = qute!(&Cmd::username()).underline();
    let datetime_styled = qute!(&Cmd::datetime()).italic();

    println!(
      "\n{}{}\n\nhello {}! {}\nuse {}, {} or {} for more information.",
      mode_styled,
      langname_styled,
      username_styled,
      datetime_styled,
      help_styled,
      copyright_styled,
      license_styled,
    );
  }

  // TODO: detect current timezone
  fn datetime() -> String {
    let now = Utc::now();
    let tz = FixedOffset::east(2 * 3600);
    let utc_time = DateTime::<Utc>::from_utc(now.naive_utc(), Utc);

    utc_time
      .with_timezone(&tz)
      .format("(%B %d %Y, %H:%M:%S)")
      .to_string()
  }

  fn username() -> String {
    const LOG_NAME: &str = "LOGNAME";
    const LOG_INCOGNITO: &str = "johndoe";

    std::env::var(LOG_NAME).unwrap_or(String::from(LOG_INCOGNITO))
  }
}

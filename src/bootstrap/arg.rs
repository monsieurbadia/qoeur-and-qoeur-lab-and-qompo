use crate::cmd::CmdResult;
use crate::kind::*;

use std::env;

#[derive(Clone, Debug)]
pub struct Arg {
  pub cmd: CmdResult<CmdKind>,
  pub input: CmdResult<InputKind>,
  pub mode: CmdResult<ModeKind>,
  pub optional: Option<OptionalKind>,
  pub path: Option<PathKind>,
}

impl Arg {
  pub fn new(args: &Vec<String>) -> Self {
    Arg {
      cmd: Arg::parse_cmd(&args[0]),
      input: Arg::parse_input(&args[1..].to_vec()),
      mode: Arg::parse_mode(&args[1..].to_vec()),
      optional: Arg::parse_optional(&args[1..].to_vec()),
      path: Arg::parse_path(&args[1..].to_vec()),
    }
  }

  pub fn parse_cmd(name: &str) -> CmdResult<CmdKind> {
    match name {
      _ if name.contains(&"copyright") => Ok(CmdKind::Copyright),
      _ if name.contains(&"help") => Ok(CmdKind::Help),
      _ if name.contains(&"license") => Ok(CmdKind::License),
      _ if name.contains(&"repl") => Ok(CmdKind::Repl),
      _ if name.contains(&"version") => Ok(CmdKind::Version),
      _ => Err(format!("cmd `{}` not exist", name)),
    }
  }

  pub fn parse_input(args: &Vec<String>) -> CmdResult<InputKind> {
    match args {
      _ if args.contains(&"-file".into()) => Ok(InputKind::File),
      _ if args.contains(&"-line".into()) => Ok(InputKind::Line),
      _ => Err(format!("input `{:?}` not exist", args)),
    }
  }

  pub fn parse_mode(args: &Vec<String>) -> CmdResult<ModeKind> {
    match args {
      _ if args.contains(&"-ast".into()) => Ok(ModeKind::Ast),
      _ if args.contains(&"-eval".into()) => Ok(ModeKind::Eval),
      _ if args.contains(&"-tokens".into()) => Ok(ModeKind::Tokens),
      _ if args.contains(&"-js".into()) => Ok(ModeKind::Js),
      _ => Err(format!("mode `{:?}` not exist", args)),
    }
  }

  pub fn parse_optional(args: &Vec<String>) -> Option<OptionalKind> {
    match args {
      _ if args.contains(&"-inline".into()) => Some(OptionalKind::Inline),
      _ if args.contains(&"-json".into()) => Some(OptionalKind::Json),
      _ if args.contains(&"-pretty".into()) => Some(OptionalKind::Pretty),
      _ => None,
    }
  }

  pub fn parse_path(args: &Vec<String>) -> Option<PathKind> {
    match args {
      _ if args.len() == 3 => {
        let absolute_path = format!(
          "{}/{}",
          env::current_dir().unwrap().display(),
          args[2].to_string(),
        );

        Some(PathKind::Exist(absolute_path))
      }
      _ => None,
    }
  }
}

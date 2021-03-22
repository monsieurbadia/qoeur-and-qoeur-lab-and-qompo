use crate::arg::Arg;
use crate::kind::{mode::ModeKind, optional::OptionalKind};

use qoeurc::analyzer;
use qoeurc::analyzer::interpreter::Interpreter;
use qoeurc::converter;
use qoeurc::tokenizer::{self, token::Token};
use qoeurc::transformer;

pub type ReaderError = String;
pub type ReaderResult<T> = Result<T, ReaderError>;

#[derive(Clone, Debug)]
pub struct Reader {
  pub arg: Option<Arg>,
  pub input: Option<String>,
  pub prompt: Option<String>,
  interpreter: Interpreter,
}

impl Reader {
  pub fn new() -> Self {
    Reader {
      arg: None,
      input: None,
      prompt: None,
      interpreter: Interpreter::new(),
    }
  }

  pub fn advance_mode(&mut self, file: &str) -> ReaderResult<()> {
    match &self.arg.to_owned().unwrap().mode.unwrap() {
      ModeKind::Ast => Ok(self.astify(file)?),
      ModeKind::Eval => Ok(self.evalify(file)?),
      ModeKind::Js => Ok(self.transformify(file)?),
      ModeKind::Tokens => Ok(self.tokenify(file)?),
    }
  }

  pub fn add_arg(mut self, arg: &Arg) -> Self {
    self.arg = Some(arg.to_owned());
    self
  }

  pub fn astify(&mut self, input: &str) -> ReaderResult<()> {
    match converter::astify(input) {
      Err(error) => Err(format!("{}", error)),
      Ok(program) => Ok(println!("{:?}", program)),
    }
  }

  pub fn evalify(&mut self, input: &str) -> ReaderResult<()> {
    match analyzer::evalify(input, &mut self.interpreter) {
      Err(error) => Err(format!("{}", error)),
      Ok(program) => Ok(program.print()),
    }
  }

  pub fn transformify(&mut self, input: &str) -> ReaderResult<()> {
    let arg = self.arg.as_ref().unwrap().to_owned();
    let optional = arg.optional.unwrap_or(OptionalKind::Inline);

    match transformer::transformify(
      input,
      &optional.to_string(),
      &self.interpreter,
    ) {
      Err(error) => Err(format!("{}", error)),
      Ok(program) => Ok(println!("{}", program)),
    }
  }

  pub fn prompt() {
    const ICON_PROMPT: &str = "📡 ";

    print!("\n{} ", ICON_PROMPT);
  }

  pub fn tokenify(&mut self, input: &str) -> ReaderResult<()> {
    let tokens = tokenizer::tokenify(input.into()).collect::<Vec<Token>>();

    Ok(println!("{:?}", tokens))
  }
}

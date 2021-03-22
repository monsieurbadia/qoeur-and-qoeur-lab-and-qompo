use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::utils::iters::{eval_statements, transpile_exprs};
use crate::value::instruction::statement::Statement;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Debug, Clone)]
pub struct Program {
  pub statements: Vec<Box<dyn Value>>,
}

impl Default for Program {
  fn default() -> Self {
    Program::new(vec![])
  }
}

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Program {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    Ok(eval_statements(interpreter, self.statements.to_owned())?)
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn ikind(&self) -> IKind {
    IKind::Program(self.statements.to_vec())
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    loop {
      if parser.scanner.is_eof() {
        self.parse_program(parser);
        break;
      }

      match parser.token.kind {
        TokenKind::Unknown => break,
        _ => {
          self.parse_program(parser);
          parser.next_token();
        }
      }
    }

    Ok(self.boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    let statements = self
      .statements
      .iter()
      .map(|stmt| format!("{}", stmt.text()))
      .collect::<Vec<String>>()
      .join("\n");

    format!("(function () {{ {} }})();", statements)
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "body": [{}],
          "type": "Program"
        }}"#,
        transpile_exprs(transpiler, &self.statements, ", ")
      ),
      _ => transpile_exprs(transpiler, &self.statements, ""),
    }
  }
}

impl Program {
  pub fn new(statements: Vec<Box<dyn Value>>) -> Self {
    Program { statements }
  }

  pub fn add_statement(&mut self, statement: Box<dyn Value>) {
    self.statements.push(statement);
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }

  fn parse_program(&mut self, parser: &mut Parser) {
    match Statement::default().parse(parser) {
      Err(error) => parser.errors.push(Err(format!("{}", error))),
      Ok(statement) => self.add_statement(statement),
    }
  }
}

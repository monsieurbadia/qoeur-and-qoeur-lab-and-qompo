use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::utils::iters::{eval_statements, strip_exprs};
use crate::value::instruction::return_value::Return;
use crate::value::instruction::statement::Statement;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};

use std::fmt;

#[derive(Clone, Debug)]
pub struct Block {
  pub statements: Vec<Box<dyn Value>>,
}

impl Default for Block {
  fn default() -> Self {
    Block::new(vec![])
  }
}

impl fmt::Display for Block {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for Block {
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
    IKind::Block(self.statements.to_vec())
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.next_token();

    while !parser.token_is(TokenKind::GroupEnd(Brace)) {
      match Statement::default().parse(parser) {
        Err(_) => break,
        Ok(statement) => self.add_statement(statement),
      };

      parser.next_token();
    }

    Ok(self.boxed())
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    strip_exprs(&self.statements, " ")
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    let block = self
      .statements
      .iter()
      .enumerate()
      .map(|(x, stmt)| {
        if x == self.statements.len() - 1
          && !stmt.to_owned().text().contains(";")
        {
          return Return::default()
            .add_value(stmt.to_owned())
            .transpile(transpiler);
        }

        format!("{}", stmt.to_owned().transpile(transpiler))
      })
      .collect::<Vec<String>>();

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "body": [{}],
          "type": "BlockStatement"
        }}"#,
        block.join(", "),
      ),
      TKind::Inline => block.join(""),
      TKind::Pretty => block.join(""),
    }
  }
}

impl Block {
  pub fn new(statements: Vec<Box<dyn Value>>) -> Self {
    Block { statements }
  }

  pub fn add_statement(&mut self, statement: Box<dyn Value>) -> &Self {
    self.statements.push(statement);
    self
  }

  pub fn add_statements(&mut self, statements: Vec<Box<dyn Value>>) -> &Self {
    self.statements = statements;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}

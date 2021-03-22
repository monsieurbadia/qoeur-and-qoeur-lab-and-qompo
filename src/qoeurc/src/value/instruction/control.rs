use crate::analyzer::interpreter::{Interpreter, ValueResult};
use crate::converter::parser::{Parser, ParserResult};
use crate::tokenizer::kind::*;
use crate::transformer::transpiler::{TKind, Transpiler};
use crate::value::instruction::block::Block;
use crate::value::instruction::expression::Expression;
use crate::value::instruction::IKind;
use crate::value::{VKind, Value};
use crate::void;

use std::fmt;

#[derive(Clone, Debug)]
pub struct If {
  pub condition: Box<dyn Value>,
  pub consequence: Box<dyn Value>,
  pub alternative: Option<Box<dyn Value>>,
}

impl Default for If {
  fn default() -> Self {
    If::new(void!(), void!(), None)
  }
}

impl fmt::Display for If {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Value for If {
  fn eval(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> ValueResult<Box<dyn Value>> {
    let condition = self.condition.eval(interpreter)?;

    if condition.is_truthy() {
      self.consequence.eval(interpreter)
    } else {
      self
        .alternative
        .as_ref()
        .unwrap()
        .to_owned()
        .eval(interpreter)
    }
  }

  fn vkind(&self) -> VKind {
    VKind::Expression
  }

  fn ikind(&self) -> IKind {
    IKind::If
  }

  fn parse(&mut self, parser: &mut Parser) -> ParserResult<Box<dyn Value>> {
    parser.next_token();

    let condition = Expression::parse_expression_by_precedence(
      parser,
      &mut PrecedenceKind::Lowest,
    )?;

    parser.expect_first(TokenKind::GroupStart(Brace))?;

    let consequence = Block::default().parse(parser)?;

    let alternative = if parser.first_is(TokenKind::Keyword(Else)) {
      parser.next_token();
      parser.expect_first(TokenKind::GroupStart(Brace))?;

      Some(Block::default().parse(parser)?)
    } else {
      None
    };

    Ok(
      self
        .add_condition(condition)
        .add_alternative(alternative)
        .add_consequence(consequence)
        .boxed(),
    )
  }

  fn print(&self) {
    println!("{}", self.text())
  }

  fn text(&self) -> String {
    let mut content = format!(
      "if {} {{ {} }}",
      self.condition.text(),
      self.consequence.text(),
    );

    if let Some(alt) = &self.alternative {
      content += format!(" else {}", alt.text()).as_str();
    }

    content
  }

  fn transpile(&mut self, transpiler: &mut Transpiler) -> String {
    let condition = &self.condition.transpile(transpiler);
    let consequence = &self.consequence.transpile(transpiler);

    let alternative = &self
      .alternative
      .as_ref()
      .unwrap()
      .to_owned()
      .transpile(transpiler);

    match transpiler.mode() {
      TKind::Json => format!(
        r#"{{
          "alternate": {},
          "consequent": {},
          "test": {},
          "type": "IfStatement"
        }}"#,
        alternative, consequence, condition
      ),
      TKind::Inline => {
        let mut content = format!("if ({}) {{ {} }}", condition, consequence);

        if let Some(alt) = &self.alternative {
          content +=
            format!(" else {{ {} }}", alt.to_owned().transpile(transpiler),)
              .as_str();
        }

        content
      }
      _ => self.text(),
    }
  }
}

impl If {
  pub fn new(
    condition: Box<dyn Value>,
    consequence: Box<dyn Value>,
    alternative: Option<Box<dyn Value>>,
  ) -> Self {
    If {
      alternative,
      condition,
      consequence,
    }
  }

  pub fn add_alternative(
    &mut self,
    alternative: Option<Box<dyn Value>>,
  ) -> &mut Self {
    self.alternative = alternative;
    self
  }

  pub fn add_condition(&mut self, condition: Box<dyn Value>) -> &mut Self {
    self.condition = condition;
    self
  }

  pub fn add_consequence(&mut self, consequence: Box<dyn Value>) -> &mut Self {
    self.consequence = consequence;
    self
  }

  pub fn boxed(&self) -> Box<dyn Value> {
    Box::new(self.to_owned())
  }
}

use crate::tokenizer::kind::*;
use crate::tokenizer::scanner::Scanner;
use crate::tokenizer::token::Token;
use crate::tokenizer::tokenify;
use crate::value::instruction::expression::Expression;
use crate::value::instruction::program::Program;
use crate::value::Value;

use std::mem;

pub type ParserError = String;
pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
  pub errors: Vec<Result<(), String>>,
  pub first: Box<Token>,
  pub scanner: Scanner<'a>,
  pub token: Box<Token>,
  pub tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self {
    Parser {
      errors: vec![],
      first: Token::default().boxed(),
      scanner: Scanner::new(input),
      token: Token::default().boxed(),
      tokens: tokenify(&input).collect(),
    }
  }

  pub fn current_precedence(&self) -> PrecedenceKind {
    TokenKind::precedence(self.token.kind)
  }

  pub fn expect_first(&mut self, kind: TokenKind) -> ParserResult<()> {
    if self.first_is(kind) {
      return Ok(self.next_token());
    }

    Err(format!(
      "token {:?} expected, but the current token is {:?}!",
      kind, self.first.kind
    ))
  }

  pub fn first_is(&self, kind: TokenKind) -> bool {
    self.first.kind == kind
  }

  pub fn next_token(&mut self) {
    self.token =
      mem::replace(&mut self.first, Box::new(self.scanner.advance_token()));
  }

  pub fn parse(&mut self) -> ParserResult<Box<dyn Value>> {
    self.next_token();
    self.next_token();

    Ok(Program::default().parse(self)?)
  }

  pub fn parse_until(
    &mut self,
    kind: TokenKind,
  ) -> ParserResult<Vec<Box<dyn Value>>> {
    let mut expressions: Vec<Box<dyn Value>> = vec![];

    if self.first_is(kind) {
      self.next_token();

      return Ok(expressions);
    }

    self.next_token();

    expressions.push(Expression::parse_expression_by_precedence(
      self,
      &mut PrecedenceKind::Lowest,
    )?);

    while self.first_is(TokenKind::Symbol(Comma)) {
      self.next_token();
      self.next_token();

      expressions.push(Expression::parse_expression_by_precedence(
        self,
        &mut PrecedenceKind::Lowest,
      )?);
    }

    self.expect_first(kind)?;

    Ok(expressions)
  }

  pub fn should_precedence_has_priority(
    &self,
    kind: &mut PrecedenceKind,
  ) -> bool {
    kind < &mut TokenKind::precedence(self.first.kind)
  }

  pub fn token_is(&self, kind: TokenKind) -> bool {
    self.token.kind == kind
  }
}

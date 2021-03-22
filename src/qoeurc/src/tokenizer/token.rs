use crate::tokenizer::kind::TokenKind;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub literal: String,
  pub len: usize,
}

impl Default for Token {
  fn default() -> Self {
    Token::new(TokenKind::EOF, "", 0)
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl PartialEq<TokenKind> for Token {
  fn eq(&self, rhs: &TokenKind) -> bool {
    self.kind == *rhs
  }
}

impl Token {
  pub fn new(kind: TokenKind, literal: &str, len: usize) -> Self {
    Token {
      kind,
      len,
      literal: literal.into(),
    }
  }

  pub fn boxed(&self) -> Box<Token> {
    Box::new(self.to_owned())
  }

  pub fn ikind(&self) -> String {
    match &self.kind {
      TokenKind::EOF => format!("EOF"),
      TokenKind::Unknown => format!("{:?}", self.kind),
      TokenKind::NewLine => format!("{:?}", self.kind),
      TokenKind::Comment(_) => format!("{:?}", self.kind),
      TokenKind::Identifier => format!("{:?}", self.kind),
      TokenKind::Keyword(_) => format!("{:?}", self.kind),
      TokenKind::GroupStart(_) => format!("{:?}", self.kind),
      TokenKind::GroupEnd(_) => format!("{:?}", self.kind),
      TokenKind::Literal(_) => format!("{:?}", self.kind),
      TokenKind::Operator(_) => format!("{:?}", self.kind),
      TokenKind::Symbol(_) => format!("{:?}", self.kind),
      TokenKind::Whitespace => format!("{:?}", self.kind),
    }
  }

  pub fn text(&self) -> String {
    format!("{}", self.literal)
  }
}

#[derive(Clone, Debug)]
pub struct TokenStream {
  pub token: Option<Token>,
  pub tokens: Vec<Token>,
}

impl Default for TokenStream {
  fn default() -> Self {
    TokenStream::new(Token::default(), vec![])
  }
}

impl TokenStream {
  pub fn new(token: Token, tokens: Vec<Token>) -> Self {
    TokenStream {
      token: Some(token),
      tokens,
    }
  }

  pub fn add_token(&mut self, token: Token) -> &mut Self {
    self.token = Some(token);
    self
  }

  pub fn push_token(&mut self, token: Token) -> &mut Self {
    self.tokens.push(token);
    self
  }

  pub fn token(&self) -> Token {
    self.token.as_ref().unwrap().to_owned()
  }
}

use crate::reporter::location::Location;
use crate::tokenizer::ascii::*;
use crate::tokenizer::kind;
use crate::tokenizer::token::{Token, TokenStream};

use std::str::Chars;

pub static EOF_CHAR: char = '\0';

#[derive(Clone, Debug)]
pub struct Scanner<'a> {
  pub input: &'a str,
  initial_len: usize,
  chars: Chars<'a>,
  location: Location,
  position: usize,
  prev: char,
  read_position: usize,
}

impl<'a> Scanner<'a> {
  pub fn new(input: &'a str) -> Scanner<'a> {
    let mut scanner = Scanner {
      chars: input.chars(),
      initial_len: input.len(),
      input: input.into(),
      location: Location::new(0, 0),
      position: 0,
      prev: EOF_CHAR,
      read_position: 0,
    };

    scanner.bump();
    scanner
  }

  pub fn advance_token(&mut self) -> Token {
    self.move_until_whitespace();

    let mut token_stream = TokenStream::default();

    match Ascii::to_kind(self.prev()) {
      AsciiKind::EOF => token_stream.add_token(self.scan_eof()),
      AsciiKind::Comment => token_stream.add_token(self.scan_comment()),
      AsciiKind::Group => token_stream.add_token(self.scan_group()),
      AsciiKind::Identifier => token_stream.add_token(self.scan_identifier()),
      AsciiKind::Number => token_stream.add_token(self.scan_number()),
      AsciiKind::Operator => token_stream.add_token(self.scan_operator()),
      AsciiKind::Quote => token_stream.add_token(self.scan_quote()),
      AsciiKind::Symbol => token_stream.add_token(self.scan_symbol()),
      AsciiKind::Unknown(_) => token_stream.add_token(self.scan_unknown()),
    };

    self.bump();

    token_stream.token()
  }

  pub fn input(&self) -> &str {
    &self.input
  }

  pub fn prev(&self) -> char {
    self.prev
  }

  pub fn first(&self) -> char {
    self.nth_char(0)
  }

  pub fn second(&self) -> char {
    self.nth_char(1)
  }

  pub fn location(&self) -> Location {
    self.location
  }

  pub fn is_eof(&self) -> bool {
    self.chars.as_str().is_empty()
  }

  pub fn len_consumed(&self) -> usize {
    self.initial_len - self.chars.as_str().len()
  }

  pub fn bump(&mut self) -> Option<char> {
    self.prev = self.chars.next()?;
    self.position = self.read_position;
    self.location.column = self.read_position;
    self.read_position += self.prev.len_utf8();

    Some(self.prev)
  }

  pub fn is(&self, c: char) -> bool {
    self.prev == c
  }

  fn chars(&self) -> Chars<'a> {
    self.chars.clone()
  }

  fn nth_char(&self, n: usize) -> char {
    self.chars().nth(n).unwrap_or(EOF_CHAR)
  }

  fn move_until_whitespace(&mut self) {
    while Ascii::is_whitespace(self.prev()) && !self.is_eof() {
      self.bump();

      // TODO
      match self.prev() {
        '\\' => {
          self.bump();
          self.bump();
          self.move_until_whitespace();
        }
        _ => (),
      };
    }
  }

  fn number(&mut self) -> (kind::LiteralKind, bool, String) {
    let mut base = kind::Int;
    let literal: String;
    let position = self.position;

    if self.prev() == '0' {
      let has_digits = match self.first() {
        'b' => {
          base = kind::Binary;
          self.bump();
          self.scan_decimal_digits()
        }
        '0'..='9' | '_' | '.' | 'e' | 'E' => {
          self.scan_decimal_digits();
          true
        }
        _ => {
          literal = self.input[position..self.len_consumed()].into();
          return (base, false, literal);
        }
      };

      if !has_digits {
        literal = self.input[position..self.len_consumed()].into();
        return (base, true, literal);
      }
    } else {
      self.scan_decimal_digits();
    };

    match self.first() {
      '.' if self.second() != '.' && !Ascii::is_id_start(self.second()) => {
        self.bump();

        let mut empty_exponent = false;

        if self.first().is_digit(10) {
          self.scan_decimal_digits();

          match self.first() {
            'e' | 'E' => {
              self.bump();
              empty_exponent = !self.scan_float_exponent();
            }
            _ => (),
          }
        }

        base = kind::Float;
        literal = self.input[position..self.len_consumed()].into();

        (base, empty_exponent, literal)
      }
      'e' | 'E' => {
        self.bump();

        let empty_exponent = !self.scan_float_exponent();
        base = kind::Float;
        literal = self.input[position..self.len_consumed()].into();

        (base, empty_exponent, literal)
      }
      _ => {
        literal = self.input[position..self.len_consumed()].into();

        (base, false, literal)
      }
    }
  }

  fn scan_comment(&mut self) -> Token {
    match self.prev() {
      '#' => match self.first() {
        '-' => self.scan_comment_block(),
        '#' => self.scan_comment_doc(),
        '!' => self.scan_shebang(),
        _ => self.scan_comment_line(),
      },
      _ => self.scan_unknown(),
    }
  }

  fn scan_comment_block(&mut self) -> Token {
    self.bump();
    self.bump();

    let mut level: isize = 1;
    let mut literal: Vec<String> = vec![];

    literal.push("#-".into());

    while level > 0 {
      if self.is_eof() {
        println!("{}", format!("unterminated block comment"));
        break;
      }

      let c = self.prev();

      match c {
        '#' if self.first() == '-' => {
          level += 1;
          self.bump();
        }
        '-' if self.first() == '#' => {
          level -= 1;
          self.bump();
        }
        '\r' => (),
        _ => literal.push(self.prev().to_string()),
      }

      self.bump();
    }

    Token::new(
      kind::Comment(kind::Block),
      &literal.join(""),
      self.len_consumed(),
    )
  }

  fn scan_comment_doc(&mut self) -> Token {
    self.bump();
    self.bump();

    let doc_comment: Vec<String> = vec![];

    Token::new(
      kind::Comment(kind::Doc),
      &doc_comment.join(""),
      self.len_consumed(),
    )
  }

  fn scan_comment_line(&mut self) -> Token {
    self.bump();

    let mut literal = vec![];

    while !Ascii::is_end_of_line(self.prev()) && !self.is_eof() {
      match self.prev() {
        a if Ascii::is_end_of_file(a) => break,
        a if Ascii::is_carriage_return(a) => {
          if Ascii::is_end_of_file(self.first()) {
            /* CRLF */
            break;
          }
        }
        _ => literal.push(self.prev().to_string()),
      }
      self.bump();
    }

    // get the last character when end of file is reached
    literal.push(self.prev().to_string());

    Token::new(
      kind::Comment(kind::Line),
      &literal.join("").trim().to_string(),
      self.len_consumed(),
    )
  }

  fn scan_decimal_digits(&mut self) -> bool {
    let mut has_digits = false;

    loop {
      match self.first() {
        '_' => {
          self.bump();
        }
        '0'..='9' => {
          has_digits = true;
          self.bump();
        }
        _ => break,
      }
    }

    has_digits
  }

  fn scan_eof(&mut self) -> Token {
    Token::new(kind::EOF, &"EOF", self.len_consumed())
  }

  fn scan_float_exponent(&mut self) -> bool {
    if self.first() == '-' || self.first() == '+' {
      self.bump();
    }

    self.scan_decimal_digits()
  }

  fn scan_identifier(&mut self) -> Token {
    let position = self.position;

    self.scan_until(Ascii::is_id_continue);

    let len = self.len_consumed();
    let literal = &self.input[position..len].to_string();

    Token::new(kind::literal::Literal::keyword(&literal), &literal, len)
  }

  fn scan_group(&mut self) -> Token {
    let len = self.len_consumed();
    let literal = self.prev.to_string();

    match self.prev() {
      '(' => Token::new(kind::GroupStart(kind::Parenthesis), &literal, len),
      ')' => Token::new(kind::GroupEnd(kind::Parenthesis), &literal, len),
      '{' => Token::new(kind::GroupStart(kind::Brace), &literal, len),
      '}' => Token::new(kind::GroupEnd(kind::Brace), &literal, len),
      '[' => Token::new(kind::GroupStart(kind::Bracket), &literal, len),
      ']' => Token::new(kind::GroupEnd(kind::Bracket), &literal, len),
      _ => self.scan_unknown(),
    }
  }

  fn scan_unknown(&mut self) -> Token {
    Token::new(kind::Unknown, &self.prev.to_string(), self.len_consumed())
  }

  fn scan_until<F>(&mut self, mut predicate: F) -> usize
  where
    F: FnMut(char) -> bool,
  {
    let mut eaten: usize = 0;

    while predicate(self.first()) && !self.is_eof() {
      eaten += 1;
      self.bump();
    }

    eaten
  }

  fn scan_number(&mut self) -> Token {
    let (knd, _is_empty, lit) = self.number();
    let len = self.len_consumed();

    Token::new(kind::Literal(knd), &lit, len)
  }

  fn scan_operator(&mut self) -> Token {
    match self.prev() {
      '+' => Token::new(
        kind::Operator(kind::Plus),
        &self.prev.to_string(),
        self.len_consumed(),
      ),
      '-' => match self.first() {
        '>' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(kind::Symbol(kind::Arrow), &literal, self.len_consumed())
        }
        _ => Token::new(
          kind::Operator(kind::Minus),
          &self.prev.to_string(),
          self.len_consumed(),
        ),
      },
      '*' => Token::new(
        kind::Operator(kind::Star),
        &self.prev.to_string(),
        self.len_consumed(),
      ),
      '/' => Token::new(
        kind::Operator(kind::Slash),
        &self.prev.to_string(),
        self.len_consumed(),
      ),
      '=' => match self.first() {
        '=' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(kind::Operator(kind::Equal), &literal, self.len_consumed())
        }
        _ => Token::new(
          kind::Operator(kind::Assign),
          &self.prev.to_string(),
          self.len_consumed(),
        ),
      },
      '<' => match self.first() {
        '<' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(
            kind::Operator(kind::ShiftLeft),
            &literal,
            self.len_consumed(),
          )
        }
        '=' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(
            kind::Operator(kind::LessThanOrEqual),
            &literal,
            self.len_consumed(),
          )
        }
        _ => Token::new(
          kind::Operator(kind::LessThan),
          &self.prev.to_string(),
          self.len_consumed(),
        ),
      },
      '>' => match self.first() {
        '>' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(
            kind::Operator(kind::ShiftRight),
            &literal,
            self.len_consumed(),
          )
        }
        '=' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(
            kind::Operator(kind::GreaterThanOrEqual),
            &literal,
            self.len_consumed(),
          )
        }
        _ => Token::new(
          kind::Operator(kind::GreaterThan),
          &self.prev.to_string(),
          self.len_consumed(),
        ),
      },
      '^' => Token::new(
        kind::Operator(kind::GreaterThan),
        &self.prev.to_string(),
        self.len_consumed(),
      ),
      '|' => match self.first() {
        '|' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(kind::Operator(kind::OrOr), &literal, self.len_consumed())
        }
        '>' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(
            kind::Symbol(kind::Attribute),
            &literal,
            self.len_consumed(),
          )
        }
        _ => Token::new(
          kind::Operator(kind::Or),
          &self.prev.to_string(),
          self.len_consumed(),
        ),
      },
      '&' => match self.first() {
        '&' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(
            kind::Operator(kind::AndAnd),
            &literal,
            self.len_consumed(),
          )
        }
        _ => Token::new(
          kind::Operator(kind::And),
          &self.prev.to_string(),
          self.len_consumed(),
        ),
      },
      '%' => Token::new(
        kind::Operator(kind::Percent),
        &self.prev.to_string(),
        self.len_consumed(),
      ),
      _ => self.scan_unknown(),
    }
  }

  fn scan_quote(&mut self) -> Token {
    match self.prev() {
      a if Ascii::is_double_quote(a) => self.scan_string(),
      _ => {
        Token::new(kind::Unknown, &self.prev().to_string(), self.len_consumed())
      }
    }
  }

  fn scan_shebang(&mut self) -> Token {
    self.bump();
    self.bump();

    let mut block: Vec<String> = vec![];

    block.push("#!".into());

    while !Ascii::is_end_of_line(self.prev()) && !self.is_eof() {
      block.push(self.prev().to_string());
      self.bump();
    }

    if Ascii::is_end_of_line(self.prev()) {
      self.bump();
    }

    // get the last character when the end of line is reached
    block.push(self.prev().to_string());

    Token::new(
      kind::Symbol(kind::Shebang),
      &block.join(""),
      self.len_consumed(),
    )
  }

  fn scan_string(&mut self) -> Token {
    self.bump();

    let position = self.position;

    while !self.is('"') {
      self.bump();
    }

    let len = self.len_consumed();
    let literal = format!("{}", &self.input[position..len - 1]);

    Token::new(kind::Literal(kind::Str), &literal, len)
  }

  fn scan_symbol(&mut self) -> Token {
    match self.prev() {
      ':' => match self.first() {
        '=' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(
            kind::Operator(kind::AssignType),
            &literal,
            self.len_consumed(),
          )
        }
        ':' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(
            kind::Symbol(kind::ColonColon),
            &literal,
            self.len_consumed(),
          )
        }
        _ => Token::new(
          kind::Symbol(kind::Colon),
          &self.prev.to_string(),
          self.len_consumed(),
        ),
      },
      ',' => Token::new(
        kind::Symbol(kind::Comma),
        &self.prev.to_string(),
        self.len_consumed(),
      ),
      ';' => Token::new(
        kind::Symbol(kind::Semicolon),
        &self.prev.to_string(),
        self.len_consumed(),
      ),
      '$' => Token::new(
        kind::Symbol(kind::Dollar),
        &self.prev.to_string(),
        self.len_consumed(),
      ),
      '.' => match self.first() {
        '.' => {
          let prev = self.prev();
          let literal = format!("{}{}", prev, self.first());

          self.bump();

          Token::new(kind::Operator(kind::Range), &literal, self.len_consumed())
        }
        _ => Token::new(
          kind::Symbol(kind::Dot),
          &self.prev.to_string(),
          self.len_consumed(),
        ),
      },
      '!' => Token::new(
        kind::Symbol(kind::Bang),
        &self.prev.to_string(),
        self.len_consumed(),
      ),
      '?' => Token::new(
        kind::Symbol(kind::Question),
        &self.prev.to_string(),
        self.len_consumed(),
      ),
      '@' => Token::new(
        kind::Symbol(kind::At),
        &self.prev.to_string(),
        self.len_consumed(),
      ),
      _ => self.scan_unknown(),
    }
  }
}

use crate::utils::ascii::{
  is_close_brace, is_colon, is_double_quote, is_equals, is_identifier,
  is_left_angle_bracket, is_open_brace, is_right_angle_bracket, is_slash,
  is_whitespace,
};

use crate::console_log;
use crate::value::attr::{AKind, Attr, PropValue};
use crate::value::capsule::Capsule;
use crate::value::program::Program;
use crate::value::tag::{KwKind::*, Tag, TagKind};
use crate::value::{HtmlValue, HtmlValueResult, VBehavior, VKind};

use qoeurc::void;

use std::str::Chars;

use rsass::compile_scss;
use wasm_bindgen::JsValue;

pub static EOF_CHAR: char = '\0';

pub type ParserError = String;
pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Clone, Debug)]
pub struct Parser<'a> {
  chars: Chars<'a>,
  position: usize,
  initial_len: usize,
  input: String,
  prev: char,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self {
    Parser {
      chars: input.chars(),
      position: 0,
      initial_len: input.len(),
      input: input.into(),
      prev: EOF_CHAR,
    }
  }

  pub fn prev(&self) -> char {
    self.prev
  }

  pub fn len_consumed(&self) -> usize {
    self.initial_len - self.chars.as_str().len()
  }

  pub fn bump(&mut self) -> char {
    self.prev = self.chars.next().unwrap_or(EOF_CHAR);
    self.position += self.prev.len_utf8();

    self.prev
  }

  pub fn is(&self, c: char) -> bool {
    self.prev == c
  }

  pub fn is_eof(&self) -> bool {
    self.chars.as_str().is_empty()
  }

  fn chars(&self) -> Chars<'a> {
    self.chars.to_owned()
  }

  pub fn first(&self) -> char {
    self.nth_char(0)
  }

  pub fn second(&self) -> char {
    self.nth_char(1)
  }

  pub fn third(&self) -> char {
    self.nth_char(2)
  }

  fn nth_char(&self, n: usize) -> char {
    self.chars().nth(n).unwrap_or(EOF_CHAR)
  }

  pub fn parse(&mut self) -> HtmlValueResult<Box<dyn HtmlValue>> {
    let nodes = match Program::default().advance_capsule(&self.input)?.vkind() {
      // parse_program
      VKind::Program(ref capsules) => capsules
        .iter()
        .map(|capsule| capsule.to_owned().parse(self))
        .collect::<HtmlValueResult<Vec<Box<dyn HtmlValue>>>>()?,
      _ => vec![],
    };

    Ok(Program::new(nodes).boxed())
  }

  fn parse_attr(&mut self) -> (String, std::option::Option<String>) {
    let name;
    let mut value = None;
    let c = self.first();

    match c {
      '{' => {
        self.bump();
        name = self.scan_until(|c| !is_close_brace(c));
        value = Some(name.to_owned());
        assert!(is_close_brace(self.bump()));
      }
      _ => {
        name = self.scan_ident();
        assert!(is_equals(self.bump()));
        value = Some(self.parse_attr_value());
      }
    };

    (name, value)
  }

  fn parse_attr_value(&mut self) -> String {
    let value;
    let c = self.bump();

    match c {
      '{' => {
        assert!(is_open_brace(c));
        value = self.scan_until(|c| !is_close_brace(c));
        assert!(is_close_brace(self.bump()));
      }
      _ => {
        assert!(is_double_quote(c));
        value = self.scan_until(|c| !is_double_quote(c));
        assert!(is_double_quote(self.bump()));
      }
    };

    value
  }

  fn parse_attrs(&mut self) -> Vec<Box<dyn HtmlValue>> {
    let mut attrs: Vec<Box<dyn HtmlValue>> = vec![];

    loop {
      self.scan_whitespace();

      match self.first() {
        c if is_right_angle_bracket(c) || is_slash(c) => {
          break;
        }
        _ => {
          let (name, value) = self.parse_attr();
          let prop_value = value.to_owned().unwrap();

          let attr = match &value {
            None => AKind::Key(JsValue::from_str(&name.to_string())),
            Some(v) => AKind::Property(
              JsValue::from_str(&name.to_string()),
              PropValue::String(JsValue::from_str(&prop_value.to_string())),
            ),
          };

          attrs.push(
            Attr::new(
              JsValue::from_str(&name),
              JsValue::from_str(&prop_value.to_string()),
              VKind::Attr(attr),
            )
            .boxed(),
          );
        }
      };
    }

    attrs
  }

  fn parse_element(&mut self) -> Box<dyn HtmlValue> {
    let tag_void = [
      "area".into(),
      "base".into(),
      "br".into(),
      "col".into(),
      "embed".into(),
      "hr".into(),
      "img".into(),
      "input".into(),
      "link".into(),
      "meta".into(),
      "param".into(),
      "source".into(),
      "track".into(),
      "wbr".into(),
    ];

    assert!(is_left_angle_bracket(self.bump()));

    let tag_name = self.scan_ident();
    let attrs = self.parse_attrs();

    if tag_void.contains(&tag_name) || tag_name.contains("qompo") {
      self.scan_until(|c| !is_right_angle_bracket(c));
      self.bump();

      return Tag::new(TagKind::tag(&tag_name), attrs, vec![]).boxed();
    }

    assert!(is_right_angle_bracket(self.bump()));

    let mut children = self.parse_vdom().unwrap();

    assert!(is_left_angle_bracket(self.bump()));
    assert!(is_slash(self.bump()));
    assert_eq!(self.scan_ident(), tag_name);
    assert!(is_right_angle_bracket(self.bump()));

    Tag::new(TagKind::tag(&tag_name), attrs, children).boxed()
  }

  fn parse_node(&mut self) -> Box<dyn HtmlValue> {
    match self.first() {
      c if is_left_angle_bracket(c) => self.parse_element(),
      _ => self.parse_text(),
    }
  }

  fn parse_text(&mut self) -> Box<dyn HtmlValue> {
    let tag_name = &self.scan_until(|c| !is_left_angle_bracket(c));

    Tag::new(TagKind::tag(&tag_name), vec![], vec![]).boxed()
  }

  pub fn parse_vdom(&mut self) -> HtmlValueResult<Vec<Box<dyn HtmlValue>>> {
    let mut nodes = vec![];

    loop {
      self.scan_whitespace();

      if self.is_eof() || self.starts_with("</") {
        break;
      }

      nodes.push(self.parse_node());
    }

    Ok(nodes)
  }

  fn scan_ident(&mut self) -> String {
    self.scan_until(|c| is_identifier(c) || is_colon(c) || c.is_digit(10))
  }

  fn scan_until<F>(&mut self, check: F) -> String
  where
    F: Fn(char) -> bool,
  {
    let mut value = String::new();

    while !self.is_eof() && check(self.first()) {
      value.push(self.bump());
    }

    value
  }

  fn scan_whitespace(&mut self) {
    self.scan_until(is_whitespace);
  }

  fn starts_with(&self, s: &str) -> bool {
    self.input[self.len_consumed()..].starts_with(s)
  }
}

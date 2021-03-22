pub use crate::value::tag::KwKind::*;

use crate::compiler::parser::Parser;
use crate::compiler::transpiler::Transpiler;
use crate::value::attr::{AKind, Attr, PropValue};
use crate::value::tag::KwKind::*;
use crate::value::{HtmlValue, HtmlValueResult, VBehavior, VKind};
use qoeurc::analyzer::interpreter::Interpreter;

use std::cmp::PartialEq;
use std::fmt::{self, Debug};
use std::option;

use crate::console_log;

use rsass::compile_scss;

use wasm_bindgen::JsValue;

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
  pub kind: TagKind,
  pub attrs: Vec<Box<dyn HtmlValue>>,
  pub children: Vec<Box<dyn HtmlValue>>,
}

impl HtmlValue for Tag {
  fn eval(
    &mut self,
    _interpreter: &mut Interpreter,
  ) -> HtmlValueResult<Box<dyn HtmlValue>> {
    Ok(self.boxed())
  }

  fn parse(
    &mut self,
    parser: &mut Parser,
  ) -> HtmlValueResult<Box<dyn HtmlValue>> {
    Ok(self.boxed())
  }

  fn text(&self) -> String {
    match self.vkind() {
      VKind::Tag(ref kind, ref attrs, ref children) => {
        let kw_s = kind.to_string();

        if children.is_empty() {
          return format!("<{} />", kw_s);
        }

        let a = attrs
          .iter()
          .map(|child| child.text())
          .collect::<Vec<_>>()
          .join(" ");

        let c = children
          .iter()
          .map(|child| child.text())
          .collect::<Vec<_>>()
          .join("\n");

        format!("<{} {}>\n{}\n</{}>", kw_s, a, c, kw_s)
      }
      _ => {
        format!("{}", "okok")
      }
    }
  }

  fn transpile(
    &mut self,
    transpiler: &mut Transpiler,
  ) -> HtmlValueResult<Box<dyn HtmlValue>> {
    let mut children = vec![];

    match self.vkind() {
      VKind::Script(ref source) => {
        let s = qoeurc::transformer::transformify(
          &source.as_string().unwrap(),
          "inline",
          transpiler.interpreter(),
        )
        .unwrap();
        children.push(Tag::new(TagKind::Text(s), vec![], vec![]).boxed())
      }
      VKind::Tag(ref kind, ref attr, ref child) => children.push(
        Tag::new(kind.to_owned(), attr.to_owned(), child.to_owned()).boxed(),
      ),
      VKind::Ui(ref source) => {
        let ui_source_preprocessed = compile_scss(
          &source.as_string().unwrap().as_bytes(),
          Default::default(),
        )
        .and_then(|s| Ok(String::from_utf8(s)?))
        .map_err(|e| {
          eprintln!("{}", e);
          "rsass failed"
        })
        .unwrap();
        console_log!("{:?}", ui_source_preprocessed);

        children.push(
          Tag::new(TagKind::Text(ui_source_preprocessed), vec![], vec![])
            .boxed(),
        )
      }
      VKind::Text(ref value) => children.push(
        Tag::new(TagKind::Text(value.as_string().unwrap()), vec![], vec![])
          .boxed(),
      ),
      _ => {}
    };

    Ok(self.add_children(children).boxed())
  }

  fn vkind(&self) -> VKind {
    match self.kind {
      TagKind::Kw(ref value) => VKind::Tag(
        self.kind.to_owned(),
        self.attrs.to_owned(),
        self.children.to_owned(),
      ),
      TagKind::Text(ref value) | TagKind::Ui(ref value) => {
        VKind::Text(value.into())
      }
      TagKind::Script(ref value) => VKind::Script(value.to_owned()),
    }
  }
}

impl Tag {
  pub fn new(
    kind: TagKind,
    attrs: Vec<Box<dyn HtmlValue>>,
    children: Vec<Box<dyn HtmlValue>>,
  ) -> Self {
    Tag {
      kind,
      attrs,
      children,
    }
  }

  pub fn add_children(
    &mut self,
    children: Vec<Box<dyn HtmlValue>>,
  ) -> &mut Self {
    self.children = children;
    self
  }

  pub fn key(&self) -> option::Option<JsValue> {
    for attr in &self.attrs {
      if let VKind::Attr(AKind::Key(key)) = attr.vkind() {
        return Some(key);
      }
    }

    None
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TagKind {
  Kw(KwKind),
  Text(String),
  Ui(String),
  Script(JsValue),
}

impl fmt::Display for TagKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TagKind::Kw(value) => write!(f, "{}", value),
      TagKind::Text(value) | TagKind::Ui(value) => write!(f, "{}", value),
      TagKind::Script(value) => write!(f, "{:?}", value),
    }
  }
}

impl TagKind {
  pub fn tag(name: &str) -> TagKind {
    match name {
      "a" => TagKind::Kw(A),
      "abbr" => TagKind::Kw(Abbr),
      "address" => TagKind::Kw(Address),
      "area" => TagKind::Kw(Area),
      "article" => TagKind::Kw(Article),
      "aside" => TagKind::Kw(Aside),
      "audio" => TagKind::Kw(Audio),
      "b" => TagKind::Kw(B),
      "base" => TagKind::Kw(Base),
      "bdi" => TagKind::Kw(Bdi),
      "bdo" => TagKind::Kw(Bdo),
      "blockquote" => TagKind::Kw(Blockquote),
      "body" => TagKind::Kw(Body),
      "br" => TagKind::Kw(Br),
      "button" => TagKind::Kw(Button),
      "canvas" => TagKind::Kw(Canvas),
      "caption" => TagKind::Kw(Caption),
      "cite" => TagKind::Kw(Cite),
      "code" => TagKind::Kw(Code),
      "col" => TagKind::Kw(Col),
      "colgroup" => TagKind::Kw(Colgroup),
      "data" => TagKind::Kw(Data),
      "dd" => TagKind::Kw(Dd),
      "del" => TagKind::Kw(Del),
      "details" => TagKind::Kw(Details),
      "dfn" => TagKind::Kw(Dfn),
      "dialog" => TagKind::Kw(Dialog),
      "div" => TagKind::Kw(Div),
      "dl" => TagKind::Kw(Dl),
      "dt" => TagKind::Kw(Dt),
      "em" => TagKind::Kw(Em),
      "embed" => TagKind::Kw(Embed),
      "fieldset" => TagKind::Kw(Fieldset),
      "figcaption" => TagKind::Kw(Figcaption),
      "figure" => TagKind::Kw(Figure),
      "footer" => TagKind::Kw(Footer),
      "form" => TagKind::Kw(Form),
      "h1" => TagKind::Kw(H1),
      "h2" => TagKind::Kw(H2),
      "h3" => TagKind::Kw(H3),
      "h4" => TagKind::Kw(H4),
      "h5" => TagKind::Kw(H5),
      "h6" => TagKind::Kw(H6),
      "head" => TagKind::Kw(Head),
      "header" => TagKind::Kw(Header),
      "hr" => TagKind::Kw(Hr),
      "html" => TagKind::Kw(Html),
      "i" => TagKind::Kw(I),
      "iframe" => TagKind::Kw(Iframe),
      "img" => TagKind::Kw(Img),
      "input" => TagKind::Kw(Input),
      "ins" => TagKind::Kw(Ins),
      "kdb" => TagKind::Kw(Kdb),
      "label" => TagKind::Kw(Label),
      "legend" => TagKind::Kw(Legend),
      "li" => TagKind::Kw(Li),
      "link" => TagKind::Kw(Link),
      "main" => TagKind::Kw(Main),
      "map" => TagKind::Kw(Map),
      "mark" => TagKind::Kw(Mark),
      "meta" => TagKind::Kw(Meta),
      "meter" => TagKind::Kw(Meter),
      "nav" => TagKind::Kw(Nav),
      "noscript" => TagKind::Kw(Noscript),
      "object" => TagKind::Kw(Object),
      "ol" => TagKind::Kw(Ol),
      "optgroup" => TagKind::Kw(Optgroup),
      "option" => TagKind::Kw(Option),
      "output" => TagKind::Kw(Output),
      "p" => TagKind::Kw(P),
      "param" => TagKind::Kw(Param),
      "picture" => TagKind::Kw(Picture),
      "pre" => TagKind::Kw(Pre),
      "progress" => TagKind::Kw(Progress),
      "q" => TagKind::Kw(Q),
      "rp" => TagKind::Kw(Rp),
      "rt" => TagKind::Kw(Rt),
      "ruby" => TagKind::Kw(Ruby),
      "s" => TagKind::Kw(S),
      "samp" => TagKind::Kw(Samp),
      "script" => TagKind::Kw(Script),
      "section" => TagKind::Kw(Section),
      "select" => TagKind::Kw(Select),
      "small" => TagKind::Kw(Small),
      "source" => TagKind::Kw(Source),
      "span" => TagKind::Kw(Span),
      "strong" => TagKind::Kw(Strong),
      "style" => TagKind::Kw(Style),
      "sub" => TagKind::Kw(Sub),
      "summary" => TagKind::Kw(Summary),
      "sup" => TagKind::Kw(Sup),
      "svg" => TagKind::Kw(Svg),
      "table" => TagKind::Kw(Table),
      "tbody" => TagKind::Kw(Tbody),
      "td" => TagKind::Kw(Td),
      "template" => TagKind::Kw(Template),
      "textarea" => TagKind::Kw(Textarea),
      "tfoot" => TagKind::Kw(Tfoot),
      "th" => TagKind::Kw(Th),
      "thead" => TagKind::Kw(Thead),
      "time" => TagKind::Kw(Time),
      "title" => TagKind::Kw(Title),
      "tr" => TagKind::Kw(Tr),
      "track" => TagKind::Kw(Track),
      "tt" => TagKind::Kw(Tt),
      "u" => TagKind::Kw(U),
      "ui" => TagKind::Kw(Style),
      "ul" => TagKind::Kw(Ul),
      "var" => TagKind::Kw(Var),
      "video" => TagKind::Kw(Video),
      "view" => TagKind::Kw(View),
      "wbr" => TagKind::Kw(Wbr),
      _ => TagKind::Text(name.into()),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum KwKind {
  A,
  Abbr,
  Address,
  Area,
  Article,
  Aside,
  Audio,
  B,
  Base,
  Bdi,
  Bdo,
  Blockquote,
  Body,
  Br,
  Button,
  Canvas,
  Caption,
  Cite,
  Code,
  Col,
  Colgroup,
  Data,
  Datalist,
  Dd,
  Del,
  Details,
  Dfn,
  Dialog,
  Div,
  Dl,
  Dt,
  Em,
  Embed,
  Fieldset,
  Figcaption,
  Figure,
  Footer,
  Form,
  H1,
  H2,
  H3,
  H4,
  H5,
  H6,
  Head,
  Header,
  Hr,
  Html,
  I,
  Iframe,
  Img,
  Input,
  Ins,
  Kdb,
  Label,
  Legend,
  Li,
  Link,
  Main,
  Map,
  Mark,
  Meta,
  Meter,
  Nav,
  Noscript,
  Object,
  Ol,
  Optgroup,
  Option,
  Output,
  P,
  Param,
  Picture,
  Pre,
  Progress,
  Q,
  Rp,
  Rt,
  Ruby,
  S,
  Samp,
  Script,
  Section,
  Select,
  Small,
  Source,
  Span,
  Strong,
  Style,
  Sub,
  Summary,
  Sup,
  Svg,
  Table,
  Tbody,
  Td,
  Template,
  Textarea,
  Tfoot,
  Th,
  Thead,
  Time,
  Title,
  Tr,
  Track,
  Tt,
  U,
  Ui,
  Ul,
  Var,
  Video,
  View,
  Wbr,
}

#[macro_export]
macro_rules! tags {
  { $type:tt { $($kind:ident: $value:expr,)* } } => {
    impl std::fmt::Display for $type {
      fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
          $($kind => write!(f, "{}", $value),)*
        }
      }
    }
  }
}

tags! {
  KwKind {
    A: "a",
    Abbr: "abbr",
    Address: "address",
    Area: "area",
    Article: "article",
    Aside: "aside",
    Audio: "audio",
    B: "b",
    Base: "base",
    Bdi: "bdi",
    Bdo: "bdo",
    Blockquote: "blockquote",
    Body: "body",
    Br: "br",
    Button: "button",
    Canvas: "canvas",
    Caption: "caption",
    Cite: "cite",
    Code: "code",
    Col: "col",
    Colgroup: "colgroup",
    Data: "data",
    Datalist: "datalist",
    Dd: "dd",
    Del: "del",
    Details: "details",
    Dfn: "dfn",
    Dialog: "dialog",
    Div: "div",
    Dl: "dl",
    Dt: "dt",
    Em: "em",
    Embed: "embed",
    Fieldset: "fieldset",
    Figcaption: "figcaption",
    Figure: "figure",
    Footer: "footer",
    Form: "form",
    H1: "h1",
    H2: "h2",
    H3: "h3",
    H4: "h4",
    H5: "h5",
    H6: "h6",
    Head: "head",
    Header: "header",
    Hr: "hr",
    Html: "html",
    I: "i",
    Iframe: "iframe",
    Img: "img",
    Input: "input",
    Ins: "ins",
    Kdb: "kdb",
    Label: "label",
    Legend: "legend",
    Li: "li",
    Link: "link",
    Main: "main",
    Map: "map",
    Mark: "mark",
    Meta: "meta",
    Meter: "meter",
    Nav: "nav",
    Noscript: "noscript",
    Object: "object",
    Ol: "ol",
    Optgroup: "optgroup",
    Option: "option",
    Output: "output",
    P: "p",
    Param: "param",
    Picture: "picture",
    Pre: "pre",
    Progress: "progress",
    Q: "q",
    Rp: "rp",
    Rt: "rt",
    Ruby: "ruby",
    S: "s",
    Samp: "samp",
    Script: "script",
    Section: "section",
    Select: "select",
    Small: "small",
    Source: "source",
    Span: "span",
    Strong: "strong",
    Style: "style",
    Sub: "sub",
    Summary: "summary",
    Sup: "sup",
    Svg: "svg",
    Table: "table",
    Tbody: "tbody",
    Td: "td",
    Template: "template",
    Textarea: "textarea",
    Tfoot: "tfoot",
    Th: "th",
    Thead: "thead",
    Time: "time",
    Title: "title",
    Tr: "tr",
    Track: "track",
    Tt: "tt",
    U: "u",
    Ui: "style",
    Ul: "ul",
    Var: "var",
    Video: "video",
    View: "view",
    Wbr: "wbr",
  }
}

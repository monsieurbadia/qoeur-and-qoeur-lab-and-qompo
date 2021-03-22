use crate::compiler::parser::Parser;
use crate::compiler::transpiler::Transpiler;
use crate::console_log;
use crate::value::attr::{AKind, Attr, PropValue};
use crate::value::capsule::Capsule;
use crate::value::tag::{KwKind::*, Tag, TagKind};
use crate::value::{HtmlValue, HtmlValueResult, VBehavior, VKind};
use qoeurc::analyzer::interpreter::Interpreter;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Program {
  pub capsules: Vec<Box<dyn HtmlValue>>,
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

impl HtmlValue for Program {
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
    let capsules = self
      .capsules
      .iter()
      .map(|node| node.to_owned().parse(parser))
      .collect::<HtmlValueResult<Vec<Box<dyn HtmlValue>>>>()?;

    Ok(self.add_capsules(capsules).boxed())
  }

  fn text(&self) -> String {
    let capsules = self
      .capsules
      .iter()
      .map(|n| n.text())
      .collect::<Vec<String>>()
      .join("");

    format!("{}", capsules)
  }

  fn transpile(
    &mut self,
    transpiler: &mut Transpiler,
  ) -> HtmlValueResult<Box<dyn HtmlValue>> {
    let capsules = self
      .capsules
      .iter()
      .map(|v| v.to_owned().transpile(transpiler))
      .collect::<HtmlValueResult<Vec<Box<dyn HtmlValue>>>>()?;

    Ok(self.add_capsules(capsules).boxed())
  }

  fn vkind(&self) -> VKind {
    VKind::Program(self.capsules.to_vec())
  }
}

impl Program {
  pub fn new(capsules: Vec<Box<dyn HtmlValue>>) -> Self {
    Program {
      capsules: capsules.to_owned(),
    }
  }

  pub fn add_capsules(
    &mut self,
    capsules: Vec<Box<dyn HtmlValue>>,
  ) -> &mut Self {
    self.capsules = capsules.to_owned();
    self
  }

  pub fn add_capsule(&mut self, capsule: Box<dyn HtmlValue>) -> &mut Self {
    self.capsules.push(capsule);
    self
  }

  // TODO: refactoring
  pub fn advance_capsule(
    &mut self,
    input: &str,
  ) -> HtmlValueResult<Box<dyn HtmlValue>> {
    // split_capsules
    let capsules: Vec<&str> = input.trim().split("use capsule ").collect();
    let capsules_filtered = capsules
      .iter()
      .filter(|t|
      // filter_by_capsule
      t.starts_with("script")
      || t.starts_with("ui")
      || t.starts_with("view"))
      .collect::<Vec<&&str>>();

    // abort_if_possible
    assert!(capsules_filtered.len() <= 3, "multiple capsule exceed");

    let script_ = &capsules_filtered.iter().find(|t| t.starts_with("script"));
    match script_ {
      None => {}
      Some(s) => {
        // advance_script_capsule
        let script = s.trim().split("script").collect::<Vec<&str>>().join("");
        let script_s =
          script.trim().trim_start_matches('{').trim_end_matches('}');

        self.add_capsule(
          Capsule::new(
            TagKind::tag(&"script"),
            vec![],
            vec![Tag::new(
              TagKind::Text(script_s.trim().into()),
              vec![],
              vec![],
            )
            .boxed()],
          )
          .boxed(),
        );
      }
    };

    let ui_ = &capsules_filtered.iter().find(|t| t.starts_with("ui"));
    match ui_ {
      None => {}
      Some(u) => {
        // advance_ui_capsule
        let ui = u.trim().split("ui").collect::<Vec<&str>>().join("");
        let ui_s = ui.trim().trim_start_matches('{').trim_end_matches('}');

        self.add_capsule(
          Capsule::new(
            TagKind::Ui("style".into()),
            vec![],
            vec![Tag::new(TagKind::Text(ui_s.trim().into()), vec![], vec![])
              .boxed()],
          )
          .boxed(),
        );
      }
    };

    let view_ = &capsules_filtered.iter().find(|t| t.starts_with("view"));
    match view_ {
      None => {}
      Some(v) => {
        // advance_view_capsule
        let view = v.trim().split("view").collect::<Vec<&str>>().join("");
        let view_s = view.trim().trim_start_matches('{').trim_end_matches('}');

        self.add_capsule(
          Capsule::new(
            TagKind::tag(&"view"),
            vec![],
            vec![
              Tag::new(TagKind::Text(view_s.trim().into()), vec![], vec![])
                .boxed(),
            ],
          )
          .boxed(),
        );
      }
    };

    Ok(self.boxed())
  }
}

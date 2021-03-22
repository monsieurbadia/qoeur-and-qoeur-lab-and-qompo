use crate::computer::render::Renderer;
use crate::console_log;
use crate::value::program::Program;
use crate::value::tag::Tag;
use crate::value::{HtmlValue, HtmlValueResult, VBehavior, VKind};

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use web_sys;

// VDom
#[derive(Clone, Debug)]
pub struct App {
  pub lhs_vdom: RefCell<Option<Vec<Box<dyn HtmlValue>>>>,
  pub rhs_vdom: Vec<Box<dyn HtmlValue>>,
}

impl App {
  pub fn new(rhs_vdom: Vec<Box<dyn HtmlValue>>) -> Self {
    App {
      lhs_vdom: RefCell::new(None),
      rhs_vdom,
    }
  }

  pub fn render(self: &mut Rc<Self>) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let performance = window
      .performance()
      .expect("should have performance on window");

    let start_time = performance.now();
    let rhs_vdom = &self.rhs_vdom;
    let mut renderer = Renderer::new(document, self.to_owned(), vec![]);

    if let Err(err) =
      renderer.render(self, &self.lhs_vdom.borrow().as_ref(), rhs_vdom)
    {
      console_log!("renderer program error: {:?}", err);
    }

    let end_time = performance.now();
    console_log!("rendering time: {} ms", end_time - start_time);

    self.lhs_vdom.replace(Some(rhs_vdom.to_owned()));
  }
}

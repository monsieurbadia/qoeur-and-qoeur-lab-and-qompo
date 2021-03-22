use crate::computer::app::App;
use crate::console_log;
use crate::value::attr::{AKind, Attr};
use crate::value::tag::{KwKind, Tag};
use crate::value::{HtmlValue, HtmlValueResult, VBehavior, VKind};

use std::rc::Rc;

use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{self, Document, HtmlElement, Node};

#[derive(Clone, Debug)]
pub struct Renderer {
  document: Document,
  app: Rc<App>,
  to_remove: Vec<(Node, Node)>,
}

impl Renderer {
  pub fn new(
    document: Document,
    app: Rc<App>,
    to_remove: Vec<(Node, Node)>,
  ) -> Self {
    Renderer {
      document,
      app,
      to_remove,
    }
  }

  pub fn create_node(
    &mut self,
    tag: &Box<dyn HtmlValue>,
  ) -> Result<Node, JsValue> {
    match tag.vkind() {
      VKind::Tag(ref kind, ref attrs, ref children) => {
        let name = &kind.to_string();
        let node: HtmlElement =
          self.document.create_element(name)?.dyn_into()?;

        self.scope_style_tag_if_possible(&node)?;

        // create_attrs
        for attr in attrs {
          match attr.vkind() {
            // create_prop_key
            VKind::Attr(AKind::Key(ref value)) => {
              node.set_attribute(&value.as_string().unwrap(), "")?;
            }
            // create_prop_attr
            VKind::Attr(AKind::Property(ref prop, ref value)) => {
              let mut propname = prop.into();

              if prop.as_string().unwrap().contains("class") {
                propname = "className".into();
              }

              Reflect::set(node.as_ref(), &propname, &value.text().into())?;
            }
            // create_prop_style
            VKind::Attr(AKind::Style(ref prop, ref value)) => {
              node.style().set_property(
                &prop.as_string().unwrap(),
                &value.as_string().unwrap(),
              )?;
            }
            _ => {}
          };
        }

        // create_children
        for child in children.to_vec() {
          let child_node = self.create_node(&child)?;

          node.append_child(&child_node)?;
        }

        Ok(node.into())
      }
      // create_text_node
      _ => {
        let name = &tag.vkind().text();
        let node = self.document.create_text_node(name);

        Ok(node.into())
      }
    }
  }

  fn remove(&self) -> Result<(), JsValue> {
    for (parent, child) in &self.to_remove {
      parent.remove_child(&child)?;
    }

    Ok(())
  }

  pub fn render(
    &mut self,
    app: &Rc<App>,
    old_tags: &Option<&Vec<Box<dyn HtmlValue>>>,
    new_tags: &Vec<Box<dyn HtmlValue>>,
  ) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let parent: Node = document
      .get_element_by_id("app")
      .expect("#app element not found")
      .into();

    let mut renderer = Renderer::new(document, app.clone(), vec![]);

    for (x, new_tag) in new_tags.iter().enumerate() {
      let old_tag = if old_tags.is_some() {
        Some(&old_tags.unwrap()[x])
      } else {
        None
      };

      renderer.update_element(&parent, old_tag, Some(new_tag), 0)?;
      renderer.remove()?;
    }

    Ok(())
  }

  fn scope_style_tag_if_possible(
    &self,
    node: &HtmlElement,
  ) -> Result<(), JsValue> {
    if let "STYLE" = node.tag_name().as_str() {
      node.set_attribute("scoped", "")?;
    };

    Ok(())
  }

  fn update_element(
    &mut self,
    parent: &Node,
    lhs: Option<&Box<dyn HtmlValue>>,
    rhs: Option<&Box<dyn HtmlValue>>,
    index: u32,
  ) -> Result<(), JsValue> {
    match (lhs, rhs) {
      (None, Some(rhs_vdom)) => {
        console_log!("update 1");

        let node = self.create_node(rhs_vdom)?;

        parent.append_child(&node)?;
      }
      (Some(lhs_vdom), None) => {
        console_log!("update 2",);
        if let Some(child) = parent.child_nodes().item(index) {
          self.to_remove.push((parent.clone(), child));
        } else {
          console_log!("index: {}, lhs: {}", index, lhs_vdom.text());
        }
      }
      (Some(_lhs_vdom), Some(_rhs_vdom)) => {
        console_log!("update 3");
      }
      (None, None) => {
        console_log!("update 4");
      }
    }

    Ok(())
  }
}

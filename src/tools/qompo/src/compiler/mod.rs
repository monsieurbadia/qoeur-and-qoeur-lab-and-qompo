pub mod parser;
pub mod transpiler;

// use crate::value::program::Program;
// use crate::value::tag::{HtmlValue, HtmlValueResult, VKind};

// pub fn parse (source: &str) -> HtmlValueResult<Vec<Box<dyn HtmlValue>>> {
//   let mut nodes = vec![];

//   match Program::advance_capsule(source).vkind() {
//     VKind::Program(ref tags) => { nodes = tags.to_vec() },
//     _ => {},
//   };

//   Ok(nodes)
// }

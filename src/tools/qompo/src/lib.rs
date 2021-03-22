pub mod compiler;
pub mod computer;
pub mod utils;
pub mod value;

use crate::compiler::parser::Parser;
use crate::compiler::transpiler::Transpiler;
use crate::computer::app::App;
use crate::value::VKind;
use qoeurc::analyzer::interpreter::Interpreter;

use std::rc::Rc;

use cfg_if::cfg_if;
use rsass::compile_scss;
use wasm_bindgen::prelude::*;

cfg_if! {
  // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
  // allocator.
  if #[cfg(feature = "wee_alloc")] {
    extern crate wee_alloc;
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
  }
}

#[wasm_bindgen]
pub struct WebCompiler {
  source: String,
}

#[wasm_bindgen]
impl WebCompiler {
  pub fn new(source: &str) -> WebCompiler {
    WebCompiler {
      source: source.to_owned(),
    }
  }

  pub fn compile(&self) {
    let mut parser = Parser::new(&self.source);
    let program_parsed = parser.parse().unwrap();

    let mut transpiler = Transpiler::new(&program_parsed, Interpreter::new());
    let program_transpiled = transpiler.transpile().unwrap();

    if let VKind::Program(capsule) = program_transpiled.vkind() {
      let mut app = Rc::new(App::new(capsule));
      app.render();
    };
  }
}

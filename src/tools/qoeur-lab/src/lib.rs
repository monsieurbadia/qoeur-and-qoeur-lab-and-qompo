mod utils;

use qoeurc::analyzer;
use qoeurc::analyzer::interpreter::Interpreter;
use qoeurc::converter;
use qoeurc::tokenizer::{self, token::Token};
use qoeurc::transformer;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn astify(input: &str) -> String {
  converter::astify(input).unwrap().text()
}

pub fn evalify(input: &str) -> String {
  let mut interpreter = Interpreter::new();

  analyzer::evalify(input, &mut interpreter).unwrap().text()
}

pub fn tokenify(input: &str) -> String {
  format!(
    "{:?}",
    tokenizer::tokenify(input.into()).collect::<Vec<Token>>()
  )
}

pub fn transformify(input: &str, mode: &str) -> String {
  let interpreter = Interpreter::new();

  transformer::transformify(input, mode, &interpreter).unwrap()
}

#[wasm_bindgen]
pub fn wasm_tokenify(input: &str) -> String {
  tokenify(input)
}

#[wasm_bindgen]
pub fn wasm_astify(input: &str) -> String {
  astify(input)
}

#[wasm_bindgen]
pub fn wasm_evalify(input: &str) -> String {
  evalify(input)
}

#[wasm_bindgen]
pub fn wasm_transformify(input: &str, mode: &str) -> String {
  transformify(input, mode)
}

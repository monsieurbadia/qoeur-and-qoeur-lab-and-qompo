pub mod ascii;
pub mod kind;
pub mod scanner;
pub mod token;

#[cfg(test)]
mod tests;

use crate::tokenizer::scanner::Scanner;
use crate::tokenizer::token::Token;

// @from https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs#L215
pub fn first_token(input: &str) -> Token {
  debug_assert!(!input.is_empty());
  Scanner::new(input).advance_token()
}

// @from https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs#L221
pub fn tokenify(mut input: &str) -> impl Iterator<Item = Token> + '_ {
  std::iter::from_fn(move || {
    if input.is_empty() {
      return None;
    }

    let token = first_token(input);

    input = &input[token.len..];
    Some(token)
  })
}

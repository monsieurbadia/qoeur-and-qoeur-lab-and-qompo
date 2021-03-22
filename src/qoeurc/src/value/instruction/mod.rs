pub mod array;
pub mod attribute;
pub mod binary;
pub mod block;
pub mod call;
pub mod closure;
pub mod comment;
pub mod control;
pub mod expression;
pub mod function;
pub mod function_arg;
pub mod group;
pub mod hash;
pub mod identifier;
pub mod index;
pub mod loop_for_in;
pub mod loop_for_range;
pub mod loop_infinite;
pub mod loop_while;
pub mod program;
pub mod return_value;
pub mod shebang;
pub mod statement;
pub mod ty;
pub mod unary;
pub mod val;

pub use crate::value::instruction::{
  array::Array, binary::Binary, block::Block, call::Call, control::If,
  expression::Expression, function::Function, hash::Hash,
  identifier::Identifier, index::Index, loop_for_in::LoopForIn,
  loop_for_range::LoopForRange, loop_infinite::LoopInfinite,
  loop_while::LoopWhile, program::Program, return_value::Return,
  statement::Statement, unary::Unary, val::Val,
};

use crate::analyzer::environment::scope::TScope;
use crate::value::instruction::hash::HashKey;
use crate::value::Value;

#[macro_export]
macro_rules! void {
  () => {
    Box::new($crate::value::instruction::array::Array::default())
  };
}

#[derive(Clone, Debug, PartialEq)]
pub enum IKind {
  NOOP,
  Array(Vec<Box<dyn Value>>),
  Attribute,
  Binary,
  Block(Vec<Box<dyn Value>>),
  Bool(bool),
  Call,
  Char(char),
  Capsule,
  Closure,
  Comment,
  Expression,
  If,
  Function(
    Option<Vec<Box<dyn Value>>>,
    Option<Box<dyn Value>>,
    Option<Box<dyn Value>>,
    Option<Box<dyn Value>>,
    Option<TScope>,
  ),
  FunctionArg,
  Float(f64),
  Group,
  Hash(Vec<(HashKey, Box<dyn Value>)>),
  Identifier,
  Index,
  Int(i64),
  Keyword,
  LoopForIn,
  LoopForRange,
  LoopInfinite,
  LoopWhile,
  Program(Vec<Box<dyn Value>>),
  Return,
  Shebang,
  Statement,
  Str(String),
  Ty,
  Unary,
  Val,
  While,
}

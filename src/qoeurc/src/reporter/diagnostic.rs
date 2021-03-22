use crate::reporter::handler::Handler;

#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
  pub code: Option<String>,
  pub message: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticBuilder<'a> {
  handler: &'a Handler,
  diagnostic: Vec<Diagnostic>,
}

impl<'a> DiagnosticBuilder<'a> {
  pub fn print_errors(errors: Vec<Result<(), String>>) {
    if errors.len() > 0 {
      errors.iter().for_each(|e| println!("{:?}", e));
    }
  }
}

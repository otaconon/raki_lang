#[derive(Clone, PartialEq, Debug)]
pub enum RakiError {
  Scanner(String),
  Syntax{line: u32, at: String, message: String},
  Runtime{}
}
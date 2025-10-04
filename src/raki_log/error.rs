pub enum RakiError {
  Syntax{line: u32, at: String, message: String}
}
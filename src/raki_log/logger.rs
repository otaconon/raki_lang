use crate::raki_log::RakiError;

pub fn raki_log(err: &RakiError) {
  match err {
    RakiError::Syntax { line, at, message } => println!("Syntax error on line {}, {}: {}", line, at, message),
  }
}
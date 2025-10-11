use crate::raki_log::RakiError;
use log::error;

pub fn raki_log(err: &RakiError) {
  match err {
    RakiError::Scanner(msg) => error!("Parser error => {}", msg),
    RakiError::Syntax { line, at, message } => error!("Syntax error on line {}, {}: {}", line, at, message),
    RakiError::Runtime {} => {  }
  }
}
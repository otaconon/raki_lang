mod lexer;
mod raki_log;

use lexer::{Token, TokenType};

fn main() {
  let tok = TokenType::Number;
  println!("{}", tok.to_string());
}

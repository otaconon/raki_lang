mod raki_log;
mod lexer;
mod parser;

use lexer::{Token, TokenType};

fn main() {
  let tok = TokenType::Number;
  println!("{}", tok.to_string());
}

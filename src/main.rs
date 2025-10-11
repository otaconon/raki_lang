mod raki_log;
mod lexer;
mod parser;
mod interpreter;

use lexer::{Token, TokenType};

use crate::{interpreter::Interpreter, lexer::Scanner, parser::{Parser, Visitor}};

fn main() {
  let mut scanner = Scanner::new("1 > 2 ? 3 : 4".to_string());
  let mut parser = Parser::new(scanner.scan_tokens().unwrap());
  let exprs = parser.parse();

  let interpreter = Interpreter{};
  let res = interpreter.visit_expr(&exprs[0]);
  match res {
    Ok(obj) => println!("{}", obj),
    Err(_) => {}
  }
}

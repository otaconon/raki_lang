use super::Expr;
use super::Visitor;
use crate::Token;

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
  fn visit_expr(&self, e: &Expr) -> String {
    match e {
      Expr::Binary { left, right, operator } => return self.parenthesize(&operator.lexeme, [left, right]),
      Expr::Grouping { expr } => return self.parenthesize("group", [expr]),
      Expr::Literal { value } => return value.to_string(),
      Expr::Unary { right, operator } => return self.parenthesize(&operator.lexeme, [right]),
      Expr::Ternary { condition, left, right } => return self.parenthesize("ternary", [condition, left, right]),
    }
  }
}

impl AstPrinter {
  fn parenthesize<'a, I>(&self, name: &str, exprs: I) -> String
  where
    I: IntoIterator<Item = &'a Box<Expr>>,
  {
    let mut res = format!("( {} ", name);
    for expr in exprs.into_iter() {
      let e = self.visit_expr(expr) + " ";
      res.push_str(&e);
    }

    res.push(')');
    res
  }
}

#[cfg(test)]
mod test {
  use crate::lexer::{LiteralType, TokenType};

use super::*;

  #[test]
  fn printer_prints() {
    let expression = Expr::Binary { 
      left: Box::new(Expr::Unary { 
        right: Box::new(Expr::Literal { value: LiteralType::I32(123) }),
        operator: Token { r#type: TokenType::Minus, lexeme: "-".to_string(), literal: LiteralType::String(String::new()), line: 0 }
      }),
      right: Box::new(Expr::Grouping { expr: Box::new(Expr::Literal { value: LiteralType::F32(45.67) }) }), 
      operator: (Token { r#type: TokenType::Star, lexeme: "*".to_string(), literal: LiteralType::String(String::new()), line: 0 }) 
    };

    let printer = AstPrinter{};
    assert_eq!(printer.visit_expr(&expression), "( * ( - 123 ) ( group 45.67 ) )".to_string());
  }
}
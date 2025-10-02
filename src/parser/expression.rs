use crate::lexer::token::Token;
use paste::paste;

enum Expr {
  Binary { left: Box<Expr>, right: Box<Expr>, operator: Token },
  Grouping { expr: Box<Expr> },
  Literal { value: Option<String> },
  Unary { right: Box<Expr>, operator: Token }
}

trait ExprVisitor<R> {
  fn visit_expr(&mut self, expr: &mut Expr) -> R;
}

impl Expr {
  fn accept<R>(&mut self, visitor: &mut dyn ExprVisitor<R>) -> R {
    visitor.visit_expr(self)
  }
}

pub struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
  fn visit_expr(&mut self, e: &mut Expr) -> String {
    match e {
      Expr::Binary { left, right, operator } => return self.parenthesize(&operator.lexeme, [left, right]),
      Expr::Grouping { expr } => return self.parenthesize("group ", [expr]),
      Expr::Literal { value } => return value.as_deref().unwrap_or("nil").to_string(),
      Expr::Unary { right, operator } => return self.parenthesize(&operator.lexeme, [right])
    }
  }
}

impl AstPrinter {
  fn parenthesize<'a, I>(&mut self, name: &str, exprs: I) -> String
  where
    I: IntoIterator<Item = &'a mut Box<Expr>>,
  {
    let mut res = format!("({}", name);
    for expr in exprs.into_iter() {
      res.push_str(&expr.accept(self));
    }

    res.push(')');
    res
  }
}

#[cfg(test)]
mod test {
  use crate::lexer::TokenType;

use super::*;

  #[test]
  fn printer_prints() {
    let mut expression = Expr::Binary { 
      left: Box::new(Expr::Unary { 
        right: Box::new(Expr::Literal { value: Some("123".to_string()) }),
        operator: Token { r#type: TokenType::Minus, lexeme: "-".to_string(), literal: String::new(), line: 0 }
      }),
      right: Box::new(Expr::Grouping { expr: Box::new(Expr::Literal { value: Some("45.67".to_string()) }) }), 
      operator: (Token { r#type: TokenType::Star, lexeme: "*".to_string(), literal: String::new(), line: 0 }) 
    };

    let mut printer = AstPrinter{};
    assert_eq!(printer.visit_expr(&mut expression), "(*(-123)(group 45.67))".to_string());
  }
}
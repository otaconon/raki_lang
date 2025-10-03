use super::Expr;
use super::Visitor;
use crate::Token;

pub struct RpnPrinter;
impl Visitor<String> for RpnPrinter {
  fn visit_expr(&self, e: &Expr) -> String {
    let mut res: Vec<String> = Vec::new();
    match e {
      Expr::Binary { left, right, operator } => {
        res.push(self.visit_expr(left));
        res.push(self.visit_expr(right));
        res.push(operator.lexeme.to_string());
      }
      Expr::Grouping { expr } => {
        res.push(self.visit_expr(expr));
        res.push("group".to_string());
      }
      Expr::Literal { value } => {
        res.push(value.as_deref().unwrap_or("nil").to_string());
      }
      Expr::Unary { right, operator } => {
        res.push(self.visit_expr(right));
        res.push(operator.lexeme.to_string());
      }
    }

    res.join(" ")
  }
}

#[cfg(test)]
mod test {
  use crate::lexer::TokenType;

use super::*;

  #[test]
  fn printer_prints() {
    let expression = Expr::Binary { 
      left: Box::new(Expr::Unary { 
        right: Box::new(Expr::Literal { value: Some("123".to_string()) }),
        operator: Token { r#type: TokenType::Minus, lexeme: "-".to_string(), literal: String::new(), line: 0 }
      }),
      right: Box::new(Expr::Grouping { expr: Box::new(Expr::Literal { value: Some("45.67".to_string()) }) }), 
      operator: (Token { r#type: TokenType::Star, lexeme: "*".to_string(), literal: String::new(), line: 0 }) 
    };

    let printer = RpnPrinter{};
    assert_eq!(printer.visit_expr(&expression), "123 - 45.67 group *".to_string());
  }
}
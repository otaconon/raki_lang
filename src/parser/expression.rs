use crate::lexer::token::Token;
use paste::paste;

enum Expr {
  Binary {left: Box<Expr>, right: Box<Expr>, token: Token},
}

trait ExprVisitor {
  fn visit_binary(&mut self, left: &mut Expr, right: &mut Expr, token: &mut Token);
}

impl Expr {
  fn accept(&mut self, visitor: &mut dyn ExprVisitor) {
    match self {
      Expr::Binary { left, right, token } => visitor.visit_binary(left, right, token),
    }
  }
}
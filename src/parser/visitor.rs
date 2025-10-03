use super::Expr;

pub trait Visitor<R> {
  fn visit_expr(&self, expr: &Expr) -> R;
}
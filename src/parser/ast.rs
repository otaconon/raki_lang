use crate::lexer::token::Token;

pub enum Expr {
  Binary { left: Box<Expr>, right: Box<Expr>, operator: Token },
  Grouping { expr: Box<Expr> },
  Literal { value: Option<String> },
  Unary { right: Box<Expr>, operator: Token }
}
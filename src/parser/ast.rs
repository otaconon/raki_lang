use crate::lexer::{token::Token, LiteralType};

#[derive(Clone)]
pub enum Expr {
  Binary { left: Box<Expr>, right: Box<Expr>, operator: Token },
  Grouping { expr: Box<Expr> },
  Literal { value: LiteralType },
  Unary { right: Box<Expr>, operator: Token },
  Ternary { condition: Box<Expr>, left: Box<Expr>, right: Box<Expr>},
}
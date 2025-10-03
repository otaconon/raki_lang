use crate::lexer::{token::Token, LiteralType};

pub enum Expr {
  Binary { left: Box<Expr>, right: Box<Expr>, operator: Token },
  Grouping { expr: Box<Expr> },
  Literal { value: LiteralType },
  Unary { right: Box<Expr>, operator: Token }
}
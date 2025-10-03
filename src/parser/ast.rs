use crate::lexer::token::Token;

/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
*/

pub enum Expr {
  Binary { left: Box<Expr>, right: Box<Expr>, operator: Token },
  Grouping { expr: Box<Expr> },
  Literal { value: Option<String> },
  Unary { right: Box<Expr>, operator: Token }
}
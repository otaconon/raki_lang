use super::Expr;
use crate::lexer::LiteralType;
use crate::Token;
use crate::TokenType;

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

pub struct Parser {
  tokens: Vec<Token>,
  current: usize,
}

impl Parser {
  fn new(tokens: Vec<Token>) -> Parser {
    Parser { tokens, current: 0 }
  }

  fn expression(&mut self) -> Expr {
    return self.equality();
  }

  fn equality(&mut self) -> Expr {
    let mut expr: Expr = self.comparison();

    while let TokenType::BangEqual | TokenType::EqualEqual = self.peek().r#type {
        let operator = self.previous().clone();
        let right = self.comparison();
        expr = Expr::Binary {
            left: Box::new(expr),
            right: Box::new(right),
            operator,
        };
    }

    expr
  }

  fn comparison(&mut self) -> Expr {
    let mut expr: Expr = self.term();

    while let TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual = self.peek().r#type {
        let operator = self.previous().clone();
        let right = self.term();
        expr = Expr::Binary {
            left: Box::new(expr),
            right: Box::new(right),
            operator,
        };
    }

    expr
  }

  fn term(&mut self) -> Expr {
    let mut expr: Expr = self.factor();

    while let TokenType::Plus | TokenType::Minus = self.peek().r#type {
        let operator = self.previous().clone();
        let right = self.factor();
        expr = Expr::Binary {
            left: Box::new(expr),
            right: Box::new(right),
            operator,
        };
    }

    expr
  }

  fn factor(&mut self) -> Expr {
    let mut expr: Expr = self.unary();

    while let TokenType::Plus | TokenType::Minus = self.peek().r#type {
        let operator = self.previous().clone();
        let right = self.unary();
        expr = Expr::Binary {
            left: Box::new(expr),
            right: Box::new(right),
            operator,
        };
    }

    expr
  }

  fn unary(&mut self) -> Expr {
    if let TokenType::Bang | TokenType::Minus = self.peek().r#type {
        let operator = self.previous().clone();
        let right = self.unary();
        return Expr::Unary {
            right: Box::new(right),
            operator,
        };
    }

    self.primary()
  }

  fn primary(&mut self) -> Expr {
    match self.peek().r#type {
      TokenType::False => return Expr::Literal { value: LiteralType::Bool(false) },
      TokenType::True => return Expr::Literal { value: LiteralType::Bool(true) },
      TokenType::Nil => return Expr::Literal { value: LiteralType::None },
      TokenType::Number | TokenType::String => return Expr::Literal {value: self.previous().literal.clone()},
      TokenType::LeftParen => {
        let expr: Expr = self.expression();
        //self.consume(TokenType::RightParen, "Expect ')' after expression.");
        return Expr::Grouping { expr: Box::new(expr) }
      }
      _ => return Expr::Literal { value: LiteralType::None }
    }
  }
  

  fn is_eof(&self) -> bool {
    self.peek().r#type == TokenType::Eof
  }

  fn advance(&mut self) -> &Token {
    self.current += 1;
    self.peek()
  }

  fn peek(&self) -> &Token {
    &self.tokens[self.current]
  }

  fn previous(&self) -> &Token {
    &self.tokens[self.current-1]
  }
}

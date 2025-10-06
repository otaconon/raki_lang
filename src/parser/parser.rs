use super::Expr;
use crate::Token;
use crate::TokenType;
use crate::lexer::LiteralType;
use crate::raki_log::{RakiError, raki_log};

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
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser { tokens, current: 0 }
  }

  pub fn parse(&mut self) -> Option<Expr> {
    match self.expression() {
      Ok(expr) => Some(expr),
      Err(_) => {
        self.synchronize();
        None
      }
    }
  }

  fn expression(&mut self) -> Result<Expr, RakiError> {
    return self.equality();
  }

  fn equality(&mut self) -> Result<Expr, RakiError> {
    let mut expr: Expr = self.comparison()?;

    while let TokenType::BangEqual | TokenType::EqualEqual = self.peek().r#type {
      self.advance();
      let operator = self.previous().clone();
      let right = self.comparison()?;
      expr = Expr::Binary { left: Box::new(expr), right: Box::new(right), operator };
    }

    Ok(expr)
  }

  fn comparison(&mut self) -> Result<Expr, RakiError> {
    let mut expr: Expr = self.term()?;

    while let TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual = self.peek().r#type {
      self.advance();
      let operator = self.previous().clone();
      let right = self.term()?;
      expr = Expr::Binary { left: Box::new(expr), right: Box::new(right), operator };
    }

    Ok(expr)
  }

  fn term(&mut self) -> Result<Expr, RakiError> {
    let mut expr: Expr = self.factor()?;

    while let TokenType::Plus | TokenType::Minus = self.peek().r#type {
      self.advance();
      let operator = self.previous().clone();
      let right = self.factor()?;
      expr = Expr::Binary { left: Box::new(expr), right: Box::new(right), operator };
    }

    Ok(expr)
  }

  fn factor(&mut self) -> Result<Expr, RakiError> {
    let mut expr: Expr = self.unary()?;

    while let TokenType::Plus | TokenType::Minus = self.peek().r#type {
      self.advance();
      let operator = self.previous().clone();
      let right = self.unary()?;
      expr = Expr::Binary { left: Box::new(expr), right: Box::new(right), operator };
    }

    Ok(expr)
  }

  fn unary(&mut self) -> Result<Expr, RakiError> {
    if let TokenType::Bang | TokenType::Minus = self.peek().r#type {
      self.advance();
      let operator = self.previous().clone();
      let right = self.unary()?;
      return Ok(Expr::Unary { right: Box::new(right), operator });
    }

    self.primary()
  }

  fn primary(&mut self) -> Result<Expr, RakiError> {
    self.advance();
    match self.previous().r#type {
      TokenType::False => return Ok(Expr::Literal { value: LiteralType::Bool(false) }),
      TokenType::True => return Ok(Expr::Literal { value: LiteralType::Bool(true) }),
      TokenType::Nil => return Ok(Expr::Literal { value: LiteralType::None }),
      TokenType::Number | TokenType::String => return Ok(Expr::Literal { value: self.previous().literal.clone() }),
      TokenType::LeftParen => {
        let expr: Expr = self.expression()?;
        let res = self.consume(TokenType::RightParen, "Expect ')' after expression.");
        if res.is_err() {
          return Ok(Expr::Literal { value: LiteralType::None });
        }
        return Ok(Expr::Grouping { expr: Box::new(expr) });
      }
      _ => {
        self.error(self.peek(), "Expect expression.");
        return Ok(Expr::Literal { value: LiteralType::None });
      }
    }
  }

  fn synchronize(&mut self) {
    self.advance();
    while !self.is_eof() {
      if self.previous().r#type == TokenType::Semicolon {
        return;
      }
      use TokenType::*;
      match self.peek().r#type {
        Class | Fun | Var | For | If | While | Print | Return => return,
        _ => self.advance(),
      };
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
    &self.tokens[self.current - 1]
  }

  fn consume(&mut self, tty: TokenType, msg: &str) -> Result<&Token, RakiError> {
    if self.peek().r#type == tty {
      return Ok(self.advance());
    }

    Err(self.error(self.peek(), msg))
  }

  fn error(&self, token: &Token, msg: &str) -> RakiError {
    let err: RakiError;
    match token.r#type {
      TokenType::Eof => {
        err = RakiError::Syntax {
          line: token.line,
          at: " at end".to_string(),
          message: msg.to_string(),
        }
      }
      _ => {
        err = RakiError::Syntax {
          line: token.line,
          at: format!(" at {}", token.lexeme),
          message: msg.to_string(),
        }
      }
    }

    raki_log(&err);
    err
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{
    lexer::Scanner,
    parser::{AstPrinter, Visitor},
  };

  #[test]
  fn parses() {
    let mut scanner = Scanner::new("123 - 45".to_string());
    let mut parser = Parser::new(scanner.scan_tokens());
    let ast_printer = AstPrinter {};
    match parser.parse() {
      Some(expr) => assert_eq!(ast_printer.visit_expr(&expr), "( - 123 45 )"),
      None => {
        assert!(false)
      }
    }
  }
}

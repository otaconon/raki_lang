use super::Expr;
use crate::Token;
use crate::TokenType;
use crate::lexer::LiteralType;
use crate::raki_log::{RakiError, raki_log};

/*
expression     → comma ;
comma          → comma "," ternary
               | ternary ;
ternary        → equality ( "?" expression ":" ternary )? ;
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
  exprs: Vec<Expr>,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser { tokens, current: 0, exprs: Vec::new() }
  }

  pub fn parse(&mut self) -> Vec<Expr> {
    loop {
      match self.expression() {
        Ok(expr) => self.exprs.push(expr),
        Err(_) => {
          self.synchronize();
        }
      };

      match self.is_eof() {
        true => break,
        false => self.advance()
      };
    }

    self.exprs.clone()
  }

  fn expression(&mut self) -> Result<Expr, RakiError> {
    return self.comma();
  }

  fn comma(&mut self) -> Result<Expr, RakiError> {
    let mut expr = self.ternary()?;
    
    if let TokenType::Comma = self.peek().r#type {
      self.advance();
      expr = self.expression()?;
    }

    Ok(expr)
  }

  fn ternary(&mut self) -> Result<Expr, RakiError> {
    let condition = self.equality()?;

    if let TokenType::QuestionMark = self.peek().r#type {
      self.advance();
      let left = Box::new(self.expression()?);
      match self.consume(TokenType::DoubleDot, "Expect ':' after ternary operator") {
        Ok(_) => {
          let right = Box::new(self.ternary()?);
          return Ok(Expr::Ternary{condition: Box::new(condition), left, right})
        }
        Err(err) => return Err(err)
      }
    }

    Ok(condition)
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
    if matches!(self.peek().r#type, TokenType::False | TokenType::True | TokenType::Nil | TokenType::Number | TokenType::LeftParen) {
      self.advance();
    }

    match self.previous().r#type {
      TokenType::False => return Ok(Expr::Literal { value: LiteralType::Bool(false) }),
      TokenType::True => return Ok(Expr::Literal { value: LiteralType::Bool(true) }),
      TokenType::Nil => return Ok(Expr::Literal { value: LiteralType::None }),
      TokenType::Number | TokenType::String => return Ok(Expr::Literal { value: self.previous().literal.clone() }),
      TokenType::LeftParen => {
        let expr: Expr = self.expression()?;
        match self.consume(TokenType::RightParen, "Expect ')' after expression.") {
          Ok(_) => return Ok(Expr::Grouping { expr: Box::new(expr) }),
          Err(err) => return Err(err)
        }
      }
      _ => return Err(self.error(self.peek(), "Expected expression."))
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
  fn handles_equality_operator() {
    let mut scanner = Scanner::new("1 == 10".to_string());
    let mut parser = Parser::new(scanner.scan_tokens());
    let ast_printer = AstPrinter {};
    let exprs = parser.parse();
    assert_eq!(ast_printer.visit_expr(&exprs[0]), "( == 1 10 )");
  }

  #[test]
  fn handles_comparison_operator() {
    let mut scanner = Scanner::new("1 > 10".to_string());
    let mut parser = Parser::new(scanner.scan_tokens());
    let ast_printer = AstPrinter {};
    let exprs = parser.parse();
    assert_eq!(ast_printer.visit_expr(&exprs[0]), "( > 1 10 )");
  }

  #[test]
  fn handles_comma_operator() {
    let mut scanner = Scanner::new("123 - 45, 48 + 25, 82 + 102".to_string());
    let mut parser = Parser::new(scanner.scan_tokens());
    let ast_printer = AstPrinter {};
    let exprs = parser.parse();
    assert_eq!(ast_printer.visit_expr(&exprs[0]), "( + 82 102 )");
  }

  /*
  #[test]
  fn handles_ternary_operator() {
    let mut scanner = Scanner::new("1 > 2 ? 3 : 4".to_string());
    let mut parser = Parser::new(scanner.scan_tokens());
    let ast_printer = AstPrinter {};
    let exprs = parser.parse();
    assert_eq!(ast_printer.visit_expr(&exprs[0]), "( ternary ( > 1 2 ) 3 4 )");
  }
  */
}

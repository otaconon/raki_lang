use super::{Token, TokenType};
use crate::raki_log::raki_log;

pub struct Scanner {
  source: String,
  tokens: Vec<Token>,
  errors: Vec<String>,
  start: usize,
  current: usize,
  line: u32
}

// Only supports ASCII
impl Scanner {
  pub fn new(source: String) -> Scanner {
    Scanner {
      source: source,
      tokens: Vec::new(),
      errors: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
    }
  }

  pub fn scan_tokens(&mut self) -> Vec<Token>{
    self.tokens = Vec::new();

    while !self.is_eof() {
      self.start = self.current;
      self.scan_token();
    }

    self.tokens.push(Token{r#type: TokenType::Eof, lexeme: String::new(), literal: String::new(), line: self.line});
    self.tokens.clone()
  }

  fn scan_token(&mut self) {
    let c = self.advance();
    match TokenType::from_char(c) {
      Some(token) => {
        if self.is_eof() {
          self.add_token(token, String::from(c));
          return;
        }

        let nc = self.source.as_bytes()[self.current] as char;
        let new_token = TokenType::make_2char(token, nc);

        if new_token != token {
          self.add_token(token, String::from(c));
          return;
        }
        else {
          self.add_token(new_token, format!("{}{}", c, nc).to_string());
          return;
        }
      }
      None => self.errors.push(format!("Error: unexpected token: {}, at line: {}", c, self.line).to_string()),
    }
  }

  fn is_eof(&self) -> bool {
    self.current >= self.source.len()
  }

  fn add_token(&mut self, r#type: TokenType, literal: String) {
    let lexeme = &self.source[self.start..self.current];
    self.tokens.push(Token{r#type, lexeme: String::from(lexeme), literal, line: self.line});
  }

  fn advance(&mut self) -> char {
    self.current += 1;
    self.source.as_bytes()[self.current-1] as char
  }

  fn check_next(&mut self, expected: char) -> bool {
    if self.is_eof() {
      return false;
    }
    if self.source.as_bytes()[self.current] as char != expected {
      return false;
    }

    self.current += 1;
    true
  }

  pub fn get_errors(&self) -> Vec<String> {
    self.errors.clone()
  }
}

#[cfg(test)] 
mod test {
  use super::*;

  #[test]
  fn test_token_type_scanning() {
    let mut scanner = Scanner::new(String::from("()}+-"));
    let tokens = scanner.scan_tokens();

    assert_eq!(tokens[0].r#type, TokenType::LeftParen);
    assert_eq!(tokens[1].r#type, TokenType::RightParen);
    assert_eq!(tokens[2].r#type, TokenType::RightBrace);
    assert_eq!(tokens[3].r#type, TokenType::Plus);
    assert_eq!(tokens[4].r#type, TokenType::Minus);
    assert_eq!(tokens[5].r#type, TokenType::Eof);
  }

  #[test]
  fn test_token_lexeme_scanning() {
    let mut scanner = Scanner::new(String::from("()}+-"));
    let tokens = scanner.scan_tokens();

    assert_eq!(tokens[0].lexeme, "(");
    assert_eq!(tokens[1].lexeme, ")");
    assert_eq!(tokens[2].lexeme, "}");
    assert_eq!(tokens[3].lexeme, "+");
    assert_eq!(tokens[4].lexeme, "-");
    assert_eq!(tokens[5].lexeme, "");
  }

  #[test]
  fn test_unexpected_token_handling() {
    let mut scanner = Scanner::new(String::from("(@+%"));
    let tokens = scanner.scan_tokens();
    let errors = scanner.get_errors();

    assert_eq!(tokens[0].r#type, TokenType::LeftParen);
    assert_eq!(tokens[1].r#type, TokenType::Plus);
    assert_eq!(errors[0], String::from("Error: unexpected token: @, at line: 1"));
    assert_eq!(errors[1], String::from("Error: unexpected token: %, at line: 1"));
  }
}
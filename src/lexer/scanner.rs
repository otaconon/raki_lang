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

    // Handle the case where c opens a string literal
    if c == '"' {
      while !self.is_eof() && self.peek() != '"' {
        if self.peek() == '\n' {
          self.line += 1;
        }
        self.advance();
      }

      if self.is_eof() {
        self.errors.push(format!("Error: uneterminated string literal at line: {}", self.line).to_string());
        return;
      }

      self.advance();
      let literal = &self.source[self.start+1..self.current-1];
      self.add_token(TokenType::String, String::from(literal));
      return;
    }

    // Match the first character
    match TokenType::from_char(c) {
      Some(token) => {
        if token == TokenType::Ignore {
          return;
        }

        if self.is_eof() {
          self.add_token(token, String::from(c));
          return;
        }

        let nc = self.source.as_bytes()[self.current] as char;
        
        // Check if literal is a comment
        if c == nc && nc == '/' {
          while !self.is_eof() && self.peek() != '\n' {
            self.advance();
          }
          return;
        }
        

        // Try matching second character to 2 char literal
        match token.get_2char_extension(nc) {
          Some(extended_token) => {
            self.add_token(extended_token, format!("{}{}", c, nc).to_string());
            self.current += 1;
          }
          None => self.add_token(token, String::from(c)),
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

  fn peek(&self) -> char {
    self.source.as_bytes()[self.current] as char
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
  fn scans_single_char_token_types() {
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
  fn scans_single_char_token_lexemes() {
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
  fn handles_unexpected_tokens() {
    let mut scanner = Scanner::new(String::from("(@+%"));
    let tokens = scanner.scan_tokens();
    let errors = scanner.get_errors();

    assert_eq!(tokens[0].r#type, TokenType::LeftParen);
    assert_eq!(tokens[1].r#type, TokenType::Plus);
    assert_eq!(errors[0], String::from("Error: unexpected token: @, at line: 1"));
    assert_eq!(errors[1], String::from("Error: unexpected token: %, at line: 1"));
  }

  #[test]
  fn scans_variable_length_token_types() {
    let mut scanner = Scanner::new(String::from("(==)=}!="));
    let tokens = scanner.scan_tokens();
    let errors = scanner.get_errors();

    assert_eq!(tokens[0].r#type, TokenType::LeftParen);
    assert_eq!(tokens[1].r#type, TokenType::EqualEqual);
    assert_eq!(tokens[2].r#type, TokenType::RightParen);
    assert_eq!(tokens[3].r#type, TokenType::Equal);
    assert_eq!(tokens[4].r#type, TokenType::RightBrace);
    assert_eq!(tokens[5].r#type, TokenType::BangEqual);
    assert_eq!(tokens[6].r#type, TokenType::Eof);
    assert_eq!(errors.len(), 0);
  }

  #[test]
  fn scans_comments() {
    let mut scanner = Scanner::new(String::from("(//}{==ab"));
    let tokens = scanner.scan_tokens();
    let errors = scanner.get_errors();

    assert_eq!(tokens[0].r#type, TokenType::LeftParen);
    assert_eq!(tokens[1].r#type, TokenType::Eof);
    assert_eq!(tokens.len(), 2);
    assert_eq!(errors.len(), 0);
  }

  #[test]
  fn ignores_whitespaces() {
    let mut scanner = Scanner::new(String::from("(  != //abc"));
    let tokens = scanner.scan_tokens();
    let errors = scanner.get_errors();

    assert_eq!(tokens[0].r#type, TokenType::LeftParen);
    assert_eq!(tokens[1].r#type, TokenType::BangEqual);
    assert_eq!(tokens[2].r#type, TokenType::Eof);
    assert_eq!(tokens.len(), 3);
    assert_eq!(errors.len(), 0);
  }

  #[test]
  fn scans_string_tokens() {
    let mut scanner = Scanner::new(String::from("(\"abc\" !="));
    let tokens = scanner.scan_tokens();
    let errors = scanner.get_errors();

    assert_eq!(tokens[0].r#type, TokenType::LeftParen);

    assert_eq!(tokens[1].r#type, TokenType::String);
    assert_eq!(tokens[1].lexeme, "\"abc\"");
    assert_eq!(tokens[1].literal, "abc");

    assert_eq!(tokens[2].r#type, TokenType::BangEqual);
    assert_eq!(tokens[3].r#type, TokenType::Eof);
    assert_eq!(tokens.len(), 4);
    assert_eq!(errors.len(), 0);
  }
}
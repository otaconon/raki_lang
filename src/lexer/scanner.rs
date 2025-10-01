use super::{Token, TokenType};
use crate::{lexer::token_type, raki_log::raki_log, lexer::utils::*};

pub struct Scanner {
  source: String,
  tokens: Vec<Token>,
  errors: Vec<String>,
  start: usize,
  current: usize,
  line: u32,
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

  pub fn scan_tokens(&mut self) -> Vec<Token> {
    self.tokens = Vec::new();

    while !self.is_eof() {
      self.start = self.current;
      self.scan_token();
    }

    self.tokens.push(Token {
      r#type: TokenType::Eof,
      lexeme: String::new(),
      literal: String::new(),
      line: self.line,
    });
    self.tokens.clone()
  }

  fn scan_token(&mut self) {
    let c = self.advance();

    // Scan one char
    let token_type = match TokenType::from_char(c) {
      Some(tty) => tty,
      None => {
        self.errors.push(format!("Error: unexpected token: {}, at line: {}", c, self.line).to_string());
        return;
      }
    };

    // Single char is enough to tell if its a string, number or beginning of an identifier
    match token_type {
      TokenType::String => {
        self.eat_string();
        self.add_token(TokenType::String);
        return;
      }
      TokenType::Number => {
        self.eat_number();
        self.add_token(TokenType::Number);
        return;
      }
      TokenType::Identifier => {
        self.eat_identifier();
        self.add_token(TokenType::Identifier);
        return;
      }
      TokenType::Ignore => return,
      _ => {}
    }

    if self.is_eof() {
      self.add_token(token_type);
      return;
    }

    // Try scanning second char and try to extend the first one with it
    let nc = self.source.as_bytes()[self.current] as char;
    let extended_token_type = match token_type.get_extension(nc) {
      Some(etty) => {
        self.current += 1;
        etty
      }
      None => {
        self.add_token(token_type);
        return;
      }
    };
    
    // If the extended_token_type requires additional handling perform it
    match extended_token_type {
      TokenType::DoubleSlash => {
        self.eat_comment();
        return;
      }
      _ => {}
    }

    self.add_token(extended_token_type);
  }

  fn is_eof(&self) -> bool {
    self.current >= self.source.len()
  }

  fn add_token(&mut self, r#type: TokenType) {
    let lexeme = &self.source[self.start..self.current];

    let r#type = match r#type.get_identifier(lexeme) {
      Some(ty) => ty,
      None => r#type
    };

    let literal = match r#type {
      TokenType::String => &self.source[self.start + 1..self.current - 1],
      _ => lexeme,
    };

    self.tokens.push(Token {
      r#type,
      lexeme: String::from(lexeme),
      literal: String::from(literal),
      line: self.line,
    });
  }

  fn advance(&mut self) -> char {
    self.current += 1;
    self.source.as_bytes()[self.current - 1] as char
  }

  fn peek(&self) -> char {
    match self.current < self.source.len() {
      true => self.source.as_bytes()[self.current] as char,
      false => '\0'
    }
  }

  fn peek_next(&self) -> char {
    match self.current + 1 < self.source.len() {
      true => self.source.as_bytes()[self.current+1] as char,
      false => '\0'
    }
  }

  // Moves current byte pointer to the right string delimiter
  fn eat_string(&mut self) {
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
  }

  // Moves current byte pointer to the last digit of the number
  fn eat_number(&mut self) {
    while self.peek().is_digit(10) {
      self.advance();
      if self.peek() == '.' && self.peek_next().is_digit(10) {
        self.advance();
      }
    }
  }

  // Moves current byte pointer to the end of line
  fn eat_comment(&mut self) {
    while !self.is_eof() && self.peek() != '\n' {
      self.advance();
    }
  }

  fn eat_identifier(&mut self) {
    while self.peek().is_alphabetic() {
      self.advance();
    }
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
    let mut scanner = Scanner::new(String::from("(  != "));
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

  #[test]
  fn catches_unterminated_string_literals() {
    let mut scanner = Scanner::new(String::from("(\"abc !="));
    let _ = scanner.scan_tokens();
    let errors = scanner.get_errors();

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0], format!("Error: uneterminated string literal at line: {}", 1).to_string());
  }

  #[test]
  fn scans_number_literals() {
    let mut scanner = Scanner::new(String::from("(123 45"));
    let tokens = scanner.scan_tokens();
    let errors = scanner.get_errors();

    assert_eq!(tokens[0].r#type, TokenType::LeftParen);

    assert_eq!(tokens[1].r#type, TokenType::Number);
    assert_eq!(tokens[1].literal, String::from("123"));

    assert_eq!(tokens[2].r#type, TokenType::Number);
    assert_eq!(tokens[2].literal, String::from("45"));

    assert_eq!(tokens[3].r#type, TokenType::Eof);
    assert_eq!(errors.len(), 0);
  }

  #[test]
  fn scans_decimal_number_literals() {
    let mut scanner = Scanner::new(String::from("(123.45"));
    let tokens = scanner.scan_tokens();
    let errors = scanner.get_errors();

    assert_eq!(tokens[0].r#type, TokenType::LeftParen);

    assert_eq!(tokens[1].r#type, TokenType::Number);
    assert_eq!(tokens[1].literal, String::from("123.45"));

    assert_eq!(tokens[2].r#type, TokenType::Eof);
    assert_eq!(errors.len(), 0);
  }

  #[test]
  fn ignores_bad_decimals() {
    let mut scanner = Scanner::new(String::from("(123. .45"));
    let tokens = scanner.scan_tokens();
    let errors = scanner.get_errors();

    assert_eq!(tokens[0].r#type, TokenType::LeftParen);

    assert_eq!(tokens[1].r#type, TokenType::Number);
    assert_eq!(tokens[1].literal, String::from("123"));

    assert_eq!(tokens[2].r#type, TokenType::Dot);
    assert_eq!(tokens[3].r#type, TokenType::Dot);

    assert_eq!(tokens[4].r#type, TokenType::Number);
    assert_eq!(tokens[4].literal, String::from("45"));

    assert_eq!(tokens[5].r#type, TokenType::Eof);
    assert_eq!(errors.len(), 0);
  }

  #[test]
  fn scans_identifier_literals() {
    let mut scanner = Scanner::new(String::from("and or for"));
    let tokens = scanner.scan_tokens();
    let errors = scanner.get_errors();

    assert_eq!(tokens[0].r#type, TokenType::And);
    assert_eq!(tokens[0].literal, "and");

    assert_eq!(tokens[1].r#type, TokenType::Or);
    assert_eq!(tokens[1].literal, "or");

    assert_eq!(tokens[2].r#type, TokenType::For);
    assert_eq!(tokens[2].literal, "for");

    assert_eq!(tokens[3].r#type, TokenType::Eof);
    assert_eq!(errors.len(), 0);
  }
}

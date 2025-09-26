use super::TokenType;

#[derive(Clone)]

pub struct Token {
  pub r#type: TokenType,
  pub lexeme: String,
  pub literal: String,
  pub line: u32
}

impl ::core::fmt::Display for Token {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::core::fmt::Result {
    write!(f, "{} {} {}", self.r#type.to_string(), self.lexeme, self.literal)
  }
}

#[cfg(test)] 
mod tests {
  use super::*;

  #[test]
  fn test_token_serialization() {
    let tok = Token{r#type: TokenType::Bang, lexeme: String::from("lexeme"), literal: String::from("literal"), line: 23};
    assert_eq!(tok.to_string(), "Bang lexeme literal")
  }
}
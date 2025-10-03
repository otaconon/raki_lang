use super::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralType {
  String(String), I32(i32), F32(f32), Bool(bool), None
}

impl ::core::fmt::Display for LiteralType {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::core::fmt::Result {
    match self {
      LiteralType::String(val) => write!(f, "{}", val),
      LiteralType::I32(val) => write!(f, "{}", val),
      LiteralType::F32(val) => write!(f, "{}", val),
      LiteralType::Bool(val) => write!(f, "{}", val),
      LiteralType::None => write!(f, "nil")
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
  pub r#type: TokenType,
  pub lexeme: String,
  pub literal: LiteralType,
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
    let tok = Token{r#type: TokenType::Bang, lexeme: String::from("lexeme"), literal: LiteralType::String("literal".to_owned()), line: 23};
    assert_eq!(tok.to_string(), "Bang lexeme literal")
  }
}
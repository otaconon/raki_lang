macro_rules! stringify_enum {
  ($name:ident {$($variant:ident),* $(,)?} ) => {
    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    pub enum $name {
      $($variant,)*
    }

    impl $name {
      #[inline]
      pub fn as_str(&self) -> &'static str {
        match self {
          $(Self::$variant => stringify!($variant), )*
        }
      }
    }

    
    impl ::core::fmt::Display for $name {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_str(self.as_str())
      }
    }

    impl ::core::convert::AsRef<str> for $name {
      #[inline]
      fn as_ref(&self) -> &str { 
        self.as_str() 
      }
    }
  }
}


stringify_enum!(TokenType {
  // Single character tokens
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two characters tokens
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof, Ignore
});

use TokenType::*;

impl TokenType {
  pub fn from_char(c: char) -> Option<TokenType> {
    match c {
      '(' => Some(LeftParen),
      ')' => Some(RightParen),
      '{' => Some(LeftBrace),
      '}' => Some(RightBrace),
      ',' => Some(Comma),
      '.' => Some(Dot),
      '-' => Some(Minus),
      '+' => Some(Plus),
      ';' => Some(Semicolon),
      '/' => Some(Slash),
      '*' => Some(Star),
      '!' => Some(Bang),
      '=' => Some(Equal),
      '<' => Some(Less),
      '>' => Some(Greater),
      ' ' | '\r' | '\t' => Some(Ignore),
      _ => None,
    }
  }

  pub fn get_2char_extension(&self, c: char) -> Option<TokenType> {
    if c != '=' {
      return None;
    }

    match *self {
      Bang => Some(BangEqual),
      Equal => Some(EqualEqual),
      Greater => Some(GreaterEqual),
      Less => Some(LessEqual),
      _ => None,
    }
  }
}
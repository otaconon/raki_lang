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
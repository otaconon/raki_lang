use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div};

use crate::raki_log::RakiError;

#[derive(Debug, Clone)]
pub enum Object {
  Double(f64),
  String(String),
  Boolean(bool),
  None,
}

impl Add for Object {
  type Output = Result<Object, RakiError>;

  fn add(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Object::Double(a), Object::Double(b)) => Ok(Object::Double(a + b)),
      (Object::String(a), Object::String(b)) => Ok(Object::String(a + &b)),
      _ => Err(RakiError::Runtime {  })
    }
  }
}

impl Sub for Object {
  type Output = Result<Object, RakiError>;

  fn sub(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Object::Double(a), Object::Double(b)) => Ok(Object::Double(a - b)),
      _ => Err(RakiError::Runtime {  })
    }
  }
}

impl Mul for Object {
  type Output = Result<Object, RakiError>;

  fn mul(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Object::Double(a), Object::Double(b)) => Ok(Object::Double(a * b)),
      _ => Err(RakiError::Runtime {  })
    }
  }
}

impl Div for Object {
  type Output = Result<Object, RakiError>;

  fn div(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Object::Double(a), Object::Double(b)) => Ok(Object::Double(a / b)),
      _ => Err(RakiError::Runtime {  })
    }
  }
}

impl PartialEq for Object {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Object::Double(a), Object::Double(b)) => a == b,
      (Object::String(a), Object::String(b)) => a == b,
      (Object::Boolean(a), Object::Boolean(b)) => a == b,
      (Object::None, Object::None) => true,
      _ => false,
    }
  }
}

impl PartialOrd for Object {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (Object::Double(a), Object::Double(b)) => a.partial_cmp(b),
      (Object::String(a), Object::String(b)) => Some(a.cmp(b)),
      (Object::Boolean(a), Object::Boolean(b)) => Some(a.cmp(b)),
      (Object::None, Object::None) => Some(Ordering::Equal),
      _ => None
    }
  }
}

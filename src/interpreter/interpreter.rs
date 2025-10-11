use crate::lexer::TokenType;
use crate::raki_log::{RakiError, raki_log};
use crate::{
  lexer::{LiteralType, Token},
  parser::{Expr, Visitor},
};

use super::Object;

pub struct Interpreter {}

impl Interpreter {
  fn error(&self) -> RakiError {
    let err: RakiError;
    err = RakiError::Runtime {};
    raki_log(&err);
    err
  }
}

impl Interpreter {
  fn visit_binary_expr(&self, left: &Expr, right: &Expr, operator: &Token) -> Result<Object, RakiError> {
    let left: Object = self.visit_expr(left)?;
    let right: Object = self.visit_expr(right)?;

    match operator.r#type {
      TokenType::Plus => return left + right,
      TokenType::Minus => return left - right,
      TokenType::Star => return left * right,
      TokenType::Slash => return left / right,
      TokenType::Greater => return Ok(Object::Boolean(left > right)),
      TokenType::GreaterEqual => return Ok(Object::Boolean(left >= right)),
      TokenType::Less => return Ok(Object::Boolean(left < right)),
      TokenType::LessEqual => return Ok(Object::Boolean(left <= right)),
      TokenType::BangEqual => return Ok(Object::Boolean(left != right)),
      TokenType::EqualEqual => return Ok(Object::Boolean(left == right)),
      _ => return Ok(Object::None),
    }
  }

  fn visit_literal_expr(&self, lit: &LiteralType) -> Result<Object, RakiError> {
    match lit {
      LiteralType::String(s) => return Ok(Object::String(s.clone())),
      LiteralType::F64(val) => return Ok(Object::Double(*val)),
      LiteralType::Bool(val) => return Ok(Object::Boolean(*val)),
      LiteralType::None => return Ok(Object::None),
    }
  }

  fn visit_unary_expr(&self, right: &Expr, operator: &Token) -> Result<Object, RakiError> {
    let right = self.visit_expr(right)?;
    match operator.r#type {
      TokenType::Minus => match right {
        Object::Double(val) => return Ok(Object::Double(-val)),
        _ => return Err(self.error()),
      },
      TokenType::Bang => match right {
        Object::Boolean(val) => return Ok(Object::Boolean(!val)),
        Object::None => return Ok(Object::Boolean(false)),
        _ => return Err(self.error()),
      },
      _ => return Ok(Object::None),
    }
  }

  fn visit_ternary_expr(&self, condition: &Expr, left: &Expr, right: &Expr) -> Result<Object, RakiError> {
    if let Object::Boolean(obj) = self.visit_expr(condition)? {
      match obj {
        true => return self.visit_expr(left),
        false => return self.visit_expr(right)
      }
    }

    return Err(self.error());
  }
}

impl Visitor<Result<Object, RakiError>> for Interpreter {
  fn visit_expr(&self, e: &Expr) -> Result<Object, RakiError> {
    match e {
      Expr::Binary { left, right, operator } => return self.visit_binary_expr(left, right, operator),
      Expr::Grouping { expr } => return self.visit_expr(&expr),
      Expr::Literal { value } => return self.visit_literal_expr(value),
      Expr::Unary { right, operator } => return self.visit_unary_expr(right, operator),
      Expr::Ternary { condition, left, right } => return self.visit_ternary_expr(condition, left, right),
    }
  }
}

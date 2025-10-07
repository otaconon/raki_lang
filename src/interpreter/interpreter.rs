use crate::lexer::TokenType;
use crate::raki_log::{RakiError, raki_log};
use crate::{
  lexer::{LiteralType, Token},
  parser::{AstPrinter, Expr, Visitor},
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

impl Visitor<Result<Object, RakiError>> for Interpreter {
  fn visit_expr(&self, e: &Expr) -> Result<Object, RakiError> {
    match e {
      Expr::Binary { left, right, operator } => {
        let left: Object = self.visit_expr(left)?;
        let right: Object = self.visit_expr(right)?;

        let (left, right) = match (left, right) {
          (Object::Double(l), Object::Double(r)) => (l, r),
          _ => return Err(RakiError::Runtime {  })
        };
        
        match operator.r#type {
          TokenType::Plus => return Ok(Object::Double(left + right)), // TODO: overload '+' operator for strings
          TokenType::Minus => return Ok(Object::Double(left - right)),
          TokenType::Star => return Ok(Object::Double(left * right)),
          TokenType::Slash => return Ok(Object::Double(left / right)),
          TokenType::Greater => return Ok(Object::Boolean(left > right)),
          TokenType::GreaterEqual => return Ok(Object::Boolean(left >= right)),
          TokenType::Less => return Ok(Object::Boolean(left < right)),
          TokenType::LessEqual => return Ok(Object::Boolean(left <= right)),
          TokenType::BangEqual => return Ok(Object::Boolean(left != right)), // TODO: abstract != and == handling to a fucntion that handles other values than Double
          TokenType::EqualEqual => return Ok(Object::Boolean(left == right)),
          _ => return Ok(Object::None)
        }
      },
      Expr::Grouping { expr } => return self.visit_expr(&expr),
      Expr::Literal { value } => match value {
        LiteralType::String(s) => return Ok(Object::String(s.clone())),
        LiteralType::I64(val) | LiteralType::F64(val) => return Ok(Object::Double(*val)),
        LiteralType::Bool(val) => return Ok(Object::Boolean(*val)),
        LiteralType::None => return Ok(Object::None),
      },
      Expr::Unary { right, operator } => {
        let right = self.visit_expr(right)?;
        match operator.r#type {
          TokenType::Minus => match right {
            Object::Double(val) => return Ok(Object::Double(-val)),
            _ => return Err(RakiError::Runtime {}),
          },
          TokenType::Bang => match right {
            Object::Boolean(val) => return Ok(Object::Boolean(!val)),
            Object::None => return Ok(Object::Boolean(false)),
            _ => return Err(RakiError::Runtime {}),
          },
          _ => return Ok(Object::None),
        }
      }
      Expr::Ternary { condition, left, right } => return Ok(Object::None),
    }
  }
}

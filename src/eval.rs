use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::value::Value;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum EvalError {
    #[error("undefined variable: {0}")]
    UndefinedVariable(String),

    #[error("bad operand type")]
    BadOperandType,
}

#[derive(Debug, Clone)]
pub struct Environment {
    variables: im::HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: im::HashMap::new(),
        }
    }

    pub fn with_variable(&self, name: impl Into<String>, value: Value) -> Environment {
        Self {
            variables: self.variables.update(name.into(), value),
        }
    }
}

pub fn eval(env: &Environment, expr: &Expr) -> Result<Value, EvalError> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(Arc::new(s.clone()))),
        Expr::UnaryOp(op, expr) => {
            let v = eval(env, expr)?;
            match op {
                UnaryOp::Neg => {
                    let Value::Number(n) = v else {
                        return Err(EvalError::BadOperandType);
                    };
                    Ok(Value::Number(-n))
                }
            }
        }
        Expr::BinaryOp(op, lhs, rhs) => {
            let Value::Number(l) = eval(env, lhs)? else {
                return Err(EvalError::BadOperandType);
            };
            let Value::Number(r) = eval(env, rhs)? else {
                return Err(EvalError::BadOperandType);
            };
            match op {
                BinaryOp::Add => Ok(Value::Number(l + r)),
                BinaryOp::Sub => Ok(Value::Number(l - r)),
                BinaryOp::Mul => Ok(Value::Number(l * r)),
                BinaryOp::Div => Ok(Value::Number(l / r)),
            }
        }
        Expr::Variable(name) => match env.variables.get(name) {
            Some(v) => Ok(v.clone()),
            None => Err(EvalError::UndefinedVariable(name.to_owned())),
        },
        Expr::Let(name, expr1, expr2) => {
            let v = eval(env, expr1)?;
            let new_env = env.with_variable(name, v);
            eval(&new_env, expr2)
        }
    }
}

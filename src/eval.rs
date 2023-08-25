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
            let l = eval(env, lhs)?;
            let r = eval(env, rhs)?;
            match op {
                BinaryOp::Add => op_add(l, r),
                BinaryOp::Sub => op_sub(l, r),
                BinaryOp::Mul => op_mul(l, r),
                BinaryOp::Div => op_div(l, r),
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

fn op_add(l: Value, r: Value) -> Result<Value, EvalError> {
    match (l, r) {
        (Value::Number(l), Value::Number(r)) =>
            Ok(Value::Number(l + r)),
        (Value::String(l), Value::String(r)) => {
            let res = (*l).clone() + &r;
            Ok(Value::String(Arc::new(res)))
        }
        _ => Err(EvalError::BadOperandType),
    }
}

fn op_sub(l: Value, r: Value) -> Result<Value, EvalError> {
    match (l, r) {
        (Value::Number(l), Value::Number(r)) =>
            Ok(Value::Number(l - r)),
        _ => Err(EvalError::BadOperandType),
    }
}

fn op_mul(l: Value, r: Value) -> Result<Value, EvalError> {
    match (l, r) {
        (Value::Number(l), Value::Number(r)) =>
            Ok(Value::Number(l * r)),
        (Value::String(l), Value::Number(r)) => {
            let res = (*l).clone().repeat(r as usize);
            Ok(Value::String(Arc::new(res)))
        }
        _ => Err(EvalError::BadOperandType),
    }
}

fn op_div(l: Value, r: Value) -> Result<Value, EvalError> {
    match (l, r) {
        (Value::Number(l), Value::Number(r)) =>
            Ok(Value::Number(l / r)),
        _ => Err(EvalError::BadOperandType),
    }
}

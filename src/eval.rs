use crate::ast::{Expr, UnaryOp, BinaryOp};

type Value = f64;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum EvalError {
    #[error("undefined variable: {0}")]
    UndefinedVariable(String),
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
        Expr::Number(n) => Ok(*n),
        Expr::UnaryOp(op, expr) => {
            let v = eval(env, expr)?;
            match op {
                UnaryOp::Neg => Ok(-v),
            }
        }
        Expr::BinaryOp(op, lhs, rhs) => {
            let l = eval(env, lhs)?;
            let r = eval(env, rhs)?;
            match op {
                BinaryOp::Add => Ok(l + r),
                BinaryOp::Sub => Ok(l - r),
                BinaryOp::Mul => Ok(l * r),
                BinaryOp::Div => Ok(l / r),
            }
        }
        Expr::Variable(name) => {
            match env.variables.get(name) {
                Some(v) => Ok(*v),
                None => Err(EvalError::UndefinedVariable(name.to_owned())),
            }
        },
        Expr::Let(name, expr1, expr2) => {
            let v = eval(env, expr1)?;
            let new_env = env.with_variable(name, v);
            eval(&new_env, expr2)
        }
    }
}

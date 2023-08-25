use crate::ast::{BinaryOp, Expr, UnaryOp, Stmt};
use crate::value::Value;
use compact_str::CompactString;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum EvalError {
    #[error("undefined variable: {0}")]
    UndefinedVariable(CompactString),

    #[error("bad operand type")]
    BadOperandType,
}

#[derive(Debug, Clone)]
pub struct Environment {
    variables: im::HashMap<CompactString, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: im::HashMap::new(),
        }
    }

    pub fn with_variable(&self, name: CompactString, value: Value) -> Environment {
        Self {
            variables: self.variables.update(name, value),
        }
    }
}

pub fn eval_program(env: &Environment, program: &[Stmt]) -> Result<Vec<Value>, EvalError> {
    let mut env = env.clone();
    let mut values = Vec::new();
    for stmt in program {
        let (value, new_env) = eval_stmt(&env, stmt)?;
        values.push(value);
        env = new_env;
    }
    Ok(values)
}

pub fn eval_stmt(env: &Environment, stmt: &Stmt) -> Result<(Value, Environment), EvalError> {
    match stmt {
        Stmt::Expr(expr) => {
            let value = eval_expr(env, &expr)?;
            Ok((value, env.clone()))
        }
    }
}

pub fn eval_expr(env: &Environment, expr: &Expr) -> Result<Value, EvalError> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(Arc::new(s.clone()))),
        Expr::UnaryOp(op, expr) => {
            let v = eval_expr(env, expr)?;
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
            let l = eval_expr(env, lhs)?;
            let r = eval_expr(env, rhs)?;
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
            let v = eval_expr(env, expr1)?;
            let new_env = env.with_variable(name.to_owned(), v);
            eval_expr(&new_env, expr2)
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
            Ok(Value::String(Arc::new(res.into())))
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

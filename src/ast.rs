use std::fmt::{self, Debug, Formatter};
use compact_str::CompactString;

#[derive(Clone)]
pub enum Stmt {
    Expr(Expr),
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Stmt::Expr(expr) => writeln!(f, "{expr:?};"),
        }
    }
}

#[derive(Clone)]
pub enum Expr {
    Number(f64),
    String(CompactString),
    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
    Variable(CompactString),
    Let(CompactString, Box<Expr>, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{n:?}"),
            Expr::String(s) => write!(f, "{s:?}"),
            Expr::UnaryOp(op, expr) => write!(f, "{op:?}{expr:?}"),
            Expr::BinaryOp(op, lhs, rhs) => write!(f, "({lhs:?} {op:?} {rhs:?})"),
            Expr::Variable(ref name) => write!(f, "{name}"),
            Expr::Let(name, expr1, expr2) => write!(f, "(let {name} = {expr1:?} in {expr2:?})"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
}

impl Debug for UnaryOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Debug for BinaryOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
        }
    }
}

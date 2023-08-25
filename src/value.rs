use crate::ast::Expr;
use compact_str::CompactString;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Number(f64),
    String(Arc<CompactString>),
    Function(CompactString, Vec<CompactString>, Box<Expr>),
}

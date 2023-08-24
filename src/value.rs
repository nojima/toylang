use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(Arc<String>),
}

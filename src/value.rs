use std::sync::Arc;
use compact_str::CompactString;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(Arc<CompactString>),
}

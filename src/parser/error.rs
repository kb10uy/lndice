use std::{fmt::Display, ops::Range};

use chumsky::error::Rich;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorElement {
    pub message: String,
    pub reason: String,
    pub span: Range<usize>,
}

impl ErrorElement {
    pub fn from_rich<T: Display>(err: Rich<'_, T>) -> ErrorElement {
        ErrorElement {
            message: err.to_string(),
            reason: err.reason().to_string(),
            span: err.span().into_range(),
        }
    }
}

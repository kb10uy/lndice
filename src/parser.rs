mod ast;
mod error;
mod expr;
mod lexer;
pub use error::ErrorElement;

use chumsky::prelude::*;

use crate::types::expr::Expr;

pub fn parse(source: &str) -> Result<Expr, Vec<ErrorElement>> {
    let length = source.len();
    let lexer_tokens = lexer::lex_tokens().parse(source).into_result().map_err(|errs| {
        let lexer_errors: Vec<_> = errs.into_iter().map(ErrorElement::from_rich).collect();
        lexer_errors
    })?;

    let mapped_tokens = lexer_tokens.map((length..length).into(), |(t, s)| (t, s));
    let (ast_expr, _) = expr::expr().parse(mapped_tokens).into_result().map_err(|errs| {
        let lexer_errors: Vec<_> = errs.into_iter().map(ErrorElement::from_rich).collect();
        lexer_errors
    })?;
    Ok(ast_expr.into())
}

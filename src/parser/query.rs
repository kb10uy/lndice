use chumsky::prelude::*;

use crate::{
    parser::constexpr::expr,
    types::query::{QueryKind, RangeQuery},
};

pub(super) fn range_query<'a>() -> impl Parser<'a, &'a str, RangeQuery, extra::Err<Rich<'a, char>>> {
    query_kind()
        .then(expr())
        .map(|(kind, value)| RangeQuery { kind, value })
}

pub(super) fn query_kind<'a>() -> impl Parser<'a, &'a str, QueryKind, extra::Err<Rich<'a, char>>> {
    choice((
        just(">=").map(|_| QueryKind::GreaterEqual),
        just(">").map(|_| QueryKind::Greater),
        just("<=").map(|_| QueryKind::LesserEqual),
        just("<").map(|_| QueryKind::Lesser),
        just("==").map(|_| QueryKind::Equal),
        just("=").map(|_| QueryKind::Equal),
        just("!=").map(|_| QueryKind::NotEqual),
        just("<>").map(|_| QueryKind::NotEqual),
    ))
}

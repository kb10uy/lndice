use chumsky::prelude::*;

use crate::types::{QueryKind, RangeQuery};

pub(super) fn int<'a>() -> impl Parser<'a, &'a str, usize, extra::Err<Rich<'a, char>>> {
    text::int::<_, extra::Err<Rich<char>>>(10).from_str().unwrapped()
}

pub(super) fn range_query<'a>() -> impl Parser<'a, &'a str, RangeQuery, extra::Err<Rich<'a, char>>> {
    query_kind().then(int()).map(|(kind, value)| RangeQuery { kind, value })
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

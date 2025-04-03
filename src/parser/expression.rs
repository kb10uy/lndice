use chumsky::prelude::*;

pub(super) fn int<'a>() -> impl Parser<'a, &'a str, usize> {
    text::int(10).from_str().unwrapped()
}

use chumsky::prelude::*;

use crate::types::choice::{Choice, ChoiceItems};

pub(super) fn choice_command<'a>() -> impl Parser<'a, &'a str, Choice, extra::Err<Rich<'a, char>>> {
    just("choice")
        .then(choice((
            choice_items_brackets(),
            choice_items_parens(),
            choice_items_spaces(),
        )))
        .map(|(_, items)| Choice(items))
}

fn choice_items_brackets<'a>() -> impl Parser<'a, &'a str, ChoiceItems, extra::Err<Rich<'a, char>>> {
    // trim before collect?
    let item = none_of("[],")
        .repeated()
        .collect::<String>()
        .map(|s| s.trim().to_string());

    item.separated_by(just(','))
        .collect()
        .map(ChoiceItems::Strings)
        .delimited_by(just('['), just(']'))
}

fn choice_items_parens<'a>() -> impl Parser<'a, &'a str, ChoiceItems, extra::Err<Rich<'a, char>>> {
    // trim before collect?
    let item = none_of("(),")
        .repeated()
        .collect::<String>()
        .map(|s| s.trim().to_string());

    item.separated_by(just(','))
        .collect()
        .map(ChoiceItems::Strings)
        .delimited_by(just('['), just(']'))
}

fn choice_items_spaces<'a>() -> impl Parser<'a, &'a str, ChoiceItems, extra::Err<Rich<'a, char>>> {
    let item = none_of(" ").repeated().collect::<String>();

    just(' ').ignore_then(item.separated_by(just(' ')).collect().map(ChoiceItems::Strings))
}

mod choice;
mod constexpr;
mod dice;
mod query;

use chumsky::prelude::*;

use crate::types::Command;

pub fn parse_command(source: &str) -> Result<Command, Vec<Rich<'_, char>>> {
    command().parse(source).into_result()
}

fn command<'a>() -> impl Parser<'a, &'a str, Command, extra::Err<Rich<'a, char>>> {
    choice((single_command(), calculate_command(), repeat_command()))
}

fn single_command<'a>() -> impl Parser<'a, &'a str, Command, extra::Err<Rich<'a, char>>> {
    choice((
        dice::sum_dice().map(Command::Sum),
        dice::individual_dice().map(Command::Individual),
        dice::replay_dice().map(Command::Replay),
        dice::infinite_dice().map(Command::Infinite),
        dice::tally_dice().map(Command::Tally),
        dice::two_six_dice().map(Command::TwoSix),
        choice::choice_command().map(Command::Choice),
    ))
}

fn calculate_command<'a>() -> impl Parser<'a, &'a str, Command, extra::Err<Rich<'a, char>>> {
    (just('c').labelled("calculate prefix"))
        .ignore_then(constexpr::expr())
        .map(Command::Calculation)
}

fn repeat_command<'a>() -> impl Parser<'a, &'a str, Command, extra::Err<Rich<'a, char>>> {
    let int = text::int::<_, extra::Err<Rich<char>>>(10).from_str().unwrapped();
    let repeat_prefix = choice((just("repeat"), just("rep"), just("x"))).labelled("repeat prefix");
    let repeat_specifier = repeat_prefix.ignore_then(int.labelled("repeat count")).padded();

    repeat_specifier
        .then(single_command())
        .map(|(count, command)| Command::Repeat(count, Box::new(command)))
}

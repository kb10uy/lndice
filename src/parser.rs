mod choice;
mod constexpr;
mod dice;
mod query;
mod repeat;

use chumsky::prelude::*;

use crate::types::Command;

pub fn parse_command(source: &str) {
    command().parse(source);
}

fn command<'a>() -> impl Parser<'a, &'a str, Command, extra::Err<Rich<'a, char>>> {
    recursive(|command| {
        let int = text::int::<_, extra::Err<Rich<char>>>(10).from_str().unwrapped();
        let repeat_command = choice((just("repeat"), just("rep"), just("x")))
            .ignore_then(int)
            .then_ignore(just(' ').repeated().at_least(1))
            .then(command)
            .map(|(count, command)| Command::Repeat(count, Box::new(command)));
        let calculate_command = just('c').ignore_then(constexpr::expr()).map(Command::Calculation);

        choice((
            dice::sum_dice().map(Command::Sum),
            dice::individual_dice().map(Command::Individual),
            dice::replay_dice().map(Command::Replay),
            dice::infinite_dice().map(Command::Infinite),
            dice::tally_dice().map(Command::Tally),
            dice::two_six_dice().map(Command::TwoSix),
            choice::choice_command().map(Command::Choice),
            calculate_command,
            repeat_command,
        ))
    })
}

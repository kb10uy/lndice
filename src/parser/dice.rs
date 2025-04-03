mod individual;
mod infinite;
mod replay;
mod sum;
mod tally;
mod two_six;

use chumsky::prelude::*;

use crate::{
    parser::expression::int,
    types::{DiceCommand, DiceElement},
};

pub(super) fn dice_command<'a>() -> impl Parser<'a, &'a str, DiceCommand, extra::Err<Rich<'a, char>>> {
    choice((
        sum::sum_dice().map(DiceCommand::Sum),
        individual::individual_dice().map(DiceCommand::Individual),
        replay::replay_dice().map(DiceCommand::Replay),
        infinite::infinite_dice().map(DiceCommand::Infinite),
        tally::tally_dice().map(DiceCommand::Tally),
        two_six::two_six_dice().map(DiceCommand::TwoSix),
    ))
}

fn dice_element<'a>(symbol: char) -> impl Parser<'a, &'a str, DiceElement, extra::Err<Rich<'a, char>>> {
    (int().labelled("dice rolls"))
        .then_ignore(just(symbol))
        .then(int().labelled("dice faces"))
        .map(|(rolls, faces)| DiceElement { rolls, faces })
}

mod individual;
mod infinite;
mod replay;
mod sum;

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
    ))
}

fn dice_element<'a>(symbol: char) -> impl Parser<'a, &'a str, DiceElement, extra::Err<Rich<'a, char>>> {
    int()
        .then_ignore(just(symbol))
        .then(int())
        .map(|(rolls, faces)| DiceElement { rolls, faces })
}

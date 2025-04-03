mod expression;
mod individual_dice;
mod replay_dice;
mod sum_dice;

use chumsky::{Parser, prelude::choice};

use crate::types::DiceRoll;

pub fn parse_dices(command: &str) {
    let parser = choice((
        sum_dice::sum_dice().map(DiceRoll::Sum),
        individual_dice::individual_dice().map(DiceRoll::Individual),
    ))
    .parse(command);
}

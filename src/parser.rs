use chumsky::Parser;

mod expression;
mod sum_dice;

pub fn parse_dices(command: &str) {
    let parser = sum_dice::sum_dice();
    parser.parse(command);
}

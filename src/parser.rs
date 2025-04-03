mod dice;
mod expression;

use chumsky::Parser;

pub fn parse_command(command: &str) {
    dice::dice_command().parse(command);
}

mod constexpr;
mod dice;
mod query;

use chumsky::Parser;

pub fn parse_command(command: &str) {
    dice::dice_command().parse(command);
}

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
    choice((
        single_command(),
        calculate_command(),
        repeat_command(),
    ))
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

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::{
        Command,
        constexpr::ConstExpr,
        dice::{DiceElement, SumDice, SumDiceElement, SumDiceExpr},
    };

    use super::command;

    #[test]
    fn repeat_parses() {
        let parser = command();
        assert_eq!(
            parser.parse("repeat5 2D6").into_result(),
            Ok(Command::Repeat(
                5,
                Box::new(Command::Sum(SumDice {
                    expression: SumDiceExpr::Element(SumDiceElement {
                        element: DiceElement {
                            rolls: 2.into(),
                            faces: 6.into(),
                        },
                        pick: None,
                    }),
                    target_query: None,
                }))
            ))
        );
        assert_eq!(
            parser.parse("c2+3*4").into_result(),
            Ok(Command::Calculation(ConstExpr::Add(
                Box::new(2.into()),
                Box::new(ConstExpr::Multiply(Box::new(3.into()), Box::new(4.into()))),
            )))
        );
    }
}

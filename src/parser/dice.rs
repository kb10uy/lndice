mod individual;
mod infinite;
mod replay;
mod sum;
mod tally;
mod two_six;

use chumsky::prelude::*;

use crate::{
    parser::constexpr::term,
    types::dice::{DiceCommand, DiceElement},
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

fn dice_element<'a>(symbol: char) -> impl Parser<'a, &'a str, DiceElement, extra::Err<Rich<'a, char>>> + Clone {
    (term().labelled("dice rolls"))
        .then_ignore(just(symbol))
        .then(term().labelled("dice faces"))
        .map(|(rolls, faces)| DiceElement { rolls, faces })
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::{constexpr::ConstExpr, dice::DiceElement};

    use super::dice_element;

    #[test]
    fn dice_element_parses_basic() {
        let parser = dice_element('X');
        assert_eq!(
            parser.parse("1X2").into_result(),
            Ok(DiceElement {
                rolls: 1.into(),
                faces: 2.into(),
            })
        );
        assert_eq!(
            parser.parse("10X20").into_result(),
            Ok(DiceElement {
                rolls: 10.into(),
                faces: 20.into(),
            })
        );
    }

    #[test]
    fn dice_element_parses_terms() {
        let parser = dice_element('T');
        assert_eq!(
            parser.parse("(1)T(2)").into_result(),
            Ok(DiceElement {
                rolls: 1.into(),
                faces: 2.into(),
            })
        );
        assert_eq!(
            parser.parse("(5*2)T(100/5)").into_result(),
            Ok(DiceElement {
                rolls: ConstExpr::Multiply(Box::new(5.into()), Box::new(2.into())),
                faces: ConstExpr::Divide(Box::new(100.into()), Box::new(5.into()), None),
            })
        );
    }
}

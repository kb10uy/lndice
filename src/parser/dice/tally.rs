use chumsky::prelude::*;

use crate::{
    parser::constexpr::term,
    types::dice::{DiceElement, TallyDice},
};

pub fn tally_dice<'a>() -> impl Parser<'a, &'a str, TallyDice, extra::Err<Rich<'a, char>>> + Clone {
    (term().labelled("dice rolls"))
        .then_ignore(just('T'))
        .then(one_of("YZ").labelled("tally specifier"))
        .then(term().labelled("dice faces"))
        .map(|((rolls, tally), faces)| TallyDice {
            element: DiceElement { rolls, faces },
            with_zero: tally == 'Z',
        })
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::dice::{DiceElement, TallyDice};

    use super::tally_dice;

    #[test]
    fn tally_dice_parses_basic() {
        let parser = tally_dice();
        assert_eq!(
            parser.parse("4TY20").into_result(),
            Ok(TallyDice {
                element: DiceElement {
                    rolls: 4.into(),
                    faces: 20.into(),
                },
                with_zero: false,
            })
        );
        assert_eq!(
            parser.parse("10TZ20").into_result(),
            Ok(TallyDice {
                element: DiceElement {
                    rolls: 10.into(),
                    faces: 20.into(),
                },
                with_zero: true,
            })
        );
    }
}

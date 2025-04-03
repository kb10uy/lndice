use chumsky::prelude::*;

use crate::types::IndividualDice;

use super::expression::int;

pub(super) fn individual_dice<'a>() -> impl Parser<'a, &'a str, IndividualDice, extra::Err<Rich<'a, char>>> {
    int()
        .then_ignore(just('B'))
        .then(int())
        .map(|(rolls, faces)| IndividualDice { rolls, faces })
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::IndividualDice;

    use super::individual_dice;

    #[test]
    fn individual_dice_parses_basic() {
        let parser = individual_dice();
        assert_eq!(
            parser.parse("2B6").into_result(),
            Ok(IndividualDice { rolls: 2, faces: 6 })
        );
    }
}

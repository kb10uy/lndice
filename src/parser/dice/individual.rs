use chumsky::prelude::*;

use crate::{parser::dice::dice_element, types::IndividualDice};

pub(super) fn individual_dice<'a>() -> impl Parser<'a, &'a str, IndividualDice, extra::Err<Rich<'a, char>>> {
    dice_element('B').map(IndividualDice)
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::{DiceElement, IndividualDice};

    use super::individual_dice;

    #[test]
    fn individual_dice_parses_basic() {
        let parser = individual_dice();
        assert_eq!(
            parser.parse("2B6").into_result(),
            Ok(IndividualDice(DiceElement { rolls: 2, faces: 6 }))
        );
    }
}

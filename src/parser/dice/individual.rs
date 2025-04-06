use chumsky::prelude::*;

use crate::{
    parser::{dice::dice_element, query::range_query},
    types::dice::IndividualDice,
};

pub fn individual_dice<'a>() -> impl Parser<'a, &'a str, IndividualDice, extra::Err<Rich<'a, char>>> + Clone {
    let elements = dice_element('B')
        .separated_by(just('+'))
        .collect()
        .labelled("dice roll elements");
    let query = range_query().labelled("query").or_not();

    elements
        .then(query)
        .map(|(elements, target_query)| IndividualDice { elements, target_query })
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::dice::{DiceElement, IndividualDice};

    use super::individual_dice;

    #[test]
    fn individual_dice_parses_basic() {
        let parser = individual_dice();
        assert_eq!(
            parser.parse("2B6").into_result(),
            Ok(IndividualDice {
                elements: vec![DiceElement {
                    rolls: 2.into(),
                    faces: 6.into(),
                }],
                target_query: None,
            })
        );
    }
}

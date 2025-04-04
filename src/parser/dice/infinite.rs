use chumsky::prelude::*;

use crate::{
    parser::{
        dice::dice_element,
        query::{int, range_query},
    },
    types::dice::InfiniteDice,
};

pub(super) fn infinite_dice<'a>() -> impl Parser<'a, &'a str, InfiniteDice, extra::Err<Rich<'a, char>>> {
    let elements = dice_element('U')
        .separated_by(just('+'))
        .collect()
        .labelled("dice roll elements");
    let threshold = surrounded_threshold().labelled("infinite roll threshold").or_not();
    let bias = just('+')
        .then(int())
        .map(|(_, v)| v)
        .labelled("infinite roll bias")
        .or_not();
    let query = range_query().labelled("query").or_not();

    elements
        .then(threshold)
        .then(bias)
        .then(query)
        .map(|(((elements, threshold), bias), target_query)| InfiniteDice {
            elements,
            threshold,
            bias,
            target_query,
        })
}

fn surrounded_threshold<'a>() -> impl Parser<'a, &'a str, usize, extra::Err<Rich<'a, char>>> {
    int().delimited_by(just('['), just(']'))
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::{
        dice::{DiceElement, InfiniteDice},
        query::{QueryKind, RangeQuery},
    };

    use super::infinite_dice;

    #[test]
    fn infinite_dice_parses_basic() {
        let parser = infinite_dice();
        assert_eq!(
            parser.parse("2U4+1U6[4]+2>=6").into_result(),
            Ok(InfiniteDice {
                elements: vec![DiceElement { rolls: 2, faces: 4 }, DiceElement { rolls: 1, faces: 6 }],
                threshold: Some(4),
                bias: Some(2),
                target_query: Some(RangeQuery {
                    kind: QueryKind::GreaterEqual,
                    value: 6,
                })
            })
        );
    }
}

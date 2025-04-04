use chumsky::prelude::*;

use crate::{
    parser::{
        constexpr::{expr, term},
        dice::dice_element,
        query::range_query,
    },
    types::{constexpr::ConstExpr, dice::InfiniteDice},
};

pub fn infinite_dice<'a>() -> impl Parser<'a, &'a str, InfiniteDice, extra::Err<Rich<'a, char>>> + Clone {
    let elements = dice_element('U')
        .separated_by(just('+'))
        .collect()
        .labelled("dice roll elements");
    let threshold = surrounded_threshold().labelled("infinite roll threshold").or_not();
    let bias = just('+').ignore_then(term()).labelled("infinite roll bias").or_not();
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

fn surrounded_threshold<'a>() -> impl Parser<'a, &'a str, ConstExpr, extra::Err<Rich<'a, char>>> + Clone {
    expr().delimited_by(just('['), just(']'))
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
                elements: vec![
                    DiceElement {
                        rolls: 2.into(),
                        faces: 4.into(),
                    },
                    DiceElement {
                        rolls: 1.into(),
                        faces: 6.into(),
                    }
                ],
                threshold: Some(4.into()),
                bias: Some(2.into()),
                target_query: Some(RangeQuery {
                    kind: QueryKind::GreaterEqual,
                    value: 6.into(),
                })
            })
        );
    }
}

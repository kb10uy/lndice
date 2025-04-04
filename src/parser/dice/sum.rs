use chumsky::{
    pratt::{infix, left, postfix},
    prelude::*,
};

use crate::{
    parser::{
        constexpr::{fraction_mode, term},
        dice::dice_element,
        query::range_query,
    },
    types::dice::{SumDice, SumDiceElement, SumDiceExpr, SumDicePick},
};

pub(super) fn sum_dice<'a>() -> impl Parser<'a, &'a str, SumDice, extra::Err<Rich<'a, char>>> {
    let elements = sum_dice_expr().labelled("sum dice expression");
    let query = range_query().labelled("query").or_not();

    elements.then(query).map(|(elements, target_query)| SumDice {
        expression: elements,
        target_query,
    })
}

fn sum_dice_expr<'a>() -> impl Parser<'a, &'a str, SumDiceExpr, extra::Err<Rich<'a, char>>> {
    recursive(|expr| {
        let term = (sum_dice_element()
            .map(SumDiceExpr::Element)
            .labelled("sum dice element"))
        .or(sum_dice_int())
        .or(expr.delimited_by(just('('), just(')')));

        term.pratt((
            infix(left(3), just('*'), |x, _, y, _| {
                SumDiceExpr::Multiply(Box::new(x), Box::new(y))
            }),
            infix(left(3), just('/'), |x, _, y, _| {
                SumDiceExpr::Divide(Box::new(x), Box::new(y), None)
            }),
            infix(left(2), just('+'), |x, _, y, _| {
                SumDiceExpr::Add(Box::new(x), Box::new(y))
            }),
            infix(left(2), just('-'), |x, _, y, _| {
                SumDiceExpr::Subtract(Box::new(x), Box::new(y))
            }),
            postfix(1, fraction_mode(), |expr, f, _| match expr {
                SumDiceExpr::Divide(x, y, _) => SumDiceExpr::Divide(x, y, Some(f)),
                _ => expr,
            }),
        ))
    })
}

fn sum_dice_int<'a>() -> impl Parser<'a, &'a str, SumDiceExpr, extra::Err<Rich<'a, char>>> + Clone {
    text::int::<_, extra::Err<Rich<char>>>(10)
        .from_str()
        .unwrapped()
        .map(SumDiceExpr::Number)
}

fn sum_dice_element<'a>() -> impl Parser<'a, &'a str, SumDiceElement, extra::Err<Rich<'a, char>>> + Clone {
    dice_element('D')
        .then(sum_dice_pick().labelled("sum pick specifier").or_not())
        .map(|(element, pick)| SumDiceElement { element, pick })
}

fn sum_dice_pick<'a>() -> impl Parser<'a, &'a str, SumDicePick, extra::Err<Rich<'a, char>>> + Clone {
    let kh = just("KH").ignore_then(term()).map(SumDicePick::KeepHighest);
    let kl = just("KL").ignore_then(term()).map(SumDicePick::KeepLowest);
    let dh = just("DH").ignore_then(term()).map(SumDicePick::DropHighest);
    let dl = just("DL").ignore_then(term()).map(SumDicePick::DropLowest);
    let max = just("MAX").map(|_| SumDicePick::KeepHighest(1.into()));
    let min = just("MIN").map(|_| SumDicePick::KeepLowest(1.into()));
    choice((kh, kl, dh, dl, max, min))
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::{
        constexpr::ConstExpr,
        dice::{DiceElement, SumDice, SumDiceElement, SumDiceExpr, SumDicePick},
        query::{QueryKind, RangeQuery},
    };

    use super::{sum_dice, sum_dice_element};

    #[test]
    fn sum_dice_parses() {
        let parser = sum_dice();
        assert_eq!(
            parser.parse("2D6+2>=7+4").into_result(),
            Ok(SumDice {
                expression: SumDiceExpr::Add(
                    Box::new(SumDiceExpr::Element(SumDiceElement {
                        element: DiceElement {
                            rolls: 2.into(),
                            faces: 6.into(),
                        },
                        pick: None,
                    })),
                    Box::new(SumDiceExpr::Number(2)),
                ),
                target_query: Some(RangeQuery {
                    kind: QueryKind::GreaterEqual,
                    value: ConstExpr::Add(Box::new(7.into()), Box::new(4.into())),
                }),
            })
        );
    }

    #[test]
    fn sum_dice_parses_pick() {
        let parser = sum_dice_element();
        assert_eq!(
            parser.parse("5D6KH3").into_result(),
            Ok(SumDiceElement {
                element: DiceElement {
                    rolls: 5.into(),
                    faces: 6.into(),
                },
                pick: Some(SumDicePick::KeepHighest(3.into(),)),
            })
        );
        assert_eq!(
            parser.parse("10D20KL10").into_result(),
            Ok(SumDiceElement {
                element: DiceElement {
                    rolls: 10.into(),
                    faces: 20.into(),
                },
                pick: Some(SumDicePick::KeepLowest(10.into(),)),
            })
        );
        assert_eq!(
            parser.parse("3D4DH1").into_result(),
            Ok(SumDiceElement {
                element: DiceElement {
                    rolls: 3.into(),
                    faces: 4.into(),
                },
                pick: Some(SumDicePick::DropHighest(1.into(),)),
            })
        );
        assert_eq!(
            parser.parse("5D12DL2").into_result(),
            Ok(SumDiceElement {
                element: DiceElement {
                    rolls: 5.into(),
                    faces: 12.into(),
                },
                pick: Some(SumDicePick::DropLowest(2.into(),)),
            })
        );
        assert_eq!(
            parser.parse("3D6MAX").into_result(),
            Ok(SumDiceElement {
                element: DiceElement {
                    rolls: 3.into(),
                    faces: 6.into(),
                },
                pick: Some(SumDicePick::KeepHighest(1.into(),)),
            })
        );
        assert_eq!(
            parser.parse("4D8MIN").into_result(),
            Ok(SumDiceElement {
                element: DiceElement {
                    rolls: 4.into(),
                    faces: 8.into(),
                },
                pick: Some(SumDicePick::KeepLowest(1.into(),)),
            })
        );
    }
}

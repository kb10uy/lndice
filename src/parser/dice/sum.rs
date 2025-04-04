use chumsky::prelude::*;

use crate::{
    parser::{constexpr::term, dice::dice_element},
    types::dice::{SumDice, SumDicePick},
};

pub(super) fn sum_dice<'a>() -> impl Parser<'a, &'a str, SumDice, extra::Err<Rich<'a, char>>> {
    dice_element('D')
        .then(sum_dice_pick().labelled("sum pick specifier").or_not())
        .map(|(element, pick)| SumDice { element, pick })
}

fn sum_dice_pick<'a>() -> impl Parser<'a, &'a str, SumDicePick, extra::Err<Rich<'a, char>>> {
    let kh = just("KH").then(term()).map(|(_, v)| SumDicePick::KeepHighest(v));
    let kl = just("KL").then(term()).map(|(_, v)| SumDicePick::KeepLowest(v));
    let dh = just("DH").then(term()).map(|(_, v)| SumDicePick::DropHighest(v));
    let dl = just("DL").then(term()).map(|(_, v)| SumDicePick::DropLowest(v));
    let max = just("MAX").map(|_| SumDicePick::KeepHighest(1.into()));
    let min = just("MIN").map(|_| SumDicePick::KeepLowest(1.into()));
    choice((kh, kl, dh, dl, max, min))
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::dice::{DiceElement, SumDice, SumDicePick};

    use super::sum_dice;

    #[test]
    fn sum_dice_parses_basic() {
        let parser = sum_dice();
        assert_eq!(
            parser.parse("1D6").into_result(),
            Ok(SumDice {
                element: DiceElement {
                    rolls: 1.into(),
                    faces: 6.into(),
                },
                pick: None
            })
        );
        assert_eq!(
            parser.parse("2D10").into_result(),
            Ok(SumDice {
                element: DiceElement {
                    rolls: 2.into(),
                    faces: 10.into(),
                },
                pick: None
            })
        );
        assert_eq!(
            parser.parse("10D4").into_result(),
            Ok(SumDice {
                element: DiceElement {
                    rolls: 10.into(),
                    faces: 4.into(),
                },
                pick: None
            })
        );
        assert_eq!(
            parser.parse("20D20").into_result(),
            Ok(SumDice {
                element: DiceElement {
                    rolls: 20.into(),
                    faces: 20.into(),
                },
                pick: None
            })
        );
    }

    #[test]
    fn sum_dice_parses_pick() {
        let parser = sum_dice();
        assert_eq!(
            parser.parse("5D6KH3").into_result(),
            Ok(SumDice {
                element: DiceElement {
                    rolls: 5.into(),
                    faces: 6.into(),
                },
                pick: Some(SumDicePick::KeepHighest(3.into(),)),
            })
        );
        assert_eq!(
            parser.parse("10D20KL10").into_result(),
            Ok(SumDice {
                element: DiceElement {
                    rolls: 10.into(),
                    faces: 20.into(),
                },
                pick: Some(SumDicePick::KeepLowest(10.into(),)),
            })
        );
        assert_eq!(
            parser.parse("3D4DH1").into_result(),
            Ok(SumDice {
                element: DiceElement {
                    rolls: 3.into(),
                    faces: 4.into(),
                },
                pick: Some(SumDicePick::DropHighest(1.into(),)),
            })
        );
        assert_eq!(
            parser.parse("5D12DL2").into_result(),
            Ok(SumDice {
                element: DiceElement {
                    rolls: 5.into(),
                    faces: 12.into(),
                },
                pick: Some(SumDicePick::DropLowest(2.into(),)),
            })
        );
        assert_eq!(
            parser.parse("3D6MAX").into_result(),
            Ok(SumDice {
                element: DiceElement {
                    rolls: 3.into(),
                    faces: 6.into(),
                },
                pick: Some(SumDicePick::KeepHighest(1.into(),)),
            })
        );
        assert_eq!(
            parser.parse("4D8MIN").into_result(),
            Ok(SumDice {
                element: DiceElement {
                    rolls: 4.into(),
                    faces: 8.into(),
                },
                pick: Some(SumDicePick::KeepLowest(1.into(),)),
            })
        );
    }
}

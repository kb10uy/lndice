use chumsky::prelude::*;

use crate::types::{SumDice, SumDicePick};

use super::expression::int;

pub(super) fn sum_dice<'a>() -> impl Parser<'a, &'a str, SumDice, extra::Err<Rich<'a, char>>> {
    int()
        .then_ignore(just('D'))
        .then(int())
        .then(sum_dice_pick().or_not())
        .map(|((rolls, faces), pick)| SumDice { rolls, faces, pick })
}

fn sum_dice_pick<'a>() -> impl Parser<'a, &'a str, SumDicePick, extra::Err<Rich<'a, char>>> {
    let kh = just("KH").then(int()).map(|(_, v)| SumDicePick::KeepHighest(v));
    let kl = just("KL").then(int()).map(|(_, v)| SumDicePick::KeepLowest(v));
    let dh = just("DH").then(int()).map(|(_, v)| SumDicePick::DropHighest(v));
    let dl = just("DL").then(int()).map(|(_, v)| SumDicePick::DropLowest(v));
    let max = just("MAX").map(|_| SumDicePick::KeepHighest(1));
    let min = just("MIN").map(|_| SumDicePick::KeepLowest(1));
    choice((kh, kl, dh, dl, max, min))
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::{SumDice, SumDicePick};

    use super::sum_dice;

    #[test]
    fn sum_dice_parses_basic() {
        let parser = sum_dice();
        assert_eq!(
            parser.parse("1D6").into_result(),
            Ok(SumDice {
                rolls: 1,
                faces: 6,
                pick: None
            })
        );
        assert_eq!(
            parser.parse("2D10").into_result(),
            Ok(SumDice {
                rolls: 2,
                faces: 10,
                pick: None
            })
        );
        assert_eq!(
            parser.parse("10D4").into_result(),
            Ok(SumDice {
                rolls: 10,
                faces: 4,
                pick: None
            })
        );
        assert_eq!(
            parser.parse("20D20").into_result(),
            Ok(SumDice {
                rolls: 20,
                faces: 20,
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
                rolls: 5,
                faces: 6,
                pick: Some(SumDicePick::KeepHighest(3)),
            })
        );
        assert_eq!(
            parser.parse("10D20KL10").into_result(),
            Ok(SumDice {
                rolls: 10,
                faces: 20,
                pick: Some(SumDicePick::KeepLowest(10)),
            })
        );
        assert_eq!(
            parser.parse("3D4DH1").into_result(),
            Ok(SumDice {
                rolls: 3,
                faces: 4,
                pick: Some(SumDicePick::DropHighest(1)),
            })
        );
        assert_eq!(
            parser.parse("5D12DL2").into_result(),
            Ok(SumDice {
                rolls: 5,
                faces: 12,
                pick: Some(SumDicePick::DropLowest(2)),
            })
        );
        assert_eq!(
            parser.parse("3D6MAX").into_result(),
            Ok(SumDice {
                rolls: 3,
                faces: 6,
                pick: Some(SumDicePick::KeepHighest(1)),
            })
        );
        assert_eq!(
            parser.parse("4D8MIN").into_result(),
            Ok(SumDice {
                rolls: 4,
                faces: 8,
                pick: Some(SumDicePick::KeepLowest(1)),
            })
        );
    }
}

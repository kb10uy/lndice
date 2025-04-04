use chumsky::prelude::*;

use crate::types::dice::TwoSixDice;

pub fn two_six_dice<'a>() -> impl Parser<'a, &'a str, TwoSixDice, extra::Err<Rich<'a, char>>> + Clone {
    just("D66")
        .ignore_then(one_of("ASDN").labelled("D66 specifier").or_not())
        .map(|c| match c {
            None => TwoSixDice::Unspecified,
            Some('A' | 'S') => TwoSixDice::Ascending,
            Some('D') => TwoSixDice::Descending,
            Some('N') => TwoSixDice::Keep,
            Some(otherwise) => unreachable!("unexpected D66 specifier: {otherwise:}"),
        })
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::dice::TwoSixDice;

    use super::two_six_dice;

    #[test]
    fn two_six_dice_parses_basic() {
        let parser = two_six_dice();
        assert_eq!(parser.parse("D66").into_result(), Ok(TwoSixDice::Unspecified));
        assert_eq!(parser.parse("D66A").into_result(), Ok(TwoSixDice::Ascending));
        assert_eq!(parser.parse("D66S").into_result(), Ok(TwoSixDice::Ascending));
        assert_eq!(parser.parse("D66D").into_result(), Ok(TwoSixDice::Descending));
        assert_eq!(parser.parse("D66N").into_result(), Ok(TwoSixDice::Keep));
    }
}

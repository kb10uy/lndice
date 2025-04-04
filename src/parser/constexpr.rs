use chumsky::{
    pratt::{infix, left, postfix},
    prelude::*,
};

use crate::types::constexpr::{ConstExpr, FractionMode};

pub(super) fn term<'a>() -> impl Parser<'a, &'a str, ConstExpr, extra::Err<Rich<'a, char>>> + Clone {
    int().or(expr().delimited_by(just('('), just(')')))
}

pub(super) fn expr<'a>() -> impl Parser<'a, &'a str, ConstExpr, extra::Err<Rich<'a, char>>> + Clone {
    recursive(|expr| {
        let term = int().or(expr.delimited_by(just('('), just(')')));
        term.pratt((
            infix(left(3), just('*'), |x, _, y, _| {
                ConstExpr::Multiply(Box::new(x), Box::new(y))
            }),
            infix(left(3), just('/'), |x, _, y, _| {
                ConstExpr::Divide(Box::new(x), Box::new(y), None)
            }),
            infix(left(2), just('+'), |x, _, y, _| {
                ConstExpr::Add(Box::new(x), Box::new(y))
            }),
            infix(left(2), just('-'), |x, _, y, _| {
                ConstExpr::Subtract(Box::new(x), Box::new(y))
            }),
            postfix(1, fraction_mode(), |expr, f, _| match expr {
                ConstExpr::Divide(x, y, _) => ConstExpr::Divide(x, y, Some(f)),
                _ => expr,
            }),
        ))
    })
}

fn int<'a>() -> impl Parser<'a, &'a str, ConstExpr, extra::Err<Rich<'a, char>>> + Clone {
    text::int::<_, extra::Err<Rich<char>>>(10)
        .from_str()
        .unwrapped()
        .map(ConstExpr::Number)
}

pub(super) fn fraction_mode<'a>() -> impl Parser<'a, &'a str, FractionMode, extra::Err<Rich<'a, char>>> + Clone {
    one_of("FCUR").map(|c| match c {
        'F' => FractionMode::Floor,
        'C' | 'U' => FractionMode::Ceil,
        'R' => FractionMode::Round,
        otherwise => unreachable!("unexpected fraction mode: {otherwise}"),
    })
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::constexpr::{ConstExpr, FractionMode};

    use super::{expr, term};

    #[test]
    fn constexpr_parses_basic() {
        let parser = term();
        assert_eq!(parser.parse("42").into_result(), Ok(ConstExpr::Number(42)));
        assert_eq!(parser.parse("(12345)").into_result(), Ok(ConstExpr::Number(12345)));
    }

    #[test]
    fn constexpr_parses_expr() {
        let parser = expr();
        assert_eq!(
            parser.parse("1+1").into_result(),
            Ok(ConstExpr::Add(
                Box::new(ConstExpr::Number(1)),
                Box::new(ConstExpr::Number(1)),
            ))
        );
        assert_eq!(
            parser.parse("2-2").into_result(),
            Ok(ConstExpr::Subtract(
                Box::new(ConstExpr::Number(2)),
                Box::new(ConstExpr::Number(2)),
            ))
        );
        assert_eq!(
            parser.parse("3*3").into_result(),
            Ok(ConstExpr::Multiply(
                Box::new(ConstExpr::Number(3)),
                Box::new(ConstExpr::Number(3)),
            ))
        );
        assert_eq!(
            parser.parse("4/4").into_result(),
            Ok(ConstExpr::Divide(
                Box::new(ConstExpr::Number(4)),
                Box::new(ConstExpr::Number(4)),
                None,
            ))
        );
    }

    #[test]
    fn constexpr_parses_fraction_mode() {
        let parser = expr();
        assert_eq!(
            parser.parse("1/2F").into_result(),
            Ok(ConstExpr::Divide(
                Box::new(ConstExpr::Number(1)),
                Box::new(ConstExpr::Number(2)),
                Some(FractionMode::Floor),
            ))
        );
        assert_eq!(
            parser.parse("2/3C").into_result(),
            Ok(ConstExpr::Divide(
                Box::new(ConstExpr::Number(2)),
                Box::new(ConstExpr::Number(3)),
                Some(FractionMode::Ceil),
            ))
        );
        assert_eq!(
            parser.parse("3/4U").into_result(),
            Ok(ConstExpr::Divide(
                Box::new(ConstExpr::Number(3)),
                Box::new(ConstExpr::Number(4)),
                Some(FractionMode::Ceil),
            ))
        );
        assert_eq!(
            parser.parse("4/5R").into_result(),
            Ok(ConstExpr::Divide(
                Box::new(ConstExpr::Number(4)),
                Box::new(ConstExpr::Number(5)),
                Some(FractionMode::Round),
            ))
        );
    }
}

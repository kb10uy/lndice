use chumsky::prelude::*;

use crate::{
    parser::{
        constexpr::expr,
        dice::dice_element,
        query::{query_kind, range_query},
    },
    types::{
        constexpr::ConstExpr,
        dice::ReplayDice,
        query::{QueryKind, RangeQuery},
    },
};

pub(super) fn replay_dice<'a>() -> impl Parser<'a, &'a str, ReplayDice, extra::Err<Rich<'a, char>>> {
    let elements = dice_element('R')
        .separated_by(just('+'))
        .collect()
        .labelled("dice roll elements");
    let replay = surrounded_replay().labelled("replay condition").or_not();
    let query = range_query().labelled("query").or_not();

    elements
        .then(replay)
        .then(query)
        .try_map(|((elements, replay), target), span| {
            let (replay_query, target_query) = match (replay, target) {
                (None, q) => (q.clone(), q),
                (Some((Some(kind), value)), q) => (Some(RangeQuery { kind, value }), q),
                (Some((None, value)), Some(q)) => (Some(RangeQuery { kind: q.kind, value }), Some(q)),
                _ => return Err(Rich::custom(span, "indeterminate query")),
            };
            Ok(ReplayDice {
                elements,
                replay_query,
                target_query,
            })
        })
}

fn surrounded_replay<'a>() -> impl Parser<'a, &'a str, (Option<QueryKind>, ConstExpr), extra::Err<Rich<'a, char>>> {
    query_kind().or_not().then(expr()).delimited_by(just('['), just(']'))
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::{
        dice::{DiceElement, ReplayDice},
        query::{QueryKind, RangeQuery},
    };

    use super::replay_dice;

    #[test]
    fn replay_dice_parses_basic() {
        let parser = replay_dice();
        assert_eq!(
            parser.parse("2R6+1R10[>3]>=5").into_result(),
            Ok(ReplayDice {
                elements: vec![
                    DiceElement {
                        rolls: 2.into(),
                        faces: 6.into()
                    },
                    DiceElement {
                        rolls: 1.into(),
                        faces: 10.into()
                    },
                ],
                replay_query: Some(RangeQuery {
                    kind: QueryKind::Greater,
                    value: 3.into(),
                }),
                target_query: Some(RangeQuery {
                    kind: QueryKind::GreaterEqual,
                    value: 5.into(),
                })
            })
        );
    }
}

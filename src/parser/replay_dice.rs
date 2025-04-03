use chumsky::prelude::*;

use crate::types::{QueryKind, RangeQuery, ReplayDice, ReplayDiceElement};

use super::expression::{int, query_kind, range_query};

pub(super) fn replay_dice<'a>() -> impl Parser<'a, &'a str, ReplayDice, extra::Err<Rich<'a, char>>> {
    (replay_dice_element().separated_by(just('+')).collect())
        .then(surrounded_replay().or_not())
        .then(range_query().or_not())
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

fn replay_dice_element<'a>() -> impl Parser<'a, &'a str, ReplayDiceElement, extra::Err<Rich<'a, char>>> {
    int()
        .then_ignore(one_of("R"))
        .then(int())
        .map(|(rolls, faces)| ReplayDiceElement { rolls, faces })
}

fn surrounded_replay<'a>() -> impl Parser<'a, &'a str, (Option<QueryKind>, usize), extra::Err<Rich<'a, char>>> {
    query_kind().or_not().then(int()).delimited_by(just('['), just(']'))
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::{QueryKind, RangeQuery, ReplayDice, ReplayDiceElement};

    use super::replay_dice;

    #[test]
    fn replay_dice_parses_basic() {
        let parser = replay_dice();
        assert_eq!(
            parser.parse("2R6+1R10[>3]>=5").into_result(),
            Ok(ReplayDice {
                elements: vec![
                    ReplayDiceElement { rolls: 2, faces: 6 },
                    ReplayDiceElement { rolls: 1, faces: 10 },
                ],
                replay_query: Some(RangeQuery {
                    kind: QueryKind::Greater,
                    value: 3
                }),
                target_query: Some(RangeQuery {
                    kind: QueryKind::GreaterEqual,
                    value: 5
                })
            })
        );
    }
}

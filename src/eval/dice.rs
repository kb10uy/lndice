use rand::{Rng, distr::Uniform};

use crate::{
    eval::{
        EvalContext,
        constexpr::eval_constexpr,
        error::Error,
        roll::{IndividualDiceRoll, ReplayDiceResult, SumDiceRoll},
    },
    types::{
        dice::{DiceElement, ReplayDice, SumDiceElement, SumDicePick},
        query::{RangeQuery, ResolvedQuery},
    },
};

pub fn eval_sum_dice_element<R: Rng + ?Sized>(
    ctx: &EvalContext,
    rng: &mut R,
    dice: &SumDiceElement,
) -> Result<SumDiceRoll, Error> {
    let rolls_count = eval_constexpr(ctx, &dice.element.rolls)? as usize;
    let faces_count = eval_constexpr(ctx, &dice.element.faces)? as i64;
    if rolls_count == 0 || faces_count <= 0 {
        return Err(Error::CountMustBePositive);
    }

    let distr = Uniform::new(1, faces_count + 1).map_err(|_| Error::InvalidDice)?;
    let mut rolls: Vec<_> = rng.sample_iter(&distr).take(rolls_count).collect();
    let effective_count = match &dice.pick {
        None => rolls.len(),
        Some(SumDicePick::KeepHighest(n)) => {
            rolls.sort_by(|a, b| b.cmp(a));
            (eval_constexpr(ctx, n)? as usize).min(rolls.len())
        }
        Some(SumDicePick::KeepLowest(n)) => {
            rolls.sort();
            (eval_constexpr(ctx, n)? as usize).min(rolls.len())
        }
        Some(SumDicePick::DropHighest(n)) => {
            rolls.sort();
            (rolls.len() - eval_constexpr(ctx, n)? as usize).max(0)
        }
        Some(SumDicePick::DropLowest(n)) => {
            rolls.sort_by(|a, b| b.cmp(a));
            (rolls.len() - eval_constexpr(ctx, n)? as usize).max(0)
        }
    };

    Ok(SumDiceRoll {
        rolled_dice: rolls.into(),
        effective_count,
    })
}

pub fn eval_individual_dice_element<R: Rng + ?Sized>(
    ctx: &EvalContext,
    rng: &mut R,
    dice: &DiceElement,
) -> Result<IndividualDiceRoll, Error> {
    let rolls_count = eval_constexpr(ctx, &dice.rolls)? as usize;
    let faces_count = eval_constexpr(ctx, &dice.faces)? as i64;
    if rolls_count == 0 || faces_count <= 0 {
        return Err(Error::CountMustBePositive);
    }

    let distr = Uniform::new(1, faces_count + 1).map_err(|_| Error::InvalidDice)?;
    let rolls: Vec<_> = rng.sample_iter(&distr).take(rolls_count).collect();

    Ok(IndividualDiceRoll {
        rolled_dice: rolls.into(),
    })
}

pub fn eval_replay_dice<R: Rng + ?Sized>(
    ctx: &EvalContext,
    rng: &mut R,
    dice: &ReplayDice,
) -> Result<ReplayDiceResult, Error> {
    // resolve conditions
    let (replay_condition, target_condition) = match (&dice.replay_query, &dice.target_query) {
        (Some(r), Some(t)) => {
            let replay = resolve_query(ctx, r)?;
            let target = resolve_query(ctx, r)?;
            (replay, Some(target))
        }
        (None, Some(t)) => {
            let target = resolve_query(ctx, t)?;
            (target.clone(), Some(target))
        }
        (Some(r), None) => {
            let replay = resolve_query(ctx, r)?;
            (replay, ctx.default_replay_target.clone())
        }
        (None, None) => return Err(Error::NoConditionProvided),
    };

    // resolve dice elements
    let resolved_elements = dice
        .elements
        .iter()
        .map(|de| {
            let rolls_count = eval_constexpr(ctx, &de.rolls)? as usize;
            let faces_count = eval_constexpr(ctx, &de.faces)? as i64;
            if replay_condition.passes_all(faces_count) {
                return Err(Error::InfiniteReplay);
            }

            let distr = Uniform::new(1, faces_count + 1).map_err(|_| Error::InvalidDice)?;
            Ok((rolls_count, distr))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    // roll
    let mut rolled_groups = vec![];
    let mut judges: Vec<_> = resolved_elements.iter().map(|_| (true, 0)).collect();
    while judges.iter().any(|(r, _)| *r) {
        let replaying_elements = resolved_elements.iter().zip(judges.iter_mut()).filter(|(_, (r, _))| *r);
        for ((rolls_count, distr), (replay, count)) in replaying_elements {
            let group: Box<_> = rng.sample_iter(distr).take(*rolls_count).collect();
            *replay = group.iter().any(|r| replay_condition.passes(*r));
            *count += 1;
            rolled_groups.push(group);
        }
    }

    let rolled_groups = rolled_groups.into();
    let replay_counts = judges.into_iter().map(|(_, c)| c).collect();
    Ok(ReplayDiceResult {
        query: target_condition,
        rolled_groups,
        replay_counts,
    })
}

fn resolve_query(ctx: &EvalContext, query: &RangeQuery) -> Result<ResolvedQuery, Error> {
    let value = eval_constexpr(ctx, &query.value)? as i64;
    Ok(ResolvedQuery(query.kind, value))
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use rand::{Rng, rngs::mock::StepRng};
    use rstest::{fixture, rstest};

    use crate::{
        eval::{EvalContext, roll::SumDiceRoll},
        types::dice::{DiceElement, SumDiceElement, SumDicePick},
    };

    use super::eval_sum_dice_element;

    #[fixture]
    fn rng() -> impl Rng {
        StepRng::new(0, 8000000000000000000)
    }

    #[fixture]
    fn ctx() -> EvalContext {
        Default::default()
    }

    #[fixture]
    fn dice_6_6() -> DiceElement {
        DiceElement {
            rolls: 6.into(),
            faces: 6.into(),
        }
    }

    #[rstest]
    fn eval_sum_dice_element_works(ctx: EvalContext, mut rng: impl Rng, dice_6_6: DiceElement) {
        let sde = |pick| SumDiceElement {
            element: dice_6_6.clone(),
            pick,
        };

        assert_eq!(
            eval_sum_dice_element(&ctx, &mut rng, &sde(None)),
            Ok(SumDiceRoll {
                rolled_dice: vec![3, 6, 2, 5, 2, 4].into(),
                effective_count: 6,
            })
        );
        assert_eq!(
            eval_sum_dice_element(&ctx, &mut rng, &sde(Some(SumDicePick::KeepHighest(3.into())))),
            Ok(SumDiceRoll {
                rolled_dice: vec![6, 5, 3, 3, 2, 1].into(),
                effective_count: 3,
            })
        );
        assert_eq!(
            eval_sum_dice_element(&ctx, &mut rng, &sde(Some(SumDicePick::KeepLowest(3.into())))),
            Ok(SumDiceRoll {
                rolled_dice: vec![1, 3, 4, 4, 5, 6].into(),
                effective_count: 3,
            })
        );
        assert_eq!(
            eval_sum_dice_element(&ctx, &mut rng, &sde(Some(SumDicePick::DropHighest(2.into())))),
            Ok(SumDiceRoll {
                rolled_dice: vec![1, 2, 3, 4, 5, 6].into(),
                effective_count: 4,
            })
        );
        assert_eq!(
            eval_sum_dice_element(&ctx, &mut rng, &sde(Some(SumDicePick::DropLowest(2.into())))),
            Ok(SumDiceRoll {
                rolled_dice: vec![6, 5, 4, 2, 1, 1].into(),
                effective_count: 4,
            })
        );
    }
}

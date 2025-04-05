use rand::{Rng, distr::Uniform};

use crate::{
    eval::{EvalContext, constexpr::eval_constexpr, error::Error, roll::SumDiceRoll},
    types::dice::{SumDiceElement, SumDicePick},
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
    fn eval_sum_dice_element_works_basic(ctx: EvalContext, mut rng: impl Rng, dice_6_6: DiceElement) {
        assert_eq!(
            eval_sum_dice_element(
                &ctx,
                &mut rng,
                &SumDiceElement {
                    element: dice_6_6,
                    pick: None,
                },
            ),
            Ok(SumDiceRoll {
                rolled_dice: vec![3, 6, 2, 5, 2, 4].into(),
                effective_count: 6,
            })
        );
    }

    #[rstest]
    fn eval_sum_dice_element_works_keep(ctx: EvalContext, mut rng: impl Rng, dice_6_6: DiceElement) {
        assert_eq!(
            eval_sum_dice_element(
                &ctx,
                &mut rng,
                &SumDiceElement {
                    element: dice_6_6.clone(),
                    pick: Some(SumDicePick::KeepHighest(3.into())),
                },
            ),
            Ok(SumDiceRoll {
                rolled_dice: vec![6, 5, 4, 3, 2, 2].into(),
                effective_count: 3,
            })
        );
        assert_eq!(
            eval_sum_dice_element(
                &ctx,
                &mut rng,
                &SumDiceElement {
                    element: dice_6_6.clone(),
                    pick: Some(SumDicePick::KeepLowest(3.into())),
                },
            ),
            Ok(SumDiceRoll {
                rolled_dice: vec![1, 2, 3, 3, 5, 6].into(),
                effective_count: 3,
            })
        );
    }

    #[rstest]
    fn eval_sum_dice_element_works_drop(ctx: EvalContext, mut rng: impl Rng, dice_6_6: DiceElement) {
        assert_eq!(
            eval_sum_dice_element(
                &ctx,
                &mut rng,
                &SumDiceElement {
                    element: dice_6_6.clone(),
                    pick: Some(SumDicePick::DropHighest(2.into())),
                },
            ),
            Ok(SumDiceRoll {
                rolled_dice: vec![2, 2, 3, 4, 5, 6].into(),
                effective_count: 4,
            })
        );
        assert_eq!(
            eval_sum_dice_element(
                &ctx,
                &mut rng,
                &SumDiceElement {
                    element: dice_6_6.clone(),
                    pick: Some(SumDicePick::DropLowest(2.into())),
                },
            ),
            Ok(SumDiceRoll {
                rolled_dice: vec![6, 5, 3, 3, 2, 1].into(),
                effective_count: 4,
            })
        );
    }
}

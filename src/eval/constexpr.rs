use num::rational::Ratio;

use crate::{
    eval::{EvalContext, error::Error},
    types::constexpr::{ConstExpr, FractionMode},
};

pub fn eval_constexpr(ctx: &EvalContext, constexpr: &ConstExpr) -> Result<isize, Error> {
    match constexpr {
        ConstExpr::Number(n) => Ok(*n as isize),
        ConstExpr::Add(lhs, rhs) => Ok(eval_constexpr(ctx, lhs)? + eval_constexpr(ctx, rhs)?),
        ConstExpr::Subtract(lhs, rhs) => Ok(eval_constexpr(ctx, lhs)? - eval_constexpr(ctx, rhs)?),
        ConstExpr::Multiply(lhs, rhs) => Ok(eval_constexpr(ctx, lhs)? * eval_constexpr(ctx, rhs)?),
        ConstExpr::Divide(lhs, rhs, fm) => {
            let numerator = eval_constexpr(ctx, lhs)?;
            let denominator = eval_constexpr(ctx, rhs)?;
            if denominator == 0 {
                return Err(Error::DivisionByZero);
            }

            let rational_value = Ratio::new(numerator, denominator);
            let cut = match fm.unwrap_or(ctx.default_fraction) {
                FractionMode::Floor => rational_value.floor().to_integer(),
                FractionMode::Ceil => rational_value.ceil().to_integer(),
                FractionMode::Round => rational_value.round().to_integer(),
            };
            Ok(cut)
        }
    }
}

use num::Rational64;

use crate::types::constexpr::{ConstExpr, FractionMode};

pub fn eval_constexpr(constexpr: &ConstExpr, default_fm: FractionMode) -> Rational64 {
    match constexpr {
        ConstExpr::Number(n) => (*n as i64).into(),
        ConstExpr::Add(lhs, rhs) => eval_constexpr(lhs, default_fm) + eval_constexpr(rhs, default_fm),
        ConstExpr::Subtract(lhs, rhs) => eval_constexpr(lhs, default_fm) - eval_constexpr(rhs, default_fm),
        ConstExpr::Multiply(lhs, rhs) => eval_constexpr(lhs, default_fm) * eval_constexpr(rhs, default_fm),
        ConstExpr::Divide(lhs, rhs, fm) => {
            let rational_value = eval_constexpr(lhs, default_fm) / eval_constexpr(rhs, default_fm);
            match fm.unwrap_or(default_fm) {
                FractionMode::Floor => rational_value.floor(),
                FractionMode::Ceil => rational_value.ceil(),
                FractionMode::Round => rational_value.round(),
            }
        }
    }
}

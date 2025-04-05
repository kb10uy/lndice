mod constexpr;
mod dice;
mod error;
mod roll;

use crate::types::constexpr::FractionMode;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct EvalContext {
    pub default_fraction: FractionMode,
}

impl Default for EvalContext {
    fn default() -> EvalContext {
        EvalContext {
            default_fraction: FractionMode::Floor,
        }
    }
}

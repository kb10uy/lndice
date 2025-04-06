mod constexpr;
mod dice;
mod error;
mod roll;

use crate::types::{constexpr::FractionMode, query::ResolvedQuery};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct EvalContext {
    pub default_fraction: FractionMode,
    pub default_replay_target: Option<ResolvedQuery>,
}

impl Default for EvalContext {
    fn default() -> EvalContext {
        EvalContext {
            default_fraction: FractionMode::Floor,
            default_replay_target: None,
        }
    }
}

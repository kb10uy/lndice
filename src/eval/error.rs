use thiserror::Error as ThisError;

#[derive(Debug, PartialEq, Eq, ThisError)]
pub enum Error {
    #[error("division by zero")]
    DivisionByZero,

    #[error("the count of rolls and faces must be positive number")]
    CountMustBePositive,

    #[error("dice is logically invalid")]
    InvalidDice,

    #[error("no condition provided")]
    NoConditionProvided,

    #[error("given condition will lead to infinite roll")]
    InfiniteReplay,
}

use crate::types::{constexpr::ConstExpr, query::RangeQuery};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiceCommand {
    Sum(SumDice),
    Individual(IndividualDice),
    Replay(ReplayDice),
    Infinite(InfiniteDice),
    Tally(TallyDice),
    TwoSix(TwoSixDice),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiceElement {
    pub rolls: ConstExpr,
    pub faces: ConstExpr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SumDice {
    pub element: DiceElement,
    pub pick: Option<SumDicePick>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SumDicePick {
    KeepHighest(ConstExpr),
    KeepLowest(ConstExpr),
    DropHighest(ConstExpr),
    DropLowest(ConstExpr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndividualDice(pub DiceElement);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReplayDice {
    pub elements: Vec<DiceElement>,
    pub replay_query: Option<RangeQuery>,
    pub target_query: Option<RangeQuery>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfiniteDice {
    pub elements: Vec<DiceElement>,
    pub threshold: Option<ConstExpr>,
    pub bias: Option<ConstExpr>,
    pub target_query: Option<RangeQuery>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TallyDice {
    pub element: DiceElement,
    pub with_zero: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TwoSixDice {
    Unspecified,
    Ascending,
    Descending,
    Keep,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiceRoll {
    Sum(SumDice),
    Individual(IndividualDice),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SumDice {
    pub rolls: usize,
    pub faces: usize,
    pub pick: Option<SumDicePick>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SumDicePick {
    KeepHighest(usize),
    KeepLowest(usize),
    DropHighest(usize),
    DropLowest(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndividualDice {
    pub rolls: usize,
    pub faces: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RangeQuery {
    pub kind: QueryKind,
    pub value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryKind {
    GreaterEqual,
    Greater,
    Lesser,
    LesserEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReplayDice {
    pub elements: Vec<ReplayDiceElement>,
    pub replay_query: Option<RangeQuery>,
    pub target_query: Option<RangeQuery>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReplayDiceElement {
    pub rolls: usize,
    pub faces: usize,
}

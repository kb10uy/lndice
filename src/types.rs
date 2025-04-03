#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiceCommand {
    Sum(SumDice),
    Individual(IndividualDice),
    Replay(ReplayDice),
    Infinite(InfiniteDice),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiceElement {
    pub rolls: usize,
    pub faces: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SumDice {
    pub element: DiceElement,
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
pub struct IndividualDice(pub DiceElement);

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
    pub elements: Vec<DiceElement>,
    pub replay_query: Option<RangeQuery>,
    pub target_query: Option<RangeQuery>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfiniteDice {
    pub elements: Vec<DiceElement>,
    pub threshold: Option<usize>,
    pub bias: Option<usize>,
    pub target_query: Option<RangeQuery>,
}

pub struct TallyDice {
    pub element: DiceElement,
    pub with_zero: bool,
}

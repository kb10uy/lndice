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

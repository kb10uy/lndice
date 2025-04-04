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

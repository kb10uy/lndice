use crate::types::constexpr::ConstExpr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RangeQuery {
    pub kind: QueryKind,
    pub value: ConstExpr,
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
pub struct ResolvedQuery(pub QueryKind, pub i64);

impl ResolvedQuery {
    pub fn passes(&self, value: i64) -> bool {
        match self.0 {
            QueryKind::GreaterEqual => value >= self.1,
            QueryKind::Greater => value > self.1,
            QueryKind::Lesser => value < self.1,
            QueryKind::LesserEqual => value <= self.1,
            QueryKind::Equal => value == self.1,
            QueryKind::NotEqual => value != self.1,
        }
    }

    pub fn passes_all(&self, faces: i64) -> bool {
        match self.0 {
            QueryKind::GreaterEqual => self.1 <= 1,
            QueryKind::Greater => self.1 < 1,
            QueryKind::Lesser => self.1 > faces,
            QueryKind::LesserEqual => self.1 >= faces,
            QueryKind::Equal => faces == 1 && self.1 == 1,
            QueryKind::NotEqual => !(1..=faces).contains(&self.1),
        }
    }
}

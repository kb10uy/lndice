#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstExpr {
    Number(usize),
    Add(Box<ConstExpr>, Box<ConstExpr>),
    Subtract(Box<ConstExpr>, Box<ConstExpr>),
    Multiply(Box<ConstExpr>, Box<ConstExpr>),
    Divide(Box<ConstExpr>, Box<ConstExpr>, Option<FractionMode>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FractionMode {
    Floor,
    Ceil,
    Round,
}

impl From<usize> for ConstExpr {
    fn from(value: usize) -> Self {
        ConstExpr::Number(value)
    }
}

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

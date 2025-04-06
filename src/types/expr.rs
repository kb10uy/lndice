#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Join,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperation {
    Spread,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// `42`
    Number(i64),

    /// `foo`
    Identifier(String),

    /// `foo(1, 2)`
    FunctionCall(Box<Expr>, Box<[Expr]>),

    /// `4d6[>=3]`
    Ranged(Box<Expr>, QueryKind, Box<Expr>),

    /// `*4d6`
    Unary(UnaryOperation, Box<Expr>),

    /// `1 + 1`
    Binary(Box<Expr>, BinaryOperation, Box<Expr>),

    /// `4d6`
    SumDice(Box<Expr>, Box<Expr>),

    /// `4b6`
    IndividualDice(Box<Expr>, Box<Expr>),
}

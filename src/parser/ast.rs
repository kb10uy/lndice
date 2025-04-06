use chumsky::span::SimpleSpan;

use crate::types::expr::{BinaryOperation, Expr, QueryKind, UnaryOperation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstBinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Join,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstUnaryOperation {
    Spread,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstQueryKind {
    GreaterEqual,
    Greater,
    Lesser,
    LesserEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstExpr<'a> {
    /// `42`
    Number(i64),

    /// `foo`
    Identifier(&'a str),

    /// `foo(1, 2)`
    FunctionCall(Box<SpannedExpr<'a>>, Box<[SpannedExpr<'a>]>),

    /// `4d6[>=3]`
    Ranged(Box<SpannedExpr<'a>>, AstQueryKind, Box<SpannedExpr<'a>>),

    /// `*4d6`
    Unary(AstUnaryOperation, Box<SpannedExpr<'a>>),

    /// `1 + 1`
    Binary(Box<SpannedExpr<'a>>, AstBinaryOperation, Box<SpannedExpr<'a>>),

    /// `4d6`
    SumDice(Box<SpannedExpr<'a>>, Box<SpannedExpr<'a>>),

    /// `4b6`
    IndividualDice(Box<SpannedExpr<'a>>, Box<SpannedExpr<'a>>),
}

impl<'a> AstExpr<'a> {
    pub fn unary(op: AstUnaryOperation, rhs: SpannedExpr<'a>) -> AstExpr<'a> {
        AstExpr::Unary(op, rhs.into())
    }

    pub fn binary(op: AstBinaryOperation, lhs: SpannedExpr<'a>, rhs: SpannedExpr<'a>) -> AstExpr<'a> {
        AstExpr::Binary(lhs.into(), op, rhs.into())
    }

    pub fn sum_dice(lhs: SpannedExpr<'a>, rhs: SpannedExpr<'a>) -> AstExpr<'a> {
        AstExpr::SumDice(lhs.into(), rhs.into())
    }

    pub fn individual_dice(lhs: SpannedExpr<'a>, rhs: SpannedExpr<'a>) -> AstExpr<'a> {
        AstExpr::IndividualDice(lhs.into(), rhs.into())
    }

    pub fn ranged(lhs: SpannedExpr<'a>, qk: AstQueryKind, rhs: SpannedExpr<'a>) -> AstExpr<'a> {
        AstExpr::Ranged(lhs.into(), qk, rhs.into())
    }

    pub fn function_call(lhs: SpannedExpr<'a>, args: impl Into<Box<[SpannedExpr<'a>]>>) -> AstExpr<'a> {
        AstExpr::FunctionCall(Box::new(lhs), args.into())
    }
}

pub type SpannedExpr<'a> = (AstExpr<'a>, SimpleSpan);

impl From<AstQueryKind> for QueryKind {
    fn from(value: AstQueryKind) -> Self {
        match value {
            AstQueryKind::GreaterEqual => QueryKind::GreaterEqual,
            AstQueryKind::Greater => QueryKind::Greater,
            AstQueryKind::Lesser => QueryKind::Lesser,
            AstQueryKind::LesserEqual => QueryKind::LesserEqual,
            AstQueryKind::Equal => QueryKind::Equal,
            AstQueryKind::NotEqual => QueryKind::NotEqual,
        }
    }
}

impl From<AstUnaryOperation> for UnaryOperation {
    fn from(value: AstUnaryOperation) -> Self {
        match value {
            AstUnaryOperation::Spread => UnaryOperation::Spread,
        }
    }
}

impl From<AstBinaryOperation> for BinaryOperation {
    fn from(value: AstBinaryOperation) -> Self {
        match value {
            AstBinaryOperation::Add => BinaryOperation::Add,
            AstBinaryOperation::Subtract => BinaryOperation::Subtract,
            AstBinaryOperation::Multiply => BinaryOperation::Multiply,
            AstBinaryOperation::Divide => BinaryOperation::Divide,
            AstBinaryOperation::Join => BinaryOperation::Join,
        }
    }
}

impl<'a> From<AstExpr<'a>> for Expr {
    fn from(value: AstExpr<'a>) -> Expr {
        match value {
            AstExpr::Number(n) => Expr::Number(n),
            AstExpr::Identifier(id) => Expr::Identifier(id.to_string()),
            AstExpr::FunctionCall(lhs, args) => {
                let lhs = Box::new(lhs.0.into());
                let args: Box<[Expr]> = args.into_iter().map(|(a, _)| a.into()).collect();
                Expr::FunctionCall(lhs, args)
            }
            AstExpr::Ranged(lhs, qk, rhs) => {
                let lhs = Box::new(lhs.0.into());
                let rhs = Box::new(rhs.0.into());
                Expr::Ranged(lhs, qk.into(), rhs)
            }
            AstExpr::Unary(op, rhs) => {
                let rhs = Box::new(rhs.0.into());
                Expr::Unary(op.into(), rhs)
            }
            AstExpr::Binary(lhs, op, rhs) => {
                let lhs = Box::new(lhs.0.into());
                let rhs = Box::new(rhs.0.into());
                Expr::Binary(lhs, op.into(), rhs)
            }
            AstExpr::SumDice(lhs, rhs) => {
                let lhs = Box::new(lhs.0.into());
                let rhs = Box::new(rhs.0.into());
                Expr::SumDice(lhs, rhs)
            }
            AstExpr::IndividualDice(lhs, rhs) => {
                let lhs = Box::new(lhs.0.into());
                let rhs = Box::new(rhs.0.into());
                Expr::IndividualDice(lhs, rhs)
            }
        }
    }
}

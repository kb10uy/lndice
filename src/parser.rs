mod choice;
mod constexpr;
mod dice;
mod query;

use chumsky::{input::ValueInput, pratt::*, prelude::*};

use crate::{lexer::Token, types::Command};

pub fn parse_command(source: &str) -> Result<Command, Vec<Rich<'_, char>>> {
    todo!();
}

pub fn expr<'a, I>() -> impl Parser<'a, I, SpannedExpr<'a>, extra::Err<Rich<'a, Token<'a>>>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    recursive(|expr| {
        let value = value().labelled("value");

        let op = |s| just(Token::Operator(s)).labelled("operator");
        let dice_op = |s| just(Token::Identifier(s)).labelled("dice nonation");
        let parens = |s| just(Token::Parens(s));

        let atom = choice((value, expr.delimited_by(parens('('), parens(')'))));
        atom.pratt(vec![
            // function call
            postfix(10, function_args().labelled("function arguments"), |lhs, args, e| {
                (AstExpr::function_call(lhs, args), e.span())
            })
            .boxed(),
            // dice nonations
            infix(left(8), dice_op("d"), |lhs, _, rhs, e| {
                (AstExpr::sum_dice(lhs, rhs), e.span())
            })
            .boxed(),
            infix(left(8), dice_op("b"), |lhs, _, rhs, e| {
                (AstExpr::individual_dice(lhs, rhs), e.span())
            })
            .boxed(),
            // range extension
            postfix(7, range_extension().labelled("range"), |lhs, (qk, rhs), e| {
                (AstExpr::ranged(lhs, qk, rhs), e.span())
            })
            .boxed(),
            // spread
            prefix(6, op("*"), |_, rhs, e| {
                (AstExpr::unary(AstUnaryOperation::Spread, rhs), e.span())
            })
            .boxed(),
            // join
            infix(left(5), op("#"), |lhs, _, rhs, e| {
                (AstExpr::binary(AstBinaryOperation::Join, lhs, rhs), e.span())
            })
            .boxed(),
            // arithmetic operators
            infix(left(2), op("*"), |lhs, _, rhs, e| {
                (AstExpr::binary(AstBinaryOperation::Multiply, lhs, rhs), e.span())
            })
            .boxed(),
            infix(left(2), op("/"), |lhs, _, rhs, e| {
                (AstExpr::binary(AstBinaryOperation::Divide, lhs, rhs), e.span())
            })
            .boxed(),
            infix(left(1), op("+"), |lhs, _, rhs, e| {
                (AstExpr::binary(AstBinaryOperation::Divide, lhs, rhs), e.span())
            })
            .boxed(),
            infix(left(1), op("-"), |lhs, _, rhs, e| {
                (AstExpr::binary(AstBinaryOperation::Divide, lhs, rhs), e.span())
            })
            .boxed(),
        ])
    })
}

fn value<'a, I>() -> impl Parser<'a, I, SpannedExpr<'a>, extra::Err<Rich<'a, Token<'a>>>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    any().try_map(|a, span| match a {
        Token::Integer(n) => Ok((AstExpr::Number(n), span)),
        Token::Identifier(id) if id.len() != 1 => Ok((AstExpr::Identifier(id), span)),
        _ => Err(Rich::custom(span, "not a value")),
    })
}

fn function_args<'a, I>() -> impl Parser<'a, I, Vec<SpannedExpr<'a>>, extra::Err<Rich<'a, Token<'a>>>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    expr()
        .separated_by(just(Token::Operator(",")))
        .collect::<Vec<_>>()
        .delimited_by(just(Token::Parens('(')), just(Token::Parens(')')))
}

fn range_extension<'a, I>()
-> impl Parser<'a, I, (AstQueryKind, SpannedExpr<'a>), extra::Err<Rich<'a, Token<'a>>>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let comparison_op = choice((
        just(Token::Operator(">=")).to(AstQueryKind::GreaterEqual),
        just(Token::Operator(">")).to(AstQueryKind::Greater),
        just(Token::Operator("<=")).to(AstQueryKind::LesserEqual),
        just(Token::Operator("<")).to(AstQueryKind::Lesser),
        just(Token::Operator("==")).to(AstQueryKind::Equal),
        just(Token::Operator("!=")).to(AstQueryKind::NotEqual),
    ));
    comparison_op
        .then(expr())
        .delimited_by(just(Token::Parens('[')), just(Token::Parens(']')))
}

pub type SpannedExpr<'a> = (AstExpr<'a>, SimpleSpan);

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

impl<'a> AstExpr<'a> {
    fn unary(op: AstUnaryOperation, rhs: SpannedExpr<'a>) -> AstExpr<'a> {
        AstExpr::Unary(op, rhs.into())
    }

    fn binary(op: AstBinaryOperation, lhs: SpannedExpr<'a>, rhs: SpannedExpr<'a>) -> AstExpr<'a> {
        AstExpr::Binary(lhs.into(), op, rhs.into())
    }

    fn sum_dice(lhs: SpannedExpr<'a>, rhs: SpannedExpr<'a>) -> AstExpr<'a> {
        AstExpr::SumDice(lhs.into(), rhs.into())
    }

    fn individual_dice(lhs: SpannedExpr<'a>, rhs: SpannedExpr<'a>) -> AstExpr<'a> {
        AstExpr::IndividualDice(lhs.into(), rhs.into())
    }

    fn ranged(lhs: SpannedExpr<'a>, qk: AstQueryKind, rhs: SpannedExpr<'a>) -> AstExpr<'a> {
        AstExpr::Ranged(lhs.into(), qk, rhs.into())
    }

    fn function_call(lhs: SpannedExpr<'a>, args: impl Into<Box<[SpannedExpr<'a>]>>) -> AstExpr<'a> {
        AstExpr::FunctionCall(Box::new(lhs), args.into())
    }
}

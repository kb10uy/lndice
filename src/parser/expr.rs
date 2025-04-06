use chumsky::{input::ValueInput, pratt::*, prelude::*};

use crate::parser::{
    ast::{AstBinaryOperation, AstExpr, AstQueryKind, AstUnaryOperation, SpannedExpr},
    lexer::Token,
};

pub fn expr<'a, I>() -> impl Parser<'a, I, SpannedExpr<'a>, extra::Err<Rich<'a, Token<'a>>>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    recursive(|expr| {
        let value = value().labelled("value");

        let op = |s| just(Token::Operator(s)).labelled("operator");
        let dice_op = |s| just(Token::Identifier(s)).labelled("dice nonation");
        let parens = |s| just(Token::Parens(s));

        let atom = choice((value, expr.clone().delimited_by(parens('('), parens(')'))));
        atom.pratt(vec![
            // function call
            postfix(
                10,
                function_args(expr.clone()).labelled("function arguments"),
                |lhs, args, e| (AstExpr::function_call(lhs, args), e.span()),
            )
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
            postfix(
                7,
                range_extension(expr.clone()).labelled("range"),
                |lhs, (qk, rhs), e| (AstExpr::ranged(lhs, qk, rhs), e.span()),
            )
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

fn function_args<'a, I>(
    expr: impl Parser<'a, I, SpannedExpr<'a>, extra::Err<Rich<'a, Token<'a>>>> + Clone,
) -> impl Parser<'a, I, Vec<SpannedExpr<'a>>, extra::Err<Rich<'a, Token<'a>>>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    expr.separated_by(just(Token::Operator(",")))
        .collect::<Vec<_>>()
        .delimited_by(just(Token::Parens('(')), just(Token::Parens(')')))
}

fn range_extension<'a, I>(
    expr: impl Parser<'a, I, SpannedExpr<'a>, extra::Err<Rich<'a, Token<'a>>>> + Clone,
) -> impl Parser<'a, I, (AstQueryKind, SpannedExpr<'a>), extra::Err<Rich<'a, Token<'a>>>> + Clone
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
        .then(expr)
        .delimited_by(just(Token::Parens('[')), just(Token::Parens(']')))
}

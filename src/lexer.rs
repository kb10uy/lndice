use chumsky::prelude::*;

pub type SpannedToken<'a> = (Token<'a>, SimpleSpan);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token<'a> {
    Integer(i64),
    Identifier(&'a str),
    Parens(char),
    Operator(&'a str),
}

pub fn lex_tokens<'a>() -> impl Parser<'a, &'a str, Vec<SpannedToken<'a>>, extra::Err<Rich<'a, char>>> + Clone {
    let token = integer().or(identifier()).or(parens()).or(operator());
    token
        .map_with(|t, e| (t, e.span()))
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}

fn integer<'a>() -> impl Parser<'a, &'a str, Token<'a>, extra::Err<Rich<'a, char>>> + Clone {
    text::int::<_, extra::Err<Rich<char>>>(10)
        .from_str()
        .unwrapped()
        .labelled("integer")
        .map(Token::Integer)
}

fn identifier<'a>() -> impl Parser<'a, &'a str, Token<'a>, extra::Err<Rich<'a, char>>> + Clone {
    any()
        .filter(|c: &char| c.is_alphabetic())
        .repeated()
        .at_least(1)
        .to_slice()
        .map(Token::Identifier)
}

fn parens<'a>() -> impl Parser<'a, &'a str, Token<'a>, extra::Err<Rich<'a, char>>> + Clone {
    one_of("(){}[]").labelled("parens").map(Token::Parens)
}

fn operator<'a>() -> impl Parser<'a, &'a str, Token<'a>, extra::Err<Rich<'a, char>>> + Clone {
    choice((
        just(">="),
        just(">"),
        just("<="),
        just("<"),
        just("=="),
        just("!="),
        just("+"),
        just("-"),
        just("*"),
        just("/"),
        just("#"),
    ))
    .labelled("operator")
    .map(Token::Operator)
}

#[cfg(test)]
mod test {
    use chumsky::{Parser, error::Rich};
    use pretty_assertions::assert_eq;

    use super::{Token, lex_tokens};

    fn split_tokens(source: &str) -> Result<Vec<Token<'_>>, Vec<Rich<char>>> {
        lex_tokens()
            .parse(source)
            .into_result()
            .map(|t| t.into_iter().map(|(t, _)| t).collect())
    }

    #[test]
    fn lexer_splits_basic() {
        assert_eq!(
            split_tokens("1 23 456 a bc def"),
            Ok(vec![
                Token::Integer(1),
                Token::Integer(23),
                Token::Integer(456),
                Token::Identifier("a"),
                Token::Identifier("bc"),
                Token::Identifier("def"),
            ])
        );
    }

    #[test]
    fn lexer_splits_compact() {
        assert_eq!(
            split_tokens("1a23bc456def"),
            Ok(vec![
                Token::Integer(1),
                Token::Identifier("a"),
                Token::Integer(23),
                Token::Identifier("bc"),
                Token::Integer(456),
                Token::Identifier("def"),
            ])
        );
    }

    #[test]
    fn lexer_splits_operators() {
        assert_eq!(
            split_tokens("1+2*3-4/5"),
            Ok(vec![
                Token::Integer(1),
                Token::Operator("+"),
                Token::Integer(2),
                Token::Operator("*"),
                Token::Integer(3),
                Token::Operator("-"),
                Token::Integer(4),
                Token::Operator("/"),
                Token::Integer(5),
            ])
        );
    }
}

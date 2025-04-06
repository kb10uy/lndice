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
    text::ascii::ident().labelled("identifier").map(Token::Identifier)
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
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use super::{SpannedToken, Token, lex_tokens};

    fn take_tokens(tokens: Vec<SpannedToken>) -> Vec<Token<'_>> {
        tokens.into_iter().map(|(t, _)| t).collect()
    }

    #[test]
    fn lexer_splits_basic() {
        let lexer = lex_tokens();
        assert_eq!(
            lexer.parse("1 23 456").into_result().map(take_tokens),
            Ok(vec![Token::Integer(1), Token::Integer(23), Token::Integer(456)])
        );
    }
}

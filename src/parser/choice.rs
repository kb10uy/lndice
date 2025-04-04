use chumsky::prelude::*;

use crate::types::choice::{Choice, ChoiceItems};

pub(super) fn choice_command<'a>() -> impl Parser<'a, &'a str, Choice, extra::Err<Rich<'a, char>>> + Clone {
    just("choice")
        .ignore_then(choice((
            choice_items_brackets(),
            choice_items_parens(),
            choice_items_spaces(),
        )))
        .map(Choice)
}

fn choice_items_brackets<'a>() -> impl Parser<'a, &'a str, ChoiceItems, extra::Err<Rich<'a, char>>> + Clone {
    // trim before collect?
    let item = none_of("[],")
        .repeated()
        .collect::<String>()
        .map(|s| s.trim().to_string());

    choice((
        choice_items_alphabetic_range(),
        choice_items_numeric_range(),
        (item.separated_by(just(',')).collect().map(ChoiceItems::Strings)),
    ))
    .delimited_by(just('['), just(']'))
}

fn choice_items_parens<'a>() -> impl Parser<'a, &'a str, ChoiceItems, extra::Err<Rich<'a, char>>> + Clone {
    // trim before collect?
    let item = none_of("(),")
        .repeated()
        .collect::<String>()
        .map(|s| s.trim().to_string());

    choice((
        choice_items_alphabetic_range(),
        choice_items_numeric_range(),
        (item.separated_by(just(',')).collect().map(ChoiceItems::Strings)),
    ))
    .delimited_by(just('('), just(')'))
}

fn choice_items_spaces<'a>() -> impl Parser<'a, &'a str, ChoiceItems, extra::Err<Rich<'a, char>>> + Clone {
    let item = none_of(" ").repeated().collect::<String>();
    (just(' ').repeated().at_least(1)).ignore_then(choice((
        choice_items_alphabetic_range(),
        choice_items_numeric_range(),
        (item
            .separated_by(just(' ').repeated().at_least(1))
            .collect()
            .map(ChoiceItems::Strings)),
    )))
}

fn choice_items_alphabetic_range<'a>() -> impl Parser<'a, &'a str, ChoiceItems, extra::Err<Rich<'a, char>>> + Clone {
    let alphabet = any().filter(char::is_ascii_alphabetic);
    alphabet
        .then_ignore(just('-'))
        .then(alphabet)
        .map(|(start, end)| ChoiceItems::AlphabeticalRange(start..=end))
}

fn choice_items_numeric_range<'a>() -> impl Parser<'a, &'a str, ChoiceItems, extra::Err<Rich<'a, char>>> + Clone {
    let int = text::int::<_, extra::Err<Rich<char>>>(10).from_str().unwrapped();
    int.then_ignore(just('-'))
        .then(int)
        .map(|(start, end)| ChoiceItems::NumericRange(start..=end))
}

#[cfg(test)]
mod test {
    use chumsky::Parser;
    use pretty_assertions::assert_eq;

    use crate::types::choice::{Choice, ChoiceItems};

    use super::choice_command;

    #[test]
    fn choice_parses_basic() {
        let parser = choice_command();
        assert_eq!(
            parser.parse("choice[foo,bar,baz]").into_result(),
            Ok(Choice(ChoiceItems::Strings(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string(),
            ]))),
        );
        assert_eq!(
            parser.parse("choice(foo,bar,baz)").into_result(),
            Ok(Choice(ChoiceItems::Strings(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string(),
            ]))),
        );
        assert_eq!(
            parser.parse("choice foo bar baz").into_result(),
            Ok(Choice(ChoiceItems::Strings(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string(),
            ]))),
        );
    }

    #[test]
    fn choice_parses_ranges() {
        let parser = choice_command();
        assert_eq!(
            parser.parse("choice[A-Z]").into_result(),
            Ok(Choice(ChoiceItems::AlphabeticalRange('A'..='Z'))),
        );
        assert_eq!(
            parser.parse("choice(1-100)").into_result(),
            Ok(Choice(ChoiceItems::NumericRange(1..=100))),
        );
        assert_eq!(
            parser.parse("choice 1-100").into_result(),
            Ok(Choice(ChoiceItems::NumericRange(1..=100))),
        );
    }
}

use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Choice(pub ChoiceItems);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChoiceItems {
    Strings(Vec<String>),
    AlphabeticalRange(RangeInclusive<char>),
    NumericRange(RangeInclusive<usize>),
}

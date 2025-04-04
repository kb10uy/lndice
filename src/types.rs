pub mod choice;
pub mod constexpr;
pub mod dice;
pub mod query;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command {
    Sum(dice::SumDice),
    Individual(dice::IndividualDice),
    Replay(dice::ReplayDice),
    Infinite(dice::InfiniteDice),
    Tally(dice::TallyDice),
    TwoSix(dice::TwoSixDice),
    Calculation(constexpr::ConstExpr),
    Choice(choice::Choice),
    Repeat(usize, Box<Command>),
}

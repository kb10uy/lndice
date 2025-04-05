#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SumDiceRoll {
    pub(in crate::eval) rolled_dice: Box<[i64]>,
    pub(in crate::eval) effective_count: usize,
}

impl SumDiceRoll {
    pub fn effective_rolls(&self) -> &[i64] {
        &self.rolled_dice[..self.effective_count]
    }

    pub fn effective_len(&self) -> usize {
        self.effective_count
    }

    pub fn all_rolls(&self) -> &[i64] {
        &self.rolled_dice
    }
}

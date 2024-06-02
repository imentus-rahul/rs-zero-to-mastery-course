// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

struct ScoreMultiplier {
    amount: usize,
    per_iteration: usize,
    per_iteration_bonus: usize,
}

impl ScoreMultiplier {
    fn new() -> Self {
        Self {
            amount: 0,
            per_iteration: 1,
            per_iteration_bonus: 0,
        }
    }
}

// implement the iterator trait
impl Iterator for ScoreMultiplier {
    type Item = usize; // equal sign
    fn next(&mut self) -> Option<Self::Item> {
        self.amount = self.amount + self.per_iteration + self.per_iteration_bonus;
        Some(self.amount)
    }
}
fn main() {
    let mut score_multiplier_instance = ScoreMultiplier::new();
    println!("{:?}", score_multiplier_instance.next());
    println!("{:?}", score_multiplier_instance.next());
    println!("{:?}", score_multiplier_instance.next());
    println!("{:?}", score_multiplier_instance.next());
    println!("{:?}", score_multiplier_instance.next());
    score_multiplier_instance.per_iteration_bonus = 2;
    println!("{:?}", score_multiplier_instance.next()); // 5+1+2 = 8 // same .next() method but different results
    println!("{:?}", score_multiplier_instance.next()); // 8+1+2 = 11

}

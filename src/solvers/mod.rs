use crate::word::{Test, Word};

mod v1;
mod v2;
mod v3;
mod v4;

pub use v1::SolverV1;
pub use v2::SolverV2;
pub use v3::SolverV3;

pub trait Solver {
    fn try_word(&mut self, guess: Word, results: [Test; 5]) -> usize;

    fn get_guess(&self) -> Word;

    fn play(mut self, real_word: Word) -> usize where Self: Sized {
        let mut i = 0;
        loop {
            i += 1;
            let guess = self.get_guess();
            let tests = real_word.test(guess);
            if tests.iter().all(|item| item == &Test::Green) {
                break
            }
            self.try_word(guess, tests);
        };
        return i;
    }
}


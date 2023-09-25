use std::path::Path;
use crate::solver::WordleGuesser;
use crate::solvers::Solver;
use crate::word::{Test, Word};

#[derive(Clone)]
pub struct SolverV1 {
    guesser: WordleGuesser
}

impl SolverV1 {
    pub fn from_words_file(p: impl AsRef<Path>) -> SolverV1 {
        SolverV1 { guesser: WordleGuesser::from_words_file(p) }
    }

    pub fn from_words(l: Vec<Word>) -> SolverV1 {
        SolverV1 { guesser: WordleGuesser::from_word_list(l) }
    }
}

impl Solver for SolverV1 {
    fn try_word(&mut self, guess: Word, results: [Test; 5]) -> usize {
        self.guesser.try_word(guess, results)
    }

    fn get_guess(&self) -> Word {
        if self.guesser.get_possible().len() == 1 {
            *self.guesser.get_possible().first().unwrap()
        } else {
            *self.guesser.get_possible().first().unwrap()
        }
    }
}
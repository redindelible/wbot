use std::array;
use std::collections::{HashMap, HashSet};

use std::path::Path;
use crate::solver::WordleGuesser;
use crate::solvers::Solver;
use crate::word::{Test, Word};

pub struct SolverV3 {
    all_possible_guesses: Vec<Word>,
    guesser: WordleGuesser
}

impl SolverV3 {
    pub fn from_words_file(p: impl AsRef<Path>) -> SolverV3 {
        let guesser = WordleGuesser::from_words_file(p);
        let all_possible_guesses = guesser.get_possible().to_vec();
        SolverV3 { all_possible_guesses, guesser }
    }

    pub fn from_words(l: Vec<Word>) -> SolverV3 {
        SolverV3 { all_possible_guesses: l.clone(), guesser: WordleGuesser::from_word_list(l) }
    }
}

impl Solver for SolverV3 {
    fn try_word(&mut self, guess: Word, results: [Test; 5]) -> usize {
        self.guesser.try_word(guess, results)
    }

    fn get_guess(&self) -> Word {
        if self.guesser.get_possible().len() == 1 {
            return *self.guesser.get_possible().last().unwrap()
        }

        let count = self.guesser.get_possible().len();
        let mut positions: [HashMap<char, usize>; 5] = array::from_fn(|_| HashMap::new());
        let mut totals: HashMap<char, usize> = HashMap::new();

        for possible in self.guesser.get_possible() {
            for (i, chr) in possible.word.iter().enumerate() {
                positions[i].entry(*chr).and_modify(|c| *c += 1).or_insert(1);
                totals.entry(*chr).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        totals.retain(|_, c| *c != count);

        let best = self.all_possible_guesses.iter().max_by_key(|&guess| {
            let mut score = 0;
            for (i, chr) in guess.word.iter().enumerate() {
                let chr_count_at_i = positions[i].get(chr).copied().unwrap_or(0);
                if chr_count_at_i == count {
                    score += totals.get(chr).copied().unwrap_or(0);
                } else {
                    score += chr_count_at_i;
                }
            }
            score
        }).unwrap();

        *best
    }
}

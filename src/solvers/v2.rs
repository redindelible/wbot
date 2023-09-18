use std::collections::{HashMap, HashSet};
use std::path::Path;
use crate::solver::WordleGuesser;
use crate::solvers::Solver;
use crate::word::{Test, Word};

pub struct SolverV2 {
    all_possible_guesses: Vec<Word>,
    guesser: WordleGuesser
}

impl SolverV2 {
    pub fn from_words_file(p: impl AsRef<Path>) -> SolverV2 {
        let guesser = WordleGuesser::from_words_file(p);
        let all_possible_guesses = guesser.get_possible().to_vec();
        SolverV2 { all_possible_guesses, guesser }
    }

    pub fn from_words(l: Vec<Word>) -> SolverV2 {
        SolverV2 { all_possible_guesses: l.clone(), guesser: WordleGuesser::from_word_list(l) }
    }
}

impl Solver for SolverV2 {
    fn try_word(&mut self, guess: Word, results: [Test; 5]) -> usize {
        self.guesser.try_word(guess, results)
    }

    fn get_guess(&self) -> Result<Word, Word> {
        if self.guesser.get_possible().len() == 1 {
            return Err(*self.guesser.get_possible().last().unwrap())
        }
        if self.guesser.get_possible().len() <= 10 {
            return Ok(*self.guesser.get_possible().last().unwrap());
        }
        let mut alphabet: HashMap<char, usize> = HashMap::new();
        for possible_word in self.guesser.get_possible() {
            for chr in possible_word.word {
                alphabet.entry(chr).and_modify(|c| *c += 1).or_insert(1);
            }
        }
        alphabet.retain(|_, c| *c != self.guesser.get_possible().len());
        let best_guess = self.all_possible_guesses.iter().max_by_key(|&guess| {
            guess.word.iter().collect::<HashSet<&char>>().iter().map(|l| alphabet.get(l).copied().unwrap_or(0)).sum::<usize>()
        });
        Ok(*best_guess.unwrap())
    }
}

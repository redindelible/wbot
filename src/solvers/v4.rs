use std::array;
use std::collections::HashMap;
use std::path::Path;
use crate::solver::WordleGuesser;
use crate::solvers::Solver;
use crate::word::{Test, Word};

#[derive(Clone)]
pub struct SolverV4 {
    guesser: WordleGuesser
}

impl SolverV4 {
    pub fn from_words_file(p: impl AsRef<Path>) -> SolverV4 {
        SolverV4 { guesser: WordleGuesser::from_words_file(p) }
    }

    pub fn from_words(l: Vec<Word>) -> SolverV4 {
        SolverV4 { guesser: WordleGuesser::from_word_list(l) }
    }
}

impl Solver for SolverV4 {
    fn try_word(&mut self, guess: Word, results: [Test; 5]) -> usize {
        self.guesser.try_word(guess, results)
    }

    fn get_guess(&self) -> Word {
        let mut positions: [HashMap<char, usize>; 5] = array::from_fn(|_| HashMap::new());

        for possible_solution in self.guesser.get_possible() {
            for (i, chr) in possible_solution.word.iter().enumerate() {
                positions[i].entry(*chr).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        let solved_positions: Vec<usize> = positions.iter().enumerate().filter_map(|(i, map)| {
            if map.len() == 1 { Some(i) } else { None }
        }).collect();

        todo!()
    }
}
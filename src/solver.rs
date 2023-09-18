use std::collections::{HashMap, HashSet};
use std::path::Path;
use crate::word::{Test, Word};

#[derive(Clone)]
pub struct WordleGuesser {
    all_possible_guesses: Vec<Word>,
    possible_words: Vec<Word>
}

impl WordleGuesser {
    pub fn from_words_file(p: impl AsRef<Path>) -> WordleGuesser {
        let f = std::fs::read_to_string(p).unwrap();
        let mut list: Vec<Word> = Vec::new();
        for line in f.lines() {
            let word = Word::new(line.chars().collect::<Vec<char>>().try_into().unwrap());
            list.push(word)
        }
        WordleGuesser::from_word_list(list)
    }

    pub fn from_word_list(list: Vec<Word>) -> WordleGuesser {
        WordleGuesser { all_possible_guesses: list.clone(), possible_words: list }
    }

    pub fn try_word(&mut self, guess: Word, results: [Test; 5]) -> usize {
        let prev_len = self.possible_words.len();
        self.possible_words.retain(|word| Self::satisfies(guess, results, *word));
        prev_len - self.possible_words.len()
    }

    pub fn get_best_guess(&self) -> Option<Word> {
        if self.possible_words.len() <= 100 {
            return self.possible_words.last().copied();
        }
        let mut alphabet: HashMap<char, usize> = HashMap::new();
        for possible_word in &self.possible_words {
            for chr in possible_word.word {
                alphabet.entry(chr).and_modify(|c| *c += 1).or_insert(1);
            }
        }
        alphabet.retain(|_, c| *c != self.possible_words.len());
        let best_guess = self.all_possible_guesses.iter().max_by_key(|&guess| {
            guess.word.iter().collect::<HashSet<&char>>().iter().map(|l| alphabet.get(l).copied().unwrap_or(0)).sum::<usize>()
        });
        best_guess.copied()
    }

    pub fn get_possible(&self) -> &[Word] {
        &self.possible_words
    }

    fn satisfies(guess: Word, tests: [Test; 5], word: Word) -> bool {
        let mut word_counts = word.counts();

        for (i, test) in tests.iter().enumerate() {
            if test == &Test::Green {
                if guess[i] == word[i] {
                    if let Some(count) = word_counts.get_mut(&word[i]) {
                        if *count == 0 {
                            return false;
                        } else {
                            *count -= 1;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else if test == &Test::Yellow {
                if guess[i] == word[i] {
                    return false;
                } else if let Some(count) = word_counts.get_mut(&guess[i]) {
                    if *count == 0 {
                        return false;
                    } else {
                        *count -= 1;
                    }
                } else {
                    return false;
                }
            }
        }

        for (i, test) in tests.iter().enumerate() {
            if test == &Test::Gray {
                if let Some(count) = word_counts.get(&guess[i]) {
                    if *count != 0 {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod test {
    use crate::solver::WordleGuesser;
    use crate::word::{Test, Word};

    fn word(s: impl AsRef<str>) -> Word {
        Word::new(s.as_ref().chars().collect::<Vec<char>>().try_into().unwrap())
    }

    #[test]
    fn test_1() {
        assert!(!WordleGuesser::satisfies(word("musty"),
                                          [Test::Gray, Test::Green, Test::Green, Test::Gray, Test::Gray],
                                          word("sassy")))
    }

    #[test]
    fn test_2() {
        assert!(WordleGuesser::satisfies(word("there"),
                                          [Test::Gray, Test::Gray, Test::Yellow, Test::Yellow, Test::Green],
                                          word("erpae")))
    }
}
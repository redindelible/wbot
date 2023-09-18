use std::array;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::Index;


#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Word {
    pub word: [char; 5]
}

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word.iter().collect::<String>())
    }
}

impl Index<usize> for Word {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.word[index]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Test {
    Green,
    Yellow,
    Gray
}

impl Word {
    pub fn new(word: [char; 5]) -> Word {
        Word { word }
    }

    pub fn counts(&self) -> HashMap<char, u8> {
        let mut char_counts: HashMap<char, u8> = HashMap::new();
        for chr in self.word {
            char_counts.entry(chr)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        char_counts
    }

    pub fn test(&self, against: Word) -> [Test; 5]{
        let mut tests = array::from_fn(|_| Test::Gray);

        let mut char_counts = self.counts();

        for (i, chr) in against.word.iter().enumerate() {
            if *chr == self.word[i] {
                tests[i] = Test::Green;
                *char_counts.get_mut(chr).unwrap() -= 1;
            }
        }

        for (i, chr) in against.word.iter().enumerate() {
            if tests[i] == Test::Green {

            } else if let Some(count) = char_counts.get_mut(chr) {
                if *count == 0 {
                    tests[i] = Test::Gray
                } else {
                    tests[i] = Test::Yellow;
                    *count -= 1;
                }
            } else {
                // chr isn't in word at all then
                tests[i] = Test::Gray;
            }
        }

        tests
    }
}

#[cfg(test)]
mod test {
    use crate::word::{Test, Word};

    fn word(s: impl AsRef<str>) -> Word {
        Word::new(s.as_ref().chars().collect::<Vec<char>>().try_into().unwrap())
    }

    #[test]
    fn test_correct() {
        assert_eq!(word("there").test(word("there")),
                   [Test::Green, Test::Green, Test::Green, Test::Green, Test::Green])
    }

    #[test]
    fn test_less_correct() {
        assert_eq!(word("there").test(word("where")),
                   [Test::Gray, Test::Green, Test::Green, Test::Green, Test::Green])
    }

    #[test]
    fn test_one_correct() {
        assert_eq!(word("there").test(word("xxxxe")),
                   [Test::Gray, Test::Gray, Test::Gray, Test::Gray, Test::Green])
    }

    #[test]
    fn test_wrong_place() {
        assert_eq!(word("there").test(word("exxxx")),
                   [Test::Yellow, Test::Gray, Test::Gray, Test::Gray, Test::Gray])
    }

    #[test]
    fn test_two_wrong_place() {
        assert_eq!(word("there").test(word("eexxx")),
                   [Test::Yellow, Test::Yellow, Test::Gray, Test::Gray, Test::Gray])
    }

    #[test]
    fn test_three_wrong_place() {
        assert_eq!(word("there").test(word("eexex")),
                   [Test::Yellow, Test::Yellow, Test::Gray, Test::Gray, Test::Gray])
    }

    #[test]
    fn test_one_right_one_wrong() {
        assert_eq!(word("there").test(word("exxxe")),
                   [Test::Yellow, Test::Gray, Test::Gray, Test::Gray, Test::Green])
    }

    #[test]
    fn test_one_right_two_wrong() {
        assert_eq!(word("there").test(word("eexxe")),
                   [Test::Yellow, Test::Gray, Test::Gray, Test::Gray, Test::Green])
    }
}
mod word;
mod solver;

use std::io;
use std::io::{stdin, Write};
use std::thread::available_parallelism;
use crate::solver::WordleGuesser;
use crate::word::{Test, Word};

fn get_input() -> String {
    let mut string = String::new();
    stdin().read_line(&mut string).expect("Error: Could not read from stdin.");
    string
}

fn test_algo(mut f: impl FnMut(WordleGuesser, Word) -> usize, n: usize) {
    let mut guesser = WordleGuesser::from_words_file("src/all_words.txt");

    let all_words = guesser.get_possible().to_vec();

    let mut total: usize = 0;
    let mut len = (0..all_words.len()).step_by(n).count();
    for (i, word) in all_words.iter().enumerate().step_by(n) {
        total += f(guesser.clone(), *word);
        print!("\rCompleted {i}/{}", len * n);
        io::stdout().flush().unwrap();
    }
    let avg = total as f64 / len as f64;
    println!("\navg guesses {}", avg);
}

fn algo_v1(mut guesser: WordleGuesser, real_word: Word) -> usize {
    let mut i = 0;
    loop {
        i += 1;
        let Some(&guess) = guesser.get_possible().first() else { break };
        // println!("Trying {:?}", guess);
        let tests = real_word.test(guess);
        // println!("{}", tests.iter().map(|t| match t {
        //     Test::Green => '🟩',
        //     Test::Yellow => '🟨',
        //     Test::Gray => '⬛'
        // }).collect::<String>());
        if tests.iter().all(|item| item == &Test::Green) {
            // println!("{:?} is correct!", guess);
            break
        }
        let eliminated = guesser.try_word(guess, tests);
        // println!("Eliminated {} words, {} words remaining", eliminated, guesser.get_possible().len());
    };
    return i;
}

fn algo_v2(mut guesser: WordleGuesser, real_word: Word) -> usize {
    let mut i = 0;
    loop {
        i += 1;
        let Some(guess) = guesser.get_best_guess() else { break };
        // println!("Trying {:?}", guess);
        let tests = real_word.test(guess);
        // println!("{}", tests.iter().map(|t| match t {
        //     Test::Green => '🟩',
        //     Test::Yellow => '🟨',
        //     Test::Gray => '⬛'
        // }).collect::<String>());
        if tests.iter().all(|item| item == &Test::Green) {
            // println!("{:?} is correct!", guess);
            break
        }
        let eliminated = guesser.try_word(guess, tests);
        // println!("Eliminated {} words, {} words remaining", eliminated, guesser.get_possible().len());
    };
    return i;
}

fn main() {
    test_algo(algo_v1, 150);
    test_algo(algo_v2, 150);

    // let real_word = Word::new(['m', 'a', 'h', 'a', 'l']);
    // let mut guesser = WordleGuesser::from_words_file("src/all_words.txt");
    //
    // let all_words = guesser.get_possible().to_vec();
    //
    // loop {
    //     let Some(guess) = guesser.get_best_guess() else { break };
    //     println!("Trying {:?}", guess);
    //     let tests = real_word.test(guess);
    //     println!("{}", tests.iter().map(|t| match t {
    //         Test::Green => '🟩',
    //         Test::Yellow => '🟨',
    //         Test::Gray => '⬛'
    //     }).collect::<String>());
    //     if tests.iter().all(|item| item == &Test::Green) {
    //         println!("{:?} is correct!", guess);
    //         break
    //     }
    //     let eliminated = guesser.try_word(guess, tests);
    //     println!("Eliminated {} words, {} words remaining", eliminated, guesser.get_possible().len());
    //     dbg!(guesser.get_possible());
    // };
}


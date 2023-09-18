mod word;
mod solver;
mod solvers;

use std::io;
use std::io::{stdin, Write};
use crate::solver::WordleGuesser;
use crate::solvers::{Solver, SolverV1, SolverV2, SolverV3};
use crate::word::{Test, Word};

fn get_input() -> String {
    let mut string = String::new();
    stdin().read_line(&mut string).expect("Error: Could not read from stdin.");
    string
}

fn test_solver<S: Solver>(solver_fn: impl Fn() -> S, all_words: impl Iterator<Item=Word>) {
    let (lower, upper) = all_words.size_hint();
    let len = upper.unwrap_or(lower);

    let mut total: usize = 0;
    for (i, word) in all_words.enumerate() {
        total += solver_fn().play(word);
        print!("\rCompleted {}/{}", i + 1, len);
        io::stdout().flush().unwrap();
    }
    let avg = total as f64 / len as f64;
    println!("\navg guesses {}", avg);
}

fn main() {
    let words = Word::list_from_file("src/all_words.txt");

    test_solver(|| SolverV1::from_words(words.clone()), words.iter().copied().step_by(200));
    test_solver(|| SolverV2::from_words(words.clone()), words.iter().copied().step_by(200));
    test_solver(|| SolverV3::from_words(words.clone()), words.iter().copied().step_by(200));

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


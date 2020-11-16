// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;

use std::collections::{HashMap};
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;


const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);
    let mut incorrect = NUM_INCORRECT_GUESSES;
    let mut left_chars: HashMap<String, Vec<usize>> = HashMap::new();
    let mut guessed = String::new();
    let mut left_cnt = secret_word_chars.len();
    let mut sofar = String::from_utf8(vec![b'-'; left_cnt]).expect("Panic");
    for (i, ch) in secret_word_chars.iter().enumerate() {
        let str = ch.to_string();
        left_chars.entry(str).or_default().push(i);
    }

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");
    while incorrect > 0 && left_cnt > 0 {
        println!("\nThe word so far is {}", sofar);
        println!("You have guessed the following letters: {}", guessed);
        println!("You have {} guesses left", incorrect);
        print!("Please guess a letter: ");
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        // pop \n
        guess.pop();
        match left_chars.get_mut(&guess) {
            Some(idxs) if idxs.len() > 0 => {
                // 猜对了
                let idx = idxs.pop().unwrap();
                left_cnt -= 1;
                guessed.push_str(&guess);
                sofar.replace_range(idx..idx + 1, &guess);
            }
            _ => {
                incorrect -= 1;
                println!("Sorry, that letter is not in the word")
            }
        }
    }
    if left_cnt <= 0 {
        println!("\nCongratulations you guessed the secret word: {}!", secret_word);
    } else {
        println!("\nSorry, you ran out of guesses!");
    }
}
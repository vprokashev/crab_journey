mod hidden_word;

use hidden_word::HiddenWord;
use crate::hidden_word::PressState;

struct Game {
    attempts: usize,
}

fn main() {
    let words: [&str; 20] = [
        "monomorphization",
        "polymorphization",
        "contravariance",
        "impredicativity",
        "skolemization",
        "canonicalization",
        "coinduction",
        "unification",
        "normalization",
        "reification",
        "quantification",
        "linearizability",
        "inhabitedness",
        "desugaring",
        "dereferenceability",
        "provenance",
        "discriminants",
        "defunctionalization",
        "devirtualization",
        "scalarization",
    ];

    loop {
        let index = rand::random_range(0..words.len());
        let word = words[index];
        let mut hidden_word: HiddenWord = HiddenWord::new(word);
        let mut game = Game {
            attempts: 4
        };
        let mut input = String::new();

        loop {
            if game.attempts <= 0 {
                println!("The end! No more attempts");
                println!("_______________________");
                break;
            }
            println!("The word is: {}", hidden_word.render_word());
            println!("Press letter and Enter:");

            input.clear();
            if std::io::stdin().read_line(&mut input).expect("Input error") == 0 {
                break;
            }

            match input.trim().parse::<char>() {
                Ok(ch) if ch.is_ascii_alphabetic() => {
                    match hidden_word.press_letter(ch) {
                        PressState::Success => {
                            if hidden_word.is_solved() {
                                println!("You won! The word is {}", word);
                                break;
                            }
                            println!("Letter {ch} is in the word.");
                            println!("Attempts still: {}", game.attempts);
                        }
                        PressState::WasPressed => {
                            println!("You already tried {ch}.");
                            println!("Attempts still: {}", game.attempts);
                        }
                        PressState::Failure => {
                            game.attempts -= 1;
                            if game.attempts == 0 {
                                println!("You lost! The word was {word}.");
                                break;
                            }
                            println!("No {ch}. Attempts left: {}", game.attempts);
                        }
                    }
                }
                _ => println!("Enter one Latin letter."),
            }
        }
    }
}

use std::io::{self, Write};

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const BANNER: &str = "=======================";
const INVALID_INPUT: u8 = 255;

fn char_to_index(c: char) -> u8 {
    let i = c as u8;
    if i < 65 || i > 90 {
        return INVALID_INPUT;
    }
    return i - 65;
}

fn guess_to_index(guess: &String) -> u8 {
    let c: char = match guess.trim().to_uppercase().parse() {
        Ok(ch) => ch,
        Err(_) => return INVALID_INPUT,
    };
    char_to_index(c)
}

fn prompt(msg: &str) {
    let prompt = ">";
    print!("{} {}: ", prompt, msg);
    io::stdout().flush().unwrap();
}

fn generate_word() -> String {
    let word = String::from("SECRET");
    return word;
}

fn solved_word(word: &String, guesses: [bool; 26]) -> bool {
    for c in word.chars() {
        if !guesses[char_to_index(c) as usize] {
            return false;
        }
    }
    return true;
}

fn print_exit_msg(solved: bool, secret: String, misses: u32) {
    if solved {
        println!(
            "🎉🎉 Congraulations! 🎉🎉\nYou solve the word '{}'!",
            secret
        );
    } else {
        println!(
            "Shoot..\nYou've exceeded {} guesses...\nBetter luck next time",
            misses
        );
    }
}

fn main() {
    let secret = generate_word();
    let max_misses = 10;
    let mut solved = false;
    let mut misses = 0;
    let mut guesses = [false; 26];
    let mut guess = String::new();

    while !solved && misses < max_misses {
        guess.clear();
        println!("{}", BANNER);
        prompt("Guess any letter");

        // Get input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");

        // Verify input
        let index = guess_to_index(&guess);
        if index == INVALID_INPUT {
            println!("Please input a letter");
            continue;
        }
        let c = ALPHABET[index as usize];
        println!("You guessed '{}'", guess.trim());

        // React to guess based on state
        if guesses[index as usize] {
            println!("You have already guessed '{}'", c);
            continue;
        } else if secret.contains(guess.to_uppercase().trim()) {
            println!("'{}' is in the word!", c);
        } else {
            misses = misses + 1;
            println!("'{}' is not in the word", c);
        }

        // Update state
        guesses[index as usize] = true;

        // If not solved, print how many guesses are left
        if solved_word(&secret, guesses) {
            solved = true;
        } else {
            println!("You have {} misses left", max_misses - misses);
        }
    }

    print_exit_msg(solved, secret, misses);
}

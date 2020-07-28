use std::io::{self, Write};
use std::process::Command;

use cursive::views::{Dialog, EditView};

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const BANNER: &str = "=======================";
const INVALID_INPUT: u8 = 255;

struct HangmanGame {
    secret: String,
    max_misses: u32,
    prompt: String,
    solved: bool,
    misses: u32,
    guesses: [bool; 26],
}

impl HangmanGame {
    fn new_game() -> HangmanGame {
        let mut game = HangmanGame {
            secret: generate_word().to_uppercase(),
            max_misses: 10,
            solved: false,
            misses: 0,
            guesses: [false; 26],
            prompt: String::new(),
        };
        return game;
    }

    fn solved_word(&self) -> bool {
        for c in self.secret.chars() {
            if !self.guesses[char_to_index(c) as usize] {
                return false;
            }
        }
        return true;
    }

    fn exit_msg(&self) -> String {
        if self.solved {
            String::from(format!(
                "🎉🎉 Congraulations! 🎉🎉\nYou solve the word '{}'!",
                self.secret
            ))
        } else {
            String::from(format!(
                "Shoot..\nYou've exceeded {} guesses...\nBetter luck next time",
                self.misses
            ))
        }
    }

    fn display(&mut self, msg: &str) {
        self.prompt = String::from(msg)
    }

    fn enter_letter(&mut self, guess: String) {
        let guess = guess.to_string();
        let index = guess_to_index(&guess);
        if index == INVALID_INPUT {
            self.display("Please input a valid letter");
            return;
        }
        let c = ALPHABET[index as usize];
        self.display(&format!("You guessed '{}'", guess));

        // React to guess based on state
        if self.guesses[index as usize] {
            self.display(&format!("You have already guessed '{}'", c));
            return;
        } else if self.secret.contains(&guess.to_uppercase()) {
            self.display(&format!("'{}' is in the word!", c));
        } else {
            self.misses = self.misses + 1;
            self.display(&format!("'{}' is not in the word", c));
        }

        // Update state
        self.guesses[index as usize] = true;
    }
}

fn char_to_index(c: char) -> u8 {
    let i = c as u8;
    if i < 65 || i > 90 {
        return INVALID_INPUT;
    }
    return i - 65;
}

fn guess_to_index(guess: &str) -> u8 {
    let c: char = match guess.trim().to_uppercase().parse() {
        Ok(ch) => ch,
        Err(_) => return INVALID_INPUT,
    };
    char_to_index(c)
}

fn prompt(msg: &str) {
    println!("{}", BANNER);
    print!("{} {}: ", ">", msg);
    io::stdout().flush().unwrap();
}

fn generate_word() -> String {
    let dict_file = "/usr/share/dict/american-english";

    let o = Command::new("shuf")
        .arg(dict_file)
        .output()
        .expect("missing")
        .stdout;
    let output = String::from_utf8(o).unwrap();

    for w in output.lines() {
        // Skip words that contain an apostrophe
        if !w.contains("'") {
            return w.to_string();
        }
    }
    return String::from("none");
}

fn build_ui() -> cursive::Cursive {
    let mut ui = cursive::default();
    let mut hangman = HangmanGame::new_game();

    let input_layer = EditView::new().on_submit(|_: &mut cursive::Cursive, guess: &str| {
        hangman.enter_letter(String::from(guess));
    });

    ui.add_layer(
        Dialog::around(input_layer)
            .title("Hangman")
            .button("Quit", |s| s.quit()),
    );
    return ui;
}

fn main() {
    let mut ui = build_ui();
    ui.run();
}

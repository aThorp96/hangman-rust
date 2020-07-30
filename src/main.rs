use std::io::{self, Write};
use std::process::Command;
use unicode_width::UnicodeWidthStr;

use cursive::traits::*;
use cursive::views::{Canvas, Dialog, EditView, LinearLayout, Panel, TextView};
use cursive::Printer;

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const BANNER: &str = "=======================";
const INVALID_INPUT: u8 = 255;
const HANGMAN: [&str; 7] = [
" ╔═══╗\n ║      \n ║      \n ║       \n═╩══",
" ╔═══╗\n ║   0   \n ║      \n ║       \n═╩══",
" ╔═══╗\n ║   0   \n ║   |   \n ║       \n═╩══",
" ╔═══╗\n ║   0   \n ║  /|   \n ║       \n═╩══",
" ╔═══╗\n ║   0   \n ║  /|\\  \n ║       \n═╩══",
" ╔═══╗\n ║   0   \n ║  /|\\  \n ║  /    \n═╩══",
" ╔═══╗\n ║   0   \n ║  /|\\  \n ║  / \\  \n═╩══"];

struct HangmanGame {
    secret: String,
    max_misses: u32,
    prompt: String,
    solved: bool,
    misses: usize,
    guesses: [bool; 26],
    canvas: String,
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
            canvas: String::new(),
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
        self.canvas = String::from(HANGMAN[self.misses]);
        println!("Misses: {}", self.misses);
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
    let mut game = HangmanGame::new_game();

    //let canvas_state = &mut game.canvas
    ui.set_user_data(game);

    let mut input_view = EditView::new()
        .on_submit(|s: &mut cursive::Cursive, guess: &str| {
            // Update game state
            s.with_user_data(|game: &mut HangmanGame| {
                game.enter_letter(String::from(guess));
            });

            // Clear input field
            s.call_on_name("input", |view: &mut EditView| view.set_content(""));

            // Update Canvas
			let mut misses: usize = 0;
            let d: Option<&mut HangmanGame> = s.user_data();
			if let Some(g) = d {
                misses = g.misses;
            };
            s.call_on_name(
                "canvas",
                |view: &mut Canvas<String>| {
                    view.set_draw(move |_, printer| {
            			let mut state = String::from(HANGMAN[misses]);
                        let mut lines = state.lines();
                        let mut i = 0;
                        for l in lines {
                            printer.print((2,i),l);
                            i = i + 1;
                        }
                    })
                }
            );
        })
        .with_name("input");

    let canvas_state = String::from(HANGMAN[0]);
    let mut hangman_view = Canvas::new(canvas_state)
        .with_draw(|state, printer| {
            let mut lines = state.lines();
            let mut i = 0;
            for l in lines {
                printer.print((2,i),l);
                i = i + 1;
            }
        })
        .with_required_size(|text, _constraints| (text.width(), 7).into())
        .with_name("canvas");

    let mut letters_store = Panel::new(TextView::new("alphabet here").with_name("letter_store"));

    ui.add_layer(
        Dialog::new()
            .title("Hangman")
            .content(
                LinearLayout::vertical()
                    .child(hangman_view)
                    .child(input_view)
                    .child(letters_store),
            )
            .button("Quit", |s| s.quit()),
    );
    return ui;
}

fn main() {
    let mut ui = build_ui();
    ui.run();
}

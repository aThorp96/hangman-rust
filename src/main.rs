use std::process::Command;
use unicode_width::UnicodeWidthStr;

use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Canvas, Dialog, EditView, LinearLayout, Panel, TextView};

const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const INVALID_INPUT: u8 = 255;
const HANGMAN: [&str; 7] = [
    " ╔═══╗\n ║    \n ║\n ║\n═╩══",
    " ╔═══╗\n ║   O\n ║\n ║\n═╩══",
    " ╔═══╗\n ║   O\n ║   |\n ║\n═╩══",
    " ╔═══╗\n ║   O\n ║  /|\n ║\n═╩══",
    " ╔═══╗\n ║   O\n ║  /|\\\n ║\n═╩══",
    " ╔═══╗\n ║   O\n ║  /|\\\n ║  /\n═╩══",
    " ╔═══╗\n ║   O\n ║  /|\\\n ║  / \\\n═╩══",
];
const MAX_MISSES: usize = 6;
const CENTER_OFFSET: i32 = 6;

#[derive(Copy, Clone, PartialEq)]
enum GameState {
    Running,
    Lost,
    Won,
}

struct HangmanGame {
    state: GameState,
    secret: String,
    misses: usize,
    guesses: [bool; 26],
    canvas: String,
}

impl HangmanGame {
    fn new_game() -> HangmanGame {
        let game = HangmanGame {
            secret: HangmanGame::generate_word().to_uppercase(),
            misses: 0,
            guesses: [false; 26],
            canvas: String::new(),
            state: GameState::Running,
        };
        return game;
    }

    fn solved_word(&self) -> bool {
        for c in self.secret.chars() {
            if !self.guesses[HangmanGame::char_to_index(c) as usize] {
                return false;
            }
        }
        return true;
    }

    fn letter_store(&self) -> String {
        let mut store = String::new();
        for (i, guessed) in self.guesses.iter().enumerate() {
            if i == 13 {
                store.push('\n')
            };
            if *guessed {
                store.push(ALPHABET[i as usize]);
            } else {
                store.push(' ');
            }
        }
        return store;
    }

    fn enter_letter(&mut self, guess: String) {
        let guess = guess.to_string();
        let index = HangmanGame::guess_to_index(&guess);
        if index == INVALID_INPUT {
            return;
        }

        // React to guess based on state
        if self.guesses[index as usize] {
            return;
        } else if self.secret.contains(&guess.to_uppercase()) {
            // Guess was in word
        } else if self.misses < MAX_MISSES {
            self.misses = self.misses + 1;
        }

        // Update state
        self.guesses[index as usize] = true;
        self.canvas = String::from(HANGMAN[self.misses]);

        // Check if the game is over
        if self.misses == MAX_MISSES {
            self.state = GameState::Lost;
        } else if self.solved_word() {
            self.state = GameState::Won;
        }
    }

    fn get_status(&self) -> String {
        let mut status = String::new();

        for c in self.secret.chars() {
            let i = HangmanGame::char_to_index(c) as usize;
            if self.guesses[i] {
                status.push(c);
            } else {
                status.push('_');
            }
        }
        return status;
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
        HangmanGame::char_to_index(c)
    }

    fn generate_word() -> String {
        let dict_file = "/usr/share/dict/american-english";

        let o = Command::new("shuf")
            .arg(dict_file)
            .output()
            .expect("error")
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
}

fn game_over(s: &mut cursive::Cursive, result: GameState) {
    // Get end message
    let message = match result {
        GameState::Won => "Congratulations! You won!",
        GameState::Lost => "Better luck next time :(",
        _ => "Something has gone terribly wrong...",
    };

    s.pop_layer();
    s.add_layer(
        Dialog::new()
            .title("Hangman")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new(message).h_align(HAlign::Center))
                    .child(TextView::new("").h_align(HAlign::Center)),
            )
            .button("Quit", |s| s.quit()),
    );
}

fn game_tick(s: &mut cursive::Cursive, guess: &str) {
    // Update game state
    s.with_user_data(|game: &mut HangmanGame| {
        game.enter_letter(String::from(guess));
    });

    // Clear input field
    s.call_on_name("input", |view: &mut EditView| view.set_content(""));

    // Get new view state
    let mut state: GameState = GameState::Running;
    let mut misses: usize = 0;
    let mut store = String::new();
    let mut status = String::new();
    let d: Option<&mut HangmanGame> = s.user_data();
    if let Some(g) = d {
        misses = g.misses;
        store = g.letter_store();
        status = g.get_status();
        state = g.state;
    };
    s.call_on_name("canvas", |view: &mut Canvas<String>| {
        view.set_draw(move |_, printer| {
            let state = String::from(HANGMAN[misses]);
            let lines = state.lines();
            for (i, l) in lines.enumerate() {
                printer.print((CENTER_OFFSET, i as i32), l);
            }
        })
    });
    s.call_on_name("letter_store", |view: &mut TextView| {
        view.set_content(store.clone());
    });
    s.call_on_name("status", |view: &mut TextView| {
        view.set_content(status);
    });

    if state != GameState::Running {
        game_over(s, state);
    }
}
fn build_ui() -> cursive::Cursive {
    let mut ui = cursive::default();
    let game = HangmanGame::new_game();
    let secret_status = game.get_status();
    let store = game.letter_store();

    ui.set_user_data(game);

    let input_view = EditView::new().on_submit(game_tick).with_name("input");

    let canvas_state = String::from(HANGMAN[0]);
    let hangman_view = Canvas::new(canvas_state)
        .with_draw(|state, printer| {
            let lines = state.lines();
            for (i, l) in lines.enumerate() {
                printer.print((CENTER_OFFSET, i as i32), l);
            }
        })
        .with_required_size(|text, _constraints| (text.width(), 7).into())
        .with_name("canvas");

    let letter_store = Panel::new(
        TextView::new(store)
            .h_align(HAlign::Center)
            .with_name("letter_store"),
    )
    .title("Guesses");

    let word_display = TextView::new(secret_status)
        .h_align(HAlign::Center)
        .with_name("status");

    ui.add_layer(
        Dialog::new()
            .title("Hangman")
            .content(
                LinearLayout::vertical()
                    .child(hangman_view)
                    .child(word_display)
                    .child(input_view)
                    .child(letter_store),
            )
            .button("Quit", |s| s.quit()),
    );
    return ui;
}

fn main() {
    let mut ui = build_ui();
    ui.run();
}

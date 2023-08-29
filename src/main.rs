mod core;

use core::Game;
use std::io::{self, Write};

fn main() {
    let mut game = init_game();
    clear_screen();
    show_game(&game);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn init_game() -> Game {
    loop {
        let width = read_usize_with_message("Please enter width: ");
        let height = read_usize_with_message("Please enter height: ");
        let mine_count = read_usize_with_message("Please enter mine count: ");

        let maybe_game = Game::new(width, height, mine_count);

        match maybe_game {
            Ok(game) => return game,
            Err(err) => eprintln!("{err}"),
        }
    }
}

fn read_usize_with_message(message: &str) -> usize {
    loop {
        print!("{}", message);
        io::stdout().flush().expect("Cannot flush stdout!");

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Cannot read from stdin!");

        if let Ok(n) = buffer.trim().parse() {
            return n;
        }
    }
}

fn show_game(game: &Game) {
    use core::CellState;

    let print_separator_line = || {
        for _ in 0..game.field_width() {
            print!("+-");
        }
        println!("+");
    };

    print_separator_line();

    for y in 0..game.field_height() {
        for x in 0..game.field_width() {
            print!("|");
            match game.cell_at((x, y)).unwrap() {
                CellState::Hidden => print!("#"),
                CellState::Flagged => print!("!"),
                CellState::Revealed(0) => print!(" "),
                CellState::Revealed(n) => print!("{}", n),
            }
        }
        println!("|");
        print_separator_line();
    }
}

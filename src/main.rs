mod core;

use core::{Game, RevealResult};
use std::io::{self, Write};

fn main() {
    let game = init_game();
    game_loop(game);
}

fn game_loop(mut game: Game) {
    loop {
        clear_screen();
        show_game(&game);

        match read_command() {
            Command::Reveal(x, y) => match game.reveal((x, y)) {
                Ok(RevealResult::Continue) => {}
                Ok(RevealResult::GameOver) => {
                    println!("You lost :(");
                    break;
                }
                Ok(RevealResult::Win) => {
                    println!("Congratulations! You won!");
                    break;
                }
                Err(e) => {
                    println!("{e}");
                }
            },
            Command::ToggleFlag(x, y) => match game.toggle_flag((x, y)) {
                Ok(_) => {}
                Err(e) => {
                    println!("{e}");
                }
            },
        }
    }

    clear_screen();
    show_game(&game);
}

#[derive(Debug)]
enum Command {
    Reveal(usize, usize),
    ToggleFlag(usize, usize),
}

fn read_command() -> Command {
    loop {
        show_prompt();
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Cannot read from stdin!");

        let parts: Vec<_> = buffer.split(' ').collect();

        if parts.len() != 3 {
            println!("Not enough arguments!");
            continue;
        }

        match parts[0] {
            "r" | "reveal" => {
                let x = match parts[1].trim().parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("`{}` is not a number", parts[1]);
                        continue;
                    }
                };

                let y = match parts[2].trim().parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("`{}` is not a number", parts[2]);
                        continue;
                    }
                };

                return Command::Reveal(x, y);
            }
            "f" | "flag" => {
                let x = match parts[1].trim().parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("`{}` is not a number", parts[1]);
                        continue;
                    }
                };

                let y = match parts[2].trim().parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("`{}` is not a number", parts[2]);
                        continue;
                    }
                };

                return Command::ToggleFlag(x, y);
            }
            _ => {
                println!("{} is not a command", parts[0]);
            }
        }
    }
}

fn show_prompt() {
    print!("\n\n> ");
    io::stdout().flush().expect("Cannot flush stdout!");
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
            print!("+---");
        }
        println!("+");
    };

    print_separator_line();

    for y in 0..game.field_height() {
        for x in 0..game.field_width() {
            print!("|");
            match game.cell_at((x, y)).unwrap() {
                CellState::Hidden => print!("###"),
                CellState::Flagged => print!("#!#"),
                CellState::Revealed(0) => print!("   "),
                CellState::Revealed(n) => print!(" {} ", n),
            }
        }
        println!("|");
        print_separator_line();
    }
}

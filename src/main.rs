mod core;

use core::Game;

fn main() {
    let mut game = Game::new(10, 10, 10).unwrap();
    show_game(&game);
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

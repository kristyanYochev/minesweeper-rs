use std::collections::HashSet;
use thiserror::Error;

pub struct Game {
    width: usize,
    height: usize,
    mines: Vec<Coords>,
    cells: Vec<CellState>,
}

pub type Coords = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RevealResult {
    GameOver,
    Continue,
    Win,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellState {
    Hidden,
    Flagged,
    Revealed(u8),
}

#[derive(Debug, Error)]
#[error("Too many mines ({mines}) for a game with field size of {width} by {height}")]
pub struct TooManyMines {
    mines: usize,
    width: usize,
    height: usize,
}

#[derive(Debug, Error)]
#[error("Invalid coordinates {coords:?} for a game with field size of {width} by {height}")]
pub struct InvalidCoords {
    coords: Coords,
    width: usize,
    height: usize,
}

#[derive(Debug, Error)]
pub enum PlaceError {
    #[error(transparent)]
    InvalidCoords(#[from] InvalidCoords),
    #[error("Mine already at {0:?}")]
    MineAlreadyAt(Coords),
}

impl Game {
    pub fn new(
        field_width: usize,
        field_height: usize,
        mine_count: usize,
    ) -> Result<Self, TooManyMines> {
        if mine_count > field_width * field_height {
            Err(TooManyMines {
                width: field_width,
                height: field_height,
                mines: mine_count,
            })
        } else {
            Ok(Self {
                width: field_width,
                height: field_height,
                mines: generate_random_mines(field_width, field_height, mine_count),
                cells: vec![CellState::Hidden; field_width * field_height],
            })
        }
    }

    pub fn empty(field_width: usize, field_height: usize) -> Self {
        Self {
            width: field_width,
            height: field_height,
            mines: Vec::new(),
            cells: vec![CellState::Hidden; field_width * field_height],
        }
    }

    pub fn place_mine(&mut self, at: Coords) -> Result<(), PlaceError> {
        self.index(at)?;

        if self.is_mine_at(at) {
            Err(PlaceError::MineAlreadyAt(at))
        } else {
            self.mines.push(at);
            Ok(())
        }
    }

    pub fn reveal(&mut self, at: Coords) -> Result<RevealResult, InvalidCoords> {
        todo!();
    }

    pub fn cell_at(&self, at: Coords) -> Result<CellState, InvalidCoords> {
        Ok(self.cells[self.index(at)?])
    }

    pub fn mine_count(&self) -> usize {
        self.mines.len()
    }

    pub fn field_width(&self) -> usize {
        self.width
    }

    pub fn field_height(&self) -> usize {
        self.height
    }

    fn is_mine_at(&self, at: Coords) -> bool {
        self.mines.contains(&at)
    }

    fn index(&self, coords: Coords) -> Result<usize, InvalidCoords> {
        if coords.0 < self.width && coords.1 < self.height {
            Ok(self.width * coords.1 + coords.0)
        } else {
            Err(InvalidCoords {
                coords,
                width: self.width,
                height: self.height,
            })
        }
    }
}

fn generate_random_mines(width: usize, height: usize, count: usize) -> Vec<Coords> {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    let mut generated = Vec::with_capacity(count);

    while generated.len() < count {
        let new_coords = (rng.gen_range(0..width), rng.gen_range(0..height));
        if !generated.contains(&new_coords) {
            generated.push(new_coords);
        }
    }

    generated
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_all_fields<F>(width: usize, height: usize, predicate: F)
    where
        F: Fn((usize, usize)) -> bool,
    {
        for x in 0..width {
            for y in 0..height {
                assert!(predicate((x, y)));
            }
        }
    }

    #[test]
    fn creation() {
        let game1 = Game::new(10, 15, 5).unwrap();
        assert_eq!(game1.field_width(), 10);
        assert_eq!(game1.field_height(), 15);
        assert_eq!(game1.mine_count(), 5);

        assert_all_fields(game1.field_width(), game1.field_height(), |coords| {
            game1.cell_at(coords).unwrap() == CellState::Hidden
        });

        let game2 = Game::empty(20, 30);
        assert_eq!(game2.field_width(), 20);
        assert_eq!(game2.field_height(), 30);
        assert_eq!(game2.mine_count(), 0);

        assert_all_fields(game2.field_width(), game2.field_height(), |coords| {
            game2.cell_at(coords).unwrap() == CellState::Hidden
        });

        let game3 = Game::new(1, 1, 5);
        assert!(matches!(game3, Err(TooManyMines { .. })))
    }

    #[test]
    fn mine_placement() {
        let mut game = Game::empty(5, 5);

        assert_eq!(game.mine_count(), 0);
        game.place_mine((2, 3)).unwrap();
        assert_eq!(game.mine_count(), 1);

        let res = game.place_mine((10, 15));
        assert!(res.is_err());
        assert!(matches!(
            res,
            Err(PlaceError::InvalidCoords(InvalidCoords { .. }))
        ));

        let res = game.place_mine((2, 3));
        assert!(res.is_err());
        assert!(matches!(res, Err(PlaceError::MineAlreadyAt((2, 3)))))
    }

    #[test]
    fn reveal_basics() {
        let mut empty_game = Game::empty(10, 10);
        assert_eq!(empty_game.mine_count(), 0);
        assert_eq!(empty_game.reveal((2, 3)).unwrap(), RevealResult::Win);

        let mut one_mine_game = Game::empty(10, 10);
        one_mine_game.place_mine((2, 3)).unwrap();
        assert_eq!(
            one_mine_game.reveal((2, 3)).unwrap(),
            RevealResult::GameOver
        );
    }
}

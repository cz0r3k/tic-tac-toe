use array2d::Array2D;
use std::cmp;
use strum::IntoEnumIterator;

pub use crate::errors::GameError;
pub use crate::player_enum::PlayerEnum;

const ROW: usize = 3;
const COLUMN: usize = 3;

pub struct Game {
    board: Array2D<Option<PlayerEnum>>,
    turn: PlayerEnum,
    winner: Option<PlayerEnum>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: Array2D::filled_with(None, ROW, COLUMN),
            turn: PlayerEnum::X,
            winner: None,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }
    fn check_rows(&self) -> Option<PlayerEnum> {
        for player in PlayerEnum::iter() {
            for row in self.board.rows_iter() {
                if row.into_iter().all(|&x| x == Some(player)) {
                    return Some(player);
                }
            }
        }
        None
    }
    fn check_columns(&self) -> Option<PlayerEnum> {
        for player in PlayerEnum::iter() {
            for column in self.board.columns_iter() {
                if column.into_iter().all(|&x| x == Some(player)) {
                    return Some(player);
                }
            }
        }
        None
    }
    fn check_diagonals(&self) -> Option<PlayerEnum> {
        for player in PlayerEnum::iter() {
            if (0..cmp::min(self.board.column_len(), self.board.row_len()))
                .all(|i| *self.board.get(i, i).unwrap() == Some(player))
            {
                return Some(player);
            }
            if (0..cmp::min(self.board.column_len(), self.board.row_len()))
                .all(|i| *self.board.get(i, COLUMN - i - 1).unwrap() == Some(player))
            {
                return Some(player);
            }
        }
        None
    }
    fn check_all(&self) -> Option<PlayerEnum> {
        let rows = self.check_rows();
        let columns = self.check_columns();
        let diagonals = self.check_diagonals();
        if rows.is_some() {
            return rows;
        }
        if columns.is_some() {
            return columns;
        }
        if diagonals.is_some() {
            return diagonals;
        }
        None
    }
    fn set_winner(&mut self) {
        self.winner = self.check_all();
    }
    pub fn get_winner(&self) -> Option<PlayerEnum> {
        self.winner
    }
    pub fn get_turn(&self) -> PlayerEnum {
        self.turn
    }
    pub fn make_move(&mut self, x: usize, y: usize, player: PlayerEnum) -> Result<(), GameError> {
        if self.get_winner() != None {
            return Err(GameError::GameEnded);
        }
        if player != self.turn {
            return Err(GameError::DifferentPlayerTurn);
        }
        if self.board.get(x, y) != None {
            return Err(GameError::FieldOccupied);
        }
        if self.board.set(x, y, Some(self.turn)).is_err() {
            return Err(GameError::OutOfBounds);
        }
        self.set_winner();
        Ok(())
    }
}

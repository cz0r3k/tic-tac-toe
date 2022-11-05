use crate::errors::GameError;
use crate::player_enum::PlayerEnum;
use array2d::Array2D;
use std::cmp;
use strum::IntoEnumIterator;

const ROW: usize = 3;
const COLUMN: usize = 3;

pub struct GameBoard {
    board: Array2D<Option<PlayerEnum>>,
}
impl Default for GameBoard {
    fn default() -> Self {
        GameBoard {
            board: Array2D::filled_with(None, ROW, COLUMN),
        }
    }
}
impl GameBoard {
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
    pub fn check_all(&self) -> Option<PlayerEnum> {
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
    pub fn get(&self, x: usize, y: usize) -> Result<&Option<PlayerEnum>, GameError> {
        match self.board.get(x, y) {
            Some(player_enum) => Ok(player_enum),
            None => Err(GameError::OutOfBounds),
        }
    }
    pub fn set(&mut self, x: usize, y: usize, player: Option<PlayerEnum>) -> Result<(), GameError> {
        match self.board.set(x, y, player) {
            Ok(..) => Ok(()),
            Err(..) => Err(GameError::OutOfBounds),
        }
    }
}

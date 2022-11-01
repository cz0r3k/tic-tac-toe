use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum GameError {
    #[snafu(display("Different player turn"))]
    DifferentPlayerTurn,
    #[snafu(display("Out of bounds"))]
    OutOfBounds,
    #[snafu(display("Field is just occupied"))]
    FieldOccupied,
    #[snafu(display("Game is just ended"))]
    GameEnded,
}

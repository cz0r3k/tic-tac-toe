use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum GameError {
    #[snafu(display("Diffrent player turn"))]
    DiffrentPlayerTurn,
    #[snafu(display("Diffrent player turn"))]
    OutOfBounds,
    #[snafu(display("Field is just occipied"))]
    FieldOccupied,
    #[snafu(display("Game is just ended"))]
    GameEnded,
}

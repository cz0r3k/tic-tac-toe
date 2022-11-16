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

pub fn map_error_enum_on_string(game_error: &GameError) -> String {
    match game_error {
        GameError::GameEnded => "GE".to_string(),
        GameError::FieldOccupied => "FO".to_string(),
        GameError::OutOfBounds => "OOB".to_string(),
        GameError::DifferentPlayerTurn => "DPT".to_string(),
    }
}

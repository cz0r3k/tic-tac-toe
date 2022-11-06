use crate::errors::GameError;
use crate::game_board::GameBoard;
use crate::player_enum::PlayerEnum;

pub struct Game {
    board: GameBoard,
    turn: PlayerEnum,
    winner: Option<PlayerEnum>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: GameBoard::default(),
            turn: PlayerEnum::X,
            winner: None,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }
    fn set_winner(&mut self) {
        self.winner = self.board.check_all();
    }
    fn change_player_turn(&mut self) {
        match self.turn {
            PlayerEnum::X => {
                self.turn = PlayerEnum::O;
            }
            PlayerEnum::O => {
                self.turn = PlayerEnum::X;
            }
        }
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
        if *self.board.get(x, y)? != None {
            return Err(GameError::FieldOccupied);
        }
        self.board.set(x, y, Some(self.turn))?;
        self.set_winner();
        self.change_player_turn();
        Ok(())
    }
}

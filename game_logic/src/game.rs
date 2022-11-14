use crate::errors::GameError;
use crate::game_board::GameBoard;
use crate::player_enum::*;
use crate::player_move::Move;

#[derive(Clone)]
pub struct Game {
    board: GameBoard,
    turn: PlayerEnum,
    winner: Option<PlayerEnum>,
    end: bool,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: GameBoard::default(),
            turn: PlayerEnum::X,
            winner: None,
            end: false,
        }
    }
}

impl From<String> for Game {
    fn from(s: String) -> Self {
        let mut it = s.split(';');
        let turn = it
            .next()
            .unwrap()
            .chars()
            .map(map_char_on_player_enum)
            .take(1)
            .next()
            .unwrap();
        let winner = it
            .next()
            .unwrap()
            .chars()
            .map(map_char_on_option_player_enum)
            .take(1)
            .next()
            .unwrap();
        let end = it
            .next()
            .unwrap()
            .chars()
            .map(|x| match x {
                '0' => false,
                '1' => true,
                _ => false,
            })
            .take(1)
            .next()
            .unwrap();
        let board = GameBoard::from(String::from(it.next().unwrap()));
        Game {
            board,
            turn,
            winner,
            end,
        }
    }
}
impl From<Game> for String {
    fn from(game: Game) -> Self {
        let board: String = game.board.into();
        let turn = map_player_enum_on_char(&game.turn);
        let winner = map_option_player_enum_on_char(&game.winner);
        let end = match game.end {
            true => '1',
            false => '0',
        };
        format!("{};{};{};{}", turn, winner, end, board)
    }
}

impl Game {
    pub fn new(x: usize, y: usize) -> Self {
        Game {
            board: GameBoard::new(x, y),
            turn: PlayerEnum::X,
            winner: None,
            end: false,
        }
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
    pub fn get_end(&self) -> bool {
        self.end
    }
    fn set_end(&mut self) {
        self.set_winner();
        if self.winner.is_some() || self.board.check_full() {
            self.end = true;
        }
    }
    pub fn player_surender(&mut self, player: &PlayerEnum) -> Result<(), GameError> {
        self.end = true;
        self.winner = Some(different_player_enum(player));
        Ok(())
    }
    pub fn get_turn(&self) -> PlayerEnum {
        self.turn
    }
    pub fn make_move(&mut self, player_move: Move) -> Result<(), GameError> {
        if self.get_winner().is_some() || self.get_end() {
            return Err(GameError::GameEnded);
        }
        if player_move.get_player_option() != Some(self.turn) {
            return Err(GameError::DifferentPlayerTurn);
        }
        if (self.board.get(&player_move)?).is_some() {
            return Err(GameError::FieldOccupied);
        }
        self.board.set(&player_move)?;
        self.set_end();
        self.change_player_turn();
        Ok(())
    }
}

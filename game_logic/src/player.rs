use crate::errors::GameError;
use crate::game::Game;
use crate::player_enum::PlayerEnum;

pub struct Player {
    player_enum: PlayerEnum,
}

impl Player {
    pub fn new(player_enum: PlayerEnum) -> Self {
        Player { player_enum }
    }
    pub fn make_move(&self, x: usize, y: usize, game: &mut Game) -> Result<(), GameError> {
        game.make_move(x, y, self.player_enum)
    }
    pub fn get_winner(&self, game: &Game) -> Option<PlayerEnum> {
        game.get_winner()
    }
    pub fn get_turn(&self, game: &Game) -> PlayerEnum {
        game.get_turn()
    }
}

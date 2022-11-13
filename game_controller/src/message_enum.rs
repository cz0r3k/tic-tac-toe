use game_logic::errors::GameError;
use game_logic::player_enum::PlayerEnum;
use game_logic::player_move::Move;

pub enum Message {
    _Move(Move),
    _UpdateBoard,
    _GameEnded,
    _Win(PlayerEnum),
    _Error(GameError),
}

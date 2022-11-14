use game_logic::errors::GameError;
use game_logic::player_enum::PlayerEnum;
use game_logic::player_move::Move;

pub enum MessageFromGame {
    UpdateBoard,      //UB|'Game Signature'
    GameEnded,        //D
    PlayerTurn,       //PT
    Win(PlayerEnum),  //W|X W|O
    Error(GameError), //GE|'GameError Signature'
}
pub enum MessageFromPlayer {
    Move(Move),         //M|'Move Signature'
    GiveUp(PlayerEnum), //GV|O GV|X
}

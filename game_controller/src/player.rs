use crate::message_enum::Message;
use game_logic::game::Game;
use game_logic::player_enum::PlayerEnum;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};

pub struct PlayerController {
    game: Arc<RwLock<Game>>,
    from_player: Sender<Message>,
    to_player: Receiver<Message>,
    player_enum: PlayerEnum,
    stream: TcpStream,
}

impl PlayerController {
    pub fn new(
        game: Arc<RwLock<Game>>,
        from_player: Sender<Message>,
        to_player: Receiver<Message>,
        player_enum: PlayerEnum,
        stream: TcpStream,
    ) -> PlayerController {
        PlayerController {
            game,
            from_player,
            to_player,
            player_enum,
            stream,
        }
    }
    pub fn run(player: PlayerController) {}
}

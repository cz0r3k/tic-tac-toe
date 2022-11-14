use crate::message_enum::{MessageFromGame, MessageFromPlayer};
use game_logic::game::Game;
use game_logic::player_enum::PlayerEnum;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};

pub struct PlayerController {
    game: Arc<RwLock<Game>>,
    from_player: Sender<MessageFromPlayer>,
    to_player: Receiver<MessageFromGame>,
    player_enum: PlayerEnum,
    stream: TcpStream,
}

impl PlayerController {
    pub fn new(
        game: Arc<RwLock<Game>>,
        from_player: Sender<MessageFromPlayer>,
        to_player: Receiver<MessageFromGame>,
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
    pub fn run(_player: PlayerController) {}

    pub fn map_from_message(&mut self, message: MessageFromGame) {
        match message {
            MessageFromGame::UpdateBoard => {
                let game = (*self.game.read().unwrap()).clone();
                let _err = self.stream.write(String::from(game).as_ref()).unwrap();
            }
            MessageFromGame::GameEnded => {}
            _ => {}
        }
    }
}

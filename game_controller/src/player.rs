use crate::message_enum::{MessageFromGame, MessageFromPlayer};
use game_logic::game::Game;
use game_logic::player_enum::PlayerEnum;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};

pub struct PlayerController {
    game: Arc<RwLock<Game>>,
    _from_player: Sender<MessageFromPlayer>,
    _to_player: Receiver<MessageFromGame>,
    _player_enum: PlayerEnum,
    stream: TcpStream,
}

impl PlayerController {
    pub fn new(
        game: Arc<RwLock<Game>>,
        _from_player: Sender<MessageFromPlayer>,
        _to_player: Receiver<MessageFromGame>,
        _player_enum: PlayerEnum,
        stream: TcpStream,
    ) -> PlayerController {
        PlayerController {
            game,
            _from_player,
            _to_player,
            _player_enum,
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
    pub fn get_player_enum(&self) -> PlayerEnum {
        self._player_enum
    }
}

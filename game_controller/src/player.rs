use crate::message_enum::MessageFromPlayer::{GiveUp, Move};
use crate::message_enum::{MessageFromGame, MessageFromPlayer};
use game_logic::errors::map_error_enum_on_string;
use game_logic::game::Game;
use game_logic::player_enum::{map_player_enum_on_char, PlayerEnum};
use game_logic::player_move::Move as PlayerMove;
use regex::Regex;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};

pub struct PlayerController {
    game: Arc<RwLock<Game>>,
    _from_player: Sender<MessageFromPlayer>,
    to_player: Receiver<MessageFromGame>,
    _player_enum: PlayerEnum,
    stream: TcpStream,
}

impl PlayerController {
    pub fn new(
        game: Arc<RwLock<Game>>,
        _from_player: Sender<MessageFromPlayer>,
        to_player: Receiver<MessageFromGame>,
        _player_enum: PlayerEnum,
        stream: TcpStream,
    ) -> PlayerController {
        PlayerController {
            game,
            _from_player,
            to_player,
            _player_enum,
            stream,
        }
    }
    pub fn run(&mut self) {
        loop {
            let message = self.to_player.recv();
            match message {
                Ok(message) => match message {
                    MessageFromGame::PlayerTurn => {
                        let _err = self
                            .stream
                            .write(PlayerController::map_from_message(self, &message).as_ref());
                        let mut message_string = String::new();
                        let _res = self.stream.read_to_string(&mut message_string);
                        let _mess = Self::map_to_message(message_string);
                        todo!();
                    }
                    message => {
                        let _err = self
                            .stream
                            .write(PlayerController::map_from_message(self, &message).as_ref());
                    }
                },
                Err(_) => break,
            }
        }
    }

    fn map_from_message(&self, message: &MessageFromGame) -> String {
        match message {
            MessageFromGame::UpdateBoard => {
                let game = self.game.read().unwrap().clone();
                format!("UB|{}", String::from(game))
            }
            MessageFromGame::GameEnded => "D".to_string(),
            MessageFromGame::PlayerTurn => "PT".to_string(),
            MessageFromGame::Win(winner) => {
                format!("W|{}", map_player_enum_on_char(winner))
            }
            MessageFromGame::Error(game_error) => {
                format!("GE|{}", map_error_enum_on_string(game_error))
            }
            MessageFromGame::GameError => "ER".to_string(),
        }
    }
    fn map_to_message(s: String) -> MessageFromPlayer {
        let re1 = Regex::new(r"GE\|(.+)").unwrap();
        let re2 = Regex::new(r"(M\|.+)").unwrap();
        if re1.is_match(&s) {
            let res = re1.captures(&s).unwrap();
            return GiveUp(PlayerEnum::from(String::from(res.get(0).unwrap().as_str())));
        }
        if re2.is_match(&s) {
            let res = re2.captures(&s).unwrap();
            return Move(PlayerMove::from(String::from(res.get(0).unwrap().as_str())));
        }
        todo!()
    }
    pub fn get_player_enum(&self) -> PlayerEnum {
        self._player_enum
    }
}

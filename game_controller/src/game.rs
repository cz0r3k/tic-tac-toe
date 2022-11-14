use crate::message_enum::MessageFromGame;
use crate::message_enum::MessageFromPlayer;
use crate::player::PlayerController;
use game_logic::errors::GameError;
use game_logic::game::Game;
use game_logic::player_enum::{different_player_enum, PlayerEnum};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;

pub struct GameController {
    game: Arc<RwLock<Game>>,
    to_player1: Sender<MessageFromGame>,
    to_player2: Sender<MessageFromGame>,
    from_player1: Receiver<MessageFromPlayer>,
    from_player2: Receiver<MessageFromPlayer>,
    player1: Option<PlayerController>,
    player2: Option<PlayerController>,
}

impl GameController {
    pub fn new(player1_stream: TcpStream, player2_stream: TcpStream) -> GameController {
        let (tx_from_player1, rx_from_player1) = channel::<MessageFromPlayer>();
        let (tx_from_player2, rx_from_player2) = channel::<MessageFromPlayer>();
        let (tx_to_player1, rx_to_player1) = channel::<MessageFromGame>();
        let (tx_to_player2, rx_to_player2) = channel::<MessageFromGame>();
        let game = Arc::new(RwLock::new(Game::default()));
        let mut players_enum = [PlayerEnum::X, PlayerEnum::O];
        players_enum.shuffle(&mut thread_rng());

        let player1 = Some(PlayerController::new(
            game.clone(),
            tx_from_player1,
            rx_to_player1,
            players_enum[0],
            player1_stream,
        ));
        let player2 = Some(PlayerController::new(
            game.clone(),
            tx_from_player2,
            rx_to_player2,
            players_enum[1],
            player2_stream,
        ));

        GameController {
            game,
            to_player1: tx_to_player1,
            to_player2: tx_to_player2,
            from_player1: rx_from_player1,
            from_player2: rx_from_player2,
            player1,
            player2,
        }
    }
    pub fn run(&mut self) {
        let player1 = std::mem::replace(&mut self.player1, None);
        let player2 = std::mem::replace(&mut self.player2, None);

        thread::spawn(move || PlayerController::run(player1.unwrap()));
        thread::spawn(move || PlayerController::run(player2.unwrap()));
    }
    pub fn map_from_message(&mut self, message: MessageFromPlayer) -> Result<(), GameError> {
        match message {
            MessageFromPlayer::Move(m) => self.game.write().unwrap().make_move(m),
            MessageFromPlayer::GiveUp(pe) => self.game.write().unwrap().player_surender(&pe),
        }
    }
}

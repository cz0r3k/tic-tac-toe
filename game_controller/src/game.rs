use crate::message_enum::MessageFromGame;
use crate::message_enum::MessageFromPlayer;
use crate::player::PlayerController;
use game_logic::errors::GameError;
use game_logic::game::Game;
use game_logic::player_enum::PlayerEnum;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub struct GameController {
    game: Arc<RwLock<Game>>,
    to_player: [Sender<MessageFromGame>; 2],
    from_player: [Receiver<MessageFromPlayer>; 2],
    player: [Option<PlayerController>; 2],
}

impl GameController {
    pub fn new(player0_stream: TcpStream, player1_stream: TcpStream) -> GameController {
        let (tx_from_player1, rx_from_player1) = channel::<MessageFromPlayer>();
        let (tx_from_player2, rx_from_player2) = channel::<MessageFromPlayer>();
        let (tx_to_player1, rx_to_player1) = channel::<MessageFromGame>();
        let (tx_to_player2, rx_to_player2) = channel::<MessageFromGame>();
        let game = Arc::new(RwLock::new(Game::default()));
        let mut players_enum = [PlayerEnum::X, PlayerEnum::O];
        players_enum.shuffle(&mut thread_rng());

        let player0 = Some(PlayerController::new(
            game.clone(),
            tx_from_player1,
            rx_to_player1,
            players_enum[0],
            player0_stream,
        ));
        let player1 = Some(PlayerController::new(
            game.clone(),
            tx_from_player2,
            rx_to_player2,
            players_enum[1],
            player1_stream,
        ));

        GameController {
            game,
            to_player: [tx_to_player1, tx_to_player2],
            from_player: [rx_from_player1, rx_from_player2],
            player: [player0, player1],
        }
    }
    pub fn run(&mut self) {
        let player1 = std::mem::replace(&mut self.player[0], None);
        let player2 = std::mem::replace(&mut self.player[1], None);

        let mut start_player = 0;
        for i in 0..=1 {
            if self.player[i].as_ref().unwrap().get_player_enum() == PlayerEnum::X {
                start_player = i;
                break;
            }
        }

        let join_handle1 = thread::spawn(move || player1.unwrap().run());
        let join_handle2 = thread::spawn(move || player2.unwrap().run());

        self.to_player[start_player]
            .send(MessageFromGame::PlayerTurn)
            .unwrap();

        for i in 0..=1 {
            self.to_player[i]
                .send(MessageFromGame::UpdateBoard)
                .unwrap();
        }

        while !(*self.game.read().unwrap()).get_end() {
            for i in 0..=1 {
                let message = self.from_player[i].try_recv();
                if GameController::map_message(self, &message, i).is_some() {
                    self.to_player[(i + 1) % 2]
                        .send(MessageFromGame::PlayerTurn)
                        .unwrap();
                    continue;
                }
            }
            thread::sleep(Duration::from_millis(100));
        }

        if let Some(winner) = (*self.game.read().unwrap()).get_winner() {
            self.to_player[0]
                .send(MessageFromGame::Win(winner))
                .unwrap();
            self.to_player[1]
                .send(MessageFromGame::Win(winner))
                .unwrap();
        } else {
            self.to_player[0].send(MessageFromGame::GameEnded).unwrap();
            self.to_player[1].send(MessageFromGame::GameEnded).unwrap();
        }

        let _res1 = join_handle1.join();
        let _res2 = join_handle2.join();
    }
    fn map_from_message(&mut self, message: &MessageFromPlayer) -> Result<(), GameError> {
        match message {
            MessageFromPlayer::Move(m) => self.game.write().unwrap().make_move(m),
            MessageFromPlayer::GiveUp(pe) => self.game.write().unwrap().player_surender(pe),
        }
    }
    fn map_message(
        &mut self,
        message: &Result<MessageFromPlayer, TryRecvError>,
        player_num: usize,
    ) -> Option<()> {
        match message {
            Ok(message) => {
                let result = GameController::map_from_message(self, message);
                match result {
                    Ok(()) => return Some(()),
                    Err(game_error) => {
                        self.to_player[player_num]
                            .send(MessageFromGame::Error(game_error))
                            .unwrap();
                    }
                }
            }
            Err(err) => match err {
                TryRecvError::Empty => {}
                TryRecvError::Disconnected => {
                    self.to_player[0].send(MessageFromGame::GameError).unwrap();
                    self.to_player[1].send(MessageFromGame::GameError).unwrap();
                }
            },
        }
        None
    }
}

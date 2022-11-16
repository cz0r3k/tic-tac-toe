use crate::player_enum::PlayerEnum;

pub struct Move {
    x: usize,
    y: usize,
    player: PlayerEnum,
}

impl From<String> for Move {
    fn from(_s: String) -> Self {
        todo!()
    }
}

impl From<Move> for String {
    fn from(_player_move: Move) -> Self {
        todo!()
    }
}

impl Move {
    pub fn new(x: usize, y: usize, player: PlayerEnum) -> Move {
        Move { x, y, player }
    }
    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn get_y(&self) -> usize {
        self.y
    }
    pub fn get_player_option(&self) -> Option<PlayerEnum> {
        Some(self.player)
    }
}

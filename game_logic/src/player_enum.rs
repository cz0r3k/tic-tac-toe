use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum PlayerEnum {
    X,
    O,
}

impl From<char> for PlayerEnum {
    fn from(ch: char) -> Self {
        match ch {
            'O' => PlayerEnum::O,
            'X' => PlayerEnum::X,
            _ => panic!(),
        }
    }
}
pub fn map_char_on_player_enum(ch: char) -> PlayerEnum {
    match ch {
        'O' | 'X' => PlayerEnum::from(ch),
        _ => panic!(),
    }
}
pub fn map_char_on_option_player_enum(ch: char) -> Option<PlayerEnum> {
    match ch {
        'N' => None,
        'O' | 'X' => Some(PlayerEnum::from(ch)),
        _ => panic!(),
    }
}

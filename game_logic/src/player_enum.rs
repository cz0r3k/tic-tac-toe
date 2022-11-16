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
impl From<String> for PlayerEnum {
    fn from(s: String) -> Self {
        match s.as_str() {
            "O" => PlayerEnum::O,
            "X" => PlayerEnum::X,
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
pub fn map_player_enum_on_char(player: &PlayerEnum) -> char {
    match player {
        PlayerEnum::X => 'X',
        PlayerEnum::O => 'O',
    }
}
pub fn map_char_on_option_player_enum(ch: char) -> Option<PlayerEnum> {
    match ch {
        'N' => None,
        'O' | 'X' => Some(PlayerEnum::from(ch)),
        _ => panic!(),
    }
}
pub fn map_option_player_enum_on_char(player: &Option<PlayerEnum>) -> char {
    match player {
        Some(PlayerEnum::X) => 'X',
        Some(PlayerEnum::O) => 'O',
        None => 'N',
    }
}
pub fn from_num_to_player_enum(num: usize) -> Option<PlayerEnum> {
    match num {
        0 => Some(PlayerEnum::X),
        1 => Some(PlayerEnum::O),
        _ => None,
    }
}
pub fn different_player_enum(player: &PlayerEnum) -> PlayerEnum {
    match player {
        PlayerEnum::X => PlayerEnum::O,
        PlayerEnum::O => PlayerEnum::X,
    }
}

use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum PlayerEnum {
    X,
    O,
}
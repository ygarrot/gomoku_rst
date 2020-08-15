use super::player::Player;

#[derive(Debug)]
pub struct Move {
    pub player: Player,
    pub x: u32,
    pub y: u32
}
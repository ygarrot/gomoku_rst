#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub name: String,
    pub is_ai: bool,
    pub free_threes: u8,
}
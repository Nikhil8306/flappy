use crate::utils::Transform;

pub struct Sprite {
    pub sprite: Vec<char>,
    pub height: u32,
    pub width: u32,
    pub render: bool,
    pub transform: Transform,
}
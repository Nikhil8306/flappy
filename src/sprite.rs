use crate::utils::Transform;

pub struct Sprite {
    pub name: String,
    pub sprite: Vec<char>,
    pub height: u32,
    pub width: u32,
    pub transform: Transform,
}

use crate::utils::Transform;
use crate::{GameState, Status}; 
use crate::sprite::Sprite;

pub struct GameObject {

    pub name: Option<String>,
    pub tag: Option<String>,
    pub sprite: Vec<char>,
    pub transform: Transform,

    pub start: fn(GameState) -> Status,
    pub update: fn(GameState) -> Status,
    pub fixedUpdate: fn(GameState) -> Status,
}
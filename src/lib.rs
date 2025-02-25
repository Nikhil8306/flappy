#![allow(non_snake_case)]


pub mod render;
pub mod gameobject;
pub mod input;
pub mod time;
pub mod scene;
pub mod utils;
pub mod sprite;

use input::Input;
use time::Time;
use render::Window;
use scene::Scene;


// For game loop
pub trait Runnable {

    fn start(&mut self, gameState: &mut GameState) {
        
    }
    
    fn update(&mut self, gameState: &mut GameState) {
        
    }

}

// Enum to control the state of main game loop

pub enum Status {
    Ok,
    Quit,
    ChangeScene(Scene),
}

// Game State to store the current state of game such as Time and Input
pub struct GameState {

    time: Time,
    input: Input

}

impl GameState {

    fn new() -> Self {
        todo!("To Make new");
    }


    // To update the state of game
    fn update(&self) {
        todo!("Make update calls for the game state variables");
    }

}


pub struct Game {

    gameState: GameState,   
    scene: Scene,

}




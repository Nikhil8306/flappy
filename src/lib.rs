#![allow(non_snake_case)]


pub mod render;
pub mod input;
pub mod time;
pub mod utils;
pub mod sprite;

use input::Input;
use time::Time;
use render::{Window};

pub trait Runnable {

    // Check if the script should run (default is true)
    fn run(&self) -> bool {
        return true;
    }

    fn start(&mut self, gameState: &mut GameState, window: &Window) {

    }   

    fn update(&mut self, gameState: &mut GameState, window: &Window) {

    }

    fn fixedUpdate(&mut self, gameState: &mut GameState, window: &Window) {

    }

}

pub struct GameState {
    pub input: Input,
    pub time: Time,
    onPlay: bool,
}

impl GameState {
    fn new() -> Self {
        return Self {
            input: Input::new(),
            time: Time::new(),
            onPlay: false,
        }
    }

    fn init(&mut self) {
        self.time.init();
        // self.input.init();
    }

    fn update(&mut self) {

        // Updating the time and input
        self.time.updateDeltaTime();
        // self.input.update();   

    }

    pub fn stop(&mut self) {
        self.onPlay = false;
    }
}


pub struct Game {

    gameState: GameState,
    window: Window,
    scripts: Vec<Box<dyn Runnable>>
}
    
impl Game {

    pub fn new() -> Self {
        return Self {
            
            gameState: GameState::new(),
            window: Window::default(),
            scripts: Vec::new(),

        };
    }

    pub fn addScript(&mut self, script: Box<dyn Runnable>) {
        self.scripts.push(script);
    }


    // Running the game
    pub fn run(&mut self) 
    {
        
        // Initalizing the game
        // self.window.init();
        self.gameState.init();

        // Loading the start function 
        for script in self.scripts.iter_mut() {
            script.start(&mut self.gameState, &self.window);
        }

        self.gameState.onPlay = true;
        'update: while self.gameState.onPlay {
            
            // Updating the game state
            self.gameState.update();


            // Running the action
            for script in self.scripts.iter_mut() {
                if !script.run() {
                    continue;
                }
                
                script.update(&mut self.gameState, &self.window);

                // Fixed Update
                while self.gameState.time.updateFixed() {
                    script.fixedUpdate(&mut self.gameState, &self.window);
                }
            }
            
        }

    }

}


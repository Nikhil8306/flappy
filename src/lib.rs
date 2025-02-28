#![allow(non_snake_case)]


pub mod render;
pub mod input;
pub mod time;
pub mod utils;
pub mod sprite;
pub mod scene;

use input::Input;
use time::Time;
use render::{Window};
use scene::{SceneManager, Scene};

pub trait Runnable {

    fn start(&mut self, gameState: &mut GameState) {

    }   

    fn update(&mut self, gameState: &mut GameState) {

    }

    fn fixedUpdate(&mut self, gameState: &mut GameState) {

    }

}

pub struct GameState {
    pub input: Input,
    pub time: Time,
    pub sceneManager: SceneManager,
    onPlay: bool,
    // currScene: u32, // Change it to scene manager which will manage the scenes, Make it in a way that it can communicate with the game loop
}

impl GameState {
    fn new() -> Self {
        return Self {
            input: Input::new(),
            time: Time::new(),
            onPlay: false,
            sceneManager: SceneManager::new(),
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

    pub fn quit(&mut self) {
        self.onPlay = false;
    }
}


pub struct Game {

    gameState: GameState,
    window: Window,
    scenes: Vec<Scene>

}
    
impl Game {

    pub fn new() -> Self {
        return Self {
            
            gameState: GameState::new(),
            window: Window::default(),
            scenes: Vec::new(),

        };
    }

    // Function to add scene
    pub fn addScene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    // Running the game
    pub fn run(&mut self) 
    {
        
        // Initalizing the game
        // self.window.init();
        self.gameState.init();


        self.gameState.onPlay = true;
        'update: while self.gameState.onPlay {

            // Updating game state
            self.gameState.update();
            
            // Valid scene Check
            if self.gameState.sceneManager.currScene >= self.scenes.len() {
                // Handle Error
                println!("Invalid Scene !!!");
                break 'update;
            } 

            let scene = &mut self.scenes[self.gameState.sceneManager.currScene];
            let scripts = &mut scene.scripts;

            // Check if new scene is loaded
            if self.gameState.sceneManager.onSceneChange() {
                for script in scripts.iter_mut() {
                    script.start(&mut self.gameState);
                    
                    if !self.gameState.onPlay {
                        break 'update;
                    }
                }
            }   

            // Action
            for script in scripts.iter_mut() {
                script.update(&mut self.gameState);
                if !self.gameState.onPlay {
                    break 'update;
                }  

                // Fixed Update
                while self.gameState.time.updateFixed() {
                    script.fixedUpdate(&mut self.gameState);
                    
                    if !self.gameState.onPlay {
                        break 'update;
                    }
                }
            }
            
        }

    }

}


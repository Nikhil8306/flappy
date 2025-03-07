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
use sprite::Sprite;

use std::cmp::Ordering;

pub trait Runnable {

    fn start(&mut self, gameState: &mut GameState) {

    }   

    fn update(&mut self, gameState: &mut GameState) {

    }

    fn fixedUpdate(&mut self, gameState: &mut GameState) {

    }


    // For Rendering
    fn sprite(&self) -> Option<&Sprite> {
        None
    }

}

pub struct GameState {
    pub input: Input,
    pub time: Time,
    pub sceneManager: SceneManager,
    onPlay: bool,
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
        self.input.init();
    }

    fn update(&mut self) {

        // Updating the time and input
        self.time.updateDeltaTime();
        self.input.update();

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
        self.window.init();
        self.gameState.init();

        // Sorting scene gameObjects based on level for levelwise effective rendering (Decending Order)
        let scenes = &mut self.scenes;
        for scene in scenes.iter_mut() {
            scene.gameObjects.sort_by(|a, b| {
                let spriteA = a.sprite();
                let spriteB = b.sprite();

                if spriteA.is_none() && spriteB.is_none() {
                    return Ordering::Equal;
                }
                else if spriteA.is_none() {
                    return Ordering::Greater;
                }
                else if spriteB.is_none() {
                    return Ordering::Less;
                }
                
                let spriteA = spriteA.unwrap();
                let spriteB = spriteB.unwrap();

                return spriteA.transform.level.cmp(&spriteB.transform.level);
            });
        }


        // Settings play to true
        self.gameState.onPlay = true;
        'update: while self.gameState.onPlay {
            // continue;

            // Updating game state
            self.gameState.update();
            
            // Valid scene Check
            if self.gameState.sceneManager.currScene >= self.scenes.len() {
                // ToDO  : Handle it beautifully 
                println!("Invalid Scene !!!");
                break 'update;
            } 

            let scene = &mut self.scenes[self.gameState.sceneManager.currScene];
            let gameObjects = &mut scene.gameObjects;

            // Check if new scene is loaded
            if self.gameState.sceneManager.onSceneChange() {
                for gameObject in gameObjects.iter_mut() {
                    gameObject.start(&mut self.gameState);
                    
                    if !self.gameState.onPlay {
                        break 'update;
                    }
                }
            }   

            // Update
            for gameObject in gameObjects.iter_mut() {
                gameObject.update(&mut self.gameState);
                if !self.gameState.onPlay {
                    break 'update;
                }
            }

            // Fixed Update
            while self.gameState.time.updateFixed() {

                if !self.gameState.onPlay {
                    break 'update;
                }

                // Clearing Window Buffer
                self.window.clearBuffer();
                self.window.addSpriteToBufferFromGameObjects(gameObjects); // Adding sprites to the window buffer

                for gameObject in gameObjects.iter_mut() {
                    gameObject.fixedUpdate(&mut self.gameState);
                    if !self.gameState.onPlay {
                        break 'update;
                    }

                }
                
                // Rendering stuff (currently in fixedupdate :/)
                self.window.render();
            }
        }


        // Closing the window
        self.window.clean();
        self.window.close();
    }

    pub fn setWindow(&mut self, height: u16, width: u16) {
        if self.gameState.onPlay {
            return;
        }

        self.window.height = height;
        self.window.width = width;
    }

}
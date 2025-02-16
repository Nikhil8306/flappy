use std::{time::{Duration, Instant}};
use crossterm::SynchronizedUpdate;

use super::render::Window;
use super::input::Input;
use super::time::Time;
use super::scene::{Scene, SceneManager};

pub trait Runnable {

    fn update(&mut self, game:&mut Game) {

    }

    fn fixedUpdate(&mut self, game: &mut Game) {

    }
}

pub struct Game<'a> {

    onPlay: bool,
    time: Time,
    input: Input,
    window: Window,
    sceneManager: &'a SceneManager,

}


impl <'a> Game <'a> {
    pub fn new(window: Window, sm: &'a SceneManager) -> Self {

        let input = Input {};
        return Self{

            onPlay: false,
            time: Time::new(),
            input,
            window,
            sceneManager: sm,

        }
    }
}


impl <'a> Game <'a>{

    pub fn run(&mut self, scene: &mut Scene) {

        self.onPlay = true;

        let sm = self.sceneManager;
        while self.onPlay {
            self.time.updateDeltaTime();
            
            for script in scene.scripts.iter_mut() {
                
                script.update(self);

            }

        }

    }

    

    pub fn quit(&mut self) {
        self.onPlay = false;
    }

}


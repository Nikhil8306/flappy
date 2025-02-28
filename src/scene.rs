
use crate::Runnable;

pub struct SceneManager {
    pub currScene: usize,
    onChangeScene: bool,
}

impl SceneManager {
    pub(super) fn new() -> Self {
        return Self {
            currScene: 0,
            onChangeScene: true,
        };
    }
}
impl SceneManager {

    pub fn changeScene(&mut self, scene: usize) {
        self.currScene = scene;
        self.onChangeScene = true;
    }

    // Check for game loop if new scene is loaded ...
    pub(super) fn onSceneChange(&mut self) -> bool {
        return match self.onChangeScene {
            true => {
                self.onChangeScene = false;
                true
            },

            false => {
                false
            }
        };
    }

}

pub struct Scene {
    pub(super) scripts: Vec<Box<dyn Runnable>>,
}

impl Scene {

    pub fn new() -> Self {
        return Self {
            scripts: Vec::new(),
        }
    }

    pub fn addScript(&mut self, script: Box<dyn Runnable>) {
        self.scripts.push(script);
    }

}
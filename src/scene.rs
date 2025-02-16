use super::game::Runnable;

pub struct SceneManager{

    pub scenes: Vec<Scene>,
    pub currScene: usize,

}

impl SceneManager {

    pub fn new(scenes: Vec<Scene>, startScene: usize) -> Result<Self, ()> {

        if (startScene >= scenes.len()) {
            return Err(());
        }

        return Ok(Self {
            scenes,
            currScene: startScene,
        })

    }

    pub fn addScene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn changeScene(&mut self, scene: usize) -> Result<usize, usize> {

        if (scene >= self.scenes.len()) {
            return Err(self.currScene);
        }
        self.currScene = scene;

        return Ok(scene);

    }


}




pub struct Scene {

    pub scripts: Vec<Box<dyn Runnable>>,

}




impl Scene {

    pub fn new() -> Self {
        Self {
            scripts: vec![],
        }
    }

}
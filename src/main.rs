use std::io::Empty;

use flappy::game::{Game, Runnable};
use flappy::render::Window;
use flappy::scene::Scene;

struct Player {

    x: u32,
    y: u32

}

impl Runnable for Player {

    fn update(&mut self, game: &mut Game) {

        self.x += 1;
        println!("Player's Position: {}, {}", self.x, self.y);

        if self.x >= 100 {
            game.quit();
        }

    }

}

struct Enemy {

    x: u32,
    y: u32

}

impl Runnable for Enemy {

    fn update(&mut self, game:&mut Game) {
        if self.x >= 10 {
            game.quit();
        }
        self.x += 1;
        println!("Enemy's Position: {} {}", self.x, self.y);
    }

}


fn main() {


    let player = Box::new(Player {x: 90, y: 90});
    let enem = Box::new(Enemy{x: 5, y: 0});
    let mut scene1: Scene = Scene{scripts: vec![player, enem]};

    let mut game: Game = Game::new(Window::new(1, 1));

    game.loadScene(&mut scene1);

}
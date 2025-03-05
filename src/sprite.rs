use crate::{utils::Transform, Window};

pub struct Sprite {
    pub name: String,
    pub sprite: Vec<char>,
    pub height: u16,
    pub width: u16,
    pub transform: Transform,
}

impl Sprite {

    pub fn fromVec(name: String, sprite: Vec<char>, height: u16, width: u16, transform: Transform) -> Self{

        return Self {
            name,
            sprite,
            height,
            width,
            transform
        };

    }

    // String will have new line character denoting the new line (obviously)
    // If less characters are there, " " will be added in the end to match the width
    // If more than mentioned characters are there, extra will be removed (same with height)
    pub fn fromString(name: String, sprite: &String,  height: u16, width: u16, transform: Transform) -> Self {
        
        let lines:Vec<&str> = sprite.lines().collect();

        let mut sprite: Vec<char> = vec![];
        let mut currHeight = 0;
        for line in lines {
            if currHeight == height {
                break;
            }

            if currHeight == 0 && line.len() == 0 {
                continue;
            }

            let mut count = 0;
            for char in line.chars() {
                if count >= width {
                    break;
                }
                else {
                    sprite.push(char);
                }
                count += 1;
            }

            for _ in count..width {
                sprite.push(' ');
            }
            
            currHeight += 1;
        }

        for _ in 0..((height-currHeight) as u32 * width as u32) {
            sprite.push(' ');
        }

        return Self {
            name,
            sprite,
            height,
            width,
            transform
        };
    }



}
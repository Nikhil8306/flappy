use crate::sprite::Sprite;
use crossterm::{cursor::{MoveDown, MoveLeft, MoveRight, MoveUp}, ExecutableCommand};
use std::io::{stdout, Write};
use crate::utils::Transform;
use crate::Runnable;



pub struct Window {
    height: u16,
    width: u16,
    rightBorder:char,
    leftBorder:char,
    topBorder:char,
    bottomBorder:char,

    buffer: Vec<char>,

    cursorX: u16,
    cursorY: u16,
}


impl Window {
    pub fn new(height: u16, width: u16) -> Self{
        return Self{
            height,
            width,
            rightBorder:'█',
            leftBorder:'█',
            topBorder:'▀',
            bottomBorder:'▄',
            // rightBorder:'|',
            // leftBorder:'|',
            // topBorder:'-',
            // bottomBorder:'-',

            cursorX: 0,
            cursorY: 0,

            buffer: vec![]
        }
    }

    pub fn newc(height: u16, width: u16, rightBorder: char, leftBorder: char, topBorder: char, bottomBorder: char) -> Self{
        return Self{
            height,
            width,
            rightBorder,
            leftBorder,
            topBorder,
            bottomBorder,

            cursorX: 0,
            cursorY: 0,

            buffer: vec![]

        }
    }

    pub fn default() -> Self {
        return Self::new(10, 50);
    }
    
}

impl Window {
    fn drawBorder(&mut self) {
        self.cursorMoveStart();

        let right = self.rightBorder;
        let left = self.leftBorder;
        let top = self.topBorder;
        let bottom = self.bottomBorder;

        for row in 0..self.height+2 {
            
            for col in 0..self.width+2 {

                if col == self.width+1 {
                    print!("{}", right);
                }
                else if col == 0 {
                    print!("{}", left);
                } 
                else if row == 0 {
                    print!("{}", top);
                } 
                else if row == self.height+1 {
                    print!("{}", bottom);
                }
                else {
                    print!(" ");
                }

            }

            println!();
        }

        self.cursorY = self.height+2;


    }
    pub fn init(&mut self) {
        self.drawBorder();
        self.cursorMoveStart();
    }

    pub fn clean(&mut self) {
        self.drawBorder();
    }


    fn cursorMoveStart(&mut self) {

        self.cursorMoveOrigin();
        stdout().execute(MoveUp(1)).unwrap();
        stdout().execute(MoveLeft(1)).unwrap();

        self.cursorY = 0;
        self.cursorX = 0;
    }

    pub fn cursorMoveOrigin(&mut self) {
        
        self.cursorMoveTo(0, 0);

    }

    pub fn cursorMoveTo(&mut self, x: u16, y: u16) {
        
        if self.cursorX > x+1 {
            stdout().execute(MoveLeft(self.cursorX-1 - x)).unwrap();
        }
        else if self.cursorX < x+1 { // cannot use else because MoveRight(0) move cursor right 
            stdout().execute(MoveRight(x+1 - (self.cursorX))).unwrap();
        }
        
        if self.cursorY > y+1 {
            stdout().execute(MoveUp(self.cursorY-1 - y)).unwrap(); 
        }
        else if self.cursorY < y+1{
            stdout().execute(MoveDown(y+1 - (self.cursorY))).unwrap(); 
        }

        self.cursorX = x+1;
        self.cursorY = y+1;
    }


    // ToDo - save the rendering part in buffer first before flushing
    pub fn render(&mut self, script: &Box<dyn Runnable>) {
        let sprite = script.sprite();
        if sprite.is_none() {
            return;
        }

        let sprite = sprite.unwrap();

        for row in 0..sprite.height {
            if (sprite.transform.y+row as i32) < 0 {
                continue;
            }
            
            let mut colStart = 0;
            while colStart+sprite.transform.x < 0 && colStart < sprite.width as i32 {
                colStart+=1;
            } 

            self.cursorMoveTo((colStart  + sprite.transform.x)as u16, (row as i32 + sprite.transform.y) as u16);
            
            for col in colStart as u32..sprite.width {
                print!("{}", sprite.sprite[(row*sprite.width+col) as usize]);
                self.cursorX += 1;
            }

        }

        stdout().flush().unwrap();
    }

    pub fn close(&mut self) {
        println!("Window Closed!!")
    }
    
}


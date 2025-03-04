use crossterm::{cursor::{self, MoveDown, MoveLeft, MoveRight, MoveUp}, execute, ExecutableCommand};
use crossterm::terminal::{self, Clear, ClearType};
use std::io::{stdout, Write};
use crate::Runnable;


pub(super) struct Window {
    pub height: u16,
    pub width: u16,
    rightBorder:char,
    leftBorder:char,
    topBorder:char,
    bottomBorder:char,

    buffer: Vec<char>,

    cursorX: u16,
    cursorY: u16,

    isOpen: bool,
}


impl Window {
    pub fn new(height: u16, width: u16) -> Self{
        return Self{
            height,
            width,
            // rightBorder:'█',
            // leftBorder:'█',
            // topBorder:'▀',
            // bottomBorder:'▄',
            rightBorder:'|',
            leftBorder:'|',
            topBorder:'-',
            bottomBorder:'-',

            cursorX: 0,
            cursorY: 0,

            buffer: vec![],

            isOpen: false,
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

            buffer: vec![],
            isOpen: false,
        }
    }

    pub fn default() -> Self {
        return Self::new(30, 80);
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

            println!("\r");
        }

        self.cursorY = self.height+2;


    }
    pub fn init(&mut self) {
        if self.isOpen {
            return;
        }


        // Enabling raw mode
        terminal::enable_raw_mode();        
        
        // Drawing the Borders
        self.drawBorder();
        self.cursorMoveStart();

        self.buffer = vec![' '; (self.height*self.width) as usize]; 

    }

    pub fn clean(&mut self) {
        
        self.cursorMoveStart();

        execute!(stdout(), Clear(ClearType::FromCursorDown));

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
        else if self.cursorX < x+1 { // cannot use else because MoveRight(0) move cursor right by one place
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

    pub(super) fn clearBuffer(&mut self) {
        self.buffer = vec![' '; (self.height * self.width) as usize];
    }

    pub(super) fn addSpriteToBufferFromGameObjects(&mut self, gameObjects: &Vec<Box<dyn Runnable>>) {
        for gameObject in gameObjects {
            let sprite = gameObject.sprite();
            if sprite.is_none() {
                return;
            }

            let sprite = sprite.unwrap();

            for i in 0..sprite.height {

                let row = sprite.transform.y + i as i32;
                if row < 0 || row >= self.height as i32{
                    continue;
                }

                for j in 0..sprite.width {

                    let col = sprite.transform.x + j as i32;

                    if col < 0 {
                        continue;
                    } else if col >= self.width as i32 {
                        break;
                    }

                    self.buffer[((row * self.width as i32) + col) as usize] = sprite.sprite[(i*sprite.width + j) as usize];
                }

            }
        }

    }

    // Todo : currently levels are reversed, make it render levelwise
    pub fn render(&mut self) {

        // Printing buffer on the window
        self.cursorMoveOrigin();

        for i in 0..self.height {
            self.cursorMoveTo(0, i);
            for j in 0..self.width {

                print!("{}", self.buffer[((i*self.width) + j) as usize]);
                self.cursorX += 1;

            }

        }

        stdout().flush().unwrap();
    }

    // Clear the window also
    pub fn close(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
    
}


pub struct Viewport {
    height: u32,
    width: u32,
    rightBorder:char,
    leftBorder:char,
    topBorder:char,
    bottomBorder:char,
    fill:char,
}


impl Viewport {
    pub fn new(height: u32, width: u32) -> Self{
        return Self{
            height,
            width,
            rightBorder:'█',
            leftBorder:'█',
            topBorder:'▀',
            bottomBorder:'▄',
            fill: ' '
        }
    }

    pub fn newc(height: u32, width: u32, rightBorder: char, leftBorder: char, topBorder: char, bottomBorder: char, fill: char) -> Self{
        return Self{
            height,
            width,
            rightBorder,
            leftBorder,
            topBorder,
            bottomBorder,
            fill
        }
    }
}

impl Viewport {
    pub fn render(&self) {
        for i in 1..=self.height {
            print!("{}", self.rightBorder);
            
            for j in 2..self.width {
                
                if i == 1  {
                    print!("{}", self.topBorder);
                }
                else if i == self.height {
                    print!("{}", self.bottomBorder);
                }
                else {
                    print!("{}", self.fill);
                }
            }
            println!("{}", self.leftBorder);
        }
    }
}


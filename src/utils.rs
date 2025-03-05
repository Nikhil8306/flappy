pub struct Transform {
    pub x: i32,
    pub y: i32,
    pub level: u32,
    pub tag: Option<String>
}

impl Transform {

    pub fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

}
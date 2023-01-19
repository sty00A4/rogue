use crate::*;

pub struct Object {
    pub x: i32,
    pub y: i32,
    pub display: char,
    pub color: Color
}
impl Object {
    pub fn new(display: char, x: i32, y: i32, color: Color) -> Self {
        Self { x, y, display, color }
    }
    pub fn draw(&self, root: &mut Offscreen) {
        root.set_key_color(self.color);
        root.put_char(self.x, self.y, self.display, BackgroundFlag::None);
    }
}
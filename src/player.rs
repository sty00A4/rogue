use crate::*;

pub struct Player {
    object: Object
}
impl Player {
    pub fn init() -> Self {
        Self {
            object: Object::new('@', WIDTH as i32 / 2, HEIGHT as i32 / 2, Color::new(255, 0, 0))
        }
    }
    pub fn draw(&self, root: &mut Offscreen) {
        self.object.draw(root)
    }
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.object.x = x;
        self.object.y = y;
    }
    pub fn right(&mut self, map: &mut Map) {
        if !Tile::solid(map, self.object.x + 1, self.object.y) {
            self.object.x += 1;
        }
    }
    pub fn left(&mut self, map: &mut Map) {
        if !Tile::solid(map, self.object.x - 1, self.object.y) {
            self.object.x -= 1;
        }
    }
    pub fn up(&mut self, map: &mut Map) {
        if !Tile::solid(map, self.object.x, self.object.y - 1) {
            self.object.y -= 1;
        }
    }
    pub fn down(&mut self, map: &mut Map) {
        if !Tile::solid(map, self.object.x, self.object.y + 1) {
            self.object.y += 1;
        }
    }
}
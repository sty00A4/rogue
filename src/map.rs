use crate::*;
use std::cmp::{max, min};

pub type Map = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile { Wall, Ground }
impl Tile {
    pub fn color(&self) -> Color {
        match self {
            Self::Wall => WHITE,
            Self::Ground => GREY,
            // Self::Empty => BLACK,
        }
    }
    pub fn display(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Ground => ' ',
            // Self::Empty => ' ',
        }
    }
    pub fn solid(map: &Map, x: i32, y: i32) -> bool {
        match map.get(y as usize) {
            Some(row) => match row.get(x as usize) {
                Some(tile) => tile == &Tile::Wall,
                None => false
            }
            None => false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    x1: i32, y1: i32,
    x2: i32, y2: i32,
}
impl Room {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x1: x, y1: y, x2: x + w, y2: y + h }
    }
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }
    pub fn intersects_with(&self, other: &Room) -> bool {
        // returns true if this rectangle intersects with another one
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

pub fn create_room(room: &Room, map: &mut Map) -> Option<()> {
    for y in (room.y1 + 1)..room.y2 {
        for x in (room.x1 + 1)..room.x2 {
            match map.get_mut(y as usize) {
                Some(row) => match row.get_mut(x as usize) {
                    Some(tile) => *tile = Tile::Ground,
                    None => {}
                }
                None => {}
            }
        }
    }
    Some(())
}
pub fn create_tunnel_h(x1: i32, x2: i32, y: i32, map: &mut Map) {
    match map.get_mut(y as usize) {
        Some(row) => for x in min(x1, x2)..max(x1, x2) + 1 {
            match row.get_mut(x as usize) {
                Some(tile) => *tile = Tile::Ground,
                None => {}
            }
        }
        None => {}
    }
}
pub fn create_tunnel_v(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in min(y1, y2)..max(y1, y2) {
        match map.get_mut(y as usize) {
            Some(row) => match row.get_mut(x as usize) {
                Some(tile) => *tile = Tile::Ground,
                None => {}
            }
            None => {}
        }
    }
}

pub fn generate_map(width: u32, height: u32, player: &mut Player) -> Map {
    let mut map: Map = vec![vec![Tile::Wall; width as usize]; height as usize];
    let mut rooms = vec![];
    let rooms_count = rand::thread_rng().gen_range(MIN_ROOMS..MAX_ROOMS);
    while rooms.len() <= rooms_count as usize {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE..ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE..ROOM_MAX_SIZE + 1);
        let x = rand::thread_rng().gen_range(0..WIDTH - w);
        let y = rand::thread_rng().gen_range(0..HEIGHT - h);
        let new_room = Room::new(x as i32, y as i32, w as i32, h as i32);
        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));
        if !failed {
            create_room(&new_room, &mut map);
            let (new_x, new_y) = new_room.center();
            if rooms.is_empty() {
                player.set_pos(new_x, new_y)
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rand::random() {
                    create_tunnel_h(prev_x, new_x, prev_y, &mut map);
                    create_tunnel_v(prev_y, new_y, new_x, &mut map);
                } else {
                    create_tunnel_v(prev_y, new_y, prev_x, &mut map);
                    create_tunnel_h(prev_x, new_x, new_y, &mut map);
                }
            }
            rooms.push(new_room);
        }
    }
    map
}
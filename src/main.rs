#![allow(unused)]
use tcod::{
    colors::*,
    console::*,
    input::{Key, KeyCode},
};
use rand::prelude::*;
mod object;
mod map;
mod player;
use object::*;
use map::*;
use player::*;

const WIDTH: u32 = 1920 / 16;
const HEIGHT: u32 = 1080 / 16;

const ROOM_MAX_SIZE: u32 = 30;
const ROOM_MIN_SIZE: u32 = 5;
const MAX_ROOMS: u32 = 30;
const MIN_ROOMS: u32 = 3;

struct Game {
    root: Root,
    player: Player,
    con: Offscreen,
    map: Map
}
impl Game {
    pub fn render(&mut self) {
        self.con.clear();

        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                self.con.set_key_color(tile.color());
                self.con.put_char(x as i32, y as i32, tile.display(), BackgroundFlag::Set);
            }
        }

        self.player.draw(&mut self.con);

        blit(&self.con, (0, 0), (WIDTH as i32, HEIGHT as i32), &mut self.root, (0, 0), 1.0, 1.0);
        self.root.flush();
    }
}

pub fn loading_screen(con: &mut Offscreen, root: &mut Root) {
    con.clear();
    let text = "Loading Map...";
    con.print(WIDTH as i32 / 2 - (text.len() / 2) as i32, HEIGHT as i32 / 2, text);
    blit(con, (0, 0), (WIDTH as i32, HEIGHT as i32), root, (0, 0), 1.0, 1.0);
    root.flush();
}

fn main() {
    let mut root = Root::initializer()
        .font("assets/terminal.png", FontLayout::AsciiInCol)
        .size(WIDTH as i32, HEIGHT as i32)
        .title("rogue")
        .init();
    let mut player = Player::init();
    let mut con = Offscreen::new(WIDTH as i32, HEIGHT as i32);

    con.set_default_background(BLACK);
    con.set_default_foreground(WHITE);
    loading_screen(&mut con, &mut root);
    
    let mut map = generate_map(WIDTH, HEIGHT, &mut player);
    let mut game = Game { root, con, player, map };

    'main: while !game.root.window_closed() {
        game.render();
        let key = game.root.wait_for_keypress(true);
        match key {
            Key { code: KeyCode::Char, printable: 'd', .. } => game.player.right(&mut game.map),
            Key { code: KeyCode::Char, printable: 'a', .. } => game.player.left(&mut game.map),
            Key { code: KeyCode::Char, printable: 'w', .. } => game.player.up(&mut game.map),
            Key { code: KeyCode::Char, printable: 's', .. } => game.player.down(&mut game.map),
            Key { code: KeyCode::Enter, alt: true, .. } => game.root.set_fullscreen(!game.root.is_fullscreen()),
            Key { code: KeyCode::Escape, .. } => break 'main,
            _ => {}
        }
    }
}
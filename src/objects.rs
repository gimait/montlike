pub mod ai;
pub mod equipment;
pub mod fighter;
pub mod game;
pub mod item;
pub mod object;
pub mod player;

use rand::Rng;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::{Key, Mouse};
use tcod::map::Map as FovMap;

use crate::constants::*;
use crate::misc::mut_two;
use crate::render::*;

use ai::AI;
use game::Game;
use item::Item;
use object::Object;

pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
    pub panel: Offscreen,
    pub fov: FovMap,
    pub key: Key,
    pub mouse: Mouse,
}

fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    let (x, y) = objects[id].pos();
    if !is_blocked(x + dx, y + dy, map, objects) {
        objects[id].set_pos(x + dx, y + dy);
    }
}

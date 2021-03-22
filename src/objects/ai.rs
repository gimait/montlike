
use serde::{Deserialize, Serialize};
use crate::objects::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AI {
    Basic,
    Confused { previous_ai: Box<AI>, num_turns: i32 },
}

pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) {
    use AI::*;
    if let Some(ai) = objects[monster_id].ai.take() {
        let new_ai = match ai {
            Basic => ai_basic(monster_id, tcod, game, objects),
            Confused { previous_ai, num_turns } => ai_confused(monster_id, tcod, game, objects, previous_ai, num_turns),
        };
        objects[monster_id].ai = Some(new_ai);
    }
}

fn ai_basic(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) -> AI {
    let (monster_x, monster_y) = objects[monster_id].pos();
    if tcod.fov.is_in_fov(monster_x, monster_y) {
        if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
            let (player_x, player_y) = objects[PLAYER].pos();
            move_towards(monster_id, player_x, player_y, &game.map, objects);
        } else if objects[PLAYER].fighter.map_or(false, |f| f.hp > 0) {
            let (monster, player) = mut_two(monster_id, PLAYER, objects);
            monster.attack(player, game)
        }
    }
    AI::Basic
}

fn ai_confused(
    monster_id: usize,
    _tcod: &Tcod,
    game: &mut Game,
    objects: &mut [Object],
    previous_ai: Box<AI>,
    num_turns: i32,
) -> AI {
    if num_turns >= 0 {
        // Confused
        move_by(
            monster_id,
            rand::thread_rng().gen_range(-1, 2),
            rand::thread_rng().gen_range(-1, 2),
            &game.map,
            objects,
        );
        AI::Confused {
            previous_ai: previous_ai,
            num_turns: num_turns - 1,
        }
    } else {
        // Back to normal.
        game.messages
            .add(format!("The {} is confused no more!", objects[monster_id].name), RED);
        *previous_ai
    }
}

fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
    let dx = target_x - objects[id].x;
    let dy = target_y - objects[id].y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    move_by(id, dx, dy, map, objects);
}

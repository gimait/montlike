use crate::objects::*;
use serde::{Deserialize, Serialize};
use tcod::input::{self, Event};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Item {
    Heal,
    Lightning,
    Confuse,
    Fireball,
}

pub enum UseResult {
    UsedUp,
    Cancelled,
}

fn closest_monster(tcod: &Tcod, objects: &[Object], max_range: i32) -> Option<usize> {
    let mut closest_enemy = None;
    let mut closest_dist = (max_range + 1) as f32;

    for (id, object) in objects.iter().enumerate() {
        if (id != PLAYER) && object.fighter.is_some() && object.ai.is_some() && tcod.fov.is_in_fov(object.x, object.y) {
            let dist = objects[PLAYER].distance_to(object);
            if dist < closest_dist {
                closest_enemy = Some(id);
                closest_dist = dist;
            }
        }
    }
    closest_enemy
}

fn target_tile(tcod: &mut Tcod, game: &mut Game, objects: &[Object], max_range: Option<f32>) -> Option<(i32, i32)> {
    use tcod::input::KeyCode::Escape;
    loop {
        tcod.root.flush();
        let event = input::check_for_event(input::KEY_PRESS | input::MOUSE).map(|e| e.1);
        match event {
            Some(Event::Mouse(m)) => tcod.mouse = m,
            Some(Event::Key(k)) => tcod.key = k,
            None => tcod.key = Default::default(),
        }
        render_all(tcod, game, objects, false);

        let (x, y) = (tcod.mouse.cx as i32, tcod.mouse.cy as i32);
        let in_fov = (x < MAP_WIDTH) && (y < MAP_HEIGHT) && tcod.fov.is_in_fov(x, y);
        let in_range = max_range.map_or(true, |range| objects[PLAYER].distance(x, y) <= range);
        if tcod.mouse.lbutton_pressed && in_fov && in_range {
            return Some((x, y));
        }

        if tcod.mouse.rbutton_pressed || tcod.key.code == Escape {
            return None;
        }
    }
}

fn target_monster(tcod: &mut Tcod, game: &mut Game, objects: &[Object], max_range: Option<f32>) -> Option<usize> {
    loop {
        match target_tile(tcod, game, objects, max_range) {
            Some((x, y)) => {
                // return the first clicked monster, otherwise continue looping
                for (id, obj) in objects.iter().enumerate() {
                    if obj.pos() == (x, y) && obj.fighter.is_some() && id != PLAYER {
                        return Some(id);
                    }
                }
            }
            None => return None,
        }
    }
}

pub fn cast_heal(_inventory_id: usize, _tcod: &mut Tcod, game: &mut Game, objects: &mut [Object]) -> UseResult {
    // heal the player
    if let Some(fighter) = objects[PLAYER].fighter {
        if fighter.hp == fighter.max_hp {
            game.messages.add("You are already at full health.", RED);
            return UseResult::Cancelled;
        }
        game.messages.add("Your wounds start to feel better!", LIGHT_VIOLET);
        objects[PLAYER].heal(HEAL_AMOUNT);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}

pub fn cast_lightning(_inventory_id: usize, _tcod: &mut Tcod, game: &mut Game, objects: &mut [Object]) -> UseResult {
    let monster_id = closest_monster(_tcod, objects, LIGHTNING_RANGE);
    if let Some(monster_id) = monster_id {
        game.messages.add(
            format!(
                "A lightning bolt stricks the {}, the damage is {} hit points.",
                objects[monster_id].name, LIGHTNING_DAMAGE
            ),
            LIGHT_BLUE,
        );
        if let Some(xp) = objects[monster_id].take_damage(LIGHTNING_DAMAGE, game) {
            objects[PLAYER].fighter.as_mut().unwrap().xp += xp;
        }
        UseResult::UsedUp
    } else {
        game.messages.add("No enemy is close enough to strike.", RED);
        UseResult::Cancelled
    }
}

pub fn cast_confuse(_inventory_id: usize, _tcod: &mut Tcod, game: &mut Game, objects: &mut [Object]) -> UseResult {
    game.messages.add(
        "Left-click an enemy to confuse it, or right-click to cancel.",
        LIGHT_CYAN,
    );
    let monster_id = target_monster(_tcod, game, objects, Some(CONFUSE_RANGE as f32));
    if let Some(monster_id) = monster_id {
        let old_ai = objects[monster_id].ai.take().unwrap_or(AI::Basic);
        objects[monster_id].ai = Some(AI::Confused {
            previous_ai: Box::new(old_ai),
            num_turns: CONFUSE_NUM_TURNS,
        });
        game.messages.add(
            format!(
                "The eyes of {} look vacant, as he starts to stumble around!",
                objects[monster_id].name
            ),
            LIGHT_GREEN,
        );
        UseResult::UsedUp
    } else {
        game.messages.add("No enemy in range to strike.", RED);
        UseResult::Cancelled
    }
}

pub fn cast_fireball(_inventory_id: usize, tcod: &mut Tcod, game: &mut Game, objects: &mut [Object]) -> UseResult {
    game.messages.add(
        "Left-click a target tile for the fireball, or right-click to cancel.",
        LIGHT_CYAN,
    );
    let (x, y) = match target_tile(tcod, game, objects, None) {
        Some(tile_pos) => tile_pos,
        None => return UseResult::Cancelled,
    };
    game.messages.add(
        format!(
            "The fireball explodes, burning everything within {} tiles!",
            FIREBALL_RADIUS
        ),
        ORANGE,
    );

    let mut total_xp = 0;
    for (id, obj) in objects.iter_mut().enumerate() {
        if obj.distance(x, y) <= FIREBALL_RADIUS as f32 && obj.fighter.is_some() {
            game.messages.add(
                format!("The {} gets burned for {} hit points.", obj.name, FIREBALL_DAMAGE),
                ORANGE,
            );
            if let Some(xp) = obj.take_damage(FIREBALL_DAMAGE, game) {
                if id != PLAYER {
                    total_xp += xp;
                }
            }
        }
    }
    objects[PLAYER].fighter.as_mut().unwrap().xp += total_xp;
    UseResult::UsedUp
}

use tcod::colors::*;
use tcod::console::*;
use tcod::map::Map as FovMap;

pub mod constants;
pub mod controls;
pub mod misc;
pub mod objects;
pub mod render;
use constants::*;
use controls::*;
use objects::*;
use render::*;

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("rusty game totorial")
        .init();

    let mut tcod = Tcod {
        root,
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
    };

    // Player
    let mut player = Object::new(0, 0, '@', "Mi", WHITE, true);
    player.alive = true;
    player.fighter = Some(Fighter {
        max_hp: 50,
        hp: 50,
        defense: 10,
        power: 5,
        on_death: DeathCallback::Player,
    });
    // NPC
    let npc = Object::new(
        SCREEN_WIDTH / 2 + 1,
        SCREEN_HEIGHT / 2 + 1,
        '@',
        "Yu",
        YELLOW,
        true,
    );

    let mut objects = vec![player];

    let mut game = Game {
        map: make_map(&mut objects),
    };

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game.map[x as usize][y as usize].block_sight,
                !game.map[x as usize][y as usize].blocked,
            );
        }
    }

    let mut previous_player_position = (-1, -1);

    while !tcod.root.window_closed() {
        tcod.con.clear();
        let fov_recompute = previous_player_position != (objects[PLAYER].pos());
        render_all(&mut tcod, &mut game, &objects, fov_recompute);
        tcod.root.flush();

        previous_player_position = objects[PLAYER].pos();
        let player_action = handle_keys(&mut tcod, &game, &mut objects);
        if player_action == PlayerAction::Exit {
            break;
        }

        if objects[PLAYER].alive && player_action == PlayerAction::TookTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai_take_turn(id, &tcod, &game, &mut objects);
                }
            }
        }
    }
}

use tcod::colors::*;
use crate::predefs::structs::{Tcod, Object, Game};

use crate::predefs::structs::*;
use PlayerAction::*;
use crate::predefs::constants::*;
use crate::libs::make_map::*;
use crate::libs::ai::*;

pub fn handle_keys(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let player_alive = objects[PLAYER].alive;
    match (tcod.key, tcod.key.text(), player_alive) {
        (Key {
            code: Enter,
            alt: true,
            ..
        },
        _,
        _,)
         => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        }
        (Key { code: Escape, .. },_,_,) => return Exit, // exit game
        
        // movement keys
        (Key { code: Up, .. },_,true,) => {
            player_move_or_attack( 0, -1, game, objects);
            TookTurn 
        },
        (Key { code: Down, .. },_,true,) => {
            player_move_or_attack( 0, 1, game, objects);
            TookTurn
        },
        (Key { code: Left, .. },_,true,) => {
            player_move_or_attack( -1, 0, game, objects);
            TookTurn
        },
        (Key { code: Right, .. },_,true,) => {
            player_move_or_attack( 1, 0, game, objects);
            TookTurn
        },

        _ => DidntTakeTurn
    }
}

pub fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, objects: &mut [Object]) {
    // the coordinates the player is moving to/attacking
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    // try to find an attackable object there
    let target_id = objects
    .iter()
    .position(|object| object.fighter.is_some() && object.pos() == (x, y));

    
    // attack if target found, move otherwise
    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(PLAYER, target_id, objects);
            player.attack(target, game);
        }
        None => {
            move_by(PLAYER, dx, dy, &game.map, objects);
        }
    }
}
pub fn player_death(player: &mut Object, game: &mut Game) {
    // the game ended!
    game.messages.add("You died!", RED);

    // for added effect, transform the player into a corpse!
    player.char = '%';
    player.color = DARK_RED;
}

pub fn monster_death(monster: &mut Object, game: &mut Game) {
    // transform it into a nasty corpse! it doesn't block, can't be
    // attacked and doesn't move
    game.messages.add(format!("{} is dead!", monster.name), ORANGE);
    monster.char = '%';
    monster.color = DARK_RED;
    monster.blocks = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("remains of {}", monster.name);
}
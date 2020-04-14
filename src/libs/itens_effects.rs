use tcod::colors::*;
use crate::predefs::constants::*;
use crate::predefs::structs::*;
use crate::libs::make_map::{closest_monster, target_tile};

pub fn cast_heal(
    _inventory_id: usize,
    _tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) -> UseResult {
    // heal the player
    if let Some(fighter) = objects[PLAYER].fighter {
        if fighter.hp == fighter.max_hp {
            game.messages.add("You are already at full health.", RED);
            return UseResult::Cancelled;
        }
        game.messages
            .add("Your wounds start to feel better!", LIGHT_VIOLET);

        match game.inventory[_inventory_id].item {
            Some(Item::MinorHeal) => objects[PLAYER].heal(MINOR_HEAL_AMOUNT),
            Some(Item::Heal) => objects[PLAYER].heal(HEAL_AMOUNT),
            Some(Item::MajorHeal) => objects[PLAYER].heal(MAJOR_HEAL_AMOUNT),
            _ => (),
        }
        
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}

pub fn cast_potion_pwr(
    _inventory_id: usize,
    _tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) -> UseResult {
    // heal the player
    if let Some(fighter) = objects[PLAYER].fighter {
        if fighter.power >= MAX_POWER {
            game.messages.add("You are already at full power.", RED);
            return UseResult::Cancelled;
        }
        game.messages
            .add("You feel the power through your veins!", LIGHT_VIOLET);
        objects[PLAYER].inc_power(1);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}
pub fn cast_potion_def(
    _inventory_id: usize,
    _tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) -> UseResult {
    // heal the player
    if let Some(fighter) = objects[PLAYER].fighter {
        if fighter.defense >= MAX_DEFENSE {
            game.messages.add("You are already a defense lord.", RED);
            return UseResult::Cancelled;
        }
        game.messages
            .add("You feel you can resist more!", LIGHT_VIOLET);
        objects[PLAYER].inc_defense(1);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}

pub fn cast_potion_hp(
    _inventory_id: usize,
    _tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) -> UseResult {
    // heal the player
    if let Some(fighter) = objects[PLAYER].fighter {
        if fighter.max_hp >= MAX_MAX_HP {
            game.messages.add("You are already at full max HP.", RED);
            return UseResult::Cancelled;
        }
        game.messages
            .add("You feel you can resist more!", LIGHT_VIOLET);
        objects[PLAYER].inc_max_hp(5);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}

pub fn cast_lightning(
    _inventory_id: usize,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) -> UseResult {
    // find closest enemy (inside a maximum range and damage it)
    let monster_id = closest_monster(tcod, objects, LIGHTNING_RANGE);
    if let Some(monster_id) = monster_id {
        // zap it!
        game.messages.add(
            format!(
                "A lightning bolt strikes the {} with a loud thunder! \
                 The damage is {} hit points.",
                objects[monster_id].name, LIGHTNING_DAMAGE
            ),
            LIGHT_BLUE,
        );
        objects[monster_id].take_damage(LIGHTNING_DAMAGE, game);
        UseResult::UsedUp
    } else {
        // no enemy found within maximum range
        game.messages
            .add("No enemy is close enough to strike.", RED);
        UseResult::Cancelled
    }
}

pub fn cast_confusion(
    _inventory_id: usize,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) -> UseResult {
    // find closest enemy in-range and confuse it
    let monster_id = closest_monster(tcod, objects,CONFUSE_RANGE) ;
    if let Some(monster_id) = monster_id {
        let old_ai = objects[monster_id].ai.take().unwrap_or(Ai::Basic);
        // replace the monster's AI with a "confused" one; after
        // some turns it will restore the old AI
        objects[monster_id].ai = Some(Ai::Confused {
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
        // no enemy fonud within maximum range
        game.messages
            .add("No enemy is close enough to strike.", RED);
        UseResult::Cancelled
    }
}
pub fn cast_fireball(
    _inventory_id: usize,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut [Object],
) -> UseResult {
    // ask the player for a target tile to throw a fireball at
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

    for obj in objects {
        if obj.distance(x, y) <= FIREBALL_RADIUS as f32 && obj.fighter.is_some() {
            game.messages.add(
                format!(
                    "The {} gets burned for {} hit points.",
                    obj.name, FIREBALL_DAMAGE
                ),
                ORANGE,
            );
            obj.take_damage(FIREBALL_DAMAGE, game);
        }
    }

    UseResult::UsedUp
}
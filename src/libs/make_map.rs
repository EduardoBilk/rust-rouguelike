use std::cmp;
use rand::Rng;
use rand::distributions::{IndependentSample, Weighted, WeightedChoice};

use tcod::colors::*;
use tcod::console::*;
use tcod::map::Map as FovMap;
use tcod::input;

use crate::predefs::constants::*;
use crate::predefs::structs::*;
use crate::libs::render::*;
use crate::libs::menu::{get_equipped_in_slot};


pub fn make_map(objects: &mut Vec<Object>, level: u32) -> Map{
    
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    let mut rooms = vec![];
    assert_eq!(&objects[PLAYER] as *const _, &objects[0] as *const _);
    objects.truncate(1);
    

    for _ in 0..MAX_ROOMS {
        // random width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // random position without going out of the boundaries of the map
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);
        let new_room = Rect::new(x, y, w, h);
        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));
        if !failed {
            // this means there are no intersections, so this room is valid

            // "paint" it to the map's tiles
            create_room(new_room, &mut map);
            place_objects(new_room, &map, objects, level);

            // center coordinates of the new room, will be useful later
            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                // this is the first room, where the player starts at
                objects[PLAYER].set_pos(new_x, new_y);
            }else{
                // all rooms after the first:
                // connect it to the previous room with a tunnel

                // center coordinates of the previous room
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                // toss a coin (random bool value -- either true or false)
                if rand::random() {
                    // first move horizontally, then vertically
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    // first move vertically, then horizontally
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }
            rooms.push(new_room);
        }      
        
    }
    // create stairs at the center of the last room
    let (last_room_x, last_room_y) = rooms[rooms.len() - 1].center();
    let mut stairs = Object::new(last_room_x, last_room_y, '<', "stairs", WHITE, false);
    stairs.always_visible = true;
    objects.push(stairs);

    map
}
/// Advance to the next level
pub fn next_level(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) {
    game.messages.add(
        "You take a moment to rest, and recover your strength.",
        VIOLET,
    );
    let heal_hp = objects[PLAYER].fighter.map_or(0, |f| f.max_hp / 2);
    objects[PLAYER].heal(heal_hp);

    game.messages.add(
        "After a rare moment of peace, you descend deeper into \
        the heart of the dungeon...",
        RED,
    );
    game.dungeon_level += 1;
    game.map = make_map(objects, game.dungeon_level);
    initialise_fov(tcod, &game.map);
}

fn create_room(room: Rect, map: &mut Map) {
    // go through the tiles in the rectangle and make them passable
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    // horizontal tunnel. `min()` and `max()` are used in case `x1 > x2`
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    // vertical tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}
pub fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    let (x, y) = objects[id].pos();
    
    if !is_blocked(x + dx, y + dy, map, objects) {
        objects[id].set_pos(x + dx, y + dy);
    }
}

pub fn is_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool {
    // first test the map tile
    if map[x as usize][y as usize].blocked {
        return true;
    }
    // now check for any blocking objects
    objects
        .iter()
        .any(|object| object.blocks && object.pos() == (x, y))
}

fn place_objects(room: Rect, map: &Map, objects: &mut Vec<Object> , level: u32) {
    // maximum number of monsters per room
    let max_monsters = from_dungeon_level(
        &[
            Transition { level: 1, value: 2 },
            Transition { level: 4, value: 3 },
            Transition { level: 6, value: 5 },
        ],
        level,
    );
    // choose random number of monsters
    let num_monsters = rand::thread_rng().gen_range(0, max_monsters + 1);

    for _ in 0..num_monsters {
        // choose random spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);
        if !is_blocked(x, y, map, objects) {
            // monster random table
            // monster random table
            let troll_chance = from_dungeon_level(
                &[
                    Transition {
                        level: 3,
                        value: 15,
                    },
                    Transition {
                        level: 5,
                        value: 30,
                    },
                    Transition {
                        level: 7,
                        value: 60,
                    },
                ],
                level,
            );
            let monster_chances = &mut [
                Weighted {
                    weight: 80,
                    item: "orc",
                },
                Weighted {
                    weight: troll_chance,
                    item: "troll",
                },
            ];
            let monster_choice = WeightedChoice::new(monster_chances);
            let mut monster = match monster_choice.ind_sample(&mut rand::thread_rng()) {
                "orc" => {
                    let mut orc = Object::new(x, y, 'o', "Orc", DESATURATED_GREEN, true);
                    orc.fighter = Some(Fighter {
                        max_hp: 20,
                        hp: 20,
                        defense: 0,
                        power: 4,
                        xp: 35,
                        on_death: DeathCallback::Monster,
                    });
                    orc.ai = Some(Ai::Basic);
                    orc
                }
                "troll" => {
                    let mut troll = Object::new(x, y, 'T', "Troll", DARKER_GREEN, true);
                    troll.fighter = Some(Fighter {
                        max_hp: 30,
                        hp: 30,
                        defense: 2,
                        power: 8,
                        xp: 100,
                        on_death: DeathCallback::Monster,
                    });
                    troll.ai = Some(Ai::Basic);
                    troll
                }
                _ => unreachable!(),
            };
            monster.alive = true;
            objects.push(monster);
        }
    }
    let max_items = from_dungeon_level(
        &[
            Transition { level: 1, value: 1 },
            Transition { level: 4, value: 2 },
        ],
        level,
    );

    let num_items = rand::thread_rng().gen_range(0, max_items + 1);
    for _ in 0..num_items {
        // choose random spot for this item
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);
        
        // only place it if the tile is not blocked
        if !is_blocked(x, y, map, objects) {
            // item random table
            let item_chances = &mut [
            // healing potion always shows up, even if all other items have 0 chance
            Weighted {
                weight: 35,
                item: Item::Heal,
            },
            Weighted {
                weight: from_dungeon_level(
                    &[Transition {
                        level: 4,
                        value: 25,
                    }],
                    level,
                ),
                item: Item::ScrollLightning,
            },
            Weighted {
                weight: from_dungeon_level(
                    &[Transition {
                        level: 6,
                        value: 25,
                    }],
                    level,
                ),
                item: Item::ScrollFireball,
            },
            Weighted {
                weight: from_dungeon_level(
                    &[Transition {
                        level: 2,
                        value: 10,
                    }],
                    level,
                ),
                item: Item::ScrollConfusion,
            },
            Weighted {weight: 1000, item: Item::Equipment},
        ];
        let item_choice = WeightedChoice::new(item_chances);
            let mut item = match item_choice.ind_sample(&mut rand::thread_rng()) {
                Item::Heal => {
                    // create a healing potion
                    let mut object = Object::new(x, y, '!', "healing potion", VIOLET, false);
                    object.item = Some(Item::Heal);
                    object
                }
                Item::ScrollLightning => {
                    // create a lightning bolt scroll
                    let mut object =
                        Object::new(x, y, '#', "scroll of lightning bolt", LIGHT_YELLOW, false);
                    object.item = Some(Item::ScrollLightning);
                    object
                }
                Item::ScrollFireball => {
                    // create a fireball scroll
                    let mut object =
                        Object::new(x, y, '#', "scroll of fireball", LIGHT_YELLOW, false);
                    object.item = Some(Item::ScrollFireball);
                    object
                }
                Item::ScrollConfusion => {
                    // create a confuse scroll
                    let mut object =
                        Object::new(x, y, '#', "scroll of confusion", LIGHT_YELLOW, false);
                    object.item = Some(Item::ScrollConfusion);
                    object
                }
                Item::Equipment => {
                    // create a sword
                    let mut object = Object::new(x, y, '/', "sword", SKY, false);
                    object.item = Some(Item::Equipment);
                    object.equipment = Some(Equipment{equipped: false, slot: Slot::RightHand});
                    object
                }
            };
            item.always_visible = true;
            objects.push(item);

                    
        };
    };
}
   

/// return a string with the names of all objects under the mouse
pub fn get_names_under_mouse(mouse: input::Mouse, objects: &[Object], fov_map: &FovMap) -> String {
    let (x, y) = (mouse.cx as i32, mouse.cy as i32);

    // create a list with the names of all objects at the mouse's coordinates and in FOV
    let names = objects
        .iter()
        .filter(|obj| obj.pos() == (x, y) && fov_map.is_in_fov(obj.x, obj.y))
        .map(|obj| obj.name.clone())
        .collect::<Vec<_>>();

    names.join(", ") // join the names, separated by commas
}

/// add to the player's inventory and remove from the map
pub fn pick_item_up(object_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
    if game.inventory.len() >= 26 {
        game.messages.add(
            format!(
                "Your inventory is full, cannot pick up {}.",
                objects[object_id].name
            ),
            RED,
        );
    } else {
        let item = objects.swap_remove(object_id);
        game.messages
            .add(format!("You picked up a {}!", item.name), GREEN);
        let index = game.inventory.len();
        let slot = item.equipment.map(|e| e.slot);
        game.inventory.push(item);

        // automatically equip, if the corresponding equipment slot is unused
        if let Some(slot) = slot {
            if get_equipped_in_slot(slot, &game.inventory).is_none() {
                game.inventory[index].equip(&mut game.messages);
            }
        }
    }
}

/// find closest enemy, up to a maximum range, and in the player's FOV
pub fn closest_monster(tcod: &Tcod, objects: &[Object], max_range: i32) -> Option<usize> {
    let mut closest_enemy = None;
    let mut closest_dist = (max_range + 1) as f32; // start with (slightly more than) maximum range

    for (id, object) in objects.iter().enumerate() {
        if (id != PLAYER)
            && object.fighter.is_some()
            && object.ai.is_some()
            && tcod.fov.is_in_fov(object.x, object.y)
        {
            // calculate distance between this object and the player
            let dist = objects[PLAYER].distance_to(object);
            if dist < closest_dist {
                // it's closer, so remember it
                closest_enemy = Some(id);
                closest_dist = dist;
            }
        }
    }
    closest_enemy
}
pub fn target_tile(
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &[Object],
    max_range: Option<f32>,
) -> Option<(i32, i32)> {
    use tcod::input::*;
    loop {
        // render the screen. this erases the inventory and shows the names of
        // objects under the mouse.
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
        if tcod.mouse.rbutton_pressed || tcod.key.code == input::KeyCode::Escape {
            return None; // cancel if the player right-clicked or pressed Escape
        }
    }
}
pub fn target_monster(
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &[Object],
    max_range: Option<f32>,
) -> Option<usize> {
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

pub fn initialise_fov(tcod: &mut Tcod, map: &Map) {
    // create the FOV map, according to the generated map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !map[x as usize][y as usize].block_sight,
                !map[x as usize][y as usize].blocked,
            );
        }
    }
    tcod.con.clear();
}

fn from_dungeon_level(table: &[Transition], level: u32) -> u32 {
    table
        .iter()
        .rev()
        .find(|transition| level >= transition.level)
        .map_or(0, |transition| transition.value)
}

use tcod::console::*;
use tcod::colors::*;
use tcod::map::Map as FovMap;
use tcod::input::{Key, Mouse};
use crate::libs::handle_keys::*;
use crate::predefs::constants::*;


pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
    pub panel: Offscreen,
    pub fov: FovMap,
    pub key: Key,  
    pub mouse: Mouse,
}

#[derive(Debug)]
pub struct Object{
    pub x: i32,
    pub y: i32,
    pub char: char,
    pub color: Color,
    pub name: String,  
    pub blocks: bool,  
    pub alive: bool,
    pub fighter: Option<Fighter>,  
    pub ai: Option<Ai>,
    pub item: Option<Item>,
}

impl Object{
    pub fn new (x: i32, y: i32, char: char, name: &str, color: Color, blocks: bool) -> Self{
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
            name: name.into(),
            blocks: blocks,
            alive: false,
            fighter: None,
            ai: None,
            item: None,
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    /// return the distance to another object
    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn take_damage(&mut self, damage: i32, game: &mut Game) {
        // apply damage if possible
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }
        if let Some(fighter) = self.fighter {
            if fighter.hp <= 0 {
                self.alive = false;
                fighter.on_death.callback(self, game);
            }
        }
    }

    pub fn attack(&mut self, target: &mut Object, game: &mut Game) {
        // a simple formula for attack damage
        let damage = self.fighter.map_or(0, |f| f.power) - target.fighter.map_or(0, |f| f.defense);
        if damage > 0 {
            // make the target take some damage
            game.messages.add(format!(
                "{} attacks {} for {} hit points.",
                self.name, target.name, damage
            ), ORANGE);
            target.take_damage(damage, game);
        } else {
            game.messages.add(format!(
                "{} attacks {} but it has no effect!",
                self.name, target.name
            ), ORANGE);
        }
    }

    pub fn heal(&mut self, amount: i32) {
        if let Some(ref mut fighter) = self.fighter {
            fighter.hp += amount;
            if fighter.hp > fighter.max_hp {
                fighter.hp = fighter.max_hp;
            }
        }
    }
    pub fn inc_power(&mut self, amount: i32) {
        if let Some(ref mut fighter) = self.fighter {
            fighter.power += amount;
            if fighter.power > MAX_POWER {
                fighter.power = MAX_POWER;
            }
        }
    }
    pub fn inc_defense(&mut self, amount: i32) {
        if let Some(ref mut fighter) = self.fighter {
            fighter.defense += amount;
            if fighter.defense > MAX_DEFENSE {
                fighter.defense = MAX_DEFENSE;
            }
        }
    }
    pub fn inc_max_hp(&mut self, amount: i32) {
        if let Some(ref mut fighter) = self.fighter {
            fighter.max_hp += amount;
            if fighter.max_hp > MAX_MAX_HP {
                fighter.max_hp = MAX_MAX_HP;
            }
        }
    }
    
    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub explored: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            explored:false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            explored:false,
            block_sight: true,
        }
    }
}

pub type Map = Vec<Vec<Tile>>;

pub struct Game{
    pub map: Map,
    pub messages: Messages,
    pub inventory: Vec<Object>,
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }
    
    pub fn intersects_with(&self, other: &Rect) -> bool {
        // returns true if this rectangle intersects with another one
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

// combat-related properties and methods (monster, player, NPC).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub on_death: DeathCallback,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ai {
    Basic,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}
impl DeathCallback {
    pub fn callback(self, object: &mut Object, game: &mut Game) {
        use DeathCallback::*;
        let callback: fn(&mut Object, &mut Game) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(object, game);
    }
}

pub struct Messages {
    messages: Vec<(String, Color)>,
}
impl Messages {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    /// add the new message as a tuple, with the text and the color
    pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.messages.push((message.into(), color));
    }

    /// Create a `DoubleEndedIterator` over the messages
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, Color)> {
        self.messages.iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    MinorHeal,
    Heal,
    MajorHeal,
    PotionPwr,
    PotionDef,
    PotionHp,
    ScrollLighting,


}
pub enum UseResult {
    UsedUp,
    Cancelled,
}
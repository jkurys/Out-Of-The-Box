use std::fmt;

use bevy::prelude::*;
use bevy::utils::HashSet;
use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::resources::{CurrentSprite, Images};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GameObject {
    Box,
    Wall,
    HidingWall { color: usize, hidden_toggle: bool, hidden_by_def: bool },
    Empty,
    Player { powerup: Option<PowerUpType>, direction: Direction },
    Turtle { direction: Direction, color: usize },
    TurtleHead { direction: Direction, color: usize },
    PowerUp { powerup_type: PowerUpType },
}

pub fn get_obj_indices(
    obj: GameObject,
    current_sprite: &Res<CurrentSprite>,
) -> (usize, usize, usize) {
    match obj {
        GameObject::Box => (1, 0, 4),
        GameObject::Wall => (1, 0, 2),
        GameObject::HidingWall { color, hidden_toggle: _, hidden_by_def: _ } => (
            color * 3 + 1,
            color * 3,
            color * 3 + 2
        ),
        GameObject::Empty => (0, 0, 0),
        GameObject::Player { powerup, direction: _ } => 
            match powerup {
                None => (
                    current_sprite.0 * 4 + 1,
                    current_sprite.0 * 4,
                    current_sprite.0 * 4 + 2,
                ),
                Some(PowerUpType::Rocket) => (
                    current_sprite.0 * 4 + 1,
                    current_sprite.0 * 4,
                    current_sprite.0 * 4 + 2,
                ),
            }
        ,
        GameObject::Turtle { direction, .. } => (
            direction.to_num() * 6 + 1,
            direction.to_num() * 6,
            direction.to_num() * 6 + 2,
        ),
        GameObject::TurtleHead { direction, .. } => (
            direction.to_num() * 6 + 4,
            direction.to_num() * 6 + 3,
            direction.to_num() * 6 + 5,
        ),
        GameObject::PowerUp { powerup_type } => (
            powerup_type.to_num(),
            3,      // empty img
            3,
        )
    }
}

pub fn get_obj_img(
    obj: GameObject,
    images: &Res<Images>,
) -> Option<(Handle<Image>, Handle<TextureAtlasLayout>)> {
    match obj {
        GameObject::Box => images.box_images.clone(),
        GameObject::Wall => images.wall_images.clone(),
        GameObject::HidingWall { color: _, hidden_toggle: _, hidden_by_def: _ } => images.hidden_wall_images.clone(),
        GameObject::Empty => None,
        GameObject::Player { .. } => images.player_images.clone(),
        GameObject::TurtleHead { .. } => images.turtle_images.clone(),
        GameObject::Turtle { .. } => images.turtle_images.clone(),
        GameObject::PowerUp { .. } => None,
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Floor {
    HiddenWall {
        hidden_by_default: bool,
        color: usize,
    },
    Tile,
    Ice,
    Goal,
    Warp(usize),
    Button(usize),
    Dirt,
    Void,
    Obj(GameObject),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PowerUpType {
    Rocket,
}

impl PowerUpType {
    pub fn to_num(&self) -> usize {
        match self {
            Self::Rocket => {
                0
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub positions: HashSet<Position>,
}

impl Block {
    pub fn contains_position(&self, position: Position) -> bool {
        self.positions.contains(&position)
    }

    pub fn get_last_pos(&self) -> Position {
        let mut last_pos = None;
        for pos in self.positions.iter() {
            last_pos = Some(*pos)
        }
        return last_pos.unwrap();
    }
    // pub fn cmp_to_other(&self, other: &Block, dir: Direction) -> Ordering {
    // }
}

impl std::hash::Hash for Block {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        let positions_vec: Vec<Position> = self.positions.iter().map(|&p| p).collect();
        positions_vec.hash(state);
    }
}

#[derive(Component, Clone)]
pub struct PowerUp;

#[derive(Component, Clone)]
pub struct Button;

#[derive(Component, Clone)]
pub struct Turtle;

#[derive(Component, Clone)]
pub struct HiddenWall;

#[derive(Component, Clone)]
pub struct Goal;

#[derive(Component, Clone)]
pub struct Wall;

#[derive(Component, Clone)]
pub struct Background;

#[derive(Component, PartialEq, Eq, Hash, Clone)]
pub struct Box;

#[derive(Component, Clone)]
pub struct Player;

#[derive(Component, Clone)]
pub struct Ice;

// #[derive(Component)]
// pub struct BoxButton;
//
#[derive(Component, Clone)]
pub struct Glue;

#[derive(Component, Clone)]
pub struct Dirt;

#[derive(Component, Clone)]
pub struct Water;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}:{}:{}", self.x, self.y, self.z))
    }
}

struct PositionVisitor;

impl<'de> Visitor<'de> for PositionVisitor {
    type Value = Position;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a colon-separated triple of integers between 0 and 255")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut splits = s.split(':');
        let x_char_opt = splits.next();
        if x_char_opt.is_none() {
            return Err(de::Error::invalid_value(Unexpected::Str(s), &self));
        }
        let x_char = x_char_opt.unwrap();
        if let Ok(x) = x_char.parse::<i32>() {
            let y_char_opt = splits.next();
            if y_char_opt.is_none() {
                return Err(de::Error::invalid_value(Unexpected::Str(s), &self));
            }
            let y_char = y_char_opt.unwrap();
            if let Ok(y) = y_char.parse::<i32>() {
                let z_char_opt = splits.next();
                if z_char_opt.is_none() {
                    return Err(de::Error::invalid_value(Unexpected::Str(s), &self));
                }
                let z_char = z_char_opt.unwrap();
                if let Ok(z) = z_char.parse::<i32>() {
                    Ok(Position { x, y, z })
                }
                else {
                    return Err(de::Error::invalid_value(Unexpected::Str(s), &self));
                }
            } else {
                return Err(de::Error::invalid_value(Unexpected::Str(s), &self));
            }
        } else {
            return Err(de::Error::invalid_value(Unexpected::Str(s), &self));
        }
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Position, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(PositionVisitor)
    }
}

impl Position {
    pub fn next_position(&self, dir: Direction) -> Position {
        match dir {
            Direction::North => Position {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Direction::Up => Position {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
        }
    }

    pub fn prev_position(&self, dir: Direction) -> Position {
        match dir {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Direction::Left => Position {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Direction::Right => Position {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Direction::Up => Position {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        }
    }
        
    pub fn position_above(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }

    pub fn position_below(&self) -> Position {
        if self.z == 0 {
            return *self;
        }
        Position {
            x: self.x,
            y: self.y,
            z: self.z - 1
        }
    }
}
//     pub fn cmp_to_other(&self, other: &Self, dir: Direction) -> Ordering {
//         match dir {
//             Direction::Up => other.y.cmp(&self.y),
//             Direction::Down => self.y.cmp(&other.y),
//             Direction::Left => self.x.cmp(&other.x),
//             Direction::Right => other.x.cmp(&self.x),
//         }
//     }


#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    North,
    South,
    Left,
    Right,
}

impl Direction {
    pub fn to_num(self) -> usize {
        match self {
            Direction::Left => 0,
            Direction::Right => 1,
            Direction::South => 2,
            Direction::North => 3,
            Direction::Down => 4,
            Direction::Up => 5,
        }
    }
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

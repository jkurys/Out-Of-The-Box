use std::fmt;

use bevy::prelude::*;
use bevy::utils::HashSet;
use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GameObject {
    Box,
    Wall,
    HidingWall { color: usize },
    Empty,
    Player,
    Turtle { direction: Direction, color: usize },
    TurtleHead { direction: Direction, color: usize },
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub positions: HashSet<Position>,
}

impl Block {
    pub fn contains_position(&self, position: Position) -> bool {
        self.positions.contains(&position)
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

#[derive(Component)]
pub struct Button;

#[derive(Component, Clone)]
pub struct Turtle;

#[derive(Component, Clone)]
pub struct HiddenWall;

#[derive(Component)]
pub struct Goal;

#[derive(Component, Clone)]
pub struct Wall;

#[derive(Component)]
pub struct Background;

#[derive(Component, PartialEq, Eq, Hash, Clone)]
pub struct Box;

#[derive(Component, Clone)]
pub struct Player;

#[derive(Component)]
pub struct Ice;

#[derive(Component)]
pub struct Warp;

#[derive(Component)]
pub struct BoxButton;

#[derive(Component, Clone)]
pub struct Glue;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}:{}", self.x, self.y))
    }
}

struct PositionVisitor;

impl<'de> Visitor<'de> for PositionVisitor {
    type Value = Position;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a colon-separated pair of integers between 0 and 255")
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
                Ok(Position { x, y })
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
            Direction::Up => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    pub fn prev_position(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x - 1,
                y: self.y,
            },
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
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_num(self) -> usize {
        match self {
            Direction::Left => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Up => 3,
        }
    }
}

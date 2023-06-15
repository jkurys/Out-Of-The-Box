use std::{cmp::Ordering, fmt};

use bevy::prelude::*;
use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GameObject {
    Box,
    Wall,
    HidingWall { color: usize },
    Empty,
    Player,
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

#[derive(Component)]
pub struct Button {
    pub on: bool,
}

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
        if let Ok(x) = i32::from_str_radix(x_char, 10) {
            let y_char_opt = splits.next();
            if y_char_opt.is_none() {
                return Err(de::Error::invalid_value(Unexpected::Str(s), &self));
            }
            let y_char = y_char_opt.unwrap();
            if let Ok(y) = i32::from_str_radix(y_char, 10) {
                return Ok(Position { x, y });
            } else {
                return Err(de::Error::invalid_value(Unexpected::Str(s), &self));
            }
        } else {
            return Err(de::Error::invalid_value(Unexpected::Str(s), &self));
        }

        // if let Some(nums) = Regex::new(r"(\d+):(\d+)").unwrap().captures_iter(s).next() {
        //     if let Ok(x) = i32::from_str_radix(&nums[1], 10) {
        //         if let Ok(y) = i32::from_str_radix(&nums[2], 10) {
        //             Ok(Position { x, y })
        //         } else {
        //             Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        //         }
        //     } else {
        //         Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        //     }
        // } else {
        //     Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        // }
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

    pub fn cmp_to_other(&self, other: &Self, dir: Direction) -> Ordering {
        match dir {
            Direction::Up => other.y.cmp(&self.y),
            Direction::Down => self.y.cmp(&other.y),
            Direction::Left => self.x.cmp(&other.x),
            Direction::Right => other.x.cmp(&self.x),
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

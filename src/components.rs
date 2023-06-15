use bevy::prelude::*;

use crate::game::game_objects::{GameObject, Floor};

#[derive(Component,  Clone, Copy, Debug)]
pub enum GameEntity {
    Object(GameObject),
    Floor(Floor),
}

impl Default for GameEntity {
    fn default() -> Self {
        GameEntity::Object(GameObject::Wall)
    }
}
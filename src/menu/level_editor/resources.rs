use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct BoardSize {
    pub width: u32,
    pub height: u32,
}

impl Default for BoardSize {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
        }
    }
}

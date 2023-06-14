use bevy::prelude::*;

#[derive(Resource)]
pub struct BoardSize {
    pub width: u32,
    pub height: u32,
}

impl Default for BoardSize {
    fn default() -> Self {
        Self { width: 1, height: 1 }
    }
}

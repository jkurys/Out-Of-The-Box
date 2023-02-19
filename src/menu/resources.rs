use bevy::{prelude::*, utils::HashMap};

use crate::{consts::WALL_TEXTURE, game::game_objects::Position};

use super::level_editor::GameEntity;

// #[derive(Resource)]
// pub struct CurrentImage(pub UiImage);

#[derive(Resource, Debug)]
pub struct LevelEditorBoard {
    pub objects: HashMap<Position, GameEntity>,
    pub width: u32,
    pub height: u32,
    pub image: UiImage,
}

impl FromWorld for LevelEditorBoard {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("Could not get asset server from world");
        let wall_image = asset_server.load(WALL_TEXTURE);
        LevelEditorBoard {
            image: UiImage(wall_image),
            objects: HashMap::new(),
            width: 1,
            height: 1,
        }
    }
}

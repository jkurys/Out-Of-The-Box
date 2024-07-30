use bevy::prelude::*;

use crate::consts::*;
use crate::state::DisplayState;
use std::fs::File;
use std::io::Read;

#[derive(Resource, Debug)]
pub struct CurrentLevel {
    pub level_number: usize,
    pub level_map_string: String,
    pub level_amount: usize,
    pub is_in_level: bool,
}

#[derive(Resource)]
pub struct Images {
    pub goal_image: Handle<Image>,
    pub player_images: Option<Handle<TextureAtlas>>,
    pub box_images: Option<Handle<TextureAtlas>>,
    pub wall_images: Option<Handle<TextureAtlas>>,
    pub tile_image: Handle<Image>,
    pub ice_image: Handle<Image>,
    pub warp_image: Handle<Image>,
    pub button_images: [Handle<Image>; 3],
    pub dirt_image: Handle<Image>,
    pub hidden_wall_images: Option<Handle<TextureAtlas>>,
    pub turtle_images: Option<Handle<TextureAtlas>>,
    pub glue_images: Option<Handle<TextureAtlas>>,
    pub box_glue_images: Option<Handle<TextureAtlas>>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("Asset server not found in world");
        let file_res = File::open(PLAYER_TEXTURE_SAVE);
        let mut buf = [0_u8; 1];
        if let Ok(mut file) = file_res {
            file.read_exact(&mut buf).unwrap();
        }
        let wall_images = None;
        let box_images = None;
        let goal_image = asset_server.load(GOAL_TEXTURE);
        let tile_image = asset_server.load(TILE_TEXTURE);
        let ice_image = asset_server.load(ICE_TEXTURE);
        let warp_image = asset_server.load(WARP_TEXTURE);
        let button_images = [
            asset_server.load(BUTTON_TEXTURES[0]),
            asset_server.load(BUTTON_TEXTURES[1]),
            asset_server.load(BUTTON_TEXTURES[2]),
        ];
        let dirt_image = asset_server.load(DIRT_TEXTURE);
        let hidden_wall_images = None;
        let player_images = None;
        let turtle_images = None;
        let glue_images = None;
        let box_glue_images = None;

        Images {
            player_images,
            box_images,
            wall_images,
            goal_image,
            tile_image,
            ice_image,
            warp_image,
            button_images,
            hidden_wall_images,
            turtle_images,
            glue_images,
            box_glue_images,
            dirt_image,
        }
    }
}

#[derive(Resource)]
pub struct StateStack(pub Vec<DisplayState>);

impl Default for StateStack {
    fn default() -> Self {
        Self(vec![DisplayState::MainMenu])
    }
}

#[derive(Resource)]
pub struct CurrentSprite(pub usize);

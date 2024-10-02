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
    pub player_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub box_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub wall_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub tile_image: Handle<Image>,
    pub ice_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub button_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub water_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub hidden_wall_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub turtle_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub glue_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub box_glue_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub highlight_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub goal_image: Handle<Image>,
    pub dirt_image: Handle<Image>,
    pub powerup_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub telebox_images: Option<(Handle<Image>, Handle<TextureAtlasLayout>)>,
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
        let tile_image = asset_server.load(TILE_TEXTURE);
        let ice_images = None;
        let water_images = None;
        let button_images = None;
        let hidden_wall_images = None;
        let player_images = None;
        let turtle_images = None;
        let glue_images = None;
        let box_glue_images = None;
        let highlight_images = None;
        let powerup_images = None;
        let telebox_images = None;
        let goal_image = asset_server.load(GOAL_TEXTURE);
        let dirt_image = asset_server.load(DIRT_TEXTURE);

        Images {
            player_images,
            box_images,
            wall_images,
            tile_image,
            ice_images,
            button_images,
            hidden_wall_images,
            turtle_images,
            glue_images,
            box_glue_images,
            water_images,
            highlight_images,
            goal_image,
            dirt_image,
            powerup_images,
            telebox_images,
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

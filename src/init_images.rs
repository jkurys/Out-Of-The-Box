use bevy::prelude::*;

use crate::{consts::*, resources::Images};

pub fn init_images(
    mut images: ResMut<Images>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let layout = |x, y| {TextureAtlasLayout::from_grid(
        UVec2 { x: 480, y: 480 },
        x,
        y,
        Some(UVec2 { x: 20, y: 20 }),
        None,
    )};
    let wall_atlas_texture = asset_server.load(WALL_ATLAS);
    images.wall_images = Some((wall_atlas_texture, atlases.add(layout(4, 2))));

    let box_atlas_texture = asset_server.load(BOX_ATLAS);
    images.box_images = Some((box_atlas_texture, atlases.add(layout(3, 2))));

    let hiding_wall_atlas_texture = asset_server.load(HIDING_WALL_ATLAS);
    images.hidden_wall_images = Some((hiding_wall_atlas_texture, atlases.add(layout(3, 3))));

    let player_atlas_texture = asset_server.load(PLAYER_ATLAS);
    images.player_images = Some((player_atlas_texture, atlases.add(layout(7, 4))));

    let turtle_images_texture = asset_server.load(TURTLE_ATLAS);
    images.turtle_images = Some((turtle_images_texture, atlases.add(layout(6, 5))));

    let glue_images_texture = asset_server.load(GLUE_ATLAS);
    images.glue_images = Some((glue_images_texture, atlases.add(layout(2, 2))));

    let box_glue_images_texture = asset_server.load(BOX_GLUE_ATLAS);
    images.box_glue_images = Some((box_glue_images_texture, atlases.add(layout(2, 4))));

    let button_images_texture = asset_server.load(BUTTON_ATLAS);
    images.button_images = Some((button_images_texture, atlases.add(layout(3, 1))));

    let water_images_texture = asset_server.load(WATER_TEXTURE);
    images.water_images = Some((water_images_texture, atlases.add(layout(2, 2))));

    let ice_images_texture = asset_server.load(ICE_TEXTURE);
    images.ice_images = Some((ice_images_texture, atlases.add(layout(2, 2))));

    let highlight_images_texture = asset_server.load(HIGHLIGHTS_TEXTURE);
    images.highlight_images = Some((highlight_images_texture, atlases.add(layout(2, 2))));

    let powerup_images_texture = asset_server.load(POWERUP_TEXTURE);
    images.powerup_images = Some((powerup_images_texture, atlases.add(layout(2, 2))));

    let telebox_images_texture = asset_server.load(TELEBOX_TEXTURE);
    images.telebox_images = Some((telebox_images_texture, atlases.add(layout(2, 2))));
}

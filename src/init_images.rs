use bevy::prelude::*;

use crate::{consts::*, resources::Images};

pub fn init_images(
    mut images: ResMut<Images>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let wall_atlas_texture = asset_server.load(WALL_ATLAS);
    let wall_atlas = TextureAtlas::from_grid(
        wall_atlas_texture,
        Vec2 { x: 480., y: 480. },
        4,
        2,
        Some(Vec2 { x: 20., y: 20. }),
        None,
    );
    images.wall_images = Some(atlases.add(wall_atlas));

    let box_atlas_texture = asset_server.load(BOX_ATLAS);
    let box_atlas = TextureAtlas::from_grid(
        box_atlas_texture,
        Vec2 { x: 480., y: 480. },
        3,
        2,
        Some(Vec2 { x: 20., y: 20. }),
        None,
    );
    images.box_images = Some(atlases.add(box_atlas));

    let hiding_wall_atlas_texture = asset_server.load(HIDING_WALL_ATLAS);
    let hiding_wall_atlas = TextureAtlas::from_grid(
        hiding_wall_atlas_texture,
        Vec2 { x: 480., y: 480. },
        3,
        3,
        Some(Vec2 { x: 20., y: 20. }),
        None,
    );
    images.hidden_wall_images = Some(atlases.add(hiding_wall_atlas));

    let player_atlas_texture = asset_server.load(PLAYER_ATLAS);
    let player_atlas = TextureAtlas::from_grid(
        player_atlas_texture,
        Vec2 { x: 480., y: 480. },
        4,
        4,
        Some(Vec2 { x: 20., y: 20. }),
        None,
    );
    images.player_images = Some(atlases.add(player_atlas));
    let turtle_images_texture = asset_server.load(TURTLE_ATLAS);
    let turtle_atlas = TextureAtlas::from_grid(
        turtle_images_texture,
        Vec2 { x: 480., y: 480. },
        6,
        5,
        Some(Vec2 { x: 20., y: 20. }),
        None,
    );
    images.turtle_images = Some(atlases.add(turtle_atlas));
    let glue_images_texture = asset_server.load(GLUE_ATLAS);
    let glue_atlas = TextureAtlas::from_grid(
        glue_images_texture,
        Vec2::splat(480.),
        2,
        2,
        Some(Vec2::splat(20.)),
        None,
    );
    images.glue_images = Some(atlases.add(glue_atlas));
    let box_glue_images_texture = asset_server.load(BOX_GLUE_ATLAS);
    let box_glue_atlas = TextureAtlas::from_grid(
        box_glue_images_texture,
        Vec2 { x: 20., y: 16. },
        2,
        4,
        Some(Vec2 { x: 0., y: 4. }),
        None,
    );
    images.box_glue_images = Some(atlases.add(box_glue_atlas));
    let button_images_texture = asset_server.load(BUTTON_ATLAS);
    let button_atlas = TextureAtlas::from_grid(
        button_images_texture,
        Vec2 { x: 480., y: 480. },
        3,
        1,
        Some(Vec2 { x: 20., y: 20. }),
        None,
    );
    images.button_images = Some(atlases.add(button_atlas));
    let water_images_texture = asset_server.load(WATER_TEXTURE);
    let water_atlas = TextureAtlas::from_grid(
        water_images_texture,
        Vec2 { x: 480., y: 480. },
        2,
        2,
        Some(Vec2 { x: 20., y: 20. }),
        None,
    );
    images.water_images = Some(atlases.add(water_atlas));
    let ice_images_texture = asset_server.load(ICE_TEXTURE);
    let ice_atlas = TextureAtlas::from_grid(
        ice_images_texture,
        Vec2 { x: 480., y: 480. },
        2,
        2,
        Some(Vec2 { x: 20., y: 20. }),
        None,
    );
    images.ice_images = Some(atlases.add(ice_atlas));
}

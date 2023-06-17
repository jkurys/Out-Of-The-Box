use bevy::prelude::*;

use crate::{
    consts::{LOWER_HALF_OBJECT_Z_INDEX, TILE_SIZE, UPPER_HALF_OBJECT_Z_INDEX},
    game::GameItem,
};

pub fn render_object<T>(
    commands: &mut Commands,
    atlas_handle: Handle<TextureAtlas>,
    bottom_index: usize,
    top_index: usize,
    x: i32,
    y: i32,
    component: T,
) -> [Entity; 2]
where
    T: Component + Clone,
{
    let mut higher_image = TextureAtlasSprite::new(top_index);
    let mut lower_image = TextureAtlasSprite::new(bottom_index);
    higher_image.custom_size = Some(Vec2::splat(TILE_SIZE + 0.5));
    lower_image.custom_size = Some(Vec2::splat(TILE_SIZE + 0.5));
    let (upper_x, upper_y, upper_z) = (
        x as f32 * TILE_SIZE,
        (y as f32 + 0.24) * TILE_SIZE,
        UPPER_HALF_OBJECT_Z_INDEX,
    );
    let (lower_x, lower_y, lower_z) = (
        x as f32 * TILE_SIZE,
        (y as f32 - 0.375) * TILE_SIZE,
        LOWER_HALF_OBJECT_Z_INDEX,
    );
    let entity1 = commands
        .spawn(SpriteSheetBundle {
            sprite: higher_image,
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(upper_x, upper_y, upper_z),
            ..default()
        })
        .insert((component.clone(), GameItem))
        .id();
    let entity2 = commands
        .spawn(SpriteSheetBundle {
            sprite: lower_image,
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(lower_x, lower_y, lower_z),
            ..default()
        })
        .insert((component, GameItem))
        .id();
    [entity1, entity2]
}

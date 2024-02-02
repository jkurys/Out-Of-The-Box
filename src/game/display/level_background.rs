use bevy::prelude::*;

use crate::consts::BACKGROUND_TEXTURE;

pub fn render_background(asset_server: Res<AssetServer>, mut commands: Commands) {
    let background = asset_server.load(BACKGROUND_TEXTURE);
    commands.spawn((SpriteBundle {
        texture: background,
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3 {
            x: 50.,
            y: 50.,
            z: 1.,
        }),
        ..default()
    },));
}

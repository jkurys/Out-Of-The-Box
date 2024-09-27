use bevy::prelude::*;

use crate::consts::{BACKGROUND_TEXTURE, BUTTON_PRESS_TEXTURE};

use super::{resources::{ButtonAnimationTimer, ButtonState}, ButtonPopup};

pub fn render_background(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    mut timer: ResMut<ButtonAnimationTimer>,
    button_state: ResMut<ButtonState>,
) {
    timer.0.tick(time.delta());
    let background = asset_server.load(BACKGROUND_TEXTURE);
    let button_press = asset_server.load(BUTTON_PRESS_TEXTURE);
    commands.spawn((SpriteBundle {
        texture: background,
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3 {
            x: 50.,
            y: 50.,
            z: 1.,
        }),
        ..default()
    },));
    commands.spawn((SpriteBundle {
        texture: button_press,
        transform: Transform {
            translation: Vec3 {
                x: 150.,
                y: -500.,
                z: 200.,
            },
            ..default()
        },
        ..default()
        
    }, TextureAtlas {
        layout: atlases.add(TextureAtlasLayout::from_grid(    
            UVec2 { x: 480, y: 480 },
            2,
            2,
            Some(UVec2 { x: 20, y: 20 }),
            None,
        )),
        index: button_state.0 as usize,
    })).insert(ButtonPopup);
}

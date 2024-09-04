use bevy::color::palettes::css::{BROWN, LIMEGREEN, PURPLE};
use bevy::prelude::*;
// use bevy::sprite::ColorMaterial;

use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use crate::consts::{TILE_WIDTH, TILE_HEIGHT};
use crate::game::GameItem;
use crate::game::game_objects::Background;
use super::handle_click::get_frontmost_position;
use crate::resources::Images;
use crate::board::Board;
use crate::game::display::render_2_5_d::{render_object, get_offsets};

pub fn handle_highlight(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    board: ResMut<Board>,
    images: Res<Images>,
) {
    let (mouse_pos, _) = get_frontmost_position(
        &mouse,
        &windows,
        &board,
        // &mut meshes,
        // &mut materials,
        // &mut commands
    );
    let (uppers, _lowers, _sides) = get_offsets(mouse_pos.x, mouse_pos.y, mouse_pos.z, 0.);
    // let mesh = Mesh2dHandle(meshes.add(Rectangle::new(TILE_WIDTH + 3., TILE_HEIGHT + 3.)));
    //     commands.spawn(
    //         MaterialMesh2dBundle {
    //             mesh,
    //             material: materials.add(Color::Srgba(LIMEGREEN)),
    //             transform: Transform::from_xyz(
    //                 uppers.0,
    //                 uppers.1,
    //                 1000.
    //             ),
    //             ..default()
    //         }
    //     );
    // let transform = Transform::default().with_scale(Vec3::splat(128.));
    

    render_object(&mut commands, images.highlight_images.clone().unwrap(), (1, 0, 2), mouse_pos.x, mouse_pos.y, mouse_pos.z, 0., Background);
    }

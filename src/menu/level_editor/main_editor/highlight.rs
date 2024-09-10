use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use crate::game::game_objects::Background;
use super::handle_click::get_frontmost_position;
use crate::resources::Images;
use crate::board::Board;
use crate::game::display::render_2_5_d::render_object;

pub fn handle_highlight(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    board: ResMut<Board>,
    images: Res<Images>,
) {
    let (mouse_pos, _) = get_frontmost_position(
        &mouse,
        &windows,
        &board,
    );

    render_object(&mut commands, images.highlight_images.clone().unwrap(), (1, 0, 2), mouse_pos.x, mouse_pos.y, mouse_pos.z, 1., Background);
}

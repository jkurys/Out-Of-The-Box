use bevy::prelude::*;

use crate::{exit::handle_esc, state::DisplayState, utils::delete_all_components};

use self::{
    handle_click::{handle_sprite_click, SpriteSelectItem},
    setup::setup_sprite_select,
};

mod handle_click;
mod setup;
mod styles;

pub struct SpriteSelectPlugin;

impl Plugin for SpriteSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_sprite_select.in_schedule(OnEnter(DisplayState::SpriteSelect)))
            .add_systems(
                (handle_sprite_click, handle_esc).in_set(OnUpdate(DisplayState::SpriteSelect)),
            )
            .add_system(
                delete_all_components::<SpriteSelectItem>
                    .in_schedule(OnExit(DisplayState::SpriteSelect)),
            );
    }
}

use bevy::prelude::*;

use crate::{exit::handle_esc, state::DisplayState, utils::delete_all_components};

use self::{
    handle_click::{handle_sprite_click, SpriteSelectItem},
    setup::setup_sprite_select,
};

mod handle_click;
mod setup;

pub struct SpriteSelectPlugin;

impl Plugin for SpriteSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DisplayState::SpriteSelect),
            setup_sprite_select
        )
            .add_systems(
                Update,
                (handle_sprite_click, handle_esc).run_if(in_state(DisplayState::SpriteSelect)),
            )
            .add_systems(
                OnExit(DisplayState::SpriteSelect),
                delete_all_components::<SpriteSelectItem>
            );
    }
}

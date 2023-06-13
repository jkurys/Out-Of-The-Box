use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Default, Eq, Debug, Hash, States)]
pub enum DisplayState {
    Game,
    #[default]
    MainMenu,
    LevelSelect,
    Victory,
    SpriteSelect,
    LevelEditorInput,
    LevelEditorBoard,
    LevelEditorSave,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default, States)]
pub enum MoveState {
    Moving,
    #[default]
    Static,
}

use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Default, Eq, Debug, Hash, States)]
pub enum DisplayState {
    Game,
    #[default]
    MainMenu,
    LevelSelect,
    Victory,
    SpriteSelect,
    LevelEditorSelect,
    LevelEditorLevelSelect,
    LevelEditorInput,
    LevelEditorBoard,
    LevelEditorSave,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default, States)]
pub enum MoveState {
    Animation,
    #[default]
    Static,
    Calculating,
    AfterAnimationCalc,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum DisplayState {
    Game,
    MainMenu,
    LevelSelect,
    Victory,
    SpriteSelect,
    LevelEditorInput,
    LevelEditorBoard(u32, u32),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct GameState(pub Option<Move>);

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Move {
    Moving,
    Static,
}

impl Default for GameState {
    fn default() -> Self {
        GameState(Some(Move::Static))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct CurrentMap(pub Option<usize>);

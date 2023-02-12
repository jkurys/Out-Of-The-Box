#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum DisplayState {
    Game,
    MainMenu,
    LevelSelect,
    Victory,
    SpriteSelect,
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

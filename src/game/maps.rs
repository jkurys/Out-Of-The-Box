use crate::board::Board;
use crate::resources::CurrentLevel;
use bevy::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn load_starting_map(mut board: ResMut<Board>, current_level: Res<CurrentLevel>) {
    let path = format!("assets/maps/{}", current_level.level_map_string.as_str());
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader);
    *board = result.unwrap_or(Board::new());
}

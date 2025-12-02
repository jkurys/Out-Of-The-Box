use crate::board::GameData;
use crate::resources::CurrentLevel;
use bevy::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn load_starting_map(mut game_data: ResMut<GameData>, current_level: Res<CurrentLevel>) {
    let path = format!("assets/maps/{}", current_level.level_map_string.as_str());
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader);
    match result {
        Ok(map) => {
            game_data.board = map;
        }
        Err(e) => {
            error!(
                "Failed to load map: {} due to error:\n {}",
                current_level.level_map_string, e
            );
        }
    }
}

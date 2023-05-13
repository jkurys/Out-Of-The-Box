use bevy::{prelude::*, utils::HashMap};

use crate::{consts::WALL_TEXTURE, game::game_objects::Position};

use super::level_editor::GameEntity;

#[derive(Resource, Debug)]
pub struct LevelEditorBoard {
    pub curr_map: usize,
    pub boards: [Option<LevelEditorSingleBoard>; 10],
    pub image: UiImage,
    pub created_maps: usize,
}

#[derive(Resource, Debug)]
pub struct LevelNames(pub Vec<String>);

#[derive(Resource, Debug, Clone)]
pub struct LevelEditorSingleBoard {
    pub objects: HashMap<Position, GameEntity>,
    pub width: u32,
    pub height: u32,
}

impl LevelEditorBoard {
    // pub fn get_current_map(&self) -> LevelEditorSingleBoard {
    //     match &self.boards[self.curr_map] {
    //         Some(board) => board,
    //         _ => panic!("requested uninitialized board")
    //     }
    // }

    pub fn set_size(&mut self, width: u32, height: u32) {
        match &mut self.boards[self.curr_map] {
            Some(board) => {
                board.width = width;
                board.height = height;
            }
            _ => panic!("requested uninitialized board: set_size")
        };
    }

    pub fn get_width_n(&self, n: usize) -> u32 {
        match &self.boards[n] {
            Some(board) => board.width,
            _ => panic!("requested uninitialized board: get_width_n")
        }
    }

    pub fn get_height_n(&self, n: usize) -> u32 {
        match &self.boards[n] {
            Some(board) => board.height,
            _ => panic!("requested uninitialized board: get_height_n")
        }
    }

    // pub fn get_width(&self) -> u32 {
    //     match &self.boards[self.curr_map] {
    //         Some(board) => board.width,
    //         _ => panic!("requested uninitialized board")
    //     }
    // }

    // pub fn get_height(&self) -> u32 {
    //     match &self.boards[self.curr_map] {
    //         Some(board) => board.height,
    //         _ => panic!("requested uninitialized board")
    //     }
    // }

    pub fn insert_object(&mut self, position: Position, object: GameEntity) {
        match &mut self.boards[self.curr_map] {
            Some(board) => {
                board.objects.insert(position, object);
            }
            _ => panic!("requested uninitialized board: insert_object")
        }
    }

    pub fn remove_object(&mut self, position: Position) {
        match &mut self.boards[self.curr_map] {
            Some(board) => {
                board.objects.remove(&position);
            }
            _ => panic!("requested uninitialized board: remove_object")
        }
    }

    pub fn init_map_n(&mut self, n: usize) {
        self.boards[n] = Some(LevelEditorSingleBoard {
            objects: HashMap::new(),
            width: 0,
            height: 0,
        });
    }

    pub fn give_map_n(&self, n: usize) -> Option<HashMap<Position, GameEntity>> {
        match &self.boards[n] {
            Some(board) => Some(board.objects.clone()),
            _ => None
        }
    }


}

impl FromWorld for LevelEditorBoard {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("Could not get asset server from world");
        let wall_image = asset_server.load(WALL_TEXTURE);
        let mut boards = [None, None, None, None, None, None, None, None, None, None];
        let board = LevelEditorSingleBoard {
            objects: HashMap::new(),
            width: 1,
            height: 1,
        };
        boards[0] = Some(board);
        LevelEditorBoard {
            curr_map: 0,
            created_maps: 1,
            image: UiImage(wall_image),
            boards,
        }
    }
}

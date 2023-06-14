use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::game::game_objects::{Direction, Floor, GameObject, Position};
use crate::consts::*;
use crate::menu::level_editor::editor::GameEntity;
use crate::state::DisplayState;
use std::fs::File;
use std::io::Read;
use crate::consts::{INITIAL_MAP, MAX_MAPS};

#[derive(Resource)]
pub struct CurrentLevel {
    pub level_number: usize,
    pub level_map_string: String,
    pub level_amount: usize,
}

#[derive(Resource)]
pub struct Images {
    pub goal_image: Handle<Image>,
    pub player_images: [Handle<Image>; 2],
    pub box_images: [Handle<Image>; 2],
    pub box_on_goal_images: [Handle<Image>; 2],
    pub wall_images: [Handle<Image>; 2],
    pub tile_image: Handle<Image>,
    pub ice_image: Handle<Image>,
    pub warp_image: Handle<Image>,
    pub button_images: [Handle<Image>; 3],
    pub hidden_wall_images: [Handle<Image>; 3],
    pub shown_hidden_wall_images: [[Handle<Image>; 2]; 3],
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("Asset server not found in world");
        let file_res = File::open(PLAYER_TEXTURE_SAVE);
        let mut buf = [0_u8; 1];
        if let Ok(mut file) = file_res {
            file.read_exact(&mut buf).unwrap();
        }
        let player_images = [asset_server.load(LOWER_PLAYER_TEXTURES[buf[0] as usize]), asset_server.load(PLAYER_TEXTURES[buf[0] as usize])];
        let box_images = [asset_server.load(LOWER_BOX_TEXTURE), asset_server.load(HIGHER_BOX_TEXTURE)];
        let wall_images = [asset_server.load(LOWER_WALL_TEXTURE), asset_server.load(HIGHER_WALL_TEXTURE)];
        let goal_image = asset_server.load(GOAL_TEXTURE);
        let box_on_goal_images = [asset_server.load(LOWER_BOX_TEXTURE), asset_server.load(HIGHER_BOX_ON_GOAL_TEXTURE)];
        let tile_image = asset_server.load(TILE_TEXTURE);
        let ice_image = asset_server.load(ICE_TEXTURE);
        let warp_image = asset_server.load(WARP_TEXTURE);
        let button_images = [
            asset_server.load(BUTTON_TEXTURES[0]),
            asset_server.load(BUTTON_TEXTURES[1]),
            asset_server.load(BUTTON_TEXTURES[2])
        ];
        let hidden_wall_images = [asset_server.load(HIDDEN_WALL_TEXTURES[0]), asset_server.load(HIDDEN_WALL_TEXTURES[1]), asset_server.load(HIDDEN_WALL_TEXTURES[2])];
        let shown_hidden_wall_images = [
            [asset_server.load(LOWER_SHOWN_HIDDEN_WALL_TEXTURES[0]), asset_server.load(HIGHER_SHOWN_HIDDEN_WALL_TEXTURES[0])],
            [asset_server.load(LOWER_SHOWN_HIDDEN_WALL_TEXTURES[1]), asset_server.load(HIGHER_SHOWN_HIDDEN_WALL_TEXTURES[1])],
            [asset_server.load(LOWER_SHOWN_HIDDEN_WALL_TEXTURES[2]), asset_server.load(HIGHER_SHOWN_HIDDEN_WALL_TEXTURES[2])],
        ];

        Images {
            player_images,
            box_images,
            wall_images,
            goal_image,
            box_on_goal_images,
            tile_image,
            ice_image,
            warp_image,
            button_images,
            hidden_wall_images,
            shown_hidden_wall_images,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MapSize {
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Clone)]
struct SingleBoard {
    entities: HashMap<Position, [Entity; 2]>,
    objects: HashMap<Position, GameObject>,
    floors: HashMap<Position, Floor>,
    goals: Vec<Position>,
    buttons: Vec<Vec<Position>>,
    map_size: MapSize,
    player_position: Position,
    warp_positions: [Position; MAX_MAPS],
}

#[derive(Resource, Debug, Clone)]
pub struct Board {
    boards: Vec<SingleBoard>,
    current: usize,
}

impl Board {
    pub fn new() -> Self {
        let mut boards = Vec::new();
        for _ in 0..MAX_MAPS {
            boards.push(SingleBoard {
                entities: HashMap::new(),
                objects: HashMap::new(),
                floors: HashMap::new(),
                goals: Vec::new(),
                buttons: vec![Vec::new(), Vec::new(), Vec::new()],
                map_size: MapSize {
                    width: 0,
                    height: 0,
                },
                player_position: Position { x: 0, y: 0},
                warp_positions: [Position { x: 0, y: 0}; 10],
            });
        }
        Board {
            current: INITIAL_MAP,
            boards,
        }
    }

    pub fn set_current_map(&mut self, current: usize) {
        self.current = current;
    }

    pub fn set_map_size(&mut self, map_size: MapSize) {
        self.boards[self.current].map_size = map_size;
    }

    pub fn get_map_size(&self) -> MapSize {
        self.boards[self.current].map_size
    }

    pub fn get_map_size_n(&self, n: usize) -> MapSize {
        self.boards[n].map_size
    }

    pub fn get_player_position(&self) -> Position {
        self.boards[self.current].player_position
    }

    pub fn get_entities(&self, position: Position) -> Option<[Entity; 2]> {
        self.boards[self.current].entities.get(&position).copied()
    }

    pub fn get_object_type(&self, position: Position) -> GameObject {
        self.get_object_from_map(position, self.current)
    }

    pub fn get_object_from_map(&self, position: Position, map: usize) -> GameObject {
        *self.boards[map].objects.get(&position).unwrap_or(&GameObject::Empty)
    }

    pub fn get_floor_type(&self, position: Position) -> Floor {
        self.get_floor_from_map(position, self.current)
    }

    pub fn get_floor_from_map(&self, position: Position, map: usize) -> Floor {
        *self.boards[map]
            .floors
            .get(&position)
            .unwrap_or(&Floor::Tile)
    }

    pub fn get_all_goals(&self) -> Vec<Position> {
        let mut goals_vec = Vec::new();
        for map in 0..MAX_MAPS {
            goals_vec.push(self.boards[map].goals.clone());
        }
        goals_vec.concat() //realistically, this vector won't exceed 20 entries so cloning isn't a problem
    }

    pub fn get_all_buttons(&self) -> Vec<Vec<Position>> {
        self.boards[self.current].buttons.clone()
    }

    pub fn get_current_map(&self) -> usize {
        self.current
    }

    pub fn get_board_n(&self, n: usize) -> (HashMap<Position, GameObject>, HashMap<Position, Floor>) {
        (self.boards[n].objects.clone(), self.boards[n].floors.clone())
    }

    pub fn insert(&mut self, position: Position, floor_or_object: GameEntity) {
        match floor_or_object {
            GameEntity::Floor(f) => self.insert_floor_to_map(position, f, self.current),
            GameEntity::Object(o) => self.insert_object(position, o),
        };
    }

    pub fn insert_object(&mut self, position: Position, object: GameObject) {
        self.insert_object_to_map(position, object, self.current);
    }

    pub fn insert_object_to_map(&mut self, position: Position, object: GameObject, map: usize) {
        if object == GameObject::Player {
            self.boards[map].player_position = position;
        }
        self.boards[map].objects.insert(position, object);
    }

    pub fn insert_entities(&mut self, position: Position, entities: [Entity; 2]) {
        self.boards[self.current].entities.insert(position, entities);
    }

    pub fn insert_floor_to_map(&mut self, position: Position, floor: Floor, map: usize) {
        self.boards[map].floors.insert(position, floor);
        match floor {
            Floor::Goal => self.boards[map].goals.push(position),
            Floor::Button(color) => self.boards[map].buttons[color].push(position),
            Floor::Warp(next_map) => {
                self.boards[map].warp_positions[next_map] = position;
            },
            _ => ()
        };
    }

    pub fn move_object(&mut self, position: Position, dir: Direction, map: usize) {
        let object = self.boards[map]
            .objects
            .remove(&position)
            .expect("Tried to move nothing");
        if object == GameObject::Player {
            self.boards[map].player_position = position.next_position(dir);
        }
        self.boards[map]
            .objects
            .insert(position.next_position(dir), object);

        self.boards[map]
            .entities
            .remove(&position)
            .and_then(|entity| {
                self.boards[map]
                    .entities
                    .insert(position.next_position(dir), entity)
            });
    }

    pub fn delete_object(&mut self, position: Position) {
        self.delete_object_n(position, self.current);
    }

    pub fn delete_object_n(&mut self, position: Position, map: usize) {
        self.boards[map]
            .objects
            .remove(&position)
            .expect("Could not remove object");
        self.boards[map]
            .entities
            .remove(&position);
    }

    pub fn get_warp_position(&self, from: usize, to: usize) -> Position {
        self.boards[from].warp_positions[to]
    }

    pub fn get_next_position_for_move(&self, position: Position, direction: Direction, map: usize) -> (Position, usize) {
        let mut next_position = position.next_position(direction);
        if let Floor::Warp(next_map) = self.get_floor_type(next_position) {
            next_position = self.get_warp_position(next_map, map);
            return (next_position, next_map);
        }
        (next_position, map)
    }

    pub fn clear(&mut self) {
        for map in 0..MAX_MAPS {
            self.boards[map].entities.clear();
            self.boards[map].objects.clear();
            self.boards[map].floors.clear();
            self.boards[map].goals.clear();
            self.boards[map].buttons[0].clear();
            self.boards[map].buttons[1].clear();
            self.boards[map].buttons[2].clear();
        }
    }

    pub fn get_created_maps(&self) -> usize {
        for i in 1..=MAX_MAPS {
            if self.boards[i].objects.is_empty()
                && self.boards[i].floors.is_empty() {
                return i;
            }
        }
        MAX_MAPS
    }

    pub fn rise_hiding_wall(&mut self, moved_color: usize) {
        for map in 0..MAX_MAPS {
            let floors = self.boards[map].floors.clone();
            for (position, floor) in floors.iter() {
                match *floor {
                    Floor::HiddenWall { hidden_by_default, color } if color == moved_color => {
                        if self.get_object_type(*position) == GameObject::Empty && hidden_by_default {
                            self.boards[map]
                                .objects
                                .insert(*position, GameObject::HidingWall{ color: moved_color });
                        } else if self.get_object_type(*position) == (GameObject::HidingWall { color: moved_color })
                            && !hidden_by_default
                        {
                            self.boards[map].objects.remove(position);
                        }
                    }
                    _ => ()
                }
            }
        }
    }

    pub fn hide_hiding_wall(&mut self, moved_color: usize) {
        for map in 0..MAX_MAPS {
            let floors = self.boards[map].floors.clone();
            for (position, floor) in floors.iter() {
                match *floor {
                    Floor::HiddenWall { hidden_by_default, color } if color == moved_color => {
                        if self.get_object_type(*position) == GameObject::Empty && !hidden_by_default {
                            self.boards[map]
                                .objects
                                .insert(*position, GameObject::HidingWall { color: moved_color });
                        } else if self.get_object_type(*position) == (GameObject::HidingWall { color: moved_color })
                            && hidden_by_default
                        {
                            self.boards[map].objects.remove(position);
                        }
                    }
                    _ => ()
                }
            }
        }
    }
}

#[derive(Resource)]
pub struct StateStack(pub Vec<DisplayState>);

impl Default for StateStack {
    fn default() -> Self {
        Self(vec![DisplayState::MainMenu])
    }
}

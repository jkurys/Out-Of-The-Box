use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::{
    components::GameEntity,
    consts::{INITIAL_MAP, MAX_MAPS},
    game::game_objects::{Direction, Floor, GameObject, Position},
    resources::MapSize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SingleBoard {
    entities: HashMap<Position, [Entity; 2]>,
    objects: HashMap<Position, GameObject>,
    floors: HashMap<Position, Floor>,
    goals: Vec<Position>,
    buttons: Vec<Vec<Position>>,
    map_size: MapSize,
    warp_positions: [Position; MAX_MAPS],
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
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
                warp_positions: [Position { x: 0, y: 0 }; 10],
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

    pub fn get_player_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for (&pos, &obj) in self.boards[self.current].objects.iter() {
            if obj == GameObject::Player {
                positions.push(pos);
            }
        }
        positions
    }

    pub fn get_entities(&self, position: Position) -> Option<[Entity; 2]> {
        self.boards[self.current].entities.get(&position).copied()
    }

    pub fn get_object_type(&self, position: Position) -> GameObject {
        self.get_object_from_map(position, self.current)
    }

    pub fn get_object_from_map(&self, position: Position, map: usize) -> GameObject {
        *self.boards[map]
            .objects
            .get(&position)
            .unwrap_or(&GameObject::Empty)
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

    pub fn get_all_turtles(&self) -> Vec<Vec<Position>> {
        let mut turtle_vec = vec![Vec::new(), Vec::new(), Vec::new()];
        for (&pos, &obj) in self.boards[self.current].objects.iter() {
            if let GameObject::Turtle { color } = obj {
                turtle_vec[color].push(pos);
            }
        }
        turtle_vec
    }

    pub fn get_current_map(&self) -> usize {
        self.current
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
        self.boards[map].objects.remove(&position);
        self.boards[map].objects.insert(position, object);
    }

    pub fn insert_entities(&mut self, position: Position, entities: [Entity; 2]) {
        self.boards[self.current]
            .entities
            .insert(position, entities);
    }

    pub fn insert_floor(&mut self, position: Position, floor: Floor) {
        self.insert_floor_to_map(position, floor, self.current);
    }

    pub fn insert_floor_to_map(&mut self, position: Position, floor: Floor, map: usize) {
        self.boards[map].floors.remove(&position);
        self.boards[map].floors.insert(position, floor);
        match floor {
            Floor::Goal => self.boards[map].goals.push(position),
            Floor::Button(color) => self.boards[map].buttons[color].push(position),
            Floor::Warp(next_map) => {
                self.boards[map].warp_positions[next_map] = position;
            }
            _ => (),
        };
    }

    pub fn move_object(&mut self, position: Position, dir: Direction, map: usize) {
        let mut object_opt = self.boards[map].objects.remove(&position);
        let mut object = GameObject::Empty;
        if !object_opt.is_none() {
            object = object_opt.unwrap();
        }
        while !object_opt.is_none() {
            object_opt = self.boards[map].objects.remove(&position);
            if !object_opt.is_none() {
                object = object_opt.unwrap();
            }
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
            .remove(&position);
        self.boards[map].entities.remove(&position);
    }

    pub fn get_warp_position(&self, from: usize, to: usize) -> Position {
        self.boards[from].warp_positions[to]
    }

    pub fn get_next_position_for_move(
        &self,
        position: Position,
        direction: Direction,
        map: usize,
    ) -> (Position, usize) {
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

    pub fn rise_hiding_wall(&mut self, moved_color: usize) {
        for map in 0..MAX_MAPS {
            let floors = self.boards[map].floors.clone();
            for (position, floor) in floors.iter() {
                match *floor {
                    Floor::HiddenWall {
                        hidden_by_default,
                        color,
                    } if color == moved_color => {
                        if self.get_object_type(*position) == GameObject::Empty && hidden_by_default
                        {
                            self.boards[map]
                                .objects
                                .insert(*position, GameObject::HidingWall { color: moved_color });
                        } else if self.get_object_type(*position)
                            == (GameObject::HidingWall { color: moved_color })
                            && !hidden_by_default
                        {
                            self.boards[map].objects.remove(position);
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn hide_hiding_wall(&mut self, moved_color: usize) {
        for map in 0..MAX_MAPS {
            let floors = self.boards[map].floors.clone();
            for (position, floor) in floors.iter() {
                match *floor {
                    Floor::HiddenWall {
                        hidden_by_default,
                        color,
                    } if color == moved_color => {
                        if self.get_object_type(*position) == GameObject::Empty
                            && !hidden_by_default
                        {
                            self.boards[map]
                                .objects
                                .insert(*position, GameObject::HidingWall { color: moved_color });
                        } else if self.get_object_type(*position)
                            == (GameObject::HidingWall { color: moved_color })
                            && hidden_by_default
                        {
                            self.boards[map].objects.remove(position);
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

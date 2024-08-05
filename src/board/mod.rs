#[cfg(test)]
mod tests;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use serde::{Deserialize, Serialize};

use crate::{
    components::GameEntity,
    consts::{INITIAL_MAP, MAX_MAPS, EAT_COUNTER},
    game::game_objects::{Block, Direction, Floor, GameObject, Position},
    menu::level_editor::resources::BoardSize,
    utils::offset_coordinate,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SingleBoard {
    entities: HashMap<Position, [Vec<Entity>; 2]>,
    objects: HashMap<Position, GameObject>,
    floors: HashMap<Position, Floor>,
    goals: Vec<Position>,
    map_size: BoardSize,
    blocks: Vec<Block>,
    eaten_boxes: HashMap<Position, (GameObject, usize, Direction)>,
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
                map_size: BoardSize {
                    width: 0,
                    height: 0,
                },
                blocks: Vec::new(),
                eaten_boxes: HashMap::new(),
            });
        }
        Board {
            current: INITIAL_MAP,
            boards,
        }
    }

    pub fn get_block(&self, position: Position) -> Block {
        for block in self.boards[self.current].blocks.iter() {
            if block.contains_position(position) {
                return block.clone();
            }
        }
        Block {
            positions: HashSet::from([position]),
        }
    }
    pub fn is_block_empty(&self, block: &Block) -> bool {
        for &position in block.positions.iter() {
            if self.get_object_type(position) != GameObject::Empty {
                return false;
            }
        }
        true
    }

    pub fn insert_block(&mut self, block: Block) {
        if block.positions.len() > 1 {
            self.boards[self.current].blocks.push(block);
        }
    }

    pub fn delete_block(&mut self, block: &Block) {
        self.boards[self.current].blocks = self.boards[self.current]
            .blocks
            .clone()
            .into_iter()
            .filter(|b| b != block)
            .filter(|b| !self.is_block_empty(b))
            .collect();
    }

    pub fn remove_from_block(&mut self, block: &mut Block, position: Position) {
        self.delete_block(&block);
        block.positions.remove(&position);
        self.insert_block(block.clone());
    }

    pub fn add_to_block(&mut self, block: &mut Block, position: Position) {
        self.delete_block(&block);
        block.positions.insert(position);
        self.insert_block(block.clone());
    }

    pub fn modify_position_in_block(&mut self, position: Position, dir: Direction) {
        let mut block = self.get_block(position);
        self.delete_block(&block);
        block.positions.remove(&position);
        block.positions.insert(position.next_position(dir));
        self.insert_block(block);
    }

    pub fn clear_entities(&mut self) {
        self.boards[self.current].entities.clear();
    }

    pub fn set_current_map(&mut self, current: usize) {
        self.current = current;
    }

    pub fn set_map_size(&mut self, map_size: BoardSize) {
        self.boards[self.current].map_size = map_size;
    }

    pub fn get_map_size(&self) -> BoardSize {
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

    pub fn get_entities(&self, position: Position) -> Option<[Vec<Entity>; 2]> {
        self.boards[self.current].entities.get(&position).cloned()
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
            for (&pos, &floor) in self.boards[map].floors.iter() {
                if floor == Floor::Goal {
                    goals_vec.push(pos);
                }
            }
        }
        goals_vec
    }

    pub fn get_all_buttons(&self) -> Vec<Vec<Position>> {
        let mut buttons = vec![Vec::new(), Vec::new(), Vec::new()];
        for (&pos, &floor) in self.boards[self.current].floors.iter() {
            if let Floor::Button(color) = floor {
                buttons[color].push(pos);
            }
        }
        buttons
    }

    pub fn get_all_turtles(&self) -> Vec<Vec<(Position, Direction)>> {
        let mut turtle_vec = vec![Vec::new(), Vec::new(), Vec::new()];
        for (&pos, &obj) in self.boards[self.current].objects.iter() {
            if let GameObject::Turtle { color, direction } = obj {
                turtle_vec[color].push((pos, direction));
            }
        }
        turtle_vec
    }

    pub fn get_all_turtle_heads(&self) -> Vec<Vec<(Position, Direction)>> {
        let mut all_heads_vec = vec![Vec::new(), Vec::new(), Vec::new()];
        for (&pos, &obj) in self.boards[self.current].objects.iter() {
            if let GameObject::TurtleHead {
                direction: dir,
                color,
            } = obj
            {
                all_heads_vec[color].push((pos, dir));
            }
        }
        all_heads_vec
    }

    pub fn get_current_map(&self) -> usize {
        self.current
    }

    fn is_position_on_board(&self, position: Position) -> bool {
        let map_size = self.boards[self.current].map_size;
        let bottom_border = offset_coordinate(0, map_size.height as i32);
        let top_border = offset_coordinate(map_size.height as i32 - 1, map_size.height as i32);
        let left_border = offset_coordinate(0, map_size.width as i32);
        let right_border = offset_coordinate(map_size.width as i32 - 1, map_size.width as i32);
        !(position.x < left_border
            || position.x > right_border
            || position.y < bottom_border
            || position.y > top_border)
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
        if !self.is_position_on_board(position) {
            return;
        }
        self.boards[map].objects.remove(&position);
        self.boards[map].objects.insert(position, object);
    }

    pub fn insert_object_to_map_unchecked(
        &mut self,
        position: Position,
        object: GameObject,
        map: usize,
    ) {
        self.boards[map].objects.remove(&position);
        self.boards[map].objects.insert(position, object);
    }

    pub fn insert_entities(&mut self, position: Position, entities: [Vec<Entity>; 2]) {
        self.boards[self.current]
            .entities
            .insert(position, entities);
    }

    pub fn append_entities(&mut self, position: Position, mut entities: [Vec<Entity>; 2]) {
        let empty_entities = &mut [Vec::new(), Vec::new()];
        let old_entities = self.boards[self.current]
            .entities
            .get_mut(&position)
            .unwrap_or(empty_entities);

        old_entities[0].append(&mut entities[0]);
        old_entities[1].append(&mut entities[1]);
    }

    pub fn insert_floor(&mut self, position: Position, floor: Floor) {
        self.insert_floor_to_map(position, floor, self.current);
    }

    pub fn insert_floor_to_map(&mut self, position: Position, floor: Floor, map: usize) {
        if !self.is_position_on_board(position) {
            return;
        }
        self.boards[map].floors.remove(&position);
        self.boards[map].floors.insert(position, floor);
        match floor {
            Floor::Goal => self.boards[map].goals.push(position),
            _ => (),
        };
    }

    pub fn insert_eat(
        &mut self,
        position: Position,
        dir: Direction,
        object: GameObject,
    ) {
        let map = self.current;
        self.boards[map].eaten_boxes.insert(position, (object, EAT_COUNTER, dir));
    }

    pub fn remove_eat(
        &mut self,
        position: Position,
    ) {
        let map = self.current;
        self.boards[map].eaten_boxes.remove(&position);
    }

    pub fn get_eat_data(
        &self,
        position: Position
    ) -> (GameObject, Direction) {
        let map = self.current;
        let (obj, _, dir) = self.boards[map].eaten_boxes.get(&position).unwrap();
        (*obj, *dir)
    }

    pub fn get_eat_counter(
        &self,
        position: Position,
    ) -> Option<usize> {
        let map = self.current;
        let opt = self.boards[map].eaten_boxes.get(&position);
        if let Some((_, counter, _)) = opt {
            return Some(*counter);
        }
        return None;
    }

    pub fn move_object_no_countdown(&mut self, position: Position, dir: Direction, map: usize) {
        self.modify_position_in_block(position, dir);
        if self.get_object_type(position) == GameObject::Empty {
            return;
        }
        let mut object_opt = self.boards[map].objects.remove(&position);
        let mut object = GameObject::Empty;
        if let Some(obj) = object_opt {
            object = obj;
        }
        while object_opt.is_some() {
            object_opt = self.boards[map].objects.remove(&position);
            if let Some(obj) = object_opt {
                object = obj;
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
        let eaten_opt = self.boards[map].eaten_boxes.remove(&position);
        if let Some(data) = eaten_opt {
            let (obj, counter, dir2) = data;
            let new_counter = counter;
            self.boards[map].eaten_boxes.insert(position.next_position(dir), (obj, new_counter, dir2));
        }
    }

    pub fn move_object(&mut self, position: Position, dir: Direction, map: usize) {
        
        self.move_object_no_countdown(position, dir, map);
        let eaten_opt = self.boards[map].eaten_boxes.remove(&position.next_position(dir));
        if let Some(data) = eaten_opt {
            let (obj, counter, dir2) = data;
            let mut new_counter = counter;
            if counter != 0 {
                new_counter = counter - 1;
            }
            self.boards[map].eaten_boxes.insert(position.next_position(dir), (obj, new_counter, dir2));
        }

    }

    pub fn delete_object(&mut self, position: Position) {
        self.delete_object_n(position, self.current);
    }

    pub fn delete_object_n(&mut self, position: Position, map: usize) {
        self.boards[map].objects.remove(&position);
        self.boards[map].entities.remove(&position);
    }

    pub fn delete_floor(&mut self, position: Position) {
        self.delete_floor_n(position, self.current);
    }

    pub fn delete_floor_n(&mut self, position: Position, map: usize) {
        self.boards[map].floors.remove(&position);
    }

    pub fn get_next_position_for_move(
        &self,
        position: Position,
        direction: Direction,
        map: usize,
    ) -> (Position, usize) {
        let next_position = position.next_position(direction);
        (next_position, map)
    }

    pub fn clear(&mut self) {
        for map in 0..MAX_MAPS {
            self.boards[map].entities.clear();
            self.boards[map].objects.clear();
            self.boards[map].floors.clear();
            self.boards[map].goals.clear();
            self.boards[map].blocks.clear();
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

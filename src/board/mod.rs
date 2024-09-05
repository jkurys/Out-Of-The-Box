#[cfg(test)]
mod tests;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use serde::{Deserialize, Serialize};

use crate::game::{display::background::calculate_borders, game_objects::HiddenWall};
use crate::{
    components::GameEntity,
    consts::EAT_COUNTER,
    game::game_objects::{Block, Direction, Floor, GameObject, Position},
    menu::level_editor::resources::BoardSize,
    utils::offset_coordinate,
};

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct Board {
    entities: HashMap<Position, [Vec<Entity>; 2]>,
    objects: HashMap<Position, GameObject>,
    floors: HashMap<Position, Floor>,
    goals: Vec<Position>,
    map_size: BoardSize,
    blocks: Vec<Block>,
    eaten_boxes: HashMap<Position, (GameObject, usize, Direction)>,
}

impl Board {
    pub fn new() -> Self {
        Board {
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
        }
    }

    pub fn init_objs(&mut self) {
        let map_size = self.map_size;
        let bottom_border = offset_coordinate(0, map_size.height as i32);
        let top_border = offset_coordinate(map_size.height as i32 - 1, map_size.height as i32);
        let left_border = offset_coordinate(0, map_size.width as i32);
        let right_border = offset_coordinate(map_size.width as i32 - 1, map_size.width as i32);
        for x in left_border..=right_border {
            for y in bottom_border..=top_border {
                self.objects.insert(Position {x, y, z: 0}, GameObject::Wall);
                self.floors.insert(Position {x, y, z: 0}, Floor::Tile);
            }
        }
    }

    pub fn get_column(&self, x: i32, y: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for (&Position { x: x2, y: y2, z }, _) in self.objects.iter() {
            if x2 == x && y2 == y {
                res.push(z);
            }
        }
        let mut was_checked = false;
        for (&Position { x: x2, y: y2, z }, f) in self.floors.iter() {
            if x2 == x && y2 == y && f != &Floor::Void {
                res.push(z);
                was_checked = true;
            }
        }
        if !was_checked {
            res.push(0);
        }
        return res;
    }

    pub fn get_block(&self, position: Position) -> Block {
        for block in self.blocks.iter() {
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
            self.blocks.push(block);
        }
    }

    pub fn delete_block(&mut self, block: &Block) {
        self.blocks = self
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

    pub fn fall_block(&mut self, block: Block) -> Block {
        self.delete_block(&block);
        let mut any_pos = None;
        for position in block.positions {
            any_pos = Some(position.position_below());
            let obj = self.get_object_type(position);
            self.delete_object(position);
            self.insert_floor(position.position_below(), Floor::Obj(obj));
            self.insert_object(position.position_below(), obj);
        }
        self.get_block(any_pos.unwrap())
    }

    pub fn get_empty_below(&self) -> Vec<Position> {
        let mut res = Vec::new();
        for (position, obj) in self.objects.iter() {
            if self.get_object_type(position.position_below()) == GameObject::Empty
                && *obj != GameObject::Wall
                && !matches!(*obj, GameObject::HidingWall { hidden_toggle: false, .. }) {
                res.push(position.position_below());
            }
        }
        res
    }

    pub fn clear_entities(&mut self) {
        self.entities.clear();
    }

    pub fn set_map_size(&mut self, map_size: BoardSize) {
        self.map_size = map_size;
    }

    pub fn get_map_size(&self) -> BoardSize {
        self.map_size
    }

    pub fn get_player_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for (&pos, &obj) in self.objects.iter() {
            if obj == GameObject::Player {
                positions.push(pos);
            }
        }
        positions
    }

    pub fn get_all_positions(&self, floor_to_be_found: Floor) -> Vec<Position> {
        let mut positions = Vec::new();
        for (&pos, &floor) in self.floors.iter() {
            if floor_to_be_found == floor {
                positions.push(pos);
            }
        }
        positions
    }

    pub fn get_entities(&self, position: Position) -> Option<[Vec<Entity>; 2]> {
        self.entities.get(&position).cloned()
    }

    pub fn get_object_type(&self, position: Position) -> GameObject {
        *self
            .objects
            .get(&position)
            .unwrap_or(&GameObject::Empty)
    }

    pub fn get_floor_type(&self, position: Position) -> Floor {
        *self
            .floors
            .get(&position)
            .unwrap_or(&Floor::Tile)
    }

    pub fn get_all_goals(&self) -> Vec<Position> {
        let mut goals_vec = Vec::new();
        for (&pos, &floor) in self.floors.iter() {
            if floor == Floor::Goal {
                goals_vec.push(pos);
            }
        }
        goals_vec
    }

    pub fn get_all_buttons(&self) -> Vec<Vec<Position>> {
        let mut buttons = vec![Vec::new(), Vec::new(), Vec::new()];
        for (&pos, &floor) in self.floors.iter() {
            if let Floor::Button(color) = floor {
                buttons[color].push(pos);
            }
        }
        buttons
    }

    pub fn get_all_turtles(&self) -> Vec<Vec<(Position, Direction)>> {
        let mut turtle_vec = vec![Vec::new(), Vec::new(), Vec::new()];
        for (&pos, &obj) in self.objects.iter() {
            if let GameObject::Turtle { color, direction } = obj {
                turtle_vec[color].push((pos, direction));
            }
        }
        turtle_vec
    }

    pub fn get_all_turtle_heads(&self) -> Vec<Vec<(Position, Direction)>> {
        let mut all_heads_vec = vec![Vec::new(), Vec::new(), Vec::new()];
        for (&pos, &obj) in self.objects.iter() {
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

    fn is_position_on_board(&self, position: Position) -> bool {
        let map_size = self.map_size;
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
            GameEntity::Floor(f) => self.insert_floor(position, f),
            GameEntity::Object(o) => self.insert_object(position, o),
        };
    }

    pub fn insert_object(&mut self, position: Position, object: GameObject) {
        if !self.is_position_on_board(position) {
            return;
        }
        if let GameObject::HidingWall { hidden_by_def: false, .. } = object {
            self.objects.remove(&position.position_below());
        }
        self.objects.remove(&position);
        self.objects.insert(position, object);

    }

    pub fn insert_object_unchecked(
        &mut self,
        position: Position,
        object: GameObject,
    ) {
        self.objects.remove(&position);
        self.objects.insert(position, object);
    }

    pub fn insert_entities(&mut self, position: Position, entities: [Vec<Entity>; 2]) {
        self
            .entities
            .insert(position, entities);
    }

    pub fn append_entities(&mut self, position: Position, mut entities: [Vec<Entity>; 2]) {
        let empty_entities = &mut [Vec::new(), Vec::new()];
        let old_entities = self
            .entities
            .get_mut(&position)
            .unwrap_or(empty_entities);

        old_entities[0].append(&mut entities[0]);
        old_entities[1].append(&mut entities[1]);
    }

    pub fn insert_floor(&mut self, position: Position, floor: Floor) {
        if !self.is_position_on_board(position) {
            return;
        }
        self.floors.remove(&position);
        self.floors.insert(position, floor);
        match floor {
            Floor::Goal => self.goals.push(position),
            Floor::Void => {
                self.objects.remove(&position);
                ()
            },
            _ => (),
        };
    }

    pub fn insert_eat(
        &mut self,
        position: Position,
        dir: Direction,
        object: GameObject,
    ) {
        self.eaten_boxes.insert(position, (object, EAT_COUNTER, dir));
    }

    pub fn remove_eat(
        &mut self,
        position: Position,
    ) {
        self.eaten_boxes.remove(&position);
    }

    pub fn get_eat_data(
        &self,
        position: Position
    ) -> (GameObject, Direction) {
        let (obj, _, dir) = self.eaten_boxes.get(&position).unwrap();
        (*obj, *dir)
    }

    pub fn get_eat_counter(
        &self,
        position: Position,
    ) -> Option<usize> {
        let opt = self.eaten_boxes.get(&position);
        if let Some((_, counter, _)) = opt {
            return Some(*counter);
        }
        return None;
    }

    pub fn move_object_no_countdown(&mut self, position: Position, dir: Direction) {
        self.modify_position_in_block(position, dir);
        if self.get_object_type(position) == GameObject::Empty {
            return;
        }
        let mut object_opt = self.objects.remove(&position);
        let mut object = GameObject::Empty;
        if let Some(obj) = object_opt {
            object = obj;
        }
        while object_opt.is_some() {
            object_opt = self.objects.remove(&position);
            if let Some(obj) = object_opt {
                object = obj;
            }
        }

        self
            .objects
            .insert(position.next_position(dir), object);

        self
            .entities
            .remove(&position)
            .and_then(|entity| {
                self
                    .entities
                    .insert(position.next_position(dir), entity)
            });
        let eaten_opt = self.eaten_boxes.remove(&position);
        if let Some(data) = eaten_opt {
            let (obj, counter, dir2) = data;
            let new_counter = counter;
            self.eaten_boxes.insert(position.next_position(dir), (obj, new_counter, dir2));
        }
    }

    pub fn move_object(&mut self, position: Position, dir: Direction) {
        
        self.move_object_no_countdown(position, dir);
        let eaten_opt = self.eaten_boxes.remove(&position.next_position(dir));
        if let Some(data) = eaten_opt {
            let (obj, counter, dir2) = data;
            let mut new_counter = counter;
            if counter != 0 {
                new_counter = counter - 1;
            }
            self.eaten_boxes.insert(position.next_position(dir), (obj, new_counter, dir2));
        }

    }

    pub fn delete_object(&mut self, position: Position) {
        self.objects.remove(&position);
        self.entities.remove(&position);
    }

    pub fn delete_floor(&mut self, position: Position) {
        self.floors.remove(&position);
    }

    pub fn get_next_position_for_move(
        &self,
        position: Position,
        direction: Direction,
    ) -> Position {
        let next_position = position.next_position(direction);
        next_position
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.objects.clear();
        self.floors.clear();
        self.goals.clear();
        self.blocks.clear();
    }
    
    pub fn get_hidden_walls_to_move(&self, moved_color: usize, clicked: bool) -> Vec<(Direction, Position)> {
        let mut res = Vec::new();
        for (&position, &obj) in self.objects.iter() {
            if let GameObject::HidingWall { color, hidden_toggle: current_hid, hidden_by_def } = obj {
                if color != moved_color
                || (clicked && hidden_by_def != current_hid)
                || (!clicked && hidden_by_def == current_hid) {
                    continue;
                }
                if hidden_by_def ^ clicked == false {
                    res.push((Direction::Up, position));
                } else {
                    res.push((Direction::Down, position));
                }
            }
        }
        res
    }

    pub fn modify_toggle(&mut self, position: Position) {
        let obj = self.get_object_type(position);
        match obj {
            GameObject::HidingWall { color, hidden_toggle: h, hidden_by_def } => {
                self.delete_object(position);
                self.insert_object(position, GameObject::HidingWall { color, hidden_toggle: !h, hidden_by_def });
            },
            _ => ()
        };
    }

    // pub fn rise_hiding_wall(&mut self, moved_color: usize) {
    //     let floors = self.floors.clone();
    //     for (position, floor) in floors.iter() {
    //         match *floor {
    //             Floor::HiddenWall {
    //                 hidden_by_default,
    //                 color,
    //             } if color == moved_color => {
    //                 if self.get_object_type(position.position_above()) == GameObject::Empty && hidden_by_default
    //                 {
    //                     self
    //                         .objects
    //                         .insert(position.position_above(), GameObject::HidingWall { color: moved_color });
    //                 } else if self.get_object_type(*position)
    //                     == (GameObject::HidingWall { color: moved_color })
    //                     && !hidden_by_default
    //                 {
    //                     self.objects.remove(position);
    //                     self.floors.insert(position.position_below(), Floor::HiddenWall { hidden_by_default: false, color: moved_color });
    //                 }
    //             }
    //             _ => (),
    //         }
    //     }
    // }
    //
    // pub fn hide_hiding_wall(&mut self, moved_color: usize) {
    //     let floors = self.floors.clone();
    //     for (position, floor) in floors.iter() {
    //         match *floor {
    //             Floor::HiddenWall {
    //                 hidden_by_default,
    //                 color,
    //             } if color == moved_color => {
    //                 let floor = Floor::HiddenWall { hidden_by_default: false, color: moved_color };
    //                 if self.get_floor_type(*position) == floor
    //                     && !hidden_by_default
    //                 {
    //                     self
    //                         .objects
    //                         .insert(position.position_above(), GameObject::HidingWall { color: moved_color });
    //                 } else if self.get_object_type(position.position_above())
    //                     == (GameObject::HidingWall { color: moved_color })
    //                     && hidden_by_default
    //                 {
    //                     self.objects.remove(&position.position_above());
    //                 }
    //             }
    //             _ => (),
    //         }
    //     }
    // }
}

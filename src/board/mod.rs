use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use serde::{Deserialize, Serialize};

use crate::{
    components::GameEntity,
    consts::*,
    game::game_objects::{Block, Direction, Floor, GameObject, Position, SmallSet},
    menu::level_editor::resources::BoardSize,
    utils::offset_coordinate,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityStorage {
    pub entities: HashMap<Position, [Vec<Entity>; 3]>,
}

impl EntityStorage {
    pub fn new() -> Self {
        EntityStorage {
            entities: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }

    pub fn insert_entities(&mut self, position: Position, entities: [Vec<Entity>; 3]) {
        self.entities.insert(position, entities);
    }

    pub fn append_entities(&mut self, position: Position, mut entities: [Vec<Entity>; 3]) {
        let empty_entities = &mut [Vec::new(), Vec::new(), Vec::new()];
        let mut entities_clone = self.entities.clone();
        let old_entities = entities_clone.get_mut(&position).unwrap_or(empty_entities);

        old_entities[0].append(&mut entities[0]);
        old_entities[1].append(&mut entities[1]);
        old_entities[2].append(&mut entities[2]);
        self.entities.insert(position, old_entities.clone());
    }

    pub fn move_entity(&mut self, position: Position, dir: Direction) {
        self.entities
            .remove(&position)
            .and_then(|entity| self.entities.insert(position.next_position(dir), entity));
    }

    pub fn remove(&mut self, position: &Position) {
        self.entities.remove(position);
    }

    pub fn get(&self, position: Position) -> Option<[Vec<Entity>; 3]> {
        self.entities.get(&position).cloned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Board {
    entities: EntityStorage,
    objects: HashMap<Position, GameObject>,
    floors: HashMap<Position, Floor>,
    goals: Vec<Position>,
    map_size: BoardSize,
    blocks: HashSet<Block>,
    eaten_boxes: HashMap<Position, (GameObject, Option<Floor>, usize, Direction)>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            entities: EntityStorage::new(),
            objects: HashMap::new(),
            floors: HashMap::new(),
            goals: Vec::new(),
            map_size: BoardSize {
                width: 0,
                height: 0,
            },
            blocks: HashSet::new(),
            eaten_boxes: HashMap::new(),
        }
    }

    // used in level editor
    pub fn init_objs(&mut self) {
        let map_size = self.map_size;
        let bottom_border = offset_coordinate(0, map_size.height as i32);
        let top_border = offset_coordinate(map_size.height as i32 - 1, map_size.height as i32);
        let left_border = offset_coordinate(0, map_size.width as i32);
        let right_border = offset_coordinate(map_size.width as i32 - 1, map_size.width as i32);
        for x in left_border..=right_border {
            for y in bottom_border..=top_border {
                let pos = Position { x, y, z: 0 };
                if self.objects.get(&pos).is_none() {
                    self.objects.insert(pos, GameObject::Wall);
                }
                if self.floors.get(&pos).is_none() {
                    self.floors.insert(pos, Floor::Tile);
                }
            }
        }
    }

    // Gets all non-empty z-coordinates at the given (x, y)
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
            positions: SmallSet::from([position]),
        }
    }

    pub fn insert_block(&mut self, block: Block) {
        for pos in block.positions.iter() {
            if self.get_object_type(pos) == GameObject::Empty {
                error!("Block {:?} contains empty position {:?}", block, pos);
            }
            if self.get_block(pos).positions.len() > 1 {
                error!(
                    "Block {:?} overlaps with existing block at position {:?}",
                    block, pos
                );
            }
        }
        if block.positions.len() > 1 {
            self.blocks.insert(block);
        }
    }

    pub fn delete_block(&mut self, block: &Block) {
        if !self.blocks.contains(block) {
            error!("Tried to delete non-existing block {:?}", block);
        }
        self.blocks.remove(block);
    }

    /*
     * We assume that move_block is called after all objects in the block have been moved
     */
    pub fn move_block(&mut self, block: &Block, dir: Direction) {
        if !self.blocks.contains(block) && block.positions.len() > 1 {
            error!("Tried to move non-existing block {:?}", block);
        }
        self.blocks.remove(block);
        let mut new_block = Block {
            positions: SmallSet::new(),
        };
        for position in block.positions.iter() {
            new_block.positions.insert(position.next_position(dir));
        }
        self.insert_block(new_block);
    }

    pub fn get_positions_to_fall(&self) -> Vec<Position> {
        let mut res = Vec::new();
        for (position, obj) in self.objects.iter() {
            if self.get_object_type(position.position_below()) == GameObject::Empty
                && *obj != GameObject::Wall
                && !matches!(
                    *obj,
                    GameObject::HidingWall {
                        hidden_toggle: false,
                        ..
                    }
                )
            {
                res.push(position.position_below());
            }
        }
        res
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
            if matches!(obj, GameObject::Player { direction: _ }) {
                positions.push(pos);
            }
        }
        positions
    }

    pub fn get_object_type(&self, position: Position) -> GameObject {
        *self.objects.get(&position).unwrap_or(&GameObject::Empty)
    }

    pub fn get_objects(&self) -> HashMap<Position, GameObject> {
        self.objects.clone()
    }

    pub fn get_floor_type(&self, position: Position) -> Floor {
        *self.floors.get(&position).unwrap_or(&Floor::Tile)
    }

    pub fn get_floors(&self) -> HashMap<Position, Floor> {
        self.floors.clone()
    }

    pub fn insert(&mut self, position: Position, floor_or_object: GameEntity) {
        match floor_or_object {
            GameEntity::Floor(f) => self.insert_floor(position, f),
            GameEntity::Object(o) => self.insert_object(position, o),
        };
    }

    pub fn insert_object(&mut self, position: Position, object: GameObject) {
        if self.get_object_type(position) != GameObject::Empty {
            error!(
                "Tried to insert object {:?} at non-empty {:?} when there was {:?}",
                object,
                position,
                self.get_object_type(position)
            );
        }
        if let GameObject::HidingWall {
            hidden_by_def: false,
            ..
        } = object
        {
            self.objects.remove(&position.position_below());
        }
        self.objects.remove(&position);
        self.objects.insert(position, object);
    }

    pub fn insert_floor(&mut self, position: Position, floor: Floor) {
        if self.get_floor_type(position) != Floor::Tile {
            error!("Tried to insert floor at non-tile position {:?}", position);
        }
        self.floors.remove(&position);
        self.floors.insert(position, floor);
        match floor {
            Floor::Goal => self.goals.push(position),
            Floor::Void => {
                self.objects.remove(&position);
                ()
            }
            _ => (),
        };
    }

    pub fn insert_eat(
        &mut self,
        position: Position,
        dir: Direction,
        object: GameObject,
        floor: Option<Floor>,
    ) {
        self.eaten_boxes
            .insert(position, (object, floor, EAT_COUNTER, dir));
    }

    pub fn remove_eat(&mut self, position: Position) {
        self.eaten_boxes.remove(&position);
    }

    pub fn get_eat_data(&self, position: Position) -> (GameObject, Option<Floor>, Direction) {
        let (obj, floor, _, dir) = self.eaten_boxes.get(&position).unwrap();
        (*obj, *floor, *dir)
    }

    pub fn get_all_eat(&self) -> HashMap<Position, (GameObject, Option<Floor>, usize, Direction)> {
        self.eaten_boxes.clone()
    }

    pub fn get_eat_counter(&self, position: Position) -> Option<usize> {
        let opt = self.eaten_boxes.get(&position);
        if let Some((_, _, counter, _)) = opt {
            return Some(*counter);
        }
        return None;
    }

    pub fn move_object_no_countdown(&mut self, position: Position, dir: Direction) {
        if self.get_object_type(position) == GameObject::Empty {
            error!("Tried to move an empty object at position {:?}", position);
        }
        if self.get_object_type(position.next_position(dir)) != GameObject::Empty {
            error!(
                "Tried to move object at position {:?} to non-empty position {:?}",
                position,
                position.next_position(dir)
            );
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
        if let GameObject::Player { direction: _ } = object {
            let eaten_opt = self.eaten_boxes.get(&position);
            if dir != Direction::Up && dir != Direction::Down {
                if eaten_opt.is_none() || eaten_opt.unwrap().2 == EAT_COUNTER {
                    object = GameObject::Player { direction: dir };
                }
            }
        }
        let floor_opt = self.floors.remove(&position);
        if let Some(floor) = floor_opt {
            if floor != Floor::Void {
                self.floors.insert(position.next_position(dir), floor);
            } else {
                self.floors.insert(position, floor);
            }
        }

        self.objects.insert(position.next_position(dir), object);
        self.entities.move_entity(position, dir);

        let eaten_opt = self.eaten_boxes.remove(&position);
        if let Some(data) = eaten_opt {
            let (obj, floor, counter, dir2) = data;
            let new_counter = counter;
            self.eaten_boxes
                .insert(position.next_position(dir), (obj, floor, new_counter, dir2));
        }
    }

    pub fn move_object(&mut self, position: Position, dir: Direction) {
        if dir == Direction::Down && position.z == 0 {
            return;
        }
        self.move_object_no_countdown(position, dir);
        let eaten_opt = self.eaten_boxes.remove(&position.next_position(dir));
        if let Some(data) = eaten_opt {
            let (obj, floor, counter, dir2) = data;

            let mut new_counter = counter;
            if counter != 0 {
                new_counter = counter - 1;
            }
            self.eaten_boxes
                .insert(position.next_position(dir), (obj, floor, new_counter, dir2));
        }
    }

    pub fn delete_object(&mut self, position: Position, entities: &mut EntityStorage) {
        self.objects.remove(&position);
        entities.remove(&position);
    }

    pub fn delete_floor(&mut self, position: Position) {
        self.floors.remove(&position);
    }

    pub fn get_next_position_for_move(&self, position: Position, direction: Direction) -> Position {
        let next_position = position.next_position(direction);
        next_position
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.floors.clear();
        self.goals.clear();
        self.blocks.clear();
    }

    pub fn get_hidden_walls_to_move(
        &self,
        moved_color: usize,
        clicked: bool,
    ) -> Vec<(Direction, Position)> {
        let mut res = Vec::new();
        for (&position, &obj) in self.objects.iter() {
            if let GameObject::HidingWall {
                color,
                hidden_toggle: current_hid,
                hidden_by_def,
            } = obj
            {
                if color != moved_color
                    || (clicked && hidden_by_def != current_hid)
                    || (!clicked && hidden_by_def == current_hid)
                {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Resource)]
pub struct GameData {
    pub board: Board,
    pub entity_storage: EntityStorage,
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            board: Board::new(),
            entity_storage: EntityStorage::new(),
        }
    }

    pub fn modify_toggle(&mut self, position: Position) {
        let obj = self.board.get_object_type(position);
        match obj {
            GameObject::HidingWall {
                color,
                hidden_toggle: h,
                hidden_by_def,
            } => {
                self.board.delete_object(position, &mut self.entity_storage);
                self.board.insert_object(
                    position,
                    GameObject::HidingWall {
                        color,
                        hidden_toggle: !h,
                        hidden_by_def,
                    },
                );
            }
            _ => (),
        };
    }
}

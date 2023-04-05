use bevy::prelude::*; 
use crate::resources::Board;

// #[derive(Debug, Clone)]
// struct SingleBoard {
//     entities: HashMap<Position, Entity>,
//     objects: HashMap<Position, GameObject>,
//     floors: HashMap<Position, Floor>,
//     goals: Vec<Position>,
//     map_size: MapSize,
//     player_position: Position,
//     warp_positions: [Position; MAX_MAPS],
// }

// #[derive(Resource, Debug, Clone)]
// pub struct Board {
//     boards: Vec<SingleBoard>,
//     current: usize,
// }

// impl Board {
//     pub fn new() -> Self {
//         let mut boards = Vec::new();
//         for map in 0..MAX_MAPS {
//             boards.push(SingleBoard {
//                 entities: HashMap::new(),
//                 objects: HashMap::new(),
//                 floors: HashMap::new(),
//                 goals: Vec::new(),
//                 map_size: MapSize {
//                     width: 0,
//                     height: 0,
//                 },
//                 player_position: Position { x: 0, y: 0, map },
//                 warp_positions: [Position { x: 0, y: 0, map }; 10],
//             });
//         }
//         Board {
//             current: INITIAL_MAP,
//             boards,
//         }
//     }

//     pub fn set_current_map(&mut self, current: usize) {
//         self.current = current;
//     }

//     pub fn set_map_size(&mut self, map_size: MapSize) {
//         self.boards[self.current].map_size = map_size;
//     }

//     pub fn get_map_size(&self) -> MapSize {
//         self.boards[self.current].map_size
//     }

//     pub fn get_player_position(&self) -> Position {
//         self.boards[self.current].player_position
//     }

//     pub fn get_entity(&self, position: Position) -> Option<Entity> {
//         self.boards[self.current].entities.get(&position).copied()
//     }

//     pub fn get_object_type(&self, position: Position) -> GameObject {
//         *self.boards[position.map]
//             .objects
//             .get(&position)
//             .unwrap_or(&GameObject::Empty)
//     }

//     pub fn get_floor_type(&self, position: Position) -> Floor {
//         *self.boards[position.map]
//             .floors
//             .get(&position)
//             .unwrap_or(&Floor::Tile)
//     }

//     pub fn get_all_goals(&self) -> Vec<Position> {
//         let mut goals_vec = Vec::new();
//         for map in 0..MAX_MAPS {
//             goals_vec.push(self.boards[map].goals.clone());
//         }
//         goals_vec.concat() //realistically, this vector won't exceed 20 entries so cloning isn't a problem
//     }

//     pub fn get_current_map(&self) -> usize {
//         self.current
//     }

//     pub fn insert_object(&mut self, position: Position, object: GameObject) {
//         self.boards[position.map].objects.insert(position, object);
//         if object == GameObject::Player {
//             self.boards[position.map].player_position = position;
//         }
//     }

//     pub fn insert_entity(&mut self, position: Position, entity: Entity) {
//         self.boards[position.map].entities.insert(position, entity);
//     }

//     pub fn insert_floor(&mut self, position: Position, floor: Floor) {
//         self.boards[position.map].floors.insert(position, floor);
//         if floor == Floor::Goal {
//             self.boards[position.map].goals.push(position);
//         }
//         if let Floor::Warp(map) = floor {
//             self.boards[position.map].warp_positions[map] = position;
//         }
//     }

//     pub fn move_object(&mut self, position: Position, dir: Direction) {
//         let object = self.boards[position.map]
//             .objects
//             .remove(&position)
//             .expect("Tried to move nothing");
//         if object == GameObject::Player {
//             self.boards[position.map].player_position = position.next_position(dir);
//         }
//         self.boards[position.map]
//             .objects
//             .insert(position.next_position(dir), object);

//         self.boards[position.map]
//             .entities
//             .remove(&position)
//             .and_then(|entity| {
//                 self.boards[position.map]
//                     .entities
//                     .insert(position.next_position(dir), entity)
//             });
//     }

//     pub fn delete_object(&mut self, position: Position) -> Entity {
//         self.boards[position.map]
//             .objects
//             .remove(&position)
//             .expect("Could not remove object");
//         self.boards[position.map]
//             .entities
//             .remove(&position)
//             .expect("Could not remove entity")
//     }

//     pub fn get_warp_position(&self, from: usize, to: usize) -> Position {
//         self.boards[from].warp_positions[to]
//     }

//     pub fn get_next_position_for_move(&self, position: Position, direction: Direction) -> Position {
//         let mut next_position = position.next_position(direction);
//         if let Floor::Warp(map) = self.get_floor_type(next_position) {
//             next_position = self.get_warp_position(map, self.current);
//         }
//         next_position
//     }

//     pub fn clear(&mut self) {
//         for map in 0..MAX_MAPS {
//             self.boards[map].entities.clear();
//             self.boards[map].objects.clear();
//             self.boards[map].floors.clear();
//             self.boards[map].goals.clear();
//         }
//     }

//     pub fn rise_hiding_wall(&mut self) {
//         for map in 0..MAX_MAPS {
//             let floors = self.boards[map].floors.clone();
//             for (position, floor) in floors.iter() {
//                 if let &Floor::HiddenWall { hidden_by_default } = floor {
//                     if self.get_object_type(*position) == GameObject::Empty && hidden_by_default {
//                         self.boards[map]
//                             .objects
//                             .insert(*position, GameObject::HidingWall);
//                     } else if self.get_object_type(*position) == GameObject::HidingWall
//                         && !hidden_by_default
//                     {
//                         self.boards[map].objects.remove(position);
//                     }
//                 }
//             }
//         }
//     }

//     pub fn hide_hiding_wall(&mut self) {
//         for map in 0..MAX_MAPS {
//             let floors = self.boards[map].floors.clone();
//             for (position, floor) in floors.iter() {
//                 if let &Floor::HiddenWall { hidden_by_default } = floor {
//                     if self.get_object_type(*position) == GameObject::Empty && !hidden_by_default {
//                         self.boards[map]
//                             .objects
//                             .insert(*position, GameObject::HidingWall);
//                     } else if self.get_object_type(*position) == GameObject::HidingWall
//                         && hidden_by_default
//                     {
//                         self.boards[map].objects.remove(position);
//                     }
//                 }
//             }
//         }
//     }
// }

#[derive(Resource)]
pub struct BoardStates {
    pub boards: Vec<Board>,
}

#[derive(Resource)]
pub struct VictoryTimer(pub Timer);


use std::cmp::Ordering;

use crate::game::game_objects::{Direction, Position};

type SortFn = dyn FnMut(&Position, &Position) -> Ordering;

pub fn sort_positions(dir: Direction) -> Box<SortFn> {
    Box::new(
        move |&pos1: &Position, &pos2: &Position| match dir {
            Direction::South => {
                if pos1.y != pos2.y {
                    pos1.y.cmp(&pos2.y)
                } else {
                    pos1.x.cmp(&pos2.x)
                }
            }
            Direction::Left => {
                if pos1.x != pos2.x {
                    pos1.x.cmp(&pos2.x)
                } else {
                    pos1.y.cmp(&pos2.y)
                }
            }
            Direction::Right => {
                if pos1.x != pos2.x {
                    pos2.x.cmp(&pos1.x)
                } else {
                    pos2.y.cmp(&pos1.y)
                }
            }
            Direction::North => {
                if pos1.y != pos2.y {
                    pos2.y.cmp(&pos1.y)
                } else {
                    pos2.x.cmp(&pos1.x)
                }
            }
            Direction::Up => {
                if pos1.z != pos2.z {
                    pos2.z.cmp(&pos1.z)
                } else {
                    pos2.x.cmp(&pos1.x)
                }
            }
            Direction::Down => {
                if pos1.z != pos2.z {
                    pos1.z.cmp(&pos2.z)
                } else {
                    pos2.x.cmp(&pos1.x)
                }
            }
        },
    )
}

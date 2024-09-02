use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};
// use bevy::color::palettes::css::WHITE;
use crate::game::display::background::calculate_borders;
use crate::game::display::render_2_5_d::get_offsets;

use crate::{
    board::Board,
    components::GameEntity,
    consts::*,
    game::game_objects::{Block, Direction, Floor, GameObject, Position},
    // resources::Images,
};

fn offset_coordinate_x(coord: f32, window_coord: f32, y: i32) -> i32 {
    ((coord - ((window_coord) / 2.)) / TILE_WIDTH - (y as f32 * (101./300.))).round() as i32
}

fn offset_coordinate_y(coord: f32, window_coord: f32) -> i32 {
    ((coord - ((window_coord) / 2.)) / TILE_HEIGHT).floor() as i32
}

fn vec2_to_position(vec: Vec2, window_size: Vec2) -> Position {
    let y = offset_coordinate_y(window_size.y - vec.y, window_size.y);
    Position {
        x: offset_coordinate_x(vec.x, window_size.x, y),
        y,
        z: 0,
    }
}

fn get_position_from_mouse_click(
    mouse: &Res<ButtonInput<MouseButton>>,
    windows: &Query<&Window, With<PrimaryWindow>>,
) -> (Position, Option<MouseButton>) {
    let window = windows.single();
    if let Some(position) = window.cursor_position() {
        let pos = vec2_to_position(
            position,
            Vec2 {
                x: window.width(),
                y: window.height(),
            },
        );
        if mouse.just_pressed(MouseButton::Left) {
            return (pos, Some(MouseButton::Left));
        }
        if mouse.just_pressed(MouseButton::Right) {
            return (pos, Some(MouseButton::Right));
        }
        return (pos, None);
    }
    (Position { x: 0, y: 0, z: 0 }, None)
}

fn is_inside_rect(
    mouse_pos: Vec2,
    xs: (f32, f32),
    ys: (f32, f32),
) -> bool {
    let Vec2 { x, y } = mouse_pos;
    return x > xs.0 && x < xs.1
        && y > ys.0 && y < ys.1;
}

fn mouse_over_column(
    mut zs: Vec<i32>,
    x: i32,
    y: i32,
    mouse_position: Vec2,
    window: Window,
    // meshes: &mut Res<Assets<Mesh>>,
    // materials: &mut Res<Assets<ColorMaterial>>,
    // commands: &mut Commands,
) -> Option<Position> {
    zs.sort();
    zs.reverse();
    let mouse_position = Vec2 { x: mouse_position.x - window.width() / 2., y: window.height() / 2. - mouse_position.y };
    for &z in zs.iter() {
        let (uppers, lowers, mut sides) = get_offsets(x, y, z, 0.);
        sides = (sides.0 + TILE_WIDTH, sides.1, sides.2);
        if is_inside_rect(mouse_position, (uppers.0, uppers.0 + TILE_WIDTH), (uppers.1, uppers.1 + TILE_HEIGHT + 3.)) {
            // let mesh = Mesh2dHandle(meshes.add(Rectangle::new(TILE_WIDTH, TILE_HEIGHT + 3.)));
            // commands.spawn(
            //     MaterialMesh2dBundle {
            //         mesh,
            //         material: materials.add(Color::rgb(100., 100., 100.).into()),
            //         transform: Transform::from_xyz(
            //             uppers.0,
            //             uppers.1,
            //             0.
            //         ),
            //         ..default()
            //     }
            // );
            return Some(Position { x, y, z });
        }
        if is_inside_rect(mouse_position, (lowers.0, lowers.0 + TILE_WIDTH), (lowers.1, lowers.1 + TILE_FRONT_HEIGHT)) {
            println!("FRONT");
            return Some(Position { x, y: y - 1, z });
        }
        if is_inside_rect(mouse_position, (sides.0, sides.0 + SIDE_WIDTH), (sides.1, sides.1 + SIDE_HEIGHT)) {
            println!("SIDE");
            return Some(Position { x: x + 1, y, z });
        }
    }
    None
}

pub fn get_frontmost_position(
    mouse: &Res<ButtonInput<MouseButton>>,
    windows: &Query<&Window, With<PrimaryWindow>>,
    board: &ResMut<Board>,
    // meshes: &mut Res<Assets<Mesh>>,
    // materials: &mut Res<Assets<ColorMaterial>>,
    // commands: &mut Commands,
) -> (Position, Option<MouseButton>) {
    let window = windows.single();
    if let Some(position) = window.cursor_position() {
        let (bot_border, top_border, left_border, right_border) = calculate_borders(board);
        for y in bot_border..=top_border {
            for x in (left_border..=right_border).rev() {
                if let Some(pos) = mouse_over_column(
                    board.get_column(x, y),
                    x,
                    y,
                    position,
                    window.clone(),
                    // meshes,
                    // materials,
                    // commands
                ) {
                    if mouse.just_pressed(MouseButton::Left) {
                        return (pos, Some(MouseButton::Left));
                    }
                    if mouse.just_pressed(MouseButton::Right) {
                        return (pos, Some(MouseButton::Right));
                    }
                    return (pos, None);
                }
            }
        }
    }
    (Position { x: 0, y: 0, z: 0 }, None)
}

pub fn handle_level_editor_click(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut clickable_query: Query<(&Interaction, &GameEntity, &mut BackgroundColor)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut board: ResMut<Board>,
    mut entity: Local<GameEntity>,
    input: Res<ButtonInput<KeyCode>>,
    mut block_positions: Local<Option<HashSet<Position>>>,
    // images: Res<Images>,
    // mut commands: Commands,
    // mut meshes: Res<Assets<Mesh>>,
    // mut materials: Res<Assets<ColorMaterial>>,
) {
    
    if block_positions.is_some() {
        if input.just_pressed(KeyCode::KeyC) {
            board.insert_block(Block {
                positions: block_positions.clone().unwrap(),
            });
            *block_positions = None;
            return;
        }
        if let (mut pos, Some(MouseButton::Left)) = get_position_from_mouse_click(&mouse, &windows) {
            pos = Position { x: pos.x, y: pos.y, z: pos.z + 1};
            let mut positions = block_positions.clone().unwrap();
            positions.insert(pos);
            *block_positions = Some(positions);
        }

        return;
    }

    if input.just_pressed(KeyCode::KeyC) {
        *block_positions = Some(HashSet::new());
    }
    if let (mut pos, Some(button)) = get_position_from_mouse_click(&mouse, &windows) {
        if button == MouseButton::Left {
            if let GameEntity::Object(_) = *entity {
                pos = Position { x: pos.x, y: pos.y, z: 1 };
            }
            if let GameEntity::Floor(Floor::Tile) = *entity {
                pos = Position { x: pos.x, y: pos.y, z: 1 };
            }
            board.insert(pos, *entity);
            if let GameEntity::Object(GameObject::HidingWall { color }) = *entity {
                board.insert_floor(
                    pos,
                    Floor::HiddenWall {
                        hidden_by_default: false,
                        color,
                    },
                );
            }
        } else if button == MouseButton::Right {
            board.delete_object(pos);
            board.delete_floor(pos);
        }
    }
    for (&interaction, object_or_floor, mut color) in clickable_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *entity = *object_or_floor;
            }
            Interaction::Hovered => {
                // *color = Color::srgb(0.7, 0.7, 0.7).into();
                *color = Color::rgb(0.7, 0.7, 0.7).into();
            }
            Interaction::None => {
                // *color = WHITE.into();
                *color = Color::WHITE.into();
            }
        }
    }
    if let GameEntity::Object(GameObject::Turtle {
        direction: _,
        color,
    }) = *entity
    {
        let mut maybe_dir = None;
        if input.just_pressed(KeyCode::ArrowUp) {
            maybe_dir = Some(Direction::Up);
        } else if input.just_pressed(KeyCode::ArrowDown) {
            maybe_dir = Some(Direction::Down);
        } else if input.just_pressed(KeyCode::ArrowLeft) {
            maybe_dir = Some(Direction::Left);
        } else if input.just_pressed(KeyCode::ArrowRight) {
            maybe_dir = Some(Direction::Right);
        }
        if let Some(dir) = maybe_dir {
            *entity = GameEntity::Object(GameObject::Turtle {
                direction: dir,
                color,
            });
        }
    }
}

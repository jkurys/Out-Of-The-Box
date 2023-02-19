use bevy::prelude::*;

pub fn delete_all_components<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn offset_coordinate(coord: i32, max: i32) -> i32 {
    coord - (max / 2)
}

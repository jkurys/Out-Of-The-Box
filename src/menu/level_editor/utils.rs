use bevy::prelude::*;

pub fn spawn_small_image(parent: &mut ChildBuilder, image: Handle<Image>) {
    parent.spawn(ImageBundle {
        image: UiImage{ texture: image , ..default() },
        style: Style {
            size: Size {
                height: Val::Px(50.),
                width: Val::Px(50.),
            },
            ..default()
        },
        ..default()
    });
}

pub fn spawn_small_button<T>(parent: &mut ChildBuilder, image: Handle<Image>, component: T)
where
    T: Component,
{
    parent
        .spawn(ButtonBundle::default())
        .insert(ImageBundle {
            image: UiImage{ texture: image , ..default() },
            style: Style {
                size: Size {
                    height: Val::Px(50.),
                    width: Val::Px(50.),
                },
                ..default()
            },
            ..default()
        })
        .insert(component);
}

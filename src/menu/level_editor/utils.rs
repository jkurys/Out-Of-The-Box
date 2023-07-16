use bevy::prelude::*;

pub fn spawn_small_image(parent: &mut ChildBuilder, image: Handle<Image>) {
    parent.spawn(ImageBundle {
        image: UiImage {
            texture: image,
            ..default()
        },
        style: Style {
            height: Val::Px(50.),
            width: Val::Px(50.),
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
            image: UiImage {
                texture: image,
                ..default()
            },
            style: Style {
                height: Val::Px(50.),
                width: Val::Px(50.),
                ..default()
            },
            ..default()
        })
        .insert(component);
}

pub fn spawn_small_button_with_sticker<T>(
    parent: &mut ChildBuilder,
    image: Handle<Image>,
    component: T,
    sticker_image: Handle<Image>,
) where
    T: Component,
{
    parent
        .spawn(ButtonBundle::default())
        .insert(ImageBundle {
            image: UiImage {
                texture: image,
                ..default()
            },
            style: Style {
                height: Val::Px(50.),
                width: Val::Px(50.),
                ..default()
            },
            ..default()
        })
        .insert(component)
        .with_children(|parent| {
            spawn_small_image(parent, sticker_image);
        });
}

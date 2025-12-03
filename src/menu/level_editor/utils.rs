use bevy::prelude::*;

pub fn spawn_small_image(parent: &mut ChildBuilder, image: Handle<Image>) {
    parent.spawn((
        ImageNode::new(image),
        Node {
            height: Val::Px(50.),
            width: Val::Px(50.),
            ..default()
        },
    ));
}

pub fn spawn_small_button<T>(parent: &mut ChildBuilder, image: Handle<Image>, component: T)
where
    T: Component,
{
    parent
        .spawn(Button)
        .insert((
            ImageNode::new(image),
            Node {
                height: Val::Px(50.),
                width: Val::Px(50.),
                ..default()
            },
        ))
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
        .spawn(Button)
        .insert((
            ImageNode::new(image),
            Node {
                height: Val::Px(50.),
                width: Val::Px(50.),
                ..default()
            },
        ))
        .insert(component)
        .with_children(|parent| {
            spawn_small_image(parent, sticker_image);
        });
}

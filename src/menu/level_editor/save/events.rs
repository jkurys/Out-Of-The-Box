use bevy::prelude::Event;

#[derive(Clone, PartialEq, Eq, Debug, Event)]
pub struct FileSavedEvent(pub String);

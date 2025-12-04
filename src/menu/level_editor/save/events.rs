use bevy::prelude::Message;

#[derive(Clone, PartialEq, Eq, Debug, Message)]
pub struct FileSavedMessage(pub String);

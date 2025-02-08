use bevy::prelude::*;

#[derive(Debug, Component, Deref, DerefMut, Clone)]
pub struct Team(pub String);

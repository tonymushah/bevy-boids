use bevy::prelude::*;

#[derive(Debug, Component, Deref, DerefMut, Default)]
pub struct VisionRadius(pub f32);

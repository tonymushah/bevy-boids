use bevy::prelude::*;

#[derive(Debug, Component, Deref, DerefMut, Default, Clone, Copy)]
pub struct VisionRadius(pub f32);

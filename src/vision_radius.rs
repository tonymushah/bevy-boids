use bevy::prelude::*;

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct VisionRadius {
    pub min_distance: f32,
    pub neighboor_radius: f32,
    pub cohesion_radius: (f32, f32),
}

use bevy::prelude::*;

#[derive(Default, Clone, Copy, Debug)]
pub struct CohesionRadius(pub f32, pub f32);

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct VisionRadius {
    pub min_distance: f32,
    pub neighboor_radius: f32,
    pub cohesion_radius: CohesionRadius,
}

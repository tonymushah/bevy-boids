use bevy::{color::palettes::css::GREEN, prelude::*};

use crate::{velocity::Velocity, vision_radius::VisionRadius};

use super::Bird;

#[allow(clippy::type_complexity)]
pub fn align(
    mut birds: Query<(&mut Velocity, &Transform, &VisionRadius, Entity), With<Bird>>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    let mut combinaisons = birds.iter_combinations_mut();
    while let Some([mut one, two]) = combinaisons.fetch_next() {
        let next_one = one.1.translation + **one.0 * time.delta_secs();
        let next_two = two.1.translation + **two.0 * time.delta_secs();

        let min_distance = one.2.min_distance;
        let neighbor_radius = one.2.neighboor_radius;
        let distance = next_one.distance(next_two);

        if !(min_distance < distance && distance < neighbor_radius) {
            continue;
        }

        let average_vel = (**one.0 + **two.0) / 2.0;

        gizmos.arrow(
            one.1.translation,
            one.1.translation + average_vel.normalize(),
            GREEN,
        );

        **one.0 += average_vel.normalize();
    }
}
pub struct BirdAlignmentPlugin;

impl Plugin for BirdAlignmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, align);
    }
}

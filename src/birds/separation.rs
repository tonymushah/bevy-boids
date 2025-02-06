use bevy::{color::palettes::css::RED, prelude::*};

use crate::{velocity::Velocity, vision_radius::VisionRadius};

use super::Bird;

#[allow(clippy::type_complexity)]
fn separate(
    mut birds: Query<(&mut Velocity, &Transform, &VisionRadius, Entity), With<Bird>>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    let mut combinaisons = birds.iter_combinations_mut();
    while let Some([mut one, two]) = combinaisons.fetch_next() {
        let next_one = one.1.translation + **one.0 * time.delta_secs();
        let next_two = two.1.translation + **two.0 * time.delta_secs();

        let min_distance = one.2.min_distance;
        let distance = next_one.distance(next_two);

        if distance > min_distance {
            continue;
        }

        let pos_vec = next_one - next_two;

        let force_magnitude = (min_distance - distance).powi(2);

        let separation_force = pos_vec.normalize() * force_magnitude;

        gizmos.arrow(
            one.1.translation,
            one.1.translation + separation_force.normalize(),
            RED,
        );

        **one.0 += separation_force;
    }
}

pub struct BirdSeparationPlugin;

impl Plugin for BirdSeparationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, separate);
    }
}

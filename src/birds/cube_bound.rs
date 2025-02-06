use bevy::prelude::*;

use crate::velocity::Velocity;

use super::Bird;

#[derive(Resource, Deref, DerefMut)]
struct CubeBound(pub Cuboid);

fn flip_bird_transform(
    mut birds: Query<(&mut Transform, &Velocity), With<Bird>>,
    bound: Res<CubeBound>,
    time: Res<Time>,
) {
    for (mut transform, vel) in &mut birds {
        let next = transform.translation + vel.0 * time.delta_secs();
        let next_bounded = bound.closest_point(next);
        if next != next_bounded {
            transform.translation = next_bounded * -1.0;
        }
    }
}

pub struct BirdCubeBoundPlugin(pub Cuboid);

impl Plugin for BirdCubeBoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, flip_bird_transform)
            .insert_resource(CubeBound(self.0));
    }
}

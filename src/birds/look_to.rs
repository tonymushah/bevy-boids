use bevy::prelude::*;

use crate::velocity::Velocity;

use super::Bird;

pub fn look_to(mut birds: Query<(&mut Transform, &Velocity), With<Bird>>, time: Res<Time>) {
    for (mut transform, velocity) in &mut birds {
        *transform = transform.looking_at(
            transform.translation + (**velocity * time.delta_secs()),
            Vec3::Y,
        );
    }
}

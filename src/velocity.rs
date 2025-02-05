use bevy::prelude::*;

#[derive(Debug, Deref, DerefMut, Default, Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct ShowVelocityVector;

fn _move(mut has_velocities: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transfrom, velocity) in has_velocities.iter_mut() {
        transfrom.translation += velocity.0 * time.delta_secs();
    }
}

fn draw_arrow(
    has_velocities: Query<(&Transform, &Velocity), With<ShowVelocityVector>>,
    mut gizmos: Gizmos,
) {
    for (transform, velocity) in has_velocities.iter() {
        gizmos.arrow(
            transform.translation,
            transform.translation + velocity.0,
            Color::Srgba(Srgba::BLUE),
        );
    }
}

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (_move, draw_arrow).chain());
    }
}

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, DerefMut, Deref, Default, Component)]
pub struct MaxSpeed(pub f32);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct IsPaused(bool);

#[derive(Debug, Deref, DerefMut, Default, Component)]
struct PausedVelocityState(Vec3);

#[derive(Debug, Deref, DerefMut, Default, Component, Clone, Copy)]
#[require(PausedVelocityState)]
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
            transform.translation
                + velocity
                    .0
                    .clamp(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0)),
            Color::Srgba(Srgba::BLUE),
        );
    }
}

fn toggle_pause(mut is_paused: ResMut<IsPaused>) {
    **is_paused = !**is_paused;
}

fn handle_pause(
    mut velocities: Query<(&mut Velocity, &mut PausedVelocityState)>,
    is_paused: Res<IsPaused>,
) {
    for (mut velocity, mut paused_state) in &mut velocities {
        if **is_paused {
            **paused_state = **velocity;
            **velocity = Vec3::ZERO;
        } else {
            **velocity = **paused_state;
            **paused_state = Vec3::ZERO;
        }
    }
}

pub fn is_paused(is_paused: Res<IsPaused>) -> bool {
    **is_paused
}

fn pause_condition(keys: Res<ButtonInput<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::Enter)
}

fn handle_max_speed(mut max_speed_queries: Query<(&mut Velocity, &MaxSpeed)>) {
    for (mut vel, max_speed) in &mut max_speed_queries {
        if (*vel).length() > **max_speed {
            **vel = vel.normalize() * **max_speed;
        }
    }
}

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (_move, draw_arrow).chain())
            .insert_resource(IsPaused::default())
            .add_systems(
                Update,
                (toggle_pause, handle_pause).chain().run_if(pause_condition),
            )
            .add_systems(Update, handle_max_speed);
    }
}

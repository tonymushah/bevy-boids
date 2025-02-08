use bevy::{color::palettes::css::RED, math::bounding::Bounded3d, prelude::*};

use crate::{velocity::Velocity, vision_radius::VisionRadius};

use super::{Bird, ShowBirdsGizmo};

#[derive(Resource, Deref, DerefMut)]
pub struct CubeBound(pub Cuboid);

#[derive(Debug, Component)]
#[require(Mesh3d)]
pub struct ActualCube;

const MIN_SIZE: f32 = 20.0;

#[allow(clippy::type_complexity)]
fn flip_bird_transform(
    mut birds: Query<(&mut Transform, &Velocity), (With<Bird>, Without<ActualCube>)>,
    bound: Res<CubeBound>,
    cubes: Query<&Transform, (With<ActualCube>, Without<Bird>)>,
    time: Res<Time>,
) {
    if let Some(_cube) = cubes.iter().next() {
        let bound = bound.aabb_3d(Isometry3d::new(_cube.translation, _cube.rotation));
        for (mut transform, vel) in &mut birds {
            let next = transform.translation + vel.0 * time.delta_secs();
            let next_bounded: Vec3 = bound.closest_point(next).into();
            if next != next_bounded {
                transform.translation = next_bounded * -1.0;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn prevent_bird_outbound_sys(
    mut birds: Query<(&Transform, &mut Velocity, &VisionRadius), (With<Bird>, Without<ActualCube>)>,
    bound: Res<CubeBound>,
    cubes: Query<&Transform, (With<ActualCube>, Without<Bird>)>,
    time: Res<Time>,
    mut gizmos: Gizmos,
    show_gizmo: Res<ShowBirdsGizmo>,
) {
    if let Some(_cube) = cubes.iter().next() {
        let bound = bound.aabb_3d(Isometry3d::new(_cube.translation, _cube.rotation));
        for (transform, mut vel, vision) in &mut birds {
            let next =
                (transform.translation + vision.min_distance * 2.0) + vel.0 * time.delta_secs();
            let next_bounded: Vec3 = bound.closest_point(next).into();

            if next != next_bounded {
                let distance = next.distance(next_bounded);

                if distance > vision.min_distance {
                    continue;
                }

                let pos_vec = next - next_bounded;

                let force_magnitude = (vision.min_distance - distance).powi(2);

                let separation_force = pos_vec.normalize() * force_magnitude;

                if **show_gizmo {
                    gizmos.arrow(
                        transform.translation,
                        transform.translation + separation_force.normalize(),
                        RED,
                    );
                }

                **vel += separation_force;
            }
        }
    }
}

#[derive(
    Debug, Resource, Default, DerefMut, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct PreventBirdOutbound(pub bool);

pub fn prevent_bird_outbound(res: Res<PreventBirdOutbound>) -> bool {
    **res
}

/*
fn toggle_prevent_bird_outbound(mut res: ResMut<PreventBirdOutbound>) {
    **res = !**res;
}

fn toggle_prevent_bird_outbound_condition(key: Res<ButtonInput<KeyCode>>) -> bool {
    key.just_pressed(KeyCode::KeyP)
}
*/

fn update_actual_mesh_cube_size(
    mut cubes: Query<&mut Mesh3d, With<ActualCube>>,
    bound: Res<CubeBound>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if let Some(cube) = cubes.iter_mut().next() {
        meshes.insert(cube.id(), bound.mesh().into());
    }
}

fn bound_has_changed(res: Res<CubeBound>) -> bool {
    res.is_changed() && !res.is_added()
}

fn double_cube_size(mut bound: ResMut<CubeBound>) {
    bound.0.half_size *= 2.0;
}

fn double_cube_size_key(key: Res<ButtonInput<KeyCode>>) -> bool {
    key.just_pressed(KeyCode::ArrowUp)
}

fn divide_cube_size(mut bound: ResMut<CubeBound>) {
    let next = bound.0.half_size / 2.0;
    if next.length() > MIN_SIZE {
        bound.0.half_size = next;
    }
}

fn divide_cube_size_key(key: Res<ButtonInput<KeyCode>>) -> bool {
    key.just_pressed(KeyCode::ArrowDown)
}

pub struct BirdCubeBoundPlugin(pub Cuboid);

impl Plugin for BirdCubeBoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                flip_bird_transform,
                prevent_bird_outbound_sys.run_if(prevent_bird_outbound),
            )
                .chain(),
        )
        /* .add_systems(
            Update,
            toggle_prevent_bird_outbound.run_if(toggle_prevent_bird_outbound_condition),
        )*/
        .add_systems(
            Update,
            update_actual_mesh_cube_size.run_if(bound_has_changed),
        )
        .add_systems(Update, double_cube_size.run_if(double_cube_size_key))
        .add_systems(Update, divide_cube_size.run_if(divide_cube_size_key))
        .insert_resource(PreventBirdOutbound(false))
        .insert_resource(CubeBound(self.0));
    }
}

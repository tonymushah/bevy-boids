use bevy::{math::bounding::Bounded3d, prelude::*};

use crate::velocity::Velocity;

use super::Bird;

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
        app.add_systems(Update, flip_bird_transform)
            .add_systems(
                Update,
                update_actual_mesh_cube_size.run_if(bound_has_changed),
            )
            .add_systems(Update, double_cube_size.run_if(double_cube_size_key))
            .add_systems(Update, divide_cube_size.run_if(divide_cube_size_key))
            .insert_resource(CubeBound(self.0));
    }
}

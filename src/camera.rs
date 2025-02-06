use bevy::prelude::*;

use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

pub struct MainCameraPlugin;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        // PanOrbitCamera,
        Transform::from_translation(Vec3 {
            x: 50.0,
            y: 50.0,
            z: 12.0,
        })
        .looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));
}

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

pub struct MainCameraPluginGroup;

impl Plugin for MainCameraPluginGroup {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainCameraPlugin)
            .add_plugins(PanOrbitCameraPlugin);
    }
}

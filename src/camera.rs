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
            x: 65.0,
            y: 44.0,
            z: 52.0,
        })
        .looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
        Camera {
            hdr: true,
            ..Default::default()
        },
    ));
}

fn toggle_hdr(mut cameras: Query<&mut Camera, With<MainCamera>>) {
    for mut camera in &mut cameras {
        camera.hdr = !camera.hdr;
    }
}

fn toggle_hdr_condition(keys: Res<ButtonInput<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::F2)
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
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(Update, toggle_hdr.run_if(toggle_hdr_condition));
    }
}

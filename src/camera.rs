use bevy::{app::PluginGroupBuilder, prelude::*};
use pan::{PanOrbitCamera, PanOrbitCameraPlugin};

pub mod pan;

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

pub struct MainCameraPlugin;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        PanOrbitCamera,
        Transform::from_translation(Vec3 {
            x: 10.0,
            y: 10.0,
            z: 12.0,
        })
        .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

pub struct MainCameraPluginGroup;

impl PluginGroup for MainCameraPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MainCameraPlugin)
            .add(PanOrbitCameraPlugin)
    }
}

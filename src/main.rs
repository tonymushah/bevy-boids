use bevy::prelude::*;
use birds::BirdsPlugin;
use camera::MainCameraPluginGroup;
use env::EnvironmentPlugin;
use velocity::VelocityPlugin;

pub mod birds;
pub mod camera;
pub mod env;
pub mod utils;
pub mod velocity;
pub mod vision_radius;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(20.))
        .add_plugins(MainCameraPluginGroup)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(
            bevy_transform_interpolation::interpolation::TransformInterpolationPlugin::default(),
        )
        .add_plugins(BirdsPlugin)
        .add_plugins(VelocityPlugin)
        .run();
}

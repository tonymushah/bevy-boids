use bevy::prelude::*;
use bevy_boids::birds::BirdsPlugin;
use bevy_boids::camera::MainCameraPluginGroup;
use bevy_boids::env::EnvironmentPlugin;
use bevy_boids::velocity::VelocityPlugin;

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

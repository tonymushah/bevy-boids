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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainCameraPluginGroup)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(BirdsPlugin)
        .add_plugins(VelocityPlugin)
        .run();
}

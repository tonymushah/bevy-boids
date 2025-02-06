use bevy::prelude::*;
use rand::rng;

use crate::{utils::random_translation_uniform, velocity::Velocity};

use super::Bird;

fn random(mut birds: Query<&mut Velocity, With<Bird>>) {
    for mut vel in &mut birds {
        **vel = random_translation_uniform(&mut rng(), -4.0..=5.0);
    }
}

fn run_condition(keys: Res<ButtonInput<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::KeyR)
}

pub struct BirdsRandomVelPlugin;

impl Plugin for BirdsRandomVelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, random.run_if(run_condition));
    }
}

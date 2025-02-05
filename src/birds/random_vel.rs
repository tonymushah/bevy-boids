use bevy::prelude::*;
use rand::{rng, Rng};

use crate::velocity::Velocity;

use super::Bird;

fn random(mut birds: Query<&mut Velocity, With<Bird>>) {
    for mut vel in &mut birds {
        **vel = random_translation(&mut rng());
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

const TRANSLATION_BOUND_LOWER_X: f32 = -5.;
const TRANSLATION_BOUND_UPPER_X: f32 = 5.;
const TRANSLATION_BOUND_LOWER_Y: f32 = -1.;
const TRANSLATION_BOUND_UPPER_Y: f32 = 1.;
const TRANSLATION_BOUND_LOWER_Z: f32 = -2.;
const TRANSLATION_BOUND_UPPER_Z: f32 = 6.;

fn random_translation(rng: &mut impl Rng) -> Vec3 {
    let x = rng.random::<f32>() * (TRANSLATION_BOUND_UPPER_X - TRANSLATION_BOUND_LOWER_X)
        + TRANSLATION_BOUND_LOWER_X;
    let y = rng.random::<f32>() * (TRANSLATION_BOUND_UPPER_Y - TRANSLATION_BOUND_LOWER_Y)
        + TRANSLATION_BOUND_LOWER_Y;
    let z = rng.random::<f32>() * (TRANSLATION_BOUND_UPPER_Z - TRANSLATION_BOUND_LOWER_Z)
        + TRANSLATION_BOUND_LOWER_Z;

    Vec3::new(x, y, z)
}

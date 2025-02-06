use bevy::prelude::*;
use rand::{distr::uniform::SampleRange, Rng};

pub fn random_translation_uniform<R>(rng: &mut impl Rng, range: R) -> Vec3
where
    R: SampleRange<f32> + Clone,
{
    let x = rng.random_range(range.clone());
    let y = rng.random_range(range.clone());
    let z = rng.random_range(range.clone());

    Vec3::new(x, y, z)
}

pub fn random_translation<X, Y, Z>(rng: &mut impl Rng, x_range: X, y_range: Y, z_range: Z) -> Vec3
where
    X: SampleRange<f32>,
    Y: SampleRange<f32>,
    Z: SampleRange<f32>,
{
    let x = rng.random_range(x_range);
    let y = rng.random_range(y_range);
    let z = rng.random_range(z_range);

    Vec3::new(x, y, z)
}

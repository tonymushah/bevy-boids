use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    birds::{Bird, teams::Team},
    velocity::Velocity,
};

#[derive(Debug)]
pub struct BirdCacheEntry {
    pub velocity: Velocity,
    pub transform: Transform,
    pub team: Team,
}

#[derive(Debug, Resource, Deref, Default)]
pub struct BirdsCache(HashMap<Entity, BirdCacheEntry>);

fn get_bird_kd_map(
    birds_iter: Query<(&Velocity, &Transform, Entity, &Team), With<Bird>>,
    mut cache: ResMut<BirdsCache>,
) {
    cache.0 = birds_iter
        .iter()
        .map(|a| {
            (
                a.2,
                BirdCacheEntry {
                    velocity: *a.0,
                    transform: *a.1,
                    team: a.3.clone(),
                },
            )
        })
        .collect::<HashMap<_, _>>();
}

pub struct BirdsCachePlugin;

impl Plugin for BirdsCachePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BirdsCache::default())
            .add_systems(FixedPreUpdate, get_bird_kd_map);
    }
}

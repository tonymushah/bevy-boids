use acap::{kd::FlatKdTree, Coordinates, Proximity};
use bevy::prelude::*;

use crate::{
    utils::acap::InnerCoordVec3,
    velocity::{is_paused, Velocity},
};

use super::{teams::Team, Bird};

#[derive(Debug, Clone)]
pub struct BirdsKdTreeEntry {
    pub coord: Vec3,
    pub entity: Entity,
    pub vel: Velocity,
    pub team: Option<Team>,
}

impl Coordinates for BirdsKdTreeEntry {
    type Value = f32;
    fn dims(&self) -> usize {
        3
    }
    fn coord(&self, i: usize) -> Self::Value {
        let array = self.coord.as_ref();
        array[i]
    }
    fn as_vec(&self) -> Vec<Self::Value> {
        let array = self.coord.as_ref();
        array.to_vec()
    }
}

impl Proximity<InnerCoordVec3> for BirdsKdTreeEntry {
    type Distance = f32;

    fn distance(&self, other: &InnerCoordVec3) -> Self::Distance {
        self.coord.distance(**other)
    }
}

impl Proximity<BirdsKdTreeEntry> for InnerCoordVec3 {
    type Distance = f32;

    fn distance(&self, other: &BirdsKdTreeEntry) -> Self::Distance {
        self.0.distance(other.coord)
    }
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct BirdsKdTree(pub FlatKdTree<BirdsKdTreeEntry>);

fn populate_tree(
    mut tree: ResMut<BirdsKdTree>,
    birds: Query<(&Transform, &Velocity, &Team, Entity), With<Bird>>,
) {
    **tree = FlatKdTree::balanced(birds.iter().map(|(t, velocity, team, e)| BirdsKdTreeEntry {
        coord: t.translation,
        entity: e,
        vel: *velocity,
        team: Some(team.clone()),
    }));
}

pub struct BirdsKdTreePlugin;

impl Plugin for BirdsKdTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_tree.run_if(not(is_paused)))
            .insert_resource(BirdsKdTree(FlatKdTree::balanced(
                Vec::<BirdsKdTreeEntry>::new(),
            )));
    }
}

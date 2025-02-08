use acap::{knn::Neighborhood, NearestNeighbors};
use bevy::{color::palettes::css::GREEN, prelude::*};
use rand::{rng, Rng};

use crate::{
    utils::acap::InnerCoordVec3,
    velocity::{is_paused, Velocity},
    vision_radius::VisionRadius,
};

use super::{
    kd_tree::{BirdsKdTree, BirdsKdTreeEntry},
    teams::Team,
    Bird, ShowBirdsGizmo,
};

struct BirdAlignmentNeibhorHood<'k> {
    limit: Option<usize>,
    target: &'k InnerCoordVec3,
    entries: Vec<BirdsKdTreeEntry>,
    team: &'k Team,
    vision: VisionRadius,
}

impl<'k, 'v> Neighborhood<&'k InnerCoordVec3, &'v BirdsKdTreeEntry>
    for BirdAlignmentNeibhorHood<'k>
{
    fn target(&self) -> &'k InnerCoordVec3 {
        self.target
    }

    fn contains<D>(&self, distance: D) -> bool
    where
        D: PartialOrd<<&'k InnerCoordVec3 as acap::Proximity>::Distance>,
    {
        distance > self.vision.min_distance && distance < self.vision.neighboor_radius
    }

    fn consider(
        &mut self,
        item: &'v BirdsKdTreeEntry,
    ) -> <&'k InnerCoordVec3 as acap::Proximity>::Distance {
        let distance = self.target.distance(item.coord);

        if self.contains(distance)
            && item
                .team
                .as_ref()
                .map(|team| self.team == team)
                .unwrap_or(true)
        {
            if let Some(limit) = self.limit {
                if self.entries.len() <= limit {
                    self.entries.push(item.clone());
                }
            } else {
                self.entries.push(item.clone());
            }
        }
        distance
    }
}

#[allow(clippy::type_complexity)]
pub fn align(
    mut birds: Query<(&mut Velocity, &Transform, &VisionRadius, Entity, &Team), With<Bird>>,
    // time: Res<Time>,
    mut gizmos: Gizmos,
    show_gizmos: Res<ShowBirdsGizmo>,
    kd_tree: Res<BirdsKdTree>,
) {
    for mut bird in &mut birds {
        let neighbors = {
            let bird_trans = InnerCoordVec3(bird.1.translation);
            kd_tree
                .search(BirdAlignmentNeibhorHood {
                    limit: Some(rng().random_range::<u64, _>(3..20) as usize),
                    target: &bird_trans,
                    entries: Default::default(),
                    vision: *bird.2,
                    team: bird.4,
                })
                .entries
        };

        if neighbors.is_empty() {
            continue;
        }

        let average_vel = neighbors.iter().map(|b| *b.vel).sum::<Vec3>() / neighbors.len() as f32;

        if **show_gizmos {
            gizmos.arrow(
                bird.1.translation,
                bird.1.translation + average_vel.normalize(),
                GREEN,
            );
        }

        **bird.0 += average_vel.normalize();
    }
    /*
    let mut combinaisons = birds.iter_combinations_mut();
    while let Some([mut one, two]) = combinaisons.fetch_next() {
        let next_one = one.1.translation + **one.0 * time.delta_secs();
        let next_two = two.1.translation + **two.0 * time.delta_secs();

        let min_distance = one.2.min_distance;
        let neighbor_radius = one.2.neighboor_radius;
        let distance = next_one.distance(next_two);

        if !(min_distance < distance && distance < neighbor_radius) {
            continue;
        }

        let average_vel = (**one.0 + **two.0) / 2.0;

        if **show_gizmos {
            gizmos.arrow(
                one.1.translation,
                one.1.translation + average_vel.normalize(),
                GREEN,
            );
        }

        **one.0 += average_vel.normalize();
    }
    */
}
pub struct BirdAlignmentPlugin;

impl Plugin for BirdAlignmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, align.run_if(not(is_paused)));
    }
}

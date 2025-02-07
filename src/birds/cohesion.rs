use std::{collections::HashSet, time::Duration};

use acap::{knn::Neighborhood, NearestNeighbors};
use bevy::{color::palettes::css::PURPLE, prelude::*};
use rand::{rng, Rng};

use crate::{utils::acap::InnerCoordVec3, velocity::Velocity, vision_radius::VisionRadius};

use super::{
    /*birds_number::BirdNumber*/ kd_tree::{BirdsKdTree, BirdsKdTreeEntry},
    Bird,
};

const SIZE: usize = 2;

#[derive(Debug, Resource, Clone, Copy, DerefMut, Deref)]
pub struct ShowCohesionForceGizmo(pub bool);

#[derive(Debug, Resource, Clone, Copy, DerefMut, Deref)]
pub struct IsCohersionActive(pub bool);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct CohesionTimer(pub Timer);

struct BirdCohesionNeibhorHood<'k> {
    limit: Option<usize>,
    target: &'k InnerCoordVec3,
    entries: Vec<BirdsKdTreeEntry>,
    vision: VisionRadius,
}

impl<'k, 'v> Neighborhood<&'k InnerCoordVec3, &'v BirdsKdTreeEntry>
    for BirdCohesionNeibhorHood<'k>
{
    fn target(&self) -> &'k InnerCoordVec3 {
        self.target
    }

    fn contains<D>(&self, distance: D) -> bool
    where
        D: PartialOrd<<&'k InnerCoordVec3 as acap::Proximity>::Distance>,
    {
        let min = self.vision.cohesion_radius.0;
        let max = self.vision.cohesion_radius.1;
        distance > min && distance < max
    }

    fn consider(
        &mut self,
        item: &'v BirdsKdTreeEntry,
    ) -> <&'k InnerCoordVec3 as acap::Proximity>::Distance {
        let distance = self.target.distance(item.coord);

        if self.contains(distance) {
            if let Some(limit) = self.limit {
                if self.entries.len() <= limit {
                    self.entries.push(*item);
                }
            } else {
                self.entries.push(*item);
            }
        }
        distance
    }
}

#[allow(clippy::type_complexity)]
pub fn cohesion(
    mut birds: Query<(&mut Velocity, &Transform, &VisionRadius, Entity), With<Bird>>,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut timer: ResMut<CohesionTimer>,
    kd_tree: Res<BirdsKdTree>,
    // bird_number: Res<BirdNumber>,
    show_gizmo: Res<ShowCohesionForceGizmo>,
) {
    if timer.tick(time.delta()).just_finished() {
        let mut computed: HashSet<Entity> = Default::default();
        for mut bird in &mut birds {
            if !computed.contains(&bird.3) {
                computed.insert(bird.3);
            } else {
                continue;
            }
            // TODO

            let center = {
                let bird_trans = InnerCoordVec3(bird.1.translation);
                let points = kd_tree
                    .search(BirdCohesionNeibhorHood {
                        limit: Some(10),
                        target: &bird_trans,
                        entries: Default::default(),
                        vision: *bird.2,
                    })
                    .entries
                    .iter()
                    .map(|entry| entry.coord)
                    .collect::<Vec<_>>();
                if !points.is_empty() {
                    points.iter().sum::<Vec3>() / points.len() as f32
                } else {
                    Vec3::ZERO
                }
            };

            if center == Vec3::ZERO {
                continue;
            }

            let pos_vec = bird.1.translation - center;

            let center_force =
                (1.0 / rng().random_range::<i32, _>(1..=10) as f32) * pos_vec.normalize();

            if **show_gizmo {
                gizmos.arrow(
                    bird.1.translation,
                    bird.1.translation - center_force.normalize(),
                    PURPLE,
                );
            }

            **bird.0 -= center_force;
        }
        // This is old code for reference
        /*
        while let Some(mut to_cohese) = combinaisons.fetch_next() {
            let to_use = rng().random_range(0..(SIZE as u64)) as usize;

            let center = {
                let points = to_cohese
                    .iter()
                    .filter(|b| {
                        let bird = &to_cohese[to_use];
                        let next_one = bird.1.translation;
                        let next_two = b.1.translation;

                        let min = bird.2.cohesion_radius.0;
                        let max = bird.2.cohesion_radius.1;

                        let distance = next_one.distance(next_two);

                        min < distance && distance < max
                    })
                    .map(|(_, t, _, _)| t.translation)
                    .collect::<Vec<_>>();
                points.iter().sum::<Vec3>() / points.len() as f32
            };
            let bird = &mut to_cohese[to_use];

            /*
            let center_distance = bird.1.translation.distance(center);

            let min = bird.2.cohesion_radius.0;
            let max = bird.2.cohesion_radius.1;

            if !(min < center_distance && center_distance < max) {
                continue;
            } */

            let pos_vec = bird.1.translation - center;

            let center_force =
                (1.0 / rng().random_range::<i32, _>(1..=10) as f32) * pos_vec.normalize();

            if **show_gizmo {
                gizmos.arrow(
                    bird.1.translation,
                    bird.1.translation - center_force.normalize(),
                    PURPLE,
                );
            }

            **bird.0 -= center_force;
        }
        */
    }
}

fn toogle_cohesion(mut res: ResMut<IsCohersionActive>) {
    **res = !**res;
}

fn toggle_timer(is_active: Res<IsCohersionActive>, mut timer: ResMut<CohesionTimer>) {
    if **is_active {
        timer.unpause();
    } else {
        timer.pause();
    }
}

fn toogle_cohesion_condition(key: Res<ButtonInput<KeyCode>>) -> bool {
    key.just_pressed(KeyCode::KeyC)
}

pub fn is_cohesion_active(res: Res<IsCohersionActive>) -> bool {
    **res
}

fn toggle_gizmos(mut show_gizmo: ResMut<ShowCohesionForceGizmo>) {
    **show_gizmo = !**show_gizmo;
}

fn toogle_gismo_condition(key: Res<ButtonInput<KeyCode>>) -> bool {
    key.all_just_pressed([KeyCode::F1])
}

pub struct BirdCohesionPlugin;

impl Plugin for BirdCohesionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cohesion.run_if(is_cohesion_active))
            .add_systems(
                Update,
                (toogle_cohesion, toggle_timer)
                    .chain()
                    .run_if(toogle_cohesion_condition),
            )
            .insert_resource(IsCohersionActive(true))
            .insert_resource(CohesionTimer(Timer::new(
                Duration::from_millis(10 * SIZE as u64),
                TimerMode::Repeating,
            )))
            .insert_resource(ShowCohesionForceGizmo(true))
            .add_systems(Update, toggle_gizmos.run_if(toogle_gismo_condition));
    }
}

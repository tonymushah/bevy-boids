use std::num::NonZeroUsize;

use bevy::{color::palettes::css::GREEN, ecs::system::SystemParam, prelude::*};
use kiddo::dist::Chebyshev;
use rand::{RngExt, rng};

use crate::{
    birds::{birds_cache::BirdsCache, teams::Teaming},
    velocity::{Velocity, is_paused},
    vision_radius::VisionRadius,
};

use super::{Bird, ShowBirdsGizmo, kd_tree::BirdsKdTree, teams::Team};

// struct BirdAlignmentNeibhorHood<'k> {
//     limit: Option<usize>,
//     target: &'k InnerCoordVec3,
//     entries: Vec<BirdsKdTreeEntry>,
//     team: &'k Team,
//     vision: VisionRadius,
// }

// impl<'k, 'v> Neighborhood<&'k InnerCoordVec3, &'v BirdsKdTreeEntry>
//     for BirdAlignmentNeibhorHood<'k>
// {
//     fn target(&self) -> &'k InnerCoordVec3 {
//         self.target
//     }

//     fn contains<D>(&self, distance: D) -> bool
//     where
//         D: PartialOrd<<&'k InnerCoordVec3 as acap::Proximity>::Distance>,
//     {
//         distance > self.vision.min_distance && distance < self.vision.neighboor_radius
//     }

//     fn consider(
//         &mut self,
//         item: &'v BirdsKdTreeEntry,
//     ) -> <&'k InnerCoordVec3 as acap::Proximity>::Distance {
//         let distance = self.target.distance(item.coord);

//         if self.contains(distance)
//             && item
//                 .team
//                 .as_ref()
//                 .map(|team| self.team == team)
//                 .unwrap_or(true)
//         {
//             if let Some(limit) = self.limit {
//                 if self.entries.len() <= limit {
//                     self.entries.push(item.clone());
//                 }
//             } else {
//                 self.entries.push(item.clone());
//             }
//         }
//         distance
//     }
// }

#[derive(SystemParam)]
pub struct AlignParam<'w, 's> {
    pub birds: Query<
        'w,
        's,
        (
            &'static mut Velocity,
            &'static Transform,
            &'static VisionRadius,
            Entity,
            &'static Team,
        ),
        With<Bird>,
    >,
    // time: Res<Time>,
    pub gizmos: Gizmos<'w, 's>,
    pub show_gizmos: Res<'w, ShowBirdsGizmo>,
    pub kd_tree: Res<'w, BirdsKdTree>,
    pub teaming: Res<'w, Teaming>,
    pub cache_data: Res<'w, BirdsCache>,
}

#[allow(clippy::type_complexity)]
pub fn align(param: AlignParam) -> Result<(), BevyError> {
    let AlignParam {
        mut birds,
        mut gizmos,
        show_gizmos,
        kd_tree,
        teaming,
        cache_data,
    } = param;
    for mut bird in &mut birds {
        let neighbors = {
            kd_tree
                .query(&bird.1.translation.to_array())
                .best_n_within::<Chebyshev<_>>(
                    bird.2.neighboor_radius,
                    NonZeroUsize::new(rng().random_range::<u64, _>(3..20) as usize).ok_or_else(
                        || {
                            BevyError::warning(anyhow::format_err!(
                                "Cannot transform usize to NonZero variant"
                            ))
                        },
                    )?,
                )
                .without_distances()
                .with_points()
                .execute()
                .into_iter()
                .filter_map(|near_bird| {
                    if near_bird.item == bird.3.index_u32() {
                        None
                    } else {
                        let entity = Entity::from_raw_u32(near_bird.item)?;
                        let near_bird_info = cache_data.get(&entity)?;
                        if teaming.0 && &near_bird_info.team != bird.4 {
                            None
                        } else {
                            Some(near_bird_info.velocity.0)
                        }
                    }
                })
                .collect::<Box<[_]>>()
        };

        if neighbors.is_empty() {
            continue;
        }

        let average_vel = neighbors.iter().sum::<Vec3>() / neighbors.len() as f32;

        if **show_gizmos {
            gizmos.arrow(
                bird.1.translation,
                bird.1.translation + average_vel.normalize(),
                GREEN,
            );
        }

        // drop(bird);

        **bird.0 += average_vel.normalize();
    }

    Ok(())
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
        app.add_systems(FixedUpdate, align.run_if(not(is_paused)));
    }
}

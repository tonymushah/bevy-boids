use bevy::prelude::*;
use kiddo::{
    kd_tree::KdTree, leaf_strategies::VecOfArenas, stem_strategies::donnelly::DonnellySimdFull,
};

use crate::velocity::is_paused;

use super::{
    Bird,
    // teams::Teaming
};

// impl Proximity<InnerCoordVec3> for BirdsKdTreeEntry {
//     type Distance = f32;

//     fn distance(&self, other: &InnerCoordVec3) -> Self::Distance {
//         self.coord.distance(**other)
//     }
// }

// impl Proximity<BirdsKdTreeEntry> for InnerCoordVec3 {
//     type Distance = f32;

//     fn distance(&self, other: &BirdsKdTreeEntry) -> Self::Distance {
//         self.0.distance(other.coord)
//     }
// }

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct BirdsKdTree(
    pub KdTree<f32, u32, DonnellySimdFull<3>, VecOfArenas<f32, u32, 3, 32>, 3, 32>,
);

fn populate_tree(
    mut tree: ResMut<BirdsKdTree>,
    birds: Query<(&Transform, Entity), With<Bird>>,
    // res: Res<Teaming>,
) -> Result<(), BevyError> {
    let birds = birds.iter().collect::<Box<[_]>>();
    **tree = KdTree::builder()
        .with_parallel_construction()
        .build_from_source(
            &birds,
            |bird, axis| match axis {
                0 => bird.0.translation.x,
                1 => bird.0.translation.y,
                2 => bird.0.translation.z,
                _ => unreachable!(),
            },
            |_, data| data.1.index_u32(),
        )
        .map_err(BevyError::error)?;
    Ok(())
}

pub struct BirdsKdTreePlugin;

impl Plugin for BirdsKdTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPreUpdate, populate_tree.run_if(not(is_paused)))
            .insert_resource(BirdsKdTree(Default::default()));
    }
}

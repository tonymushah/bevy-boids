use bevy::prelude::*;
use kiddo::ImmutableKdTree;

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
pub struct BirdsKdTree(pub ImmutableKdTree<f32, 3>);

fn populate_tree(
    mut tree: ResMut<BirdsKdTree>,
    birds: Query<(&Transform, Entity), With<Bird>>,
    // res: Res<Teaming>,
) -> Result<(), BevyError> {
    let birds = birds.iter().collect::<Box<[_]>>();
    **tree = ImmutableKdTree::builder()
        .build_from_source(
            &birds,
            |bird, axis| {
                let point = bird.0.translation;
                match axis {
                    0 => point.x,
                    1 => point.y,
                    2 => point.z,
                    _ => unreachable!(),
                }
            },
            |_idx, point| point.1.index_u32(),
        )
        .map_err(BevyError::error)?;
    Ok(())
}

pub struct BirdsKdTreePlugin;

impl Plugin for BirdsKdTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, populate_tree.run_if(not(is_paused)))
            .insert_resource(BirdsKdTree(ImmutableKdTree::default()));
    }
}

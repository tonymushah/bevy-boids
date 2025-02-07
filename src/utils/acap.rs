use acap::{Coordinates, Proximity};
use bevy::prelude::*;

#[derive(Debug, Deref, DerefMut, Default)]
pub struct InnerCoordVec3(pub Vec3);

impl Coordinates for InnerCoordVec3 {
    type Value = f32;
    fn dims(&self) -> usize {
        3
    }
    fn coord(&self, i: usize) -> Self::Value {
        let array = self.0.as_ref();
        array[i]
    }
    fn as_vec(&self) -> Vec<Self::Value> {
        let array = self.0.as_ref();
        array.to_vec()
    }
}

impl Proximity for InnerCoordVec3 {
    type Distance = f32;
    fn distance(&self, other: &Self) -> Self::Distance {
        self.0.distance(**other)
    }
}

use crate::core::query::*;
use cglinalg::{
    Vector3,
};

pub trait Intersection {
    fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult;
}
pub trait Geometry: std::fmt::Debug + Intersection {
    fn center(&self) -> Vector3<f32>;

    fn contains(&self, point: &Vector3<f32>) -> bool;
}


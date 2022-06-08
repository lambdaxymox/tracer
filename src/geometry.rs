use crate::ray::{
    Ray,
};
use cglinalg::{
    Vector3,
};

pub trait Intersection {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectionResult>;
}
pub trait Geometry: std::fmt::Debug + Intersection {
    fn center(&self) -> Vector3<f32>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IntersectionResult {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl<'a> IntersectionResult {
    pub fn new(t: f32, point: Vector3<f32>, normal: Vector3<f32>) -> IntersectionResult {
        IntersectionResult {
            t, point, normal,
        }
    }
}


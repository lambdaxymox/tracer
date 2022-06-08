use crate::ray::{
    Ray,
};
use cglinalg::{
    Vector3,
};


pub trait Geometry: std::fmt::Debug {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<GeometryIntersectionResult>;

    fn center(&self) -> Vector3<f32>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GeometryIntersectionResult {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl<'a> GeometryIntersectionResult {
    pub fn new(t: f32, p: Vector3<f32>, normal: Vector3<f32>) -> GeometryIntersectionResult {
        GeometryIntersectionResult {
            t, p, normal,
        }
    }
}


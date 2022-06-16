use crate::core::*;
use crate::light::*;
use cglinalg::{
    Vector3,
};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight {
    pub emission: Vector3<f32>,
}

impl PointLight {
    pub fn new(emission: Vector3<f32>) -> Self {
        Self { emission, }
    }
}

impl LightMapping for PointLight {
    fn emit(&self, query: &LightingQuery) -> LightingResult {
        LightingResult {
            ray_incoming: query.ray_incoming,
            point: query.point,
            radiance: self.emission,
        }
    }
}


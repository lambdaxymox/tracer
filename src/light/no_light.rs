use crate::core::*;
use crate::light::*;
use cglinalg::{
    Vector3,
};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NoLight {}

impl NoLight {
    pub fn new() -> Self {
        Self {}
    }
}

impl LightMapping for NoLight {
    fn emit(&self, query: &LightingQuery) -> LightingResult {
        LightingResult {
            ray_incoming: query.ray_incoming,
            point: query.point,
            radiance: Vector3::zero(),
        }
    }
}


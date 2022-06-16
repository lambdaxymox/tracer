use cglinalg::{
    Vector3,
};


pub trait LightMapping: std::fmt::Debug {
    fn emit(&self, query: &LightingQuery) -> LightingResult;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LightingQuery {
    pub ray_incoming: Vector3<f32>,
    pub point: Vector3<f32>,
}

impl LightingQuery {
    pub fn new(ray_incoming: Vector3<f32>, point: Vector3<f32>) -> Self {
        Self { ray_incoming, point, }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LightingResult {
    pub ray_incoming: Vector3<f32>,
    pub point: Vector3<f32>,
    pub radiance: Vector3<f32>,
}

impl LightingResult {
    pub fn new(ray_incoming: Vector3<f32>, point: Vector3<f32>, radiance: Vector3<f32>) -> Self {
        Self { ray_incoming, point, radiance, }
    }
}


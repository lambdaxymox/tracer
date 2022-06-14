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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LightingResult {
    pub ray_incoming: Vector3<f32>,
    pub point: Vector3<f32>,
    pub emission: Vector3<f32>,
}

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
            emission: Vector3::zero(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DiffuseLight {
    emission: Vector3<f32>,
}

impl DiffuseLight {
    pub fn new(emission: Vector3<f32>) -> Self {
        Self { emission, }
    }
}

impl LightMapping for DiffuseLight {
    fn emit(&self, query: &LightingQuery) -> LightingResult {
        LightingResult {
            ray_incoming: query.ray_incoming,
            point: query.point,
            emission: self.emission,
        }
    }
}


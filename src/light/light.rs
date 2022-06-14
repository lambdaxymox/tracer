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


use crate::core::*;
use cglinalg::{
    Vector3,
};


pub trait BsdfMapping: std::fmt::Debug {
    fn sample(&self, query: &BsdfQuery) -> BsdfResult;
}

pub trait BsdfQuerySampler: std::fmt::Debug {
    type Bsdf: BsdfMapping;

    fn sample(
        &self, 
        bsdf: &Self::Bsdf, 
        ray_incoming: &Vector3<f32>, 
        normal: &Vector3<f32>, 
        point: &Vector3<f32>, 
        sampler: &mut SphereSampler) -> BsdfQuery;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BsdfQuery {
    pub ray_incoming: Vector3<f32>,
    pub ray_outgoing: Vector3<f32>,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl BsdfQuery {
    pub fn new(ray_incoming: Vector3<f32>, ray_outgoing: Vector3<f32>, point: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self { ray_incoming, ray_outgoing, point, normal, }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BsdfResult {
    pub ray_incoming: Vector3<f32>,
    pub ray_outgoing: Vector3<f32>,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub scattering_fraction: Vector3<f32>,
}

impl BsdfResult {
    pub fn new(
        ray_incoming: Vector3<f32>,
        ray_outgoing: Vector3<f32>,
        point: Vector3<f32>,
        normal: Vector3<f32>,
        scattering_fraction: Vector3<f32>) -> Self 
    {
        Self { ray_incoming, ray_outgoing, point, normal, scattering_fraction, }
    }
}


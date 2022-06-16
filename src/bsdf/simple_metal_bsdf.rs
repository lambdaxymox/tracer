use crate::core::*;
use cglinalg::{
    Vector3,
    Magnitude,
};


#[derive(Copy, Clone, Debug)]
pub struct SimpleMetalBsdf {
    reflectance: Vector3<f32>,
    fuzz: f32,
}

impl SimpleMetalBsdf {
    pub fn new(reflectance: Vector3<f32>, fuzz: f32) -> Self {
        Self { reflectance, fuzz, }
    }
}

impl BsdfMapping for SimpleMetalBsdf {
    fn sample(&self, query: &BsdfQuery) -> BsdfResult {
        let scattering_fraction = self.reflectance;

        BsdfResult::new(
            query.ray_incoming,
            query.ray_outgoing,
            query.point,
            query.normal,
            scattering_fraction,
        )
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SimpleMetalBsdfQuerySampler {}

impl SimpleMetalBsdfQuerySampler {
    pub fn new() -> Self {
        Self {}
    }
}

impl BsdfQuerySampler for SimpleMetalBsdfQuerySampler {
    type Bsdf = SimpleMetalBsdf;

    #[inline]
    fn sample(
        &self, 
        bsdf: &Self::Bsdf, 
        ray_incoming: &Vector3<f32>, 
        normal: &Vector3<f32>, 
        point: &Vector3<f32>,
        sampler: &mut SphereSampler) -> BsdfQuery
    {
        let reflected_direction = ray_incoming.reflect(normal);
        let fuzzed_vector = sampler.sample_unit_sphere() * bsdf.fuzz;
        let ray_outgoing = reflected_direction + fuzzed_vector;

        BsdfQuery::new(*ray_incoming, ray_outgoing, *point, *normal)
    }
}


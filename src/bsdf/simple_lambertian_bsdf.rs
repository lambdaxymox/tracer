use crate::core::*;
use crate::bsdf::*;
use cglinalg::{
    Vector3,
    Magnitude,
};


#[derive(Copy, Clone, Debug)]
pub struct SimpleLambertianBsdf {
    scattering_fraction: Vector3<f32>,
}

impl SimpleLambertianBsdf {
    pub fn new(scattering_fraction: Vector3<f32>) -> Self {
        Self { scattering_fraction, }
    }
}

impl BsdfMapping for SimpleLambertianBsdf {
    fn sample(&self, query: &BsdfQuery) -> BsdfResult {
        BsdfResult {
            ray_incoming: query.ray_incoming,
            ray_outgoing: query.ray_outgoing,
            point: query.point,
            normal: query.normal,
            scattering_fraction: self.scattering_fraction,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SimpleLambertianBsdfQuerySampler {}

impl SimpleLambertianBsdfQuerySampler {
    pub fn new() -> Self { 
        Self {}
    }
}

impl BsdfQuerySampler for SimpleLambertianBsdfQuerySampler {
    type Bsdf = SimpleLambertianBsdf;

    #[inline]
    fn sample(
        &self, 
        _bsdf: &Self::Bsdf, 
        ray_incoming: &Vector3<f32>, 
        normal: &Vector3<f32>, 
        point: &Vector3<f32>,
        sampler: &mut SphereSampler) -> BsdfQuery
    {
        let target = point + normal + sampler.sample_unit_sphere();
        let ray_outgoing = target - point;
        
        BsdfQuery::new(*ray_incoming, ray_outgoing, *point, *normal)
    }
}


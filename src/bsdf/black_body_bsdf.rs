use crate::core::*;
use cglinalg::{
    Vector3,
    Magnitude,
};



#[derive(Copy, Clone, Debug)]
pub struct BlackBodyBsdf {}

impl BlackBodyBsdf {
    pub fn new() -> Self {
        Self {}
    }
}

impl BsdfMapping for BlackBodyBsdf {
    fn sample(&self, query: &BsdfQuery) -> BsdfResult {
        BsdfResult::new(
            query.ray_incoming,
            query.ray_outgoing,
            query.point,
            query.normal,
            Vector3::from_fill(0.001),
        )
    }
}


#[derive(Copy, Clone, Debug, Default)]
pub struct BlackBodyBsdfQuerySampler {}

impl BlackBodyBsdfQuerySampler {
    pub fn new() -> Self {
        Self {}
    }
}

impl BsdfQuerySampler for BlackBodyBsdfQuerySampler {
    type Bsdf = BlackBodyBsdf;

    #[inline]
    fn sample(
        &self, 
        _bsdf: &Self::Bsdf, 
        ray_incoming: &Vector3<f32>, 
        normal: &Vector3<f32>, 
        point: &Vector3<f32>,
        _sampler: &mut SphereSampler) -> BsdfQuery
    {
        let ray_outgoing = *ray_incoming;

        BsdfQuery::new(*ray_incoming, ray_outgoing, *point, *normal)
    }
}

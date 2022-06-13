use crate::geometry::*;
use crate::sphere::*;
use crate::query::*;
use crate::sampler::*;
use cglinalg::{
    Vector3,
    Magnitude,
};


pub trait ModelObject: std::fmt::Debug {
    fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult;

    fn scatter(&self, query: &ScatteringQuery, sampler: &mut SphereSampler) -> Option<ScatteringResult>;

    fn center(&self) -> Vector3<f32>;

    fn contains(&self, point: &Vector3<f32>) -> bool;

    fn normal(&self, point: &Vector3<f32>) -> Option<Vector3<f32>>;
}

#[derive(Debug)]
pub struct SphereModelObject<Bsdf> 
where 
    Bsdf: BsdfMapping 
{
    geometry: Sphere,
    bsdf: Box<Bsdf>,
    sampler: Box<dyn BsdfQuerySampler<Bsdf = Bsdf>>,
}

impl<Bsdf> SphereModelObject<Bsdf> 
where 
    Bsdf: BsdfMapping 
{
    pub fn new(geometry: Sphere, bsdf: Box<Bsdf>, sampler: Box<dyn BsdfQuerySampler<Bsdf = Bsdf>>) -> Self {
        Self { geometry, bsdf, sampler, }
    }
}

impl<Bsdf> ModelObject for SphereModelObject<Bsdf>
where 
    Bsdf: BsdfMapping
{
    fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult {
        self.geometry.intersect(query)
    }

    fn scatter(&self, query: &ScatteringQuery, sampler: &mut SphereSampler) -> Option<ScatteringResult> {
        // if let Some(normal) = self.normal(&query.point) {
            let normal = (query.point - self.geometry.center()).normalize();
            let ray_incoming = query.ray_incoming;
            let bsdf_query = self.sampler.sample(&self.bsdf, &ray_incoming, &normal, &query.point, sampler);
            let bsdf_result = self.bsdf.sample(&bsdf_query);
             
            Some(ScatteringResult::new(
                bsdf_result.ray_incoming,
                bsdf_result.ray_outgoing,
                bsdf_result.point,
                bsdf_result.normal,
                bsdf_result.scattering_fraction,
            ))
        // } else {
        //     None
        // }
    }

    fn center(&self) -> Vector3<f32> {
        self.geometry.center()
    }

    fn contains(&self, point: &Vector3<f32>) -> bool {
        let diff = point - self.geometry.center;
        
        diff.dot(&diff) <= self.geometry.radius * self.geometry.radius
    }

    fn normal(&self, point: &Vector3<f32>) -> Option<Vector3<f32>> {
        if self.contains(point) {
            Some((point - self.geometry.center()).normalize())
        } else {
            None
        }
    }
}


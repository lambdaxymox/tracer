use crate::query::*;
use crate::sampler::*;
use cglinalg::{
    Magnitude, 
    Vector3,
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

#[derive(Copy, Clone, Debug)]
pub struct SimpleDielectricBsdf {
    pub refraction_index: f32,
}

impl SimpleDielectricBsdf {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index, }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SimpleDielectricBsdfQuerySampler {}

impl SimpleDielectricBsdfQuerySampler {
    pub fn new() -> Self {
        Self {}
    }
}

impl BsdfQuerySampler for SimpleDielectricBsdfQuerySampler {
    type Bsdf = SimpleDielectricBsdf;

    #[inline]
    fn sample(
        &self, 
        bsdf: &Self::Bsdf, 
        ray_incoming: &Vector3<f32>, 
        normal: &Vector3<f32>, 
        point: &Vector3<f32>,
        sampler: &mut SphereSampler) -> BsdfQuery
    {
        #[inline]
        fn refract(ray_incoming: Vector3<f32>, normal: Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
            let uv = ray_incoming.normalize();
            let dt = uv.dot(&normal);
            let discriminant = 1_f32 - ni_over_nt * ni_over_nt * (1_f32 - dt * dt);
            if discriminant > 0_f32 {
                let refracted_direction = (uv - normal * dt) * ni_over_nt - normal * discriminant.sqrt();
                Some(refracted_direction)
            } else {
                None
            }
        }
        
        #[inline]
        fn schlick(cosine: f32, refraction_index: f32) -> f32 {
            let mut r0 = (1_f32 - refraction_index) / (1_f32 + refraction_index);
            r0 = r0 * r0;
            r0 + (1_f32 - r0) * (1_f32 - cosine).powf(5_f32)
        }

        let (normal_outward, ni_over_nt, cosine) = if ray_incoming.dot(normal) > 0_f32 {
            (
                -normal,
                bsdf.refraction_index, 
                bsdf.refraction_index * ray_incoming.dot(normal) / ray_incoming.magnitude()
            )
        } else {
            (
                *normal,
                1_f32 / bsdf.refraction_index,
                -ray_incoming.dot(normal) / ray_incoming.magnitude(),
            )
        };

        let ray_outgoing = if let Some(refracted_direction) = refract(*ray_incoming, normal_outward, ni_over_nt) {
            let reflection_prob = schlick(cosine, bsdf.refraction_index);
            if sampler.sample_f32() < reflection_prob {
                ray_incoming.reflect(normal)
            } else {
                refracted_direction
            }
        } else {
            ray_incoming.reflect(normal)
        };

        BsdfQuery::new(*ray_incoming, ray_outgoing, *point, normal_outward)
    }
}

impl BsdfMapping for SimpleDielectricBsdf {
    fn sample(&self, query: &BsdfQuery) -> BsdfResult {
        BsdfResult::new(
            query.ray_incoming,
            query.ray_outgoing,
            query.point, 
            query.normal, 
            Vector3::new(1_f32, 1_f32, 1_f32),
        )
    }
}


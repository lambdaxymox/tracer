use crate::geometry::*;
use crate::query::*;
use crate::sample;
use cglinalg::{
    Magnitude, 
    Vector3,
};
use rand::prelude::*;


pub trait ObjectMaterial: std::fmt::Debug {
    // TODO: This interface is subley wrong because the input ray and the hit result are in
    // world space. We want the material queries to be in model space since that's where the mapping
    // of texture coordinates to object coordinates takes place.
    // TODO: WE are presently assuming that all intersection results hit or tangent since the interface is
    // no quite right. We must fix this.
    fn sample_bsdf(&self, ray_in: Ray, hit: &IntersectionResult, rng: &mut ThreadRng) -> ScatteringResult;
}


#[derive(Copy, Clone, Debug)]
pub struct SimpleLambertianMaterial {
    albedo: Vector3<f32>,
}

impl SimpleLambertianMaterial {
    pub fn new(albedo: Vector3<f32>) -> Self {
        Self { albedo, }
    }
}

impl ObjectMaterial for SimpleLambertianMaterial {
    fn sample_bsdf(&self, ray_in: Ray, hit: &IntersectionResult, rng: &mut ThreadRng) -> ScatteringResult {
        if let IntersectionResult::Hit(result) = hit {
            let target = result.point + result.normal + sample::random_in_unit_sphere(rng);
            let scattering_fraction = self.albedo;
            let scattering_ray = Ray::new(result.point, target - result.point);

            ScatteringResult::new(scattering_fraction, scattering_ray)
        } else if let IntersectionResult::Tangent(result) = hit {
            let target = result.point + result.normal + sample::random_in_unit_sphere(rng);
            let scattering_fraction = self.albedo;
            let scattering_ray = Ray::new(result.point, target - result.point);

            ScatteringResult::new(scattering_fraction, scattering_ray)
        } else {
            panic!("Unwrapped a miss.")
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SimpleMetalMaterial {
    albedo: Vector3<f32>,
    fuzz: f32,
}

impl SimpleMetalMaterial {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Self {
        Self { albedo, fuzz, }
    }
}

impl ObjectMaterial for SimpleMetalMaterial {
    fn sample_bsdf(&self, ray_in: Ray, hit: &IntersectionResult, rng: &mut ThreadRng) -> ScatteringResult {
        if let IntersectionResult::Hit(result) = hit {
            let reflected_direction = ray_in.direction.reflect(&result.normal);
            let scattering_fraction = self.albedo;
            let scattering_ray = Ray::new(
                result.point, 
                reflected_direction + sample::random_in_unit_sphere(rng) * self.fuzz,
            );

            ScatteringResult::new(scattering_fraction, scattering_ray)
        } else if let IntersectionResult::Tangent(result) = hit {
            let reflected_direction = ray_in.direction.reflect(&result.normal);
            let scattering_fraction = self.albedo;
            let scattering_ray = Ray::new(
                result.point, 
                reflected_direction + sample::random_in_unit_sphere(rng) * self.fuzz,
            );

            ScatteringResult::new(scattering_fraction, scattering_ray)
        } else {
            panic!("Unwrapped a miss.")
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SimpleDielectricMaterial {
    pub refraction_index: f32,
}

impl SimpleDielectricMaterial {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index, }
    }
}

impl ObjectMaterial for SimpleDielectricMaterial {
    fn sample_bsdf(&self, ray_in: Ray, hit: &IntersectionResult, rng: &mut ThreadRng) -> ScatteringResult {
        fn refract(v: Vector3<f32>, n: Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
            let uv = v.normalize();
            let dt = uv.dot(&n);
            let discriminant = 1_f32 - ni_over_nt * ni_over_nt * (1_f32 - dt * dt);
            if discriminant > 0_f32 {
                let refracted_direction = (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
                Some(refracted_direction)
            } else {
                None
            }
        }
        
        fn schlick(cosine: f32, refraction_index: f32) -> f32 {
            let mut r0 = (1_f32 - refraction_index) / (1_f32 + refraction_index);
            r0 = r0 * r0;
            r0 + (1_f32 - r0) * (1_f32 - cosine).powf(5_f32)
        }


        let result = if let IntersectionResult::Hit(value) | IntersectionResult::Tangent(value) = hit {
            value
        } else {
            panic!("Unwrapped a miss")
        };

        let (outward_normal, ni_over_nt, cosine) = if ray_in.direction.dot(&result.normal) > 0_f32 {
            (
                -result.normal,
                self.refraction_index,
                self.refraction_index * ray_in.direction.dot(&result.normal) / ray_in.direction.magnitude(),
            )
        } else {
            (
                result.normal,
                1_f32 / self.refraction_index,
                -ray_in.direction.dot(&result.normal) / ray_in.direction.magnitude(),
            )
        };

        if let Some(refracted) = refract(ray_in.direction, outward_normal, ni_over_nt) {
            let reflection_prob = schlick(cosine, self.refraction_index);
            let out_direction = if rng.gen::<f32>() < reflection_prob {
                ray_in.direction.reflect(&result.normal)
            } else {
                refracted
            };
            ScatteringResult::new(
                Vector3::new(1_f32, 1_f32, 1_f32), 
                Ray::new(result.point, out_direction)
            )
        } else {
            ScatteringResult::new(
                Vector3::new(1_f32, 1_f32, 1_f32), 
                Ray::new(result.point, ray_in.direction.reflect(&result.normal))
            )
        }
    }
}


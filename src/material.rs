use crate::ray::{
    Ray,
};
use crate::sample;
use crate::scene_object::*;
use cglinalg::{
    Magnitude, 
    Vector3,
};
use rand::prelude::*;


pub trait ObjectMaterial: std::fmt::Debug {
    fn sample_bsdf(&self, ray_in: Ray, hit: &ObjectIntersectionResult, rng: &mut ThreadRng) -> ScatteredRay;
}

#[derive(Copy, Clone, Debug)]
pub struct SimpleLambertianMaterial {
    albedo: Vector3<f32>,
}

impl SimpleLambertianMaterial {
    pub fn new(albedo: Vector3<f32>) -> SimpleLambertianMaterial {
        SimpleLambertianMaterial {
            albedo,
        }
    }
}

impl ObjectMaterial for SimpleLambertianMaterial {
    fn sample_bsdf(&self, _ray_in: Ray, hit: &ObjectIntersectionResult, rng: &mut ThreadRng) -> ScatteredRay {
        let target = hit.p + hit.normal + sample::random_in_unit_sphere(rng);
        let attenuation = self.albedo;
        let scattering_ray = Ray::new(hit.p, target - hit.p);

        ScatteredRay::new(attenuation, scattering_ray)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SimpleMetalMaterial {
    albedo: Vector3<f32>,
    fuzz: f32,
}

impl SimpleMetalMaterial {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> SimpleMetalMaterial {
        SimpleMetalMaterial {
            albedo, fuzz,
        }
    }
}

impl ObjectMaterial for SimpleMetalMaterial {
    fn sample_bsdf(&self, ray_in: Ray, hit: &ObjectIntersectionResult, rng: &mut ThreadRng) -> ScatteredRay {
        let reflected_direction = ray_in.direction.reflect(&hit.normal);
        let scattering_fraction = self.albedo;
        let scattering_ray = Ray::new(
            hit.p, 
            reflected_direction + sample::random_in_unit_sphere(rng) * self.fuzz
        );
        
        ScatteredRay::new(scattering_fraction, scattering_ray)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SimpleDielectricMaterial {
    pub refraction_index: f32,
}

impl SimpleDielectricMaterial {
    pub fn new(refraction_index: f32) -> SimpleDielectricMaterial {
        SimpleDielectricMaterial {
            refraction_index,
        }
    }
}

impl ObjectMaterial for SimpleDielectricMaterial {
    fn sample_bsdf(&self, ray: Ray, hit: &ObjectIntersectionResult, rng: &mut ThreadRng) -> ScatteredRay {
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


        let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(&hit.normal) > 0_f32 {
            (
                -hit.normal,
                self.refraction_index,
                self.refraction_index * ray.direction.dot(&hit.normal) / ray.direction.magnitude(),
            )
        } else {
            (
                hit.normal,
                1_f32 / self.refraction_index,
                -ray.direction.dot(&hit.normal) / ray.direction.magnitude(),
            )
        };

        if let Some(refracted) = refract(ray.direction, outward_normal, ni_over_nt) {
            let reflection_prob = schlick(cosine, self.refraction_index);
            let out_dir = if rng.gen::<f32>() < reflection_prob {
                ray.direction.reflect(&hit.normal)
            } else {
                refracted
            };
            ScatteredRay::new(
                Vector3::new(1_f32, 1_f32, 1_f32), 
                Ray::new(hit.p, out_dir)
            )
        } else {
            ScatteredRay::new(
                Vector3::new(1_f32, 1_f32, 1_f32), 
                Ray::new(hit.p, ray.direction.reflect(&hit.normal))
            )
        }
    }
}


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


// TODO: Move this to cglinalg crate.
#[inline]
fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - n * (2_f32 * v.dot(&n))
}

pub trait ObjectMaterial {
    fn sample_bsdf(&self, ray_in: Ray, hit: &IntersectionResult, rng: &mut ThreadRng) -> ScatteredRay;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Lambertian {
        Lambertian {
            albedo: albedo,
        }
    }

    pub fn sample_bsdf(&self, _ray_in: Ray, hit: &IntersectionResult, rng: &mut ThreadRng) -> ScatteredRay {
        let target = hit.p + hit.normal + sample::random_in_unit_sphere(rng);
        let attenuation = self.albedo;
        let scattering_ray = Ray::new(hit.p, target - hit.p);

        ScatteredRay::new(attenuation, scattering_ray)
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vector3<f32>,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: fuzz,
        }
    }

    pub fn sample_bsdf(&self, ray_in: Ray, hit: &IntersectionResult, rng: &mut ThreadRng) -> ScatteredRay {
        let reflected_direction = reflect(ray_in.direction.normalize(), hit.normal);
        let attenuation = self.albedo;
        let scattering_ray = Ray::new(
            hit.p, 
            reflected_direction + sample::random_in_unit_sphere(rng) * self.fuzz
        );
        
        ScatteredRay::new(attenuation, scattering_ray)
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub refraction_index: f32,
}

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

impl Dielectric {
    fn new(refraction_index: f32) -> Dielectric {
        Dielectric {
            refraction_index: refraction_index,
        }
    }

    pub fn sample_bsdf(&self, ray: Ray, hit: IntersectionResult, rng: &mut ThreadRng) -> ScatteredRay {
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
                reflect(ray.direction, hit.normal)
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
                Ray::new(hit.p, reflect(ray.direction, hit.normal))
            )
        }
    }
}



#[derive(Copy, Clone)]
pub enum Material {
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
}

impl Material {
    pub fn lambertian(albedo: Vector3<f32>) -> Material {
        Material::Lambertian(Lambertian::new(albedo))
    }
    
    pub fn metal(albedo: Vector3<f32>, fuzz: f32) -> Material {
        Material::Metal(Metal::new(albedo, fuzz))
    }

    pub fn dielectric(refraction_index: f32) -> Material {
        Material::Dielectric(Dielectric::new(refraction_index))
    }
}

impl ObjectMaterial for Material {
    fn sample_bsdf(&self, ray_in: Ray, hit: &IntersectionResult, rng: &mut ThreadRng) -> ScatteredRay {
        match *self {
            Material::Metal(metal) => metal.sample_bsdf(ray_in, hit, rng),
            Material::Lambertian(lambertian) => lambertian.sample_bsdf(ray_in, hit, rng),
            Material::Dielectric(dielectric) => dielectric.sample_bsdf(ray_in, *hit, rng),
        }
    }
}


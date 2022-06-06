use crate::ray::{
    Ray,
};
use crate::sample;
use cglinalg::{
    Magnitude, 
    Vector3,
};
use rand::prelude::*;


#[inline]
fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - n * 2_f32 * v.dot(&n)
}

#[derive(Copy, Clone)]
pub struct Scatter {
    pub attenuation: Vector3<f32>,
    pub ray: Ray,
}

impl Scatter {
    pub fn new(attenuation: Vector3<f32>, ray: Ray) -> Scatter {
        Scatter { 
            attenuation: attenuation, 
            ray: ray,
        }
    }
}

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vector3<f32>, normal: Vector3<f32>, material: &'a Material) -> HitRecord<'a> {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            material: material,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
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

    pub fn scatter(&self, _ray_in: Ray, hit: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        let target = hit.p + hit.normal + sample::random_in_unit_sphere(rng);
        let attenuation = self.albedo;
        let scattered = Ray::new(hit.p, target - hit.p);

        Scatter::new(attenuation, scattered)
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

    pub fn scatter(&self, ray_in: Ray, hit: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        let reflected = reflect(ray_in.direction.normalize(), hit.normal);
        let attenuation = self.albedo;
        let scattered = Ray::new(hit.p, reflected + sample::random_in_unit_sphere(rng) * self.fuzz);
        
        Scatter::new(attenuation, scattered)
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
        let refracted = (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
        Some(refracted)
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

    pub fn scatter(&self, ray: Ray, hit: HitRecord, rng: &mut ThreadRng) -> Scatter {
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
            Scatter::new(
                Vector3::new(1_f32, 1_f32, 1_f32), 
                Ray::new(hit.p, out_dir)
            )
        } else {
            Scatter::new(
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
    pub fn scatter(&self, ray_in: Ray, hit: &HitRecord, rng: &mut ThreadRng) -> Scatter {
        match *self {
            Material::Metal(metal) => metal.scatter(ray_in, hit, rng),
            Material::Lambertian(lambertian) => lambertian.scatter(ray_in, hit, rng),
            Material::Dielectric(dielectric) => dielectric.scatter(ray_in, *hit, rng),
        }
    }

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


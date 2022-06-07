use crate::ray::Ray;
use crate::material::{
    Material, 
};
use cglinalg::{
    Vector3,
};
use rand::prelude::*;


pub trait Intersect {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectionRecord>;
}

#[derive(Copy, Clone)]
pub struct IntersectionRecord<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub object: &'a SceneObject,
}

impl<'a> IntersectionRecord<'a> {
    pub fn new(t: f32, p: Vector3<f32>, normal: Vector3<f32>, object: &'a SceneObject) -> IntersectionRecord<'a> {
        IntersectionRecord {
            t: t,
            p: p,
            normal: normal,
            object: object,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ScatteredRay {
    pub attenuation: Vector3<f32>,
    pub ray: Ray,
}

impl ScatteredRay {
    pub fn new(attenuation: Vector3<f32>, ray: Ray) -> ScatteredRay {
        ScatteredRay { 
            attenuation: attenuation, 
            ray: ray,
        }
    }
}

pub struct SceneObject {
    geometry: Sphere,
    material: Material,
}

impl SceneObject {
    pub fn new(geometry: Sphere, material: Material) -> Self {
        Self { geometry, material }
    }

    pub fn sample_bsdf(&self, ray_in: Ray, hit: &IntersectionRecord, rng: &mut ThreadRng) -> ScatteredRay {
        self.material.sample_bsdf(ray_in, hit, rng)
    }
}

impl Intersect for SceneObject {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectionRecord> {
        let oc = ray.origin - self.geometry.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.geometry.radius * self.geometry.radius;
        let discriminant = b * b - a * c; // 4 * a * c?

        if discriminant > 0_f32 {
            let mut temp = (-b - f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(IntersectionRecord::new(
                    temp,
                    hit_point,
                    (hit_point - self.geometry.center) / self.geometry.radius,
                    &self
                ));
            }
            temp = (-b + f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(IntersectionRecord::new(
                    temp,
                    hit_point,
                    (hit_point - self.geometry.center) / self.geometry.radius,
                    &self
                ));
            }
        }

        None
    }
}

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}


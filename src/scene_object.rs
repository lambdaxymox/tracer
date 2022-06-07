use crate::ray::Ray;
use crate::material::{
    Material, 
};
use cglinalg::{
    Vector3,
};
use rand::prelude::*;


pub trait Intersect {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<GeometryIntersectionResult>;
}

#[derive(Copy, Clone)]
pub struct IntersectionResult<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub object: &'a SceneObject,
}

impl<'a> IntersectionResult<'a> {
    pub fn new(t: f32, p: Vector3<f32>, normal: Vector3<f32>, object: &'a SceneObject) -> IntersectionResult<'a> {
        IntersectionResult {
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

    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectionResult> {
        self.geometry.intersect(ray, t_min, t_max).map(|res| IntersectionResult::new(
            res.t,
            res.p,
            res.normal,
            &self
        ))
    }

    pub fn sample_bsdf(&self, ray_in: Ray, hit: &IntersectionResult, rng: &mut ThreadRng) -> ScatteredRay {
        self.material.sample_bsdf(ray_in, hit, rng)
    }
}

#[derive(Copy, Clone)]
pub struct GeometryIntersectionResult {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl<'a> GeometryIntersectionResult {
    pub fn new(t: f32, p: Vector3<f32>, normal: Vector3<f32>) -> GeometryIntersectionResult {
        GeometryIntersectionResult {
            t: t,
            p: p,
            normal: normal,
        }
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

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<GeometryIntersectionResult> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c; // 4 * a * c?

        if discriminant > 0_f32 {
            let mut temp = (-b - f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(GeometryIntersectionResult::new(
                    temp,
                    hit_point,
                    (hit_point - self.center) / self.radius,
                ));
            }
            temp = (-b + f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(GeometryIntersectionResult::new(
                    temp,
                    hit_point,
                    (hit_point - self.center) / self.radius,
                ));
            }
        }

        None
    }
}


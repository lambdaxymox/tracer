use crate::ray::Ray;
use crate::material::{
    Material, 
    Intersect, 
    IntersectionRecord,
    ScatteredRay,
};
use cglinalg::{
    Vector3,
};
use rand::prelude::*;


pub struct SceneObject {
    geometry: Sphere,
    material: Material,
}

impl SceneObject {
    pub fn new(geometry: Sphere, material: Material) -> Self {
        Self { geometry, material }
    }

    pub fn sample_bsdf(&self, ray_in: Ray, hit: &IntersectionRecord, rng: &mut ThreadRng) -> ScatteredRay {
        self.material.scatter(ray_in, hit, rng)
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


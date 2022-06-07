use crate::ray::Ray;
use crate::material::{
    Material, 
    Intersect, 
    IntersectionRecord,
    Scatter,
};
use cglinalg::{
    Vector3,
};
use rand::prelude::*;


pub struct SceneObject {
    sphere: Sphere,
    material: Material,
}

impl SceneObject {
    pub fn new(sphere: Sphere, material: Material) -> Self {
        Self { sphere, material }
    }

    pub fn scatter(&self, ray_in: Ray, hit: &IntersectionRecord, rng: &mut ThreadRng) -> Scatter {
        self.material.scatter(ray_in, hit, rng)
    }
}

impl Intersect for SceneObject {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectionRecord> {
        let oc = ray.origin - self.sphere.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.sphere.radius * self.sphere.radius;
        let discriminant = b * b - a * c; // 4 * a * c?

        if discriminant > 0_f32 {
            let mut temp = (-b - f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(IntersectionRecord::new(
                    temp,
                    hit_point,
                    (hit_point - self.sphere.center) / self.sphere.radius,
                    &self
                ));
            }
            temp = (-b + f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(IntersectionRecord::new(
                    temp,
                    hit_point,
                    (hit_point - self.sphere.center) / self.sphere.radius,
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


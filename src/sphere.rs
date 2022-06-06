use crate::ray::Ray;
use crate::material::{
    Material, 
    Hitable, 
    HitRecord
};
use cglinalg::{
    Vector3,
};


pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Material) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c; // 4 * a * c?

        if discriminant > 0_f32 {
            let mut temp = (-b - f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(HitRecord::new(
                    temp,
                    hit_point,
                    (hit_point - self.center) / self.radius,
                    &self.material
                ));
            }
            temp = (-b + f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(HitRecord::new(
                    temp,
                    hit_point,
                    (hit_point - self.center) / self.radius,
                    &self.material
                ));
            }
        }

        None
    }
}

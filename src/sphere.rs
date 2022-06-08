use crate::geometry::*;
use crate::ray::{
    Ray,
};
use cglinalg::{
    Vector3,
};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
}

impl Sphere {
    /// Construct a new model space sphere.
    pub fn new(center: Vector3<f32>, radius: f32) -> Sphere {
        Sphere {
            center, radius,
        }
    }

    /// Returns the radius of the sphere in model space.
    #[inline]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Returns the diameter of the sphere in model space.
    #[inline]
    pub fn diameter(&self) -> f32 {
        self.radius + self.radius
    }
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<GeometryIntersectionResult> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c; // 4 * a * c?
        eprintln!("ray = {:?}; discriminant = {}", ray, discriminant);
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

    #[inline]
    fn center(&self) -> Vector3<f32> {
        self.center
    }
}


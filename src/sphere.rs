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

impl Intersection for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> IntersectionResult {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c; // 4 * a * c?
        if discriminant > 0_f32 {
            let t_intersect1 = (-b - f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if t_intersect1 > t_min && t_intersect1 < t_max {
                let point_of_intersection = ray.interpolate(t_intersect1);
                return IntersectionResult::new_hit(
                    t_intersect1,
                    point_of_intersection,
                    (point_of_intersection - self.center) / self.radius,
                );
            }

            let t_intersect2 = (-b + f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if t_intersect2 > t_min && t_intersect2 < t_max {
                let point_of_intersection = ray.interpolate(t_intersect2);
                return IntersectionResult::new_hit(
                    t_intersect2,
                    point_of_intersection,
                    (point_of_intersection - self.center) / self.radius,
                );
            }
        } 
        
        if discriminant == 0_f32 {
            let t_intersect1 = (-b - f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if t_intersect1 > t_min && t_intersect1 < t_max {
                let point_of_intersection = ray.interpolate(t_intersect1);
                return IntersectionResult::new_tangent(
                    t_intersect1,
                    point_of_intersection,
                    (point_of_intersection - self.center) / self.radius,
                );
            }

            let t_intersect2 = (-b + f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if t_intersect2 > t_min && t_intersect2 < t_max {
                let point_of_intersection = ray.interpolate(t_intersect2);
                return IntersectionResult::new_tangent(
                    t_intersect2,
                    point_of_intersection,
                    (point_of_intersection - self.center) / self.radius,
                );
            }
        }
            
        IntersectionResult::new_miss()
    }
}

impl Geometry for Sphere {
    #[inline]
    fn center(&self) -> Vector3<f32> {
        self.center
    }
}


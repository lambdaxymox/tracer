use cglinalg::{
    Vector3,
};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub t_min: f32,
    pub t_max: f32,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>, t_min: f32, t_max: f32) -> Ray {
        Ray {
            origin, direction, t_min, t_max
        }
    }

    pub fn interpolate(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }
}


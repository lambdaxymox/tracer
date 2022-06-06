use cglinalg::{
    Vector3,
};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }
}


use cglinalg::{
    Vector3,
};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Ray { origin, direction }
    }

    pub fn interpolate(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IntersectionQuery {
    pub ray: Ray,
    pub t_min: f32,
    pub t_max: f32,
}

impl IntersectionQuery {
    pub fn new(ray: Ray, t_min: f32, t_max: f32) -> Self {
        Self { ray, t_min, t_max }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScatteringResult {
    pub scattering_fraction: Vector3<f32>,
    pub ray: Ray,
}

impl ScatteringResult {
    pub fn new(scattering_fraction: Vector3<f32>, ray: Ray) -> ScatteringResult {
        ScatteringResult { 
            scattering_fraction, ray,
        }
    }
}


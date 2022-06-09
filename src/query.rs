use crate::ray::*;


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


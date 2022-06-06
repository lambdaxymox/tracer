use crate::ray::{
    Ray,
};
use crate::sample;
use cglinalg::{
    Magnitude, 
    Vector3
};
use rand::prelude::*;


#[derive(Copy, Clone)]
pub struct Camera {
    eye: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vector3<f32>,
        look_at: Vector3<f32>,
        v_up: Vector3<f32>,
        v_fov: f32,
        aspect: f32, aperture: f32, focus_dist: f32) -> Camera {

        let lens_radius = 0.5 * aperture;
        let theta = v_fov * std::f32::consts::PI / 180.0;
        let half_height = (0.5 * theta).tan();
        let half_width = aspect * half_height;

        let eye = look_from;

        let w = (look_from - look_at).normalize();
        let u = v_up.cross(&w).normalize();
        let v = w.cross(&u);

        let lower_left_corner =
            eye - u * (half_width * focus_dist) - v * (half_height * focus_dist) - w * focus_dist;
        let horizontal = u * (2.0 * half_width * focus_dist);
        let vertical = v * (2.0 * half_height * focus_dist);

        Camera {
            eye,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, rng: &mut ThreadRng, u: f32, v: f32) -> Ray {
        let rd = sample::random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let lens_position = self.eye + offset;
        Ray::new(
            lens_position,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - lens_position,
        )
    }
}


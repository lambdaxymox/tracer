use crate::query::*;
use crate::sampler::*;
use cglinalg::{
    Magnitude, 
    Vector3
};


#[derive(Copy, Clone)]
pub struct Camera {
    eye: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    lens_radius: f32,
    forward: Vector3<f32>,
}

impl Camera {
    pub fn new(
        look_from: Vector3<f32>,
        look_at: Vector3<f32>,
        v_up: Vector3<f32>,
        v_fov: f32,
        aspect: f32, 
        aperture: f32, 
        focus_dist: f32) -> Camera
    {
        let lens_radius = 0.5 * aperture;
        let theta = v_fov * std::f32::consts::PI / 180.0;
        let half_height = (0.5 * theta).tan();
        let half_width = aspect * half_height;

        let eye = look_from;

        let forward = (look_from - look_at).normalize();
        let u = v_up.cross(&forward).normalize();
        let v = forward.cross(&u);

        let lower_left_corner = eye - 
            u * (half_width * focus_dist) - 
            v * (half_height * focus_dist) - 
            forward * focus_dist;
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
            forward,
        }
    }

    pub fn cast_ray(&self, sampler: &mut SphereSampler, u: f32, v: f32) -> Ray {
        // TODO: Cast a ray in eye space, and convert is back to world space?
        // That is, all the aspects of the camera construction, namely, lens position, lower left corner, horizontal, vertical,
        // forward axis, vertical axis, horizontal axis, Take place in eye space. When we cast a ray through the camera through its viewport
        // we are getting a eye space ray that must be converted back to world space via its viewing matrix. This way,
        // the camera parameters are decoupled from the specifics of the scene and we have a more consistent way of querying
        // through the eyes of the camera.
        // TODO: Make Rays have a coordinate space. A ray in eye space, a ray in model space, and a ray in world space are different things.
        // How do we convert between them?
        //
        // Ray<EyeSpace> -> Ray<WorldSpace>
        let rd = sampler.sample_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let lens_position = self.eye + offset;
        
        // TODO: Camera ray range?
        Ray::new(
            lens_position,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - lens_position,
        )
    }

    #[inline]
    pub fn position(&self) -> Vector3<f32> {
        self.eye
    }

    #[inline]
    pub fn forward(&self) -> Vector3<f32> {
        self.forward
    }
}

